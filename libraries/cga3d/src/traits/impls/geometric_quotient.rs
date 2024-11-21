// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 625
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       1       1
//   Median:         3       4       1
//  Average:         4       5       1
//  Maximum:        23      27       1
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       1       1
//   Median:         3      10       1
//  Average:         4      11       1
//  Maximum:        23      48       1
impl std::ops::Div<geometric_quotient> for AntiCircleRotor {
    type Output = geometric_quotient_partial<AntiCircleRotor>;
    fn div(self, _rhs: geometric_quotient) -> Self::Output {
        geometric_quotient_partial(self)
    }
}
impl GeometricQuotient<AntiCircleRotor> for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        6        1
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        7        9        1
    //  no simd        7       17        1
    fn geometric_quotient(self, other: AntiCircleRotor) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e45], 2) + f32::powi(other[scalar], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - 2.0 * (other[e41] * other[e15])
                - 2.0 * (other[e42] * other[e25])
                - 2.0 * (other[e43] * other[e35]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, scalar
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[scalar]]),
        );
    }
}
impl GeometricQuotient<AntiDipoleInversion> for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10        8        1
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       10       11        1
    //  no simd       10       19        1
    fn geometric_quotient(self, other: AntiDipoleInversion) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                + f32::powi(other[e1], 2)
                + f32::powi(other[e2], 2)
                + f32::powi(other[e3], 2)
                - f32::powi(other[e321], 2)
                - 2.0 * (other[e4] * other[e5]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, scalar
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[scalar]]),
        );
    }
}
impl GeometricQuotient<AntiDualNum> for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        1
    //  no simd        0       11        1
    fn geometric_quotient(self, other: AntiDualNum) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, scalar
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[scalar]]),
        );
    }
}
impl GeometricQuotient<AntiFlatPoint> for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        4        1
    //  no simd        0       12        1
    fn geometric_quotient(self, other: AntiFlatPoint) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e321], 2) * -1.0);
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, scalar
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[scalar]]),
        );
    }
}
impl GeometricQuotient<AntiFlector> for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        3        1
    //  no simd        3       11        1
    fn geometric_quotient(self, other: AntiFlector) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2) - f32::powi(other[e321], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, scalar
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[scalar]]),
        );
    }
}
impl GeometricQuotient<AntiLine> for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        2        3        1
    //  no simd        2       11        1
    fn geometric_quotient(self, other: AntiLine) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ -f32::powi(other[e23], 2) - f32::powi(other[e31], 2) - f32::powi(other[e12], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, scalar
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[scalar]]),
        );
    }
}
impl GeometricQuotient<AntiMotor> for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        3        1
    //  no simd        3       11        1
    fn geometric_quotient(self, other: AntiMotor) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[scalar], 2) - f32::powi(other[e23], 2) - f32::powi(other[e31], 2) - f32::powi(other[e12], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, scalar
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[scalar]]),
        );
    }
}
impl GeometricQuotient<AntiPlane> for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        2        3        1
    //  no simd        2       11        1
    fn geometric_quotient(self, other: AntiPlane) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, scalar
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[scalar]]),
        );
    }
}
impl GeometricQuotient<AntiScalar> for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        4        1
    //  no simd        0       12        1
    fn geometric_quotient(self, other: AntiScalar) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e12345], 2) * -1.0);
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, scalar
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[scalar]]),
        );
    }
}
impl GeometricQuotient<Circle> for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        6        1
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        6        9        1
    //  no simd        6       17        1
    fn geometric_quotient(self, other: Circle) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                - f32::powi(other[e321], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, scalar
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[scalar]]),
        );
    }
}
impl GeometricQuotient<CircleRotor> for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        6        1
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        7        9        1
    //  no simd        7       17        1
    fn geometric_quotient(self, other: CircleRotor) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                - f32::powi(other[e321], 2)
                - f32::powi(other[e12345], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, scalar
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[scalar]]),
        );
    }
}
impl GeometricQuotient<Dipole> for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        6        1
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        6        9        1
    //  no simd        6       17        1
    fn geometric_quotient(self, other: Dipole) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e45], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - 2.0 * (other[e41] * other[e15])
                - 2.0 * (other[e42] * other[e25])
                - 2.0 * (other[e43] * other[e35]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, scalar
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[scalar]]),
        );
    }
}
impl GeometricQuotient<DipoleInversion> for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10        8        1
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       10       11        1
    //  no simd       10       19        1
    fn geometric_quotient(self, other: DipoleInversion) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e1234] * other[e3215]) + f32::powi(other[e45], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - f32::powi(other[e4235], 2)
                - f32::powi(other[e4315], 2)
                - f32::powi(other[e4125], 2)
                - 2.0 * (other[e41] * other[e15])
                - 2.0 * (other[e42] * other[e25])
                - 2.0 * (other[e43] * other[e35]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, scalar
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[scalar]]),
        );
    }
}
impl GeometricQuotient<DualNum> for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        4        1
    //  no simd        0       12        1
    fn geometric_quotient(self, other: DualNum) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e12345], 2) * -1.0);
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, scalar
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[scalar]]),
        );
    }
}
impl GeometricQuotient<FlatPoint> for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        1
    //  no simd        0       11        1
    fn geometric_quotient(self, other: FlatPoint) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e45], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, scalar
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[scalar]]),
        );
    }
}
impl GeometricQuotient<Flector> for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        3        1
    //  no simd        3       11        1
    fn geometric_quotient(self, other: Flector) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e45], 2) - f32::powi(other[e4235], 2) - f32::powi(other[e4315], 2) - f32::powi(other[e4125], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, scalar
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[scalar]]),
        );
    }
}
impl GeometricQuotient<Line> for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        2        3        1
    //  no simd        2       11        1
    fn geometric_quotient(self, other: Line) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e415], 2) + f32::powi(other[e425], 2) + f32::powi(other[e435], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, scalar
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[scalar]]),
        );
    }
}
impl GeometricQuotient<Motor> for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        3        1
    //  no simd        3       11        1
    fn geometric_quotient(self, other: Motor) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e415], 2) + f32::powi(other[e425], 2) + f32::powi(other[e435], 2) - f32::powi(other[e12345], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, scalar
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[scalar]]),
        );
    }
}
impl GeometricQuotient<MultiVector> for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       23       16        1
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       23       19        1
    //  no simd       23       27        1
    fn geometric_quotient(self, other: MultiVector) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + 2.0 * (other[e3215] * other[e1234])
                + f32::powi(other[scalar], 2)
                + f32::powi(other[e1], 2)
                + f32::powi(other[e2], 2)
                + f32::powi(other[e3], 2)
                + f32::powi(other[e45], 2)
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                - f32::powi(other[e12345], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - f32::powi(other[e321], 2)
                - f32::powi(other[e4235], 2)
                - f32::powi(other[e4315], 2)
                - f32::powi(other[e4125], 2)
                - 2.0 * (other[e4] * other[e5])
                - 2.0 * (other[e15] * other[e41])
                - 2.0 * (other[e25] * other[e42])
                - 2.0 * (other[e35] * other[e43]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, scalar
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[scalar]]),
        );
    }
}
impl GeometricQuotient<Plane> for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        2        3        1
    //  no simd        2       11        1
    fn geometric_quotient(self, other: Plane) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ -f32::powi(other[e4235], 2) - f32::powi(other[e4315], 2) - f32::powi(other[e4125], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, scalar
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[scalar]]),
        );
    }
}
impl GeometricQuotient<RoundPoint> for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        2        1
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        5        1
    //  no simd        3       13        1
    fn geometric_quotient(self, other: RoundPoint) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2) - 2.0 * (other[e4] * other[e5]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, scalar
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[scalar]]),
        );
    }
}
impl GeometricQuotient<Scalar> for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        1
    //  no simd        0       11        1
    fn geometric_quotient(self, other: Scalar) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, scalar
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[scalar]]),
        );
    }
}
impl GeometricQuotient<Sphere> for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        2        1
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        5        1
    //  no simd        3       13        1
    fn geometric_quotient(self, other: Sphere) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e3215] * other[e1234]) - f32::powi(other[e4235], 2) - f32::powi(other[e4315], 2) - f32::powi(other[e4125], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, scalar
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[scalar]]),
        );
    }
}
impl GeometricQuotient<VersorEven> for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11        8        1
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       11       11        1
    //  no simd       11       19        1
    fn geometric_quotient(self, other: VersorEven) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                + f32::powi(other[e1], 2)
                + f32::powi(other[e2], 2)
                + f32::powi(other[e3], 2)
                - f32::powi(other[e12345], 2)
                - f32::powi(other[e321], 2)
                - 2.0 * (other[e5] * other[e4]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, scalar
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[scalar]]),
        );
    }
}
impl GeometricQuotient<VersorOdd> for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11        8        1
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       11       11        1
    //  no simd       11       19        1
    fn geometric_quotient(self, other: VersorOdd) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e1234] * other[e3215]) + f32::powi(other[scalar], 2) + f32::powi(other[e45], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - f32::powi(other[e4235], 2)
                - f32::powi(other[e4315], 2)
                - f32::powi(other[e4125], 2)
                - 2.0 * (other[e41] * other[e15])
                - 2.0 * (other[e42] * other[e25])
                - 2.0 * (other[e43] * other[e35]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, scalar
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[scalar]]),
        );
    }
}
impl std::ops::Div<geometric_quotient> for AntiDipoleInversion {
    type Output = geometric_quotient_partial<AntiDipoleInversion>;
    fn div(self, _rhs: geometric_quotient) -> Self::Output {
        geometric_quotient_partial(self)
    }
}
impl GeometricQuotient<AntiCircleRotor> for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        6        1
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        7       10        1
    //  no simd        7       21        1
    fn geometric_quotient(self, other: AntiCircleRotor) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e45], 2) + f32::powi(other[scalar], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - 2.0 * (other[e41] * other[e15])
                - 2.0 * (other[e42] * other[e25])
                - 2.0 * (other[e43] * other[e35]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e4
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e4]]),
            // e1, e2, e3, e5
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricQuotient<AntiDipoleInversion> for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10        8        1
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd       10       12        1
    //  no simd       10       23        1
    fn geometric_quotient(self, other: AntiDipoleInversion) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                + f32::powi(other[e1], 2)
                + f32::powi(other[e2], 2)
                + f32::powi(other[e3], 2)
                - f32::powi(other[e321], 2)
                - 2.0 * (other[e4] * other[e5]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e4
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e4]]),
            // e1, e2, e3, e5
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricQuotient<AntiDualNum> for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        4        1
    //  no simd        0       15        1
    fn geometric_quotient(self, other: AntiDualNum) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e4
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e4]]),
            // e1, e2, e3, e5
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricQuotient<AntiFlatPoint> for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        5        1
    //  no simd        0       16        1
    fn geometric_quotient(self, other: AntiFlatPoint) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e321], 2) * -1.0);
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e4
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e4]]),
            // e1, e2, e3, e5
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricQuotient<AntiFlector> for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        3        4        1
    //  no simd        3       15        1
    fn geometric_quotient(self, other: AntiFlector) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2) - f32::powi(other[e321], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e4
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e4]]),
            // e1, e2, e3, e5
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricQuotient<AntiLine> for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        2        4        1
    //  no simd        2       15        1
    fn geometric_quotient(self, other: AntiLine) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ -f32::powi(other[e23], 2) - f32::powi(other[e31], 2) - f32::powi(other[e12], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e4
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e4]]),
            // e1, e2, e3, e5
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricQuotient<AntiMotor> for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        3        4        1
    //  no simd        3       15        1
    fn geometric_quotient(self, other: AntiMotor) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[scalar], 2) - f32::powi(other[e23], 2) - f32::powi(other[e31], 2) - f32::powi(other[e12], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e4
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e4]]),
            // e1, e2, e3, e5
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricQuotient<AntiPlane> for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        2        4        1
    //  no simd        2       15        1
    fn geometric_quotient(self, other: AntiPlane) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e4
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e4]]),
            // e1, e2, e3, e5
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricQuotient<AntiScalar> for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        5        1
    //  no simd        0       16        1
    fn geometric_quotient(self, other: AntiScalar) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e12345], 2) * -1.0);
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e4
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e4]]),
            // e1, e2, e3, e5
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricQuotient<Circle> for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        6        1
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        6       10        1
    //  no simd        6       21        1
    fn geometric_quotient(self, other: Circle) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                - f32::powi(other[e321], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e4
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e4]]),
            // e1, e2, e3, e5
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricQuotient<CircleRotor> for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        6        1
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        7       10        1
    //  no simd        7       21        1
    fn geometric_quotient(self, other: CircleRotor) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                - f32::powi(other[e321], 2)
                - f32::powi(other[e12345], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e4
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e4]]),
            // e1, e2, e3, e5
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricQuotient<Dipole> for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        6        1
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        6       10        1
    //  no simd        6       21        1
    fn geometric_quotient(self, other: Dipole) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e45], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - 2.0 * (other[e41] * other[e15])
                - 2.0 * (other[e42] * other[e25])
                - 2.0 * (other[e43] * other[e35]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e4
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e4]]),
            // e1, e2, e3, e5
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricQuotient<DipoleInversion> for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10        8        1
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd       10       12        1
    //  no simd       10       23        1
    fn geometric_quotient(self, other: DipoleInversion) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e1234] * other[e3215]) + f32::powi(other[e45], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - f32::powi(other[e4235], 2)
                - f32::powi(other[e4315], 2)
                - f32::powi(other[e4125], 2)
                - 2.0 * (other[e41] * other[e15])
                - 2.0 * (other[e42] * other[e25])
                - 2.0 * (other[e43] * other[e35]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e4
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e4]]),
            // e1, e2, e3, e5
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricQuotient<DualNum> for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        5        1
    //  no simd        0       16        1
    fn geometric_quotient(self, other: DualNum) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e12345], 2) * -1.0);
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e4
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e4]]),
            // e1, e2, e3, e5
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricQuotient<FlatPoint> for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        4        1
    //  no simd        0       15        1
    fn geometric_quotient(self, other: FlatPoint) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e45], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e4
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e4]]),
            // e1, e2, e3, e5
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricQuotient<Flector> for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        3        4        1
    //  no simd        3       15        1
    fn geometric_quotient(self, other: Flector) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e45], 2) - f32::powi(other[e4235], 2) - f32::powi(other[e4315], 2) - f32::powi(other[e4125], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e4
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e4]]),
            // e1, e2, e3, e5
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricQuotient<Line> for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        2        4        1
    //  no simd        2       15        1
    fn geometric_quotient(self, other: Line) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e415], 2) + f32::powi(other[e425], 2) + f32::powi(other[e435], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e4
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e4]]),
            // e1, e2, e3, e5
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricQuotient<Motor> for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        3        4        1
    //  no simd        3       15        1
    fn geometric_quotient(self, other: Motor) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e415], 2) + f32::powi(other[e425], 2) + f32::powi(other[e435], 2) - f32::powi(other[e12345], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e4
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e4]]),
            // e1, e2, e3, e5
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricQuotient<MultiVector> for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       23       16        1
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd       23       20        1
    //  no simd       23       31        1
    fn geometric_quotient(self, other: MultiVector) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + 2.0 * (other[e3215] * other[e1234])
                + f32::powi(other[scalar], 2)
                + f32::powi(other[e1], 2)
                + f32::powi(other[e2], 2)
                + f32::powi(other[e3], 2)
                + f32::powi(other[e45], 2)
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                - f32::powi(other[e12345], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - f32::powi(other[e321], 2)
                - f32::powi(other[e4235], 2)
                - f32::powi(other[e4315], 2)
                - f32::powi(other[e4125], 2)
                - 2.0 * (other[e4] * other[e5])
                - 2.0 * (other[e15] * other[e41])
                - 2.0 * (other[e25] * other[e42])
                - 2.0 * (other[e35] * other[e43]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e4
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e4]]),
            // e1, e2, e3, e5
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricQuotient<Plane> for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        2        4        1
    //  no simd        2       15        1
    fn geometric_quotient(self, other: Plane) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ -f32::powi(other[e4235], 2) - f32::powi(other[e4315], 2) - f32::powi(other[e4125], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e4
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e4]]),
            // e1, e2, e3, e5
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricQuotient<RoundPoint> for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        2        1
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        3        6        1
    //  no simd        3       17        1
    fn geometric_quotient(self, other: RoundPoint) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2) - 2.0 * (other[e4] * other[e5]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e4
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e4]]),
            // e1, e2, e3, e5
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricQuotient<Scalar> for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        4        1
    //  no simd        0       15        1
    fn geometric_quotient(self, other: Scalar) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e4
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e4]]),
            // e1, e2, e3, e5
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricQuotient<Sphere> for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        2        1
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        3        6        1
    //  no simd        3       17        1
    fn geometric_quotient(self, other: Sphere) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e3215] * other[e1234]) - f32::powi(other[e4235], 2) - f32::powi(other[e4315], 2) - f32::powi(other[e4125], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e4
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e4]]),
            // e1, e2, e3, e5
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricQuotient<VersorEven> for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11        8        1
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd       11       12        1
    //  no simd       11       23        1
    fn geometric_quotient(self, other: VersorEven) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                + f32::powi(other[e1], 2)
                + f32::powi(other[e2], 2)
                + f32::powi(other[e3], 2)
                - f32::powi(other[e12345], 2)
                - f32::powi(other[e321], 2)
                - 2.0 * (other[e5] * other[e4]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e4
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e4]]),
            // e1, e2, e3, e5
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricQuotient<VersorOdd> for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11        8        1
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd       11       12        1
    //  no simd       11       23        1
    fn geometric_quotient(self, other: VersorOdd) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e1234] * other[e3215]) + f32::powi(other[scalar], 2) + f32::powi(other[e45], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - f32::powi(other[e4235], 2)
                - f32::powi(other[e4315], 2)
                - f32::powi(other[e4125], 2)
                - 2.0 * (other[e41] * other[e15])
                - 2.0 * (other[e42] * other[e25])
                - 2.0 * (other[e43] * other[e35]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e4
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e4]]),
            // e1, e2, e3, e5
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl std::ops::Div<geometric_quotient> for AntiDualNum {
    type Output = geometric_quotient_partial<AntiDualNum>;
    fn div(self, _rhs: geometric_quotient) -> Self::Output {
        geometric_quotient_partial(self)
    }
}
impl GeometricQuotient<AntiCircleRotor> for AntiDualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        6        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        7        7        1
    //  no simd        7        8        1
    fn geometric_quotient(self, other: AntiCircleRotor) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e45], 2) + f32::powi(other[scalar], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - 2.0 * (other[e41] * other[e15])
                - 2.0 * (other[e42] * other[e25])
                - 2.0 * (other[e43] * other[e35]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from(inverse[scalar]) * Simd32x2::from([self[e3215], self[scalar]]));
    }
}
impl GeometricQuotient<AntiDipoleInversion> for AntiDualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10        8        1
    //    simd2        0        1        0
    // Totals...
    // yes simd       10        9        1
    //  no simd       10       10        1
    fn geometric_quotient(self, other: AntiDipoleInversion) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                + f32::powi(other[e1], 2)
                + f32::powi(other[e2], 2)
                + f32::powi(other[e3], 2)
                - f32::powi(other[e321], 2)
                - 2.0 * (other[e4] * other[e5]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from(inverse[scalar]) * Simd32x2::from([self[e3215], self[scalar]]));
    }
}
impl GeometricQuotient<AntiDualNum> for AntiDualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        1        1
    //  no simd        0        2        1
    fn geometric_quotient(self, other: AntiDualNum) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from(inverse[scalar]) * Simd32x2::from([self[e3215], self[scalar]]));
    }
}
impl GeometricQuotient<AntiFlatPoint> for AntiDualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        3        1
    fn geometric_quotient(self, other: AntiFlatPoint) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e321], 2) * -1.0);
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from(inverse[scalar]) * Simd32x2::from([self[e3215], self[scalar]]));
    }
}
impl GeometricQuotient<AntiFlector> for AntiDualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        3        1        1
    //  no simd        3        2        1
    fn geometric_quotient(self, other: AntiFlector) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2) - f32::powi(other[e321], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from(inverse[scalar]) * Simd32x2::from([self[e3215], self[scalar]]));
    }
}
impl GeometricQuotient<AntiLine> for AntiDualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        2        1        1
    //  no simd        2        2        1
    fn geometric_quotient(self, other: AntiLine) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ -f32::powi(other[e23], 2) - f32::powi(other[e31], 2) - f32::powi(other[e12], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from(inverse[scalar]) * Simd32x2::from([self[e3215], self[scalar]]));
    }
}
impl GeometricQuotient<AntiMotor> for AntiDualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        3        1        1
    //  no simd        3        2        1
    fn geometric_quotient(self, other: AntiMotor) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[scalar], 2) - f32::powi(other[e23], 2) - f32::powi(other[e31], 2) - f32::powi(other[e12], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from(inverse[scalar]) * Simd32x2::from([self[e3215], self[scalar]]));
    }
}
impl GeometricQuotient<AntiPlane> for AntiDualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        2        1        1
    //  no simd        2        2        1
    fn geometric_quotient(self, other: AntiPlane) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from(inverse[scalar]) * Simd32x2::from([self[e3215], self[scalar]]));
    }
}
impl GeometricQuotient<AntiScalar> for AntiDualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        3        1
    fn geometric_quotient(self, other: AntiScalar) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e12345], 2) * -1.0);
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from(inverse[scalar]) * Simd32x2::from([self[e3215], self[scalar]]));
    }
}
impl GeometricQuotient<Circle> for AntiDualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        6        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        6        7        1
    //  no simd        6        8        1
    fn geometric_quotient(self, other: Circle) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                - f32::powi(other[e321], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from(inverse[scalar]) * Simd32x2::from([self[e3215], self[scalar]]));
    }
}
impl GeometricQuotient<CircleRotor> for AntiDualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        6        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        7        7        1
    //  no simd        7        8        1
    fn geometric_quotient(self, other: CircleRotor) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                - f32::powi(other[e321], 2)
                - f32::powi(other[e12345], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from(inverse[scalar]) * Simd32x2::from([self[e3215], self[scalar]]));
    }
}
impl GeometricQuotient<Dipole> for AntiDualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        6        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        6        7        1
    //  no simd        6        8        1
    fn geometric_quotient(self, other: Dipole) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e45], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - 2.0 * (other[e41] * other[e15])
                - 2.0 * (other[e42] * other[e25])
                - 2.0 * (other[e43] * other[e35]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from(inverse[scalar]) * Simd32x2::from([self[e3215], self[scalar]]));
    }
}
impl GeometricQuotient<DipoleInversion> for AntiDualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10        8        1
    //    simd2        0        1        0
    // Totals...
    // yes simd       10        9        1
    //  no simd       10       10        1
    fn geometric_quotient(self, other: DipoleInversion) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e1234] * other[e3215]) + f32::powi(other[e45], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - f32::powi(other[e4235], 2)
                - f32::powi(other[e4315], 2)
                - f32::powi(other[e4125], 2)
                - 2.0 * (other[e41] * other[e15])
                - 2.0 * (other[e42] * other[e25])
                - 2.0 * (other[e43] * other[e35]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from(inverse[scalar]) * Simd32x2::from([self[e3215], self[scalar]]));
    }
}
impl GeometricQuotient<DualNum> for AntiDualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        3        1
    fn geometric_quotient(self, other: DualNum) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e12345], 2) * -1.0);
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from(inverse[scalar]) * Simd32x2::from([self[e3215], self[scalar]]));
    }
}
impl GeometricQuotient<FlatPoint> for AntiDualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        1        1
    //  no simd        0        2        1
    fn geometric_quotient(self, other: FlatPoint) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e45], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from(inverse[scalar]) * Simd32x2::from([self[e3215], self[scalar]]));
    }
}
impl GeometricQuotient<Flector> for AntiDualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        3        1        1
    //  no simd        3        2        1
    fn geometric_quotient(self, other: Flector) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e45], 2) - f32::powi(other[e4235], 2) - f32::powi(other[e4315], 2) - f32::powi(other[e4125], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from(inverse[scalar]) * Simd32x2::from([self[e3215], self[scalar]]));
    }
}
impl GeometricQuotient<Line> for AntiDualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        2        1        1
    //  no simd        2        2        1
    fn geometric_quotient(self, other: Line) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e415], 2) + f32::powi(other[e425], 2) + f32::powi(other[e435], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from(inverse[scalar]) * Simd32x2::from([self[e3215], self[scalar]]));
    }
}
impl GeometricQuotient<Motor> for AntiDualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        3        1        1
    //  no simd        3        2        1
    fn geometric_quotient(self, other: Motor) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e415], 2) + f32::powi(other[e425], 2) + f32::powi(other[e435], 2) - f32::powi(other[e12345], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from(inverse[scalar]) * Simd32x2::from([self[e3215], self[scalar]]));
    }
}
impl GeometricQuotient<MultiVector> for AntiDualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       23       16        1
    //    simd2        0        1        0
    // Totals...
    // yes simd       23       17        1
    //  no simd       23       18        1
    fn geometric_quotient(self, other: MultiVector) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + 2.0 * (other[e3215] * other[e1234])
                + f32::powi(other[scalar], 2)
                + f32::powi(other[e1], 2)
                + f32::powi(other[e2], 2)
                + f32::powi(other[e3], 2)
                + f32::powi(other[e45], 2)
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                - f32::powi(other[e12345], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - f32::powi(other[e321], 2)
                - f32::powi(other[e4235], 2)
                - f32::powi(other[e4315], 2)
                - f32::powi(other[e4125], 2)
                - 2.0 * (other[e4] * other[e5])
                - 2.0 * (other[e15] * other[e41])
                - 2.0 * (other[e25] * other[e42])
                - 2.0 * (other[e35] * other[e43]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from(inverse[scalar]) * Simd32x2::from([self[e3215], self[scalar]]));
    }
}
impl GeometricQuotient<Plane> for AntiDualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        2        1        1
    //  no simd        2        2        1
    fn geometric_quotient(self, other: Plane) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ -f32::powi(other[e4235], 2) - f32::powi(other[e4315], 2) - f32::powi(other[e4125], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from(inverse[scalar]) * Simd32x2::from([self[e3215], self[scalar]]));
    }
}
impl GeometricQuotient<RoundPoint> for AntiDualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        2        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        3        3        1
    //  no simd        3        4        1
    fn geometric_quotient(self, other: RoundPoint) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2) - 2.0 * (other[e4] * other[e5]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from(inverse[scalar]) * Simd32x2::from([self[e3215], self[scalar]]));
    }
}
impl GeometricQuotient<Scalar> for AntiDualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        1        1
    //  no simd        0        2        1
    fn geometric_quotient(self, other: Scalar) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from(inverse[scalar]) * Simd32x2::from([self[e3215], self[scalar]]));
    }
}
impl GeometricQuotient<Sphere> for AntiDualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        2        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        3        3        1
    //  no simd        3        4        1
    fn geometric_quotient(self, other: Sphere) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e3215] * other[e1234]) - f32::powi(other[e4235], 2) - f32::powi(other[e4315], 2) - f32::powi(other[e4125], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from(inverse[scalar]) * Simd32x2::from([self[e3215], self[scalar]]));
    }
}
impl GeometricQuotient<VersorEven> for AntiDualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11        8        1
    //    simd2        0        1        0
    // Totals...
    // yes simd       11        9        1
    //  no simd       11       10        1
    fn geometric_quotient(self, other: VersorEven) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                + f32::powi(other[e1], 2)
                + f32::powi(other[e2], 2)
                + f32::powi(other[e3], 2)
                - f32::powi(other[e12345], 2)
                - f32::powi(other[e321], 2)
                - 2.0 * (other[e5] * other[e4]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from(inverse[scalar]) * Simd32x2::from([self[e3215], self[scalar]]));
    }
}
impl GeometricQuotient<VersorOdd> for AntiDualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11        8        1
    //    simd2        0        1        0
    // Totals...
    // yes simd       11        9        1
    //  no simd       11       10        1
    fn geometric_quotient(self, other: VersorOdd) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e1234] * other[e3215]) + f32::powi(other[scalar], 2) + f32::powi(other[e45], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - f32::powi(other[e4235], 2)
                - f32::powi(other[e4315], 2)
                - f32::powi(other[e4125], 2)
                - 2.0 * (other[e41] * other[e15])
                - 2.0 * (other[e42] * other[e25])
                - 2.0 * (other[e43] * other[e35]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from(inverse[scalar]) * Simd32x2::from([self[e3215], self[scalar]]));
    }
}
impl std::ops::Div<geometric_quotient> for AntiFlatPoint {
    type Output = geometric_quotient_partial<AntiFlatPoint>;
    fn div(self, _rhs: geometric_quotient) -> Self::Output {
        geometric_quotient_partial(self)
    }
}
impl GeometricQuotient<AntiCircleRotor> for AntiFlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        6        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        7        7        1
    //  no simd        7       10        1
    fn geometric_quotient(self, other: AntiCircleRotor) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e45], 2) + f32::powi(other[scalar], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - 2.0 * (other[e41] * other[e15])
                - 2.0 * (other[e42] * other[e25])
                - 2.0 * (other[e43] * other[e35]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiFlatPoint::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e321]]),
        );
    }
}
impl GeometricQuotient<AntiDipoleInversion> for AntiFlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10        8        1
    //    simd4        0        1        0
    // Totals...
    // yes simd       10        9        1
    //  no simd       10       12        1
    fn geometric_quotient(self, other: AntiDipoleInversion) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                + f32::powi(other[e1], 2)
                + f32::powi(other[e2], 2)
                + f32::powi(other[e3], 2)
                - f32::powi(other[e321], 2)
                - 2.0 * (other[e4] * other[e5]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiFlatPoint::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e321]]),
        );
    }
}
impl GeometricQuotient<AntiDualNum> for AntiFlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        1        1
    //  no simd        0        4        1
    fn geometric_quotient(self, other: AntiDualNum) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiFlatPoint::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e321]]),
        );
    }
}
impl GeometricQuotient<AntiFlatPoint> for AntiFlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        5        1
    fn geometric_quotient(self, other: AntiFlatPoint) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e321], 2) * -1.0);
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiFlatPoint::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e321]]),
        );
    }
}
impl GeometricQuotient<AntiFlector> for AntiFlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        1        1
    //  no simd        3        4        1
    fn geometric_quotient(self, other: AntiFlector) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2) - f32::powi(other[e321], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiFlatPoint::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e321]]),
        );
    }
}
impl GeometricQuotient<AntiLine> for AntiFlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        1        1
    //  no simd        2        4        1
    fn geometric_quotient(self, other: AntiLine) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ -f32::powi(other[e23], 2) - f32::powi(other[e31], 2) - f32::powi(other[e12], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiFlatPoint::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e321]]),
        );
    }
}
impl GeometricQuotient<AntiMotor> for AntiFlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        1        1
    //  no simd        3        4        1
    fn geometric_quotient(self, other: AntiMotor) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[scalar], 2) - f32::powi(other[e23], 2) - f32::powi(other[e31], 2) - f32::powi(other[e12], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiFlatPoint::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e321]]),
        );
    }
}
impl GeometricQuotient<AntiPlane> for AntiFlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        1        1
    //  no simd        2        4        1
    fn geometric_quotient(self, other: AntiPlane) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiFlatPoint::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e321]]),
        );
    }
}
impl GeometricQuotient<AntiScalar> for AntiFlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        5        1
    fn geometric_quotient(self, other: AntiScalar) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e12345], 2) * -1.0);
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiFlatPoint::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e321]]),
        );
    }
}
impl GeometricQuotient<Circle> for AntiFlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        6        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        6        7        1
    //  no simd        6       10        1
    fn geometric_quotient(self, other: Circle) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                - f32::powi(other[e321], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiFlatPoint::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e321]]),
        );
    }
}
impl GeometricQuotient<CircleRotor> for AntiFlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        6        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        7        7        1
    //  no simd        7       10        1
    fn geometric_quotient(self, other: CircleRotor) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                - f32::powi(other[e321], 2)
                - f32::powi(other[e12345], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiFlatPoint::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e321]]),
        );
    }
}
impl GeometricQuotient<Dipole> for AntiFlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        6        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        6        7        1
    //  no simd        6       10        1
    fn geometric_quotient(self, other: Dipole) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e45], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - 2.0 * (other[e41] * other[e15])
                - 2.0 * (other[e42] * other[e25])
                - 2.0 * (other[e43] * other[e35]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiFlatPoint::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e321]]),
        );
    }
}
impl GeometricQuotient<DipoleInversion> for AntiFlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10        8        1
    //    simd4        0        1        0
    // Totals...
    // yes simd       10        9        1
    //  no simd       10       12        1
    fn geometric_quotient(self, other: DipoleInversion) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e1234] * other[e3215]) + f32::powi(other[e45], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - f32::powi(other[e4235], 2)
                - f32::powi(other[e4315], 2)
                - f32::powi(other[e4125], 2)
                - 2.0 * (other[e41] * other[e15])
                - 2.0 * (other[e42] * other[e25])
                - 2.0 * (other[e43] * other[e35]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiFlatPoint::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e321]]),
        );
    }
}
impl GeometricQuotient<DualNum> for AntiFlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        5        1
    fn geometric_quotient(self, other: DualNum) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e12345], 2) * -1.0);
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiFlatPoint::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e321]]),
        );
    }
}
impl GeometricQuotient<FlatPoint> for AntiFlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        1        1
    //  no simd        0        4        1
    fn geometric_quotient(self, other: FlatPoint) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e45], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiFlatPoint::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e321]]),
        );
    }
}
impl GeometricQuotient<Flector> for AntiFlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        1        1
    //  no simd        3        4        1
    fn geometric_quotient(self, other: Flector) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e45], 2) - f32::powi(other[e4235], 2) - f32::powi(other[e4315], 2) - f32::powi(other[e4125], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiFlatPoint::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e321]]),
        );
    }
}
impl GeometricQuotient<Line> for AntiFlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        1        1
    //  no simd        2        4        1
    fn geometric_quotient(self, other: Line) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e415], 2) + f32::powi(other[e425], 2) + f32::powi(other[e435], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiFlatPoint::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e321]]),
        );
    }
}
impl GeometricQuotient<Motor> for AntiFlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        1        1
    //  no simd        3        4        1
    fn geometric_quotient(self, other: Motor) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e415], 2) + f32::powi(other[e425], 2) + f32::powi(other[e435], 2) - f32::powi(other[e12345], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiFlatPoint::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e321]]),
        );
    }
}
impl GeometricQuotient<MultiVector> for AntiFlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       23       16        1
    //    simd4        0        1        0
    // Totals...
    // yes simd       23       17        1
    //  no simd       23       20        1
    fn geometric_quotient(self, other: MultiVector) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + 2.0 * (other[e3215] * other[e1234])
                + f32::powi(other[scalar], 2)
                + f32::powi(other[e1], 2)
                + f32::powi(other[e2], 2)
                + f32::powi(other[e3], 2)
                + f32::powi(other[e45], 2)
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                - f32::powi(other[e12345], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - f32::powi(other[e321], 2)
                - f32::powi(other[e4235], 2)
                - f32::powi(other[e4315], 2)
                - f32::powi(other[e4125], 2)
                - 2.0 * (other[e4] * other[e5])
                - 2.0 * (other[e15] * other[e41])
                - 2.0 * (other[e25] * other[e42])
                - 2.0 * (other[e35] * other[e43]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiFlatPoint::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e321]]),
        );
    }
}
impl GeometricQuotient<Plane> for AntiFlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        1        1
    //  no simd        2        4        1
    fn geometric_quotient(self, other: Plane) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ -f32::powi(other[e4235], 2) - f32::powi(other[e4315], 2) - f32::powi(other[e4125], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiFlatPoint::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e321]]),
        );
    }
}
impl GeometricQuotient<RoundPoint> for AntiFlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        2        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        3        1
    //  no simd        3        6        1
    fn geometric_quotient(self, other: RoundPoint) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2) - 2.0 * (other[e4] * other[e5]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiFlatPoint::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e321]]),
        );
    }
}
impl GeometricQuotient<Scalar> for AntiFlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        1        1
    //  no simd        0        4        1
    fn geometric_quotient(self, other: Scalar) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiFlatPoint::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e321]]),
        );
    }
}
impl GeometricQuotient<Sphere> for AntiFlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        2        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        3        1
    //  no simd        3        6        1
    fn geometric_quotient(self, other: Sphere) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e3215] * other[e1234]) - f32::powi(other[e4235], 2) - f32::powi(other[e4315], 2) - f32::powi(other[e4125], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiFlatPoint::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e321]]),
        );
    }
}
impl GeometricQuotient<VersorEven> for AntiFlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11        8        1
    //    simd4        0        1        0
    // Totals...
    // yes simd       11        9        1
    //  no simd       11       12        1
    fn geometric_quotient(self, other: VersorEven) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                + f32::powi(other[e1], 2)
                + f32::powi(other[e2], 2)
                + f32::powi(other[e3], 2)
                - f32::powi(other[e12345], 2)
                - f32::powi(other[e321], 2)
                - 2.0 * (other[e5] * other[e4]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiFlatPoint::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e321]]),
        );
    }
}
impl GeometricQuotient<VersorOdd> for AntiFlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11        8        1
    //    simd4        0        1        0
    // Totals...
    // yes simd       11        9        1
    //  no simd       11       12        1
    fn geometric_quotient(self, other: VersorOdd) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e1234] * other[e3215]) + f32::powi(other[scalar], 2) + f32::powi(other[e45], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - f32::powi(other[e4235], 2)
                - f32::powi(other[e4315], 2)
                - f32::powi(other[e4125], 2)
                - 2.0 * (other[e41] * other[e15])
                - 2.0 * (other[e42] * other[e25])
                - 2.0 * (other[e43] * other[e35]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiFlatPoint::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e321]]),
        );
    }
}
impl std::ops::Div<geometric_quotient> for AntiFlector {
    type Output = geometric_quotient_partial<AntiFlector>;
    fn div(self, _rhs: geometric_quotient) -> Self::Output {
        geometric_quotient_partial(self)
    }
}
impl GeometricQuotient<AntiCircleRotor> for AntiFlector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        6        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        7        8        1
    //  no simd        7       14        1
    fn geometric_quotient(self, other: AntiCircleRotor) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e45], 2) + f32::powi(other[scalar], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - 2.0 * (other[e41] * other[e15])
                - 2.0 * (other[e42] * other[e25])
                - 2.0 * (other[e43] * other[e35]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e321]]),
            // e1, e2, e3, e5
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricQuotient<AntiDipoleInversion> for AntiFlector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10        8        1
    //    simd4        0        2        0
    // Totals...
    // yes simd       10       10        1
    //  no simd       10       16        1
    fn geometric_quotient(self, other: AntiDipoleInversion) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                + f32::powi(other[e1], 2)
                + f32::powi(other[e2], 2)
                + f32::powi(other[e3], 2)
                - f32::powi(other[e321], 2)
                - 2.0 * (other[e4] * other[e5]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e321]]),
            // e1, e2, e3, e5
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricQuotient<AntiDualNum> for AntiFlector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        8        1
    fn geometric_quotient(self, other: AntiDualNum) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e321]]),
            // e1, e2, e3, e5
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricQuotient<AntiFlatPoint> for AntiFlector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        1
    //  no simd        0        9        1
    fn geometric_quotient(self, other: AntiFlatPoint) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e321], 2) * -1.0);
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e321]]),
            // e1, e2, e3, e5
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricQuotient<AntiFlector> for AntiFlector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        2        1
    //  no simd        3        8        1
    fn geometric_quotient(self, other: AntiFlector) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2) - f32::powi(other[e321], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e321]]),
            // e1, e2, e3, e5
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricQuotient<AntiLine> for AntiFlector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        2        2        1
    //  no simd        2        8        1
    fn geometric_quotient(self, other: AntiLine) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ -f32::powi(other[e23], 2) - f32::powi(other[e31], 2) - f32::powi(other[e12], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e321]]),
            // e1, e2, e3, e5
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricQuotient<AntiMotor> for AntiFlector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        2        1
    //  no simd        3        8        1
    fn geometric_quotient(self, other: AntiMotor) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[scalar], 2) - f32::powi(other[e23], 2) - f32::powi(other[e31], 2) - f32::powi(other[e12], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e321]]),
            // e1, e2, e3, e5
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricQuotient<AntiPlane> for AntiFlector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        2        2        1
    //  no simd        2        8        1
    fn geometric_quotient(self, other: AntiPlane) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e321]]),
            // e1, e2, e3, e5
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricQuotient<AntiScalar> for AntiFlector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        1
    //  no simd        0        9        1
    fn geometric_quotient(self, other: AntiScalar) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e12345], 2) * -1.0);
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e321]]),
            // e1, e2, e3, e5
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricQuotient<Circle> for AntiFlector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        6        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        6        8        1
    //  no simd        6       14        1
    fn geometric_quotient(self, other: Circle) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                - f32::powi(other[e321], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e321]]),
            // e1, e2, e3, e5
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricQuotient<CircleRotor> for AntiFlector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        6        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        7        8        1
    //  no simd        7       14        1
    fn geometric_quotient(self, other: CircleRotor) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                - f32::powi(other[e321], 2)
                - f32::powi(other[e12345], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e321]]),
            // e1, e2, e3, e5
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricQuotient<Dipole> for AntiFlector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        6        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        6        8        1
    //  no simd        6       14        1
    fn geometric_quotient(self, other: Dipole) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e45], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - 2.0 * (other[e41] * other[e15])
                - 2.0 * (other[e42] * other[e25])
                - 2.0 * (other[e43] * other[e35]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e321]]),
            // e1, e2, e3, e5
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricQuotient<DipoleInversion> for AntiFlector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10        8        1
    //    simd4        0        2        0
    // Totals...
    // yes simd       10       10        1
    //  no simd       10       16        1
    fn geometric_quotient(self, other: DipoleInversion) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e1234] * other[e3215]) + f32::powi(other[e45], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - f32::powi(other[e4235], 2)
                - f32::powi(other[e4315], 2)
                - f32::powi(other[e4125], 2)
                - 2.0 * (other[e41] * other[e15])
                - 2.0 * (other[e42] * other[e25])
                - 2.0 * (other[e43] * other[e35]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e321]]),
            // e1, e2, e3, e5
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricQuotient<DualNum> for AntiFlector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        1
    //  no simd        0        9        1
    fn geometric_quotient(self, other: DualNum) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e12345], 2) * -1.0);
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e321]]),
            // e1, e2, e3, e5
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricQuotient<FlatPoint> for AntiFlector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        8        1
    fn geometric_quotient(self, other: FlatPoint) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e45], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e321]]),
            // e1, e2, e3, e5
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricQuotient<Flector> for AntiFlector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        2        1
    //  no simd        3        8        1
    fn geometric_quotient(self, other: Flector) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e45], 2) - f32::powi(other[e4235], 2) - f32::powi(other[e4315], 2) - f32::powi(other[e4125], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e321]]),
            // e1, e2, e3, e5
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricQuotient<Line> for AntiFlector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        2        2        1
    //  no simd        2        8        1
    fn geometric_quotient(self, other: Line) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e415], 2) + f32::powi(other[e425], 2) + f32::powi(other[e435], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e321]]),
            // e1, e2, e3, e5
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricQuotient<Motor> for AntiFlector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        2        1
    //  no simd        3        8        1
    fn geometric_quotient(self, other: Motor) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e415], 2) + f32::powi(other[e425], 2) + f32::powi(other[e435], 2) - f32::powi(other[e12345], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e321]]),
            // e1, e2, e3, e5
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricQuotient<MultiVector> for AntiFlector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       23       16        1
    //    simd4        0        2        0
    // Totals...
    // yes simd       23       18        1
    //  no simd       23       24        1
    fn geometric_quotient(self, other: MultiVector) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + 2.0 * (other[e3215] * other[e1234])
                + f32::powi(other[scalar], 2)
                + f32::powi(other[e1], 2)
                + f32::powi(other[e2], 2)
                + f32::powi(other[e3], 2)
                + f32::powi(other[e45], 2)
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                - f32::powi(other[e12345], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - f32::powi(other[e321], 2)
                - f32::powi(other[e4235], 2)
                - f32::powi(other[e4315], 2)
                - f32::powi(other[e4125], 2)
                - 2.0 * (other[e4] * other[e5])
                - 2.0 * (other[e15] * other[e41])
                - 2.0 * (other[e25] * other[e42])
                - 2.0 * (other[e35] * other[e43]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e321]]),
            // e1, e2, e3, e5
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricQuotient<Plane> for AntiFlector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        2        2        1
    //  no simd        2        8        1
    fn geometric_quotient(self, other: Plane) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ -f32::powi(other[e4235], 2) - f32::powi(other[e4315], 2) - f32::powi(other[e4125], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e321]]),
            // e1, e2, e3, e5
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricQuotient<RoundPoint> for AntiFlector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        2        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        4        1
    //  no simd        3       10        1
    fn geometric_quotient(self, other: RoundPoint) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2) - 2.0 * (other[e4] * other[e5]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e321]]),
            // e1, e2, e3, e5
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricQuotient<Scalar> for AntiFlector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        8        1
    fn geometric_quotient(self, other: Scalar) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e321]]),
            // e1, e2, e3, e5
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricQuotient<Sphere> for AntiFlector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        2        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        4        1
    //  no simd        3       10        1
    fn geometric_quotient(self, other: Sphere) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e3215] * other[e1234]) - f32::powi(other[e4235], 2) - f32::powi(other[e4315], 2) - f32::powi(other[e4125], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e321]]),
            // e1, e2, e3, e5
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricQuotient<VersorEven> for AntiFlector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11        8        1
    //    simd4        0        2        0
    // Totals...
    // yes simd       11       10        1
    //  no simd       11       16        1
    fn geometric_quotient(self, other: VersorEven) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                + f32::powi(other[e1], 2)
                + f32::powi(other[e2], 2)
                + f32::powi(other[e3], 2)
                - f32::powi(other[e12345], 2)
                - f32::powi(other[e321], 2)
                - 2.0 * (other[e5] * other[e4]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e321]]),
            // e1, e2, e3, e5
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricQuotient<VersorOdd> for AntiFlector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11        8        1
    //    simd4        0        2        0
    // Totals...
    // yes simd       11       10        1
    //  no simd       11       16        1
    fn geometric_quotient(self, other: VersorOdd) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e1234] * other[e3215]) + f32::powi(other[scalar], 2) + f32::powi(other[e45], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - f32::powi(other[e4235], 2)
                - f32::powi(other[e4315], 2)
                - f32::powi(other[e4125], 2)
                - 2.0 * (other[e41] * other[e15])
                - 2.0 * (other[e42] * other[e25])
                - 2.0 * (other[e43] * other[e35]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e321]]),
            // e1, e2, e3, e5
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl std::ops::Div<geometric_quotient> for AntiLine {
    type Output = geometric_quotient_partial<AntiLine>;
    fn div(self, _rhs: geometric_quotient) -> Self::Output {
        geometric_quotient_partial(self)
    }
}
impl GeometricQuotient<AntiCircleRotor> for AntiLine {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        6        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        7        8        1
    //  no simd        7       12        1
    fn geometric_quotient(self, other: AntiCircleRotor) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e45], 2) + f32::powi(other[scalar], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - 2.0 * (other[e41] * other[e15])
                - 2.0 * (other[e42] * other[e25])
                - 2.0 * (other[e43] * other[e35]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e15, e25, e35
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e15], self[e25], self[e35]]),
        );
    }
}
impl GeometricQuotient<AntiDipoleInversion> for AntiLine {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10        8        1
    //    simd3        0        2        0
    // Totals...
    // yes simd       10       10        1
    //  no simd       10       14        1
    fn geometric_quotient(self, other: AntiDipoleInversion) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                + f32::powi(other[e1], 2)
                + f32::powi(other[e2], 2)
                + f32::powi(other[e3], 2)
                - f32::powi(other[e321], 2)
                - 2.0 * (other[e4] * other[e5]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e15, e25, e35
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e15], self[e25], self[e35]]),
        );
    }
}
impl GeometricQuotient<AntiDualNum> for AntiLine {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        6        1
    fn geometric_quotient(self, other: AntiDualNum) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e15, e25, e35
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e15], self[e25], self[e35]]),
        );
    }
}
impl GeometricQuotient<AntiFlatPoint> for AntiLine {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        3        1
    //  no simd        0        7        1
    fn geometric_quotient(self, other: AntiFlatPoint) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e321], 2) * -1.0);
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e15, e25, e35
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e15], self[e25], self[e35]]),
        );
    }
}
impl GeometricQuotient<AntiFlector> for AntiLine {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        3        2        1
    //  no simd        3        6        1
    fn geometric_quotient(self, other: AntiFlector) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2) - f32::powi(other[e321], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e15, e25, e35
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e15], self[e25], self[e35]]),
        );
    }
}
impl GeometricQuotient<AntiLine> for AntiLine {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        2        2        1
    //  no simd        2        6        1
    fn geometric_quotient(self, other: AntiLine) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ -f32::powi(other[e23], 2) - f32::powi(other[e31], 2) - f32::powi(other[e12], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e15, e25, e35
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e15], self[e25], self[e35]]),
        );
    }
}
impl GeometricQuotient<AntiMotor> for AntiLine {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        3        2        1
    //  no simd        3        6        1
    fn geometric_quotient(self, other: AntiMotor) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[scalar], 2) - f32::powi(other[e23], 2) - f32::powi(other[e31], 2) - f32::powi(other[e12], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e15, e25, e35
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e15], self[e25], self[e35]]),
        );
    }
}
impl GeometricQuotient<AntiPlane> for AntiLine {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        2        2        1
    //  no simd        2        6        1
    fn geometric_quotient(self, other: AntiPlane) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e15, e25, e35
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e15], self[e25], self[e35]]),
        );
    }
}
impl GeometricQuotient<AntiScalar> for AntiLine {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        3        1
    //  no simd        0        7        1
    fn geometric_quotient(self, other: AntiScalar) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e12345], 2) * -1.0);
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e15, e25, e35
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e15], self[e25], self[e35]]),
        );
    }
}
impl GeometricQuotient<Circle> for AntiLine {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        6        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        6        8        1
    //  no simd        6       12        1
    fn geometric_quotient(self, other: Circle) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                - f32::powi(other[e321], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e15, e25, e35
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e15], self[e25], self[e35]]),
        );
    }
}
impl GeometricQuotient<CircleRotor> for AntiLine {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        6        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        7        8        1
    //  no simd        7       12        1
    fn geometric_quotient(self, other: CircleRotor) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                - f32::powi(other[e321], 2)
                - f32::powi(other[e12345], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e15, e25, e35
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e15], self[e25], self[e35]]),
        );
    }
}
impl GeometricQuotient<Dipole> for AntiLine {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        6        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        6        8        1
    //  no simd        6       12        1
    fn geometric_quotient(self, other: Dipole) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e45], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - 2.0 * (other[e41] * other[e15])
                - 2.0 * (other[e42] * other[e25])
                - 2.0 * (other[e43] * other[e35]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e15, e25, e35
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e15], self[e25], self[e35]]),
        );
    }
}
impl GeometricQuotient<DipoleInversion> for AntiLine {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10        8        1
    //    simd3        0        2        0
    // Totals...
    // yes simd       10       10        1
    //  no simd       10       14        1
    fn geometric_quotient(self, other: DipoleInversion) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e1234] * other[e3215]) + f32::powi(other[e45], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - f32::powi(other[e4235], 2)
                - f32::powi(other[e4315], 2)
                - f32::powi(other[e4125], 2)
                - 2.0 * (other[e41] * other[e15])
                - 2.0 * (other[e42] * other[e25])
                - 2.0 * (other[e43] * other[e35]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e15, e25, e35
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e15], self[e25], self[e35]]),
        );
    }
}
impl GeometricQuotient<DualNum> for AntiLine {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        3        1
    //  no simd        0        7        1
    fn geometric_quotient(self, other: DualNum) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e12345], 2) * -1.0);
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e15, e25, e35
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e15], self[e25], self[e35]]),
        );
    }
}
impl GeometricQuotient<FlatPoint> for AntiLine {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        6        1
    fn geometric_quotient(self, other: FlatPoint) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e45], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e15, e25, e35
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e15], self[e25], self[e35]]),
        );
    }
}
impl GeometricQuotient<Flector> for AntiLine {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        3        2        1
    //  no simd        3        6        1
    fn geometric_quotient(self, other: Flector) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e45], 2) - f32::powi(other[e4235], 2) - f32::powi(other[e4315], 2) - f32::powi(other[e4125], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e15, e25, e35
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e15], self[e25], self[e35]]),
        );
    }
}
impl GeometricQuotient<Line> for AntiLine {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        2        2        1
    //  no simd        2        6        1
    fn geometric_quotient(self, other: Line) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e415], 2) + f32::powi(other[e425], 2) + f32::powi(other[e435], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e15, e25, e35
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e15], self[e25], self[e35]]),
        );
    }
}
impl GeometricQuotient<Motor> for AntiLine {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        3        2        1
    //  no simd        3        6        1
    fn geometric_quotient(self, other: Motor) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e415], 2) + f32::powi(other[e425], 2) + f32::powi(other[e435], 2) - f32::powi(other[e12345], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e15, e25, e35
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e15], self[e25], self[e35]]),
        );
    }
}
impl GeometricQuotient<MultiVector> for AntiLine {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       23       16        1
    //    simd3        0        2        0
    // Totals...
    // yes simd       23       18        1
    //  no simd       23       22        1
    fn geometric_quotient(self, other: MultiVector) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + 2.0 * (other[e3215] * other[e1234])
                + f32::powi(other[scalar], 2)
                + f32::powi(other[e1], 2)
                + f32::powi(other[e2], 2)
                + f32::powi(other[e3], 2)
                + f32::powi(other[e45], 2)
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                - f32::powi(other[e12345], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - f32::powi(other[e321], 2)
                - f32::powi(other[e4235], 2)
                - f32::powi(other[e4315], 2)
                - f32::powi(other[e4125], 2)
                - 2.0 * (other[e4] * other[e5])
                - 2.0 * (other[e15] * other[e41])
                - 2.0 * (other[e25] * other[e42])
                - 2.0 * (other[e35] * other[e43]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e15, e25, e35
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e15], self[e25], self[e35]]),
        );
    }
}
impl GeometricQuotient<Plane> for AntiLine {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        2        2        1
    //  no simd        2        6        1
    fn geometric_quotient(self, other: Plane) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ -f32::powi(other[e4235], 2) - f32::powi(other[e4315], 2) - f32::powi(other[e4125], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e15, e25, e35
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e15], self[e25], self[e35]]),
        );
    }
}
impl GeometricQuotient<RoundPoint> for AntiLine {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        2        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        3        4        1
    //  no simd        3        8        1
    fn geometric_quotient(self, other: RoundPoint) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2) - 2.0 * (other[e4] * other[e5]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e15, e25, e35
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e15], self[e25], self[e35]]),
        );
    }
}
impl GeometricQuotient<Scalar> for AntiLine {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        6        1
    fn geometric_quotient(self, other: Scalar) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e15, e25, e35
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e15], self[e25], self[e35]]),
        );
    }
}
impl GeometricQuotient<Sphere> for AntiLine {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        2        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        3        4        1
    //  no simd        3        8        1
    fn geometric_quotient(self, other: Sphere) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e3215] * other[e1234]) - f32::powi(other[e4235], 2) - f32::powi(other[e4315], 2) - f32::powi(other[e4125], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e15, e25, e35
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e15], self[e25], self[e35]]),
        );
    }
}
impl GeometricQuotient<VersorEven> for AntiLine {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11        8        1
    //    simd3        0        2        0
    // Totals...
    // yes simd       11       10        1
    //  no simd       11       14        1
    fn geometric_quotient(self, other: VersorEven) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                + f32::powi(other[e1], 2)
                + f32::powi(other[e2], 2)
                + f32::powi(other[e3], 2)
                - f32::powi(other[e12345], 2)
                - f32::powi(other[e321], 2)
                - 2.0 * (other[e5] * other[e4]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e15, e25, e35
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e15], self[e25], self[e35]]),
        );
    }
}
impl GeometricQuotient<VersorOdd> for AntiLine {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11        8        1
    //    simd3        0        2        0
    // Totals...
    // yes simd       11       10        1
    //  no simd       11       14        1
    fn geometric_quotient(self, other: VersorOdd) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e1234] * other[e3215]) + f32::powi(other[scalar], 2) + f32::powi(other[e45], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - f32::powi(other[e4235], 2)
                - f32::powi(other[e4315], 2)
                - f32::powi(other[e4125], 2)
                - 2.0 * (other[e41] * other[e15])
                - 2.0 * (other[e42] * other[e25])
                - 2.0 * (other[e43] * other[e35]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e15, e25, e35
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e15], self[e25], self[e35]]),
        );
    }
}
impl std::ops::Div<geometric_quotient> for AntiMotor {
    type Output = geometric_quotient_partial<AntiMotor>;
    fn div(self, _rhs: geometric_quotient) -> Self::Output {
        geometric_quotient_partial(self)
    }
}
impl GeometricQuotient<AntiCircleRotor> for AntiMotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        6        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        7        8        1
    //  no simd        7       14        1
    fn geometric_quotient(self, other: AntiCircleRotor) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e45], 2) + f32::powi(other[scalar], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - 2.0 * (other[e41] * other[e15])
                - 2.0 * (other[e42] * other[e25])
                - 2.0 * (other[e43] * other[e35]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]),
            // e15, e25, e35, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e3215]]),
        );
    }
}
impl GeometricQuotient<AntiDipoleInversion> for AntiMotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10        8        1
    //    simd4        0        2        0
    // Totals...
    // yes simd       10       10        1
    //  no simd       10       16        1
    fn geometric_quotient(self, other: AntiDipoleInversion) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                + f32::powi(other[e1], 2)
                + f32::powi(other[e2], 2)
                + f32::powi(other[e3], 2)
                - f32::powi(other[e321], 2)
                - 2.0 * (other[e4] * other[e5]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]),
            // e15, e25, e35, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e3215]]),
        );
    }
}
impl GeometricQuotient<AntiDualNum> for AntiMotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        8        1
    fn geometric_quotient(self, other: AntiDualNum) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]),
            // e15, e25, e35, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e3215]]),
        );
    }
}
impl GeometricQuotient<AntiFlatPoint> for AntiMotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        1
    //  no simd        0        9        1
    fn geometric_quotient(self, other: AntiFlatPoint) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e321], 2) * -1.0);
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]),
            // e15, e25, e35, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e3215]]),
        );
    }
}
impl GeometricQuotient<AntiFlector> for AntiMotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        2        1
    //  no simd        3        8        1
    fn geometric_quotient(self, other: AntiFlector) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2) - f32::powi(other[e321], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]),
            // e15, e25, e35, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e3215]]),
        );
    }
}
impl GeometricQuotient<AntiLine> for AntiMotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        2        2        1
    //  no simd        2        8        1
    fn geometric_quotient(self, other: AntiLine) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ -f32::powi(other[e23], 2) - f32::powi(other[e31], 2) - f32::powi(other[e12], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]),
            // e15, e25, e35, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e3215]]),
        );
    }
}
impl GeometricQuotient<AntiMotor> for AntiMotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        2        1
    //  no simd        3        8        1
    fn geometric_quotient(self, other: AntiMotor) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[scalar], 2) - f32::powi(other[e23], 2) - f32::powi(other[e31], 2) - f32::powi(other[e12], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]),
            // e15, e25, e35, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e3215]]),
        );
    }
}
impl GeometricQuotient<AntiPlane> for AntiMotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        2        2        1
    //  no simd        2        8        1
    fn geometric_quotient(self, other: AntiPlane) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]),
            // e15, e25, e35, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e3215]]),
        );
    }
}
impl GeometricQuotient<AntiScalar> for AntiMotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        1
    //  no simd        0        9        1
    fn geometric_quotient(self, other: AntiScalar) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e12345], 2) * -1.0);
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]),
            // e15, e25, e35, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e3215]]),
        );
    }
}
impl GeometricQuotient<Circle> for AntiMotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        6        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        6        8        1
    //  no simd        6       14        1
    fn geometric_quotient(self, other: Circle) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                - f32::powi(other[e321], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]),
            // e15, e25, e35, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e3215]]),
        );
    }
}
impl GeometricQuotient<CircleRotor> for AntiMotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        6        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        7        8        1
    //  no simd        7       14        1
    fn geometric_quotient(self, other: CircleRotor) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                - f32::powi(other[e321], 2)
                - f32::powi(other[e12345], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]),
            // e15, e25, e35, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e3215]]),
        );
    }
}
impl GeometricQuotient<Dipole> for AntiMotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        6        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        6        8        1
    //  no simd        6       14        1
    fn geometric_quotient(self, other: Dipole) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e45], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - 2.0 * (other[e41] * other[e15])
                - 2.0 * (other[e42] * other[e25])
                - 2.0 * (other[e43] * other[e35]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]),
            // e15, e25, e35, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e3215]]),
        );
    }
}
impl GeometricQuotient<DipoleInversion> for AntiMotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10        8        1
    //    simd4        0        2        0
    // Totals...
    // yes simd       10       10        1
    //  no simd       10       16        1
    fn geometric_quotient(self, other: DipoleInversion) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e1234] * other[e3215]) + f32::powi(other[e45], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - f32::powi(other[e4235], 2)
                - f32::powi(other[e4315], 2)
                - f32::powi(other[e4125], 2)
                - 2.0 * (other[e41] * other[e15])
                - 2.0 * (other[e42] * other[e25])
                - 2.0 * (other[e43] * other[e35]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]),
            // e15, e25, e35, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e3215]]),
        );
    }
}
impl GeometricQuotient<DualNum> for AntiMotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        1
    //  no simd        0        9        1
    fn geometric_quotient(self, other: DualNum) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e12345], 2) * -1.0);
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]),
            // e15, e25, e35, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e3215]]),
        );
    }
}
impl GeometricQuotient<FlatPoint> for AntiMotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        8        1
    fn geometric_quotient(self, other: FlatPoint) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e45], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]),
            // e15, e25, e35, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e3215]]),
        );
    }
}
impl GeometricQuotient<Flector> for AntiMotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        2        1
    //  no simd        3        8        1
    fn geometric_quotient(self, other: Flector) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e45], 2) - f32::powi(other[e4235], 2) - f32::powi(other[e4315], 2) - f32::powi(other[e4125], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]),
            // e15, e25, e35, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e3215]]),
        );
    }
}
impl GeometricQuotient<Line> for AntiMotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        2        2        1
    //  no simd        2        8        1
    fn geometric_quotient(self, other: Line) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e415], 2) + f32::powi(other[e425], 2) + f32::powi(other[e435], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]),
            // e15, e25, e35, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e3215]]),
        );
    }
}
impl GeometricQuotient<Motor> for AntiMotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        2        1
    //  no simd        3        8        1
    fn geometric_quotient(self, other: Motor) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e415], 2) + f32::powi(other[e425], 2) + f32::powi(other[e435], 2) - f32::powi(other[e12345], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]),
            // e15, e25, e35, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e3215]]),
        );
    }
}
impl GeometricQuotient<MultiVector> for AntiMotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       23       16        1
    //    simd4        0        2        0
    // Totals...
    // yes simd       23       18        1
    //  no simd       23       24        1
    fn geometric_quotient(self, other: MultiVector) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + 2.0 * (other[e3215] * other[e1234])
                + f32::powi(other[scalar], 2)
                + f32::powi(other[e1], 2)
                + f32::powi(other[e2], 2)
                + f32::powi(other[e3], 2)
                + f32::powi(other[e45], 2)
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                - f32::powi(other[e12345], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - f32::powi(other[e321], 2)
                - f32::powi(other[e4235], 2)
                - f32::powi(other[e4315], 2)
                - f32::powi(other[e4125], 2)
                - 2.0 * (other[e4] * other[e5])
                - 2.0 * (other[e15] * other[e41])
                - 2.0 * (other[e25] * other[e42])
                - 2.0 * (other[e35] * other[e43]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]),
            // e15, e25, e35, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e3215]]),
        );
    }
}
impl GeometricQuotient<Plane> for AntiMotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        2        2        1
    //  no simd        2        8        1
    fn geometric_quotient(self, other: Plane) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ -f32::powi(other[e4235], 2) - f32::powi(other[e4315], 2) - f32::powi(other[e4125], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]),
            // e15, e25, e35, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e3215]]),
        );
    }
}
impl GeometricQuotient<RoundPoint> for AntiMotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        2        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        4        1
    //  no simd        3       10        1
    fn geometric_quotient(self, other: RoundPoint) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2) - 2.0 * (other[e4] * other[e5]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]),
            // e15, e25, e35, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e3215]]),
        );
    }
}
impl GeometricQuotient<Scalar> for AntiMotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        8        1
    fn geometric_quotient(self, other: Scalar) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]),
            // e15, e25, e35, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e3215]]),
        );
    }
}
impl GeometricQuotient<Sphere> for AntiMotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        2        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        4        1
    //  no simd        3       10        1
    fn geometric_quotient(self, other: Sphere) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e3215] * other[e1234]) - f32::powi(other[e4235], 2) - f32::powi(other[e4315], 2) - f32::powi(other[e4125], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]),
            // e15, e25, e35, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e3215]]),
        );
    }
}
impl GeometricQuotient<VersorEven> for AntiMotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11        8        1
    //    simd4        0        2        0
    // Totals...
    // yes simd       11       10        1
    //  no simd       11       16        1
    fn geometric_quotient(self, other: VersorEven) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                + f32::powi(other[e1], 2)
                + f32::powi(other[e2], 2)
                + f32::powi(other[e3], 2)
                - f32::powi(other[e12345], 2)
                - f32::powi(other[e321], 2)
                - 2.0 * (other[e5] * other[e4]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]),
            // e15, e25, e35, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e3215]]),
        );
    }
}
impl GeometricQuotient<VersorOdd> for AntiMotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11        8        1
    //    simd4        0        2        0
    // Totals...
    // yes simd       11       10        1
    //  no simd       11       16        1
    fn geometric_quotient(self, other: VersorOdd) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e1234] * other[e3215]) + f32::powi(other[scalar], 2) + f32::powi(other[e45], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - f32::powi(other[e4235], 2)
                - f32::powi(other[e4315], 2)
                - f32::powi(other[e4125], 2)
                - 2.0 * (other[e41] * other[e15])
                - 2.0 * (other[e42] * other[e25])
                - 2.0 * (other[e43] * other[e35]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]),
            // e15, e25, e35, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e3215]]),
        );
    }
}
impl std::ops::Div<geometric_quotient> for AntiPlane {
    type Output = geometric_quotient_partial<AntiPlane>;
    fn div(self, _rhs: geometric_quotient) -> Self::Output {
        geometric_quotient_partial(self)
    }
}
impl GeometricQuotient<AntiCircleRotor> for AntiPlane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        6        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        7        7        1
    //  no simd        7       10        1
    fn geometric_quotient(self, other: AntiCircleRotor) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e45], 2) + f32::powi(other[scalar], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - 2.0 * (other[e41] * other[e15])
                - 2.0 * (other[e42] * other[e25])
                - 2.0 * (other[e43] * other[e35]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]));
    }
}
impl GeometricQuotient<AntiDipoleInversion> for AntiPlane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10        8        1
    //    simd4        0        1        0
    // Totals...
    // yes simd       10        9        1
    //  no simd       10       12        1
    fn geometric_quotient(self, other: AntiDipoleInversion) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                + f32::powi(other[e1], 2)
                + f32::powi(other[e2], 2)
                + f32::powi(other[e3], 2)
                - f32::powi(other[e321], 2)
                - 2.0 * (other[e4] * other[e5]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]));
    }
}
impl GeometricQuotient<AntiDualNum> for AntiPlane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        1        1
    //  no simd        0        4        1
    fn geometric_quotient(self, other: AntiDualNum) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]));
    }
}
impl GeometricQuotient<AntiFlatPoint> for AntiPlane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        5        1
    fn geometric_quotient(self, other: AntiFlatPoint) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e321], 2) * -1.0);
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]));
    }
}
impl GeometricQuotient<AntiFlector> for AntiPlane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        1        1
    //  no simd        3        4        1
    fn geometric_quotient(self, other: AntiFlector) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2) - f32::powi(other[e321], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]));
    }
}
impl GeometricQuotient<AntiLine> for AntiPlane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        1        1
    //  no simd        2        4        1
    fn geometric_quotient(self, other: AntiLine) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ -f32::powi(other[e23], 2) - f32::powi(other[e31], 2) - f32::powi(other[e12], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]));
    }
}
impl GeometricQuotient<AntiMotor> for AntiPlane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        1        1
    //  no simd        3        4        1
    fn geometric_quotient(self, other: AntiMotor) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[scalar], 2) - f32::powi(other[e23], 2) - f32::powi(other[e31], 2) - f32::powi(other[e12], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]));
    }
}
impl GeometricQuotient<AntiPlane> for AntiPlane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        1        1
    //  no simd        2        4        1
    fn geometric_quotient(self, other: AntiPlane) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]));
    }
}
impl GeometricQuotient<AntiScalar> for AntiPlane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        5        1
    fn geometric_quotient(self, other: AntiScalar) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e12345], 2) * -1.0);
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]));
    }
}
impl GeometricQuotient<Circle> for AntiPlane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        6        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        6        7        1
    //  no simd        6       10        1
    fn geometric_quotient(self, other: Circle) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                - f32::powi(other[e321], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]));
    }
}
impl GeometricQuotient<CircleRotor> for AntiPlane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        6        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        7        7        1
    //  no simd        7       10        1
    fn geometric_quotient(self, other: CircleRotor) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                - f32::powi(other[e321], 2)
                - f32::powi(other[e12345], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]));
    }
}
impl GeometricQuotient<Dipole> for AntiPlane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        6        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        6        7        1
    //  no simd        6       10        1
    fn geometric_quotient(self, other: Dipole) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e45], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - 2.0 * (other[e41] * other[e15])
                - 2.0 * (other[e42] * other[e25])
                - 2.0 * (other[e43] * other[e35]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]));
    }
}
impl GeometricQuotient<DipoleInversion> for AntiPlane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10        8        1
    //    simd4        0        1        0
    // Totals...
    // yes simd       10        9        1
    //  no simd       10       12        1
    fn geometric_quotient(self, other: DipoleInversion) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e1234] * other[e3215]) + f32::powi(other[e45], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - f32::powi(other[e4235], 2)
                - f32::powi(other[e4315], 2)
                - f32::powi(other[e4125], 2)
                - 2.0 * (other[e41] * other[e15])
                - 2.0 * (other[e42] * other[e25])
                - 2.0 * (other[e43] * other[e35]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]));
    }
}
impl GeometricQuotient<DualNum> for AntiPlane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        5        1
    fn geometric_quotient(self, other: DualNum) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e12345], 2) * -1.0);
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]));
    }
}
impl GeometricQuotient<FlatPoint> for AntiPlane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        1        1
    //  no simd        0        4        1
    fn geometric_quotient(self, other: FlatPoint) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e45], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]));
    }
}
impl GeometricQuotient<Flector> for AntiPlane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        1        1
    //  no simd        3        4        1
    fn geometric_quotient(self, other: Flector) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e45], 2) - f32::powi(other[e4235], 2) - f32::powi(other[e4315], 2) - f32::powi(other[e4125], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]));
    }
}
impl GeometricQuotient<Line> for AntiPlane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        1        1
    //  no simd        2        4        1
    fn geometric_quotient(self, other: Line) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e415], 2) + f32::powi(other[e425], 2) + f32::powi(other[e435], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]));
    }
}
impl GeometricQuotient<Motor> for AntiPlane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        1        1
    //  no simd        3        4        1
    fn geometric_quotient(self, other: Motor) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e415], 2) + f32::powi(other[e425], 2) + f32::powi(other[e435], 2) - f32::powi(other[e12345], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]));
    }
}
impl GeometricQuotient<MultiVector> for AntiPlane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       23       16        1
    //    simd4        0        1        0
    // Totals...
    // yes simd       23       17        1
    //  no simd       23       20        1
    fn geometric_quotient(self, other: MultiVector) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + 2.0 * (other[e3215] * other[e1234])
                + f32::powi(other[scalar], 2)
                + f32::powi(other[e1], 2)
                + f32::powi(other[e2], 2)
                + f32::powi(other[e3], 2)
                + f32::powi(other[e45], 2)
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                - f32::powi(other[e12345], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - f32::powi(other[e321], 2)
                - f32::powi(other[e4235], 2)
                - f32::powi(other[e4315], 2)
                - f32::powi(other[e4125], 2)
                - 2.0 * (other[e4] * other[e5])
                - 2.0 * (other[e15] * other[e41])
                - 2.0 * (other[e25] * other[e42])
                - 2.0 * (other[e35] * other[e43]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]));
    }
}
impl GeometricQuotient<Plane> for AntiPlane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        1        1
    //  no simd        2        4        1
    fn geometric_quotient(self, other: Plane) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ -f32::powi(other[e4235], 2) - f32::powi(other[e4315], 2) - f32::powi(other[e4125], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]));
    }
}
impl GeometricQuotient<RoundPoint> for AntiPlane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        2        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        3        1
    //  no simd        3        6        1
    fn geometric_quotient(self, other: RoundPoint) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2) - 2.0 * (other[e4] * other[e5]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]));
    }
}
impl GeometricQuotient<Scalar> for AntiPlane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        1        1
    //  no simd        0        4        1
    fn geometric_quotient(self, other: Scalar) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]));
    }
}
impl GeometricQuotient<Sphere> for AntiPlane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        2        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        3        1
    //  no simd        3        6        1
    fn geometric_quotient(self, other: Sphere) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e3215] * other[e1234]) - f32::powi(other[e4235], 2) - f32::powi(other[e4315], 2) - f32::powi(other[e4125], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]));
    }
}
impl GeometricQuotient<VersorEven> for AntiPlane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11        8        1
    //    simd4        0        1        0
    // Totals...
    // yes simd       11        9        1
    //  no simd       11       12        1
    fn geometric_quotient(self, other: VersorEven) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                + f32::powi(other[e1], 2)
                + f32::powi(other[e2], 2)
                + f32::powi(other[e3], 2)
                - f32::powi(other[e12345], 2)
                - f32::powi(other[e321], 2)
                - 2.0 * (other[e5] * other[e4]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]));
    }
}
impl GeometricQuotient<VersorOdd> for AntiPlane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11        8        1
    //    simd4        0        1        0
    // Totals...
    // yes simd       11        9        1
    //  no simd       11       12        1
    fn geometric_quotient(self, other: VersorOdd) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e1234] * other[e3215]) + f32::powi(other[scalar], 2) + f32::powi(other[e45], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - f32::powi(other[e4235], 2)
                - f32::powi(other[e4315], 2)
                - f32::powi(other[e4125], 2)
                - 2.0 * (other[e41] * other[e15])
                - 2.0 * (other[e42] * other[e25])
                - 2.0 * (other[e43] * other[e35]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]));
    }
}
impl std::ops::Div<geometric_quotient> for AntiScalar {
    type Output = geometric_quotient_partial<AntiScalar>;
    fn div(self, _rhs: geometric_quotient) -> Self::Output {
        geometric_quotient_partial(self)
    }
}
impl GeometricQuotient<AntiCircleRotor> for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        7        1
    fn geometric_quotient(self, other: AntiCircleRotor) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e45], 2) + f32::powi(other[scalar], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - 2.0 * (other[e41] * other[e15])
                - 2.0 * (other[e42] * other[e25])
                - 2.0 * (other[e43] * other[e35]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiScalar::from_groups(/* e12345 */ self[e12345] * inverse[scalar]);
    }
}
impl GeometricQuotient<AntiDipoleInversion> for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10        9        1
    fn geometric_quotient(self, other: AntiDipoleInversion) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                + f32::powi(other[e1], 2)
                + f32::powi(other[e2], 2)
                + f32::powi(other[e3], 2)
                - f32::powi(other[e321], 2)
                - 2.0 * (other[e4] * other[e5]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiScalar::from_groups(/* e12345 */ self[e12345] * inverse[scalar]);
    }
}
impl GeometricQuotient<AntiDualNum> for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        1
    fn geometric_quotient(self, other: AntiDualNum) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiScalar::from_groups(/* e12345 */ self[e12345] * inverse[scalar]);
    }
}
impl GeometricQuotient<AntiFlatPoint> for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        1
    fn geometric_quotient(self, other: AntiFlatPoint) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e321], 2) * -1.0);
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiScalar::from_groups(/* e12345 */ self[e12345] * inverse[scalar]);
    }
}
impl GeometricQuotient<AntiFlector> for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        1        1
    fn geometric_quotient(self, other: AntiFlector) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2) - f32::powi(other[e321], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiScalar::from_groups(/* e12345 */ self[e12345] * inverse[scalar]);
    }
}
impl GeometricQuotient<AntiLine> for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        1        1
    fn geometric_quotient(self, other: AntiLine) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ -f32::powi(other[e23], 2) - f32::powi(other[e31], 2) - f32::powi(other[e12], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiScalar::from_groups(/* e12345 */ self[e12345] * inverse[scalar]);
    }
}
impl GeometricQuotient<AntiMotor> for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        1        1
    fn geometric_quotient(self, other: AntiMotor) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[scalar], 2) - f32::powi(other[e23], 2) - f32::powi(other[e31], 2) - f32::powi(other[e12], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiScalar::from_groups(/* e12345 */ self[e12345] * inverse[scalar]);
    }
}
impl GeometricQuotient<AntiPlane> for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        1        1
    fn geometric_quotient(self, other: AntiPlane) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiScalar::from_groups(/* e12345 */ self[e12345] * inverse[scalar]);
    }
}
impl GeometricQuotient<AntiScalar> for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        1
    fn geometric_quotient(self, other: AntiScalar) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e12345], 2) * -1.0);
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiScalar::from_groups(/* e12345 */ self[e12345] * inverse[scalar]);
    }
}
impl GeometricQuotient<Circle> for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        7        1
    fn geometric_quotient(self, other: Circle) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                - f32::powi(other[e321], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiScalar::from_groups(/* e12345 */ self[e12345] * inverse[scalar]);
    }
}
impl GeometricQuotient<CircleRotor> for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        7        1
    fn geometric_quotient(self, other: CircleRotor) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                - f32::powi(other[e321], 2)
                - f32::powi(other[e12345], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiScalar::from_groups(/* e12345 */ self[e12345] * inverse[scalar]);
    }
}
impl GeometricQuotient<Dipole> for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        7        1
    fn geometric_quotient(self, other: Dipole) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e45], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - 2.0 * (other[e41] * other[e15])
                - 2.0 * (other[e42] * other[e25])
                - 2.0 * (other[e43] * other[e35]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiScalar::from_groups(/* e12345 */ self[e12345] * inverse[scalar]);
    }
}
impl GeometricQuotient<DipoleInversion> for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10        9        1
    fn geometric_quotient(self, other: DipoleInversion) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e1234] * other[e3215]) + f32::powi(other[e45], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - f32::powi(other[e4235], 2)
                - f32::powi(other[e4315], 2)
                - f32::powi(other[e4125], 2)
                - 2.0 * (other[e41] * other[e15])
                - 2.0 * (other[e42] * other[e25])
                - 2.0 * (other[e43] * other[e35]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiScalar::from_groups(/* e12345 */ self[e12345] * inverse[scalar]);
    }
}
impl GeometricQuotient<DualNum> for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        1
    fn geometric_quotient(self, other: DualNum) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e12345], 2) * -1.0);
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiScalar::from_groups(/* e12345 */ self[e12345] * inverse[scalar]);
    }
}
impl GeometricQuotient<FlatPoint> for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        1
    fn geometric_quotient(self, other: FlatPoint) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e45], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiScalar::from_groups(/* e12345 */ self[e12345] * inverse[scalar]);
    }
}
impl GeometricQuotient<Flector> for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        1        1
    fn geometric_quotient(self, other: Flector) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e45], 2) - f32::powi(other[e4235], 2) - f32::powi(other[e4315], 2) - f32::powi(other[e4125], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiScalar::from_groups(/* e12345 */ self[e12345] * inverse[scalar]);
    }
}
impl GeometricQuotient<Line> for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        1        1
    fn geometric_quotient(self, other: Line) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e415], 2) + f32::powi(other[e425], 2) + f32::powi(other[e435], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiScalar::from_groups(/* e12345 */ self[e12345] * inverse[scalar]);
    }
}
impl GeometricQuotient<Motor> for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        1        1
    fn geometric_quotient(self, other: Motor) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e415], 2) + f32::powi(other[e425], 2) + f32::powi(other[e435], 2) - f32::powi(other[e12345], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiScalar::from_groups(/* e12345 */ self[e12345] * inverse[scalar]);
    }
}
impl GeometricQuotient<MultiVector> for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       23       17        1
    fn geometric_quotient(self, other: MultiVector) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + 2.0 * (other[e3215] * other[e1234])
                + f32::powi(other[scalar], 2)
                + f32::powi(other[e1], 2)
                + f32::powi(other[e2], 2)
                + f32::powi(other[e3], 2)
                + f32::powi(other[e45], 2)
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                - f32::powi(other[e12345], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - f32::powi(other[e321], 2)
                - f32::powi(other[e4235], 2)
                - f32::powi(other[e4315], 2)
                - f32::powi(other[e4125], 2)
                - 2.0 * (other[e4] * other[e5])
                - 2.0 * (other[e15] * other[e41])
                - 2.0 * (other[e25] * other[e42])
                - 2.0 * (other[e35] * other[e43]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiScalar::from_groups(/* e12345 */ self[e12345] * inverse[scalar]);
    }
}
impl GeometricQuotient<Plane> for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        1        1
    fn geometric_quotient(self, other: Plane) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ -f32::powi(other[e4235], 2) - f32::powi(other[e4315], 2) - f32::powi(other[e4125], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiScalar::from_groups(/* e12345 */ self[e12345] * inverse[scalar]);
    }
}
impl GeometricQuotient<RoundPoint> for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        3        1
    fn geometric_quotient(self, other: RoundPoint) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2) - 2.0 * (other[e4] * other[e5]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiScalar::from_groups(/* e12345 */ self[e12345] * inverse[scalar]);
    }
}
impl GeometricQuotient<Scalar> for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        1
    fn geometric_quotient(self, other: Scalar) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiScalar::from_groups(/* e12345 */ self[e12345] * inverse[scalar]);
    }
}
impl GeometricQuotient<Sphere> for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        3        1
    fn geometric_quotient(self, other: Sphere) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e3215] * other[e1234]) - f32::powi(other[e4235], 2) - f32::powi(other[e4315], 2) - f32::powi(other[e4125], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiScalar::from_groups(/* e12345 */ self[e12345] * inverse[scalar]);
    }
}
impl GeometricQuotient<VersorEven> for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       11        9        1
    fn geometric_quotient(self, other: VersorEven) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                + f32::powi(other[e1], 2)
                + f32::powi(other[e2], 2)
                + f32::powi(other[e3], 2)
                - f32::powi(other[e12345], 2)
                - f32::powi(other[e321], 2)
                - 2.0 * (other[e5] * other[e4]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiScalar::from_groups(/* e12345 */ self[e12345] * inverse[scalar]);
    }
}
impl GeometricQuotient<VersorOdd> for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       11        9        1
    fn geometric_quotient(self, other: VersorOdd) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e1234] * other[e3215]) + f32::powi(other[scalar], 2) + f32::powi(other[e45], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - f32::powi(other[e4235], 2)
                - f32::powi(other[e4315], 2)
                - f32::powi(other[e4125], 2)
                - 2.0 * (other[e41] * other[e15])
                - 2.0 * (other[e42] * other[e25])
                - 2.0 * (other[e43] * other[e35]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiScalar::from_groups(/* e12345 */ self[e12345] * inverse[scalar]);
    }
}
impl std::ops::Div<geometric_quotient> for Circle {
    type Output = geometric_quotient_partial<Circle>;
    fn div(self, _rhs: geometric_quotient) -> Self::Output {
        geometric_quotient_partial(self)
    }
}
impl GeometricQuotient<AntiCircleRotor> for Circle {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        6        1
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        7        9        1
    //  no simd        7       16        1
    fn geometric_quotient(self, other: AntiCircleRotor) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e45], 2) + f32::powi(other[scalar], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - 2.0 * (other[e41] * other[e15])
                - 2.0 * (other[e42] * other[e25])
                - 2.0 * (other[e43] * other[e35]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
        );
    }
}
impl GeometricQuotient<AntiDipoleInversion> for Circle {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10        8        1
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       10       11        1
    //  no simd       10       18        1
    fn geometric_quotient(self, other: AntiDipoleInversion) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                + f32::powi(other[e1], 2)
                + f32::powi(other[e2], 2)
                + f32::powi(other[e3], 2)
                - f32::powi(other[e321], 2)
                - 2.0 * (other[e4] * other[e5]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
        );
    }
}
impl GeometricQuotient<AntiDualNum> for Circle {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        1
    //  no simd        0       10        1
    fn geometric_quotient(self, other: AntiDualNum) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
        );
    }
}
impl GeometricQuotient<AntiFlatPoint> for Circle {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        4        1
    //  no simd        0       11        1
    fn geometric_quotient(self, other: AntiFlatPoint) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e321], 2) * -1.0);
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
        );
    }
}
impl GeometricQuotient<AntiFlector> for Circle {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        3        1
    //  no simd        3       10        1
    fn geometric_quotient(self, other: AntiFlector) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2) - f32::powi(other[e321], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
        );
    }
}
impl GeometricQuotient<AntiLine> for Circle {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        3        1
    //  no simd        2       10        1
    fn geometric_quotient(self, other: AntiLine) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ -f32::powi(other[e23], 2) - f32::powi(other[e31], 2) - f32::powi(other[e12], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
        );
    }
}
impl GeometricQuotient<AntiMotor> for Circle {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        3        1
    //  no simd        3       10        1
    fn geometric_quotient(self, other: AntiMotor) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[scalar], 2) - f32::powi(other[e23], 2) - f32::powi(other[e31], 2) - f32::powi(other[e12], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
        );
    }
}
impl GeometricQuotient<AntiPlane> for Circle {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        3        1
    //  no simd        2       10        1
    fn geometric_quotient(self, other: AntiPlane) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
        );
    }
}
impl GeometricQuotient<AntiScalar> for Circle {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        4        1
    //  no simd        0       11        1
    fn geometric_quotient(self, other: AntiScalar) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e12345], 2) * -1.0);
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
        );
    }
}
impl GeometricQuotient<Circle> for Circle {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        6        1
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        6        9        1
    //  no simd        6       16        1
    fn geometric_quotient(self, other: Circle) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                - f32::powi(other[e321], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
        );
    }
}
impl GeometricQuotient<CircleRotor> for Circle {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        6        1
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        7        9        1
    //  no simd        7       16        1
    fn geometric_quotient(self, other: CircleRotor) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                - f32::powi(other[e321], 2)
                - f32::powi(other[e12345], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
        );
    }
}
impl GeometricQuotient<Dipole> for Circle {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        6        1
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        6        9        1
    //  no simd        6       16        1
    fn geometric_quotient(self, other: Dipole) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e45], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - 2.0 * (other[e41] * other[e15])
                - 2.0 * (other[e42] * other[e25])
                - 2.0 * (other[e43] * other[e35]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
        );
    }
}
impl GeometricQuotient<DipoleInversion> for Circle {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10        8        1
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       10       11        1
    //  no simd       10       18        1
    fn geometric_quotient(self, other: DipoleInversion) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e1234] * other[e3215]) + f32::powi(other[e45], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - f32::powi(other[e4235], 2)
                - f32::powi(other[e4315], 2)
                - f32::powi(other[e4125], 2)
                - 2.0 * (other[e41] * other[e15])
                - 2.0 * (other[e42] * other[e25])
                - 2.0 * (other[e43] * other[e35]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
        );
    }
}
impl GeometricQuotient<DualNum> for Circle {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        4        1
    //  no simd        0       11        1
    fn geometric_quotient(self, other: DualNum) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e12345], 2) * -1.0);
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
        );
    }
}
impl GeometricQuotient<FlatPoint> for Circle {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        1
    //  no simd        0       10        1
    fn geometric_quotient(self, other: FlatPoint) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e45], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
        );
    }
}
impl GeometricQuotient<Flector> for Circle {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        3        1
    //  no simd        3       10        1
    fn geometric_quotient(self, other: Flector) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e45], 2) - f32::powi(other[e4235], 2) - f32::powi(other[e4315], 2) - f32::powi(other[e4125], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
        );
    }
}
impl GeometricQuotient<Line> for Circle {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        3        1
    //  no simd        2       10        1
    fn geometric_quotient(self, other: Line) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e415], 2) + f32::powi(other[e425], 2) + f32::powi(other[e435], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
        );
    }
}
impl GeometricQuotient<Motor> for Circle {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        3        1
    //  no simd        3       10        1
    fn geometric_quotient(self, other: Motor) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e415], 2) + f32::powi(other[e425], 2) + f32::powi(other[e435], 2) - f32::powi(other[e12345], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
        );
    }
}
impl GeometricQuotient<MultiVector> for Circle {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       23       16        1
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       23       19        1
    //  no simd       23       26        1
    fn geometric_quotient(self, other: MultiVector) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + 2.0 * (other[e3215] * other[e1234])
                + f32::powi(other[scalar], 2)
                + f32::powi(other[e1], 2)
                + f32::powi(other[e2], 2)
                + f32::powi(other[e3], 2)
                + f32::powi(other[e45], 2)
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                - f32::powi(other[e12345], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - f32::powi(other[e321], 2)
                - f32::powi(other[e4235], 2)
                - f32::powi(other[e4315], 2)
                - f32::powi(other[e4125], 2)
                - 2.0 * (other[e4] * other[e5])
                - 2.0 * (other[e15] * other[e41])
                - 2.0 * (other[e25] * other[e42])
                - 2.0 * (other[e35] * other[e43]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
        );
    }
}
impl GeometricQuotient<Plane> for Circle {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        3        1
    //  no simd        2       10        1
    fn geometric_quotient(self, other: Plane) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ -f32::powi(other[e4235], 2) - f32::powi(other[e4315], 2) - f32::powi(other[e4125], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
        );
    }
}
impl GeometricQuotient<RoundPoint> for Circle {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        2        1
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        1
    //  no simd        3       12        1
    fn geometric_quotient(self, other: RoundPoint) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2) - 2.0 * (other[e4] * other[e5]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
        );
    }
}
impl GeometricQuotient<Scalar> for Circle {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        1
    //  no simd        0       10        1
    fn geometric_quotient(self, other: Scalar) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
        );
    }
}
impl GeometricQuotient<Sphere> for Circle {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        2        1
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        1
    //  no simd        3       12        1
    fn geometric_quotient(self, other: Sphere) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e3215] * other[e1234]) - f32::powi(other[e4235], 2) - f32::powi(other[e4315], 2) - f32::powi(other[e4125], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
        );
    }
}
impl GeometricQuotient<VersorEven> for Circle {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11        8        1
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       11       11        1
    //  no simd       11       18        1
    fn geometric_quotient(self, other: VersorEven) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                + f32::powi(other[e1], 2)
                + f32::powi(other[e2], 2)
                + f32::powi(other[e3], 2)
                - f32::powi(other[e12345], 2)
                - f32::powi(other[e321], 2)
                - 2.0 * (other[e5] * other[e4]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
        );
    }
}
impl GeometricQuotient<VersorOdd> for Circle {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11        8        1
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       11       11        1
    //  no simd       11       18        1
    fn geometric_quotient(self, other: VersorOdd) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e1234] * other[e3215]) + f32::powi(other[scalar], 2) + f32::powi(other[e45], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - f32::powi(other[e4235], 2)
                - f32::powi(other[e4315], 2)
                - f32::powi(other[e4125], 2)
                - 2.0 * (other[e41] * other[e15])
                - 2.0 * (other[e42] * other[e25])
                - 2.0 * (other[e43] * other[e35]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
        );
    }
}
impl std::ops::Div<geometric_quotient> for CircleRotor {
    type Output = geometric_quotient_partial<CircleRotor>;
    fn div(self, _rhs: geometric_quotient) -> Self::Output {
        geometric_quotient_partial(self)
    }
}
impl GeometricQuotient<AntiCircleRotor> for CircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        6        1
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        7        9        1
    //  no simd        7       17        1
    fn geometric_quotient(self, other: AntiCircleRotor) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e45], 2) + f32::powi(other[scalar], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - 2.0 * (other[e41] * other[e15])
                - 2.0 * (other[e42] * other[e25])
                - 2.0 * (other[e43] * other[e35]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e12345
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e12345]]),
        );
    }
}
impl GeometricQuotient<AntiDipoleInversion> for CircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10        8        1
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       10       11        1
    //  no simd       10       19        1
    fn geometric_quotient(self, other: AntiDipoleInversion) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                + f32::powi(other[e1], 2)
                + f32::powi(other[e2], 2)
                + f32::powi(other[e3], 2)
                - f32::powi(other[e321], 2)
                - 2.0 * (other[e4] * other[e5]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e12345
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e12345]]),
        );
    }
}
impl GeometricQuotient<AntiDualNum> for CircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        1
    //  no simd        0       11        1
    fn geometric_quotient(self, other: AntiDualNum) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e12345
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e12345]]),
        );
    }
}
impl GeometricQuotient<AntiFlatPoint> for CircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        4        1
    //  no simd        0       12        1
    fn geometric_quotient(self, other: AntiFlatPoint) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e321], 2) * -1.0);
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e12345
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e12345]]),
        );
    }
}
impl GeometricQuotient<AntiFlector> for CircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        3        1
    //  no simd        3       11        1
    fn geometric_quotient(self, other: AntiFlector) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2) - f32::powi(other[e321], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e12345
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e12345]]),
        );
    }
}
impl GeometricQuotient<AntiLine> for CircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        2        3        1
    //  no simd        2       11        1
    fn geometric_quotient(self, other: AntiLine) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ -f32::powi(other[e23], 2) - f32::powi(other[e31], 2) - f32::powi(other[e12], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e12345
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e12345]]),
        );
    }
}
impl GeometricQuotient<AntiMotor> for CircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        3        1
    //  no simd        3       11        1
    fn geometric_quotient(self, other: AntiMotor) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[scalar], 2) - f32::powi(other[e23], 2) - f32::powi(other[e31], 2) - f32::powi(other[e12], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e12345
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e12345]]),
        );
    }
}
impl GeometricQuotient<AntiPlane> for CircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        2        3        1
    //  no simd        2       11        1
    fn geometric_quotient(self, other: AntiPlane) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e12345
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e12345]]),
        );
    }
}
impl GeometricQuotient<AntiScalar> for CircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        4        1
    //  no simd        0       12        1
    fn geometric_quotient(self, other: AntiScalar) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e12345], 2) * -1.0);
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e12345
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e12345]]),
        );
    }
}
impl GeometricQuotient<Circle> for CircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        6        1
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        6        9        1
    //  no simd        6       17        1
    fn geometric_quotient(self, other: Circle) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                - f32::powi(other[e321], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e12345
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e12345]]),
        );
    }
}
impl GeometricQuotient<CircleRotor> for CircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        6        1
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        7        9        1
    //  no simd        7       17        1
    fn geometric_quotient(self, other: CircleRotor) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                - f32::powi(other[e321], 2)
                - f32::powi(other[e12345], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e12345
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e12345]]),
        );
    }
}
impl GeometricQuotient<Dipole> for CircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        6        1
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        6        9        1
    //  no simd        6       17        1
    fn geometric_quotient(self, other: Dipole) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e45], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - 2.0 * (other[e41] * other[e15])
                - 2.0 * (other[e42] * other[e25])
                - 2.0 * (other[e43] * other[e35]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e12345
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e12345]]),
        );
    }
}
impl GeometricQuotient<DipoleInversion> for CircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10        8        1
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       10       11        1
    //  no simd       10       19        1
    fn geometric_quotient(self, other: DipoleInversion) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e1234] * other[e3215]) + f32::powi(other[e45], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - f32::powi(other[e4235], 2)
                - f32::powi(other[e4315], 2)
                - f32::powi(other[e4125], 2)
                - 2.0 * (other[e41] * other[e15])
                - 2.0 * (other[e42] * other[e25])
                - 2.0 * (other[e43] * other[e35]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e12345
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e12345]]),
        );
    }
}
impl GeometricQuotient<DualNum> for CircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        4        1
    //  no simd        0       12        1
    fn geometric_quotient(self, other: DualNum) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e12345], 2) * -1.0);
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e12345
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e12345]]),
        );
    }
}
impl GeometricQuotient<FlatPoint> for CircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        1
    //  no simd        0       11        1
    fn geometric_quotient(self, other: FlatPoint) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e45], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e12345
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e12345]]),
        );
    }
}
impl GeometricQuotient<Flector> for CircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        3        1
    //  no simd        3       11        1
    fn geometric_quotient(self, other: Flector) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e45], 2) - f32::powi(other[e4235], 2) - f32::powi(other[e4315], 2) - f32::powi(other[e4125], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e12345
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e12345]]),
        );
    }
}
impl GeometricQuotient<Line> for CircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        2        3        1
    //  no simd        2       11        1
    fn geometric_quotient(self, other: Line) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e415], 2) + f32::powi(other[e425], 2) + f32::powi(other[e435], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e12345
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e12345]]),
        );
    }
}
impl GeometricQuotient<Motor> for CircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        3        1
    //  no simd        3       11        1
    fn geometric_quotient(self, other: Motor) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e415], 2) + f32::powi(other[e425], 2) + f32::powi(other[e435], 2) - f32::powi(other[e12345], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e12345
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e12345]]),
        );
    }
}
impl GeometricQuotient<MultiVector> for CircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       23       16        1
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       23       19        1
    //  no simd       23       27        1
    fn geometric_quotient(self, other: MultiVector) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + 2.0 * (other[e3215] * other[e1234])
                + f32::powi(other[scalar], 2)
                + f32::powi(other[e1], 2)
                + f32::powi(other[e2], 2)
                + f32::powi(other[e3], 2)
                + f32::powi(other[e45], 2)
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                - f32::powi(other[e12345], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - f32::powi(other[e321], 2)
                - f32::powi(other[e4235], 2)
                - f32::powi(other[e4315], 2)
                - f32::powi(other[e4125], 2)
                - 2.0 * (other[e4] * other[e5])
                - 2.0 * (other[e15] * other[e41])
                - 2.0 * (other[e25] * other[e42])
                - 2.0 * (other[e35] * other[e43]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e12345
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e12345]]),
        );
    }
}
impl GeometricQuotient<Plane> for CircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        2        3        1
    //  no simd        2       11        1
    fn geometric_quotient(self, other: Plane) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ -f32::powi(other[e4235], 2) - f32::powi(other[e4315], 2) - f32::powi(other[e4125], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e12345
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e12345]]),
        );
    }
}
impl GeometricQuotient<RoundPoint> for CircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        2        1
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        5        1
    //  no simd        3       13        1
    fn geometric_quotient(self, other: RoundPoint) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2) - 2.0 * (other[e4] * other[e5]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e12345
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e12345]]),
        );
    }
}
impl GeometricQuotient<Scalar> for CircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        1
    //  no simd        0       11        1
    fn geometric_quotient(self, other: Scalar) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e12345
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e12345]]),
        );
    }
}
impl GeometricQuotient<Sphere> for CircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        2        1
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        5        1
    //  no simd        3       13        1
    fn geometric_quotient(self, other: Sphere) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e3215] * other[e1234]) - f32::powi(other[e4235], 2) - f32::powi(other[e4315], 2) - f32::powi(other[e4125], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e12345
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e12345]]),
        );
    }
}
impl GeometricQuotient<VersorEven> for CircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11        8        1
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       11       11        1
    //  no simd       11       19        1
    fn geometric_quotient(self, other: VersorEven) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                + f32::powi(other[e1], 2)
                + f32::powi(other[e2], 2)
                + f32::powi(other[e3], 2)
                - f32::powi(other[e12345], 2)
                - f32::powi(other[e321], 2)
                - 2.0 * (other[e5] * other[e4]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e12345
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e12345]]),
        );
    }
}
impl GeometricQuotient<VersorOdd> for CircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11        8        1
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       11       11        1
    //  no simd       11       19        1
    fn geometric_quotient(self, other: VersorOdd) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e1234] * other[e3215]) + f32::powi(other[scalar], 2) + f32::powi(other[e45], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - f32::powi(other[e4235], 2)
                - f32::powi(other[e4315], 2)
                - f32::powi(other[e4125], 2)
                - 2.0 * (other[e41] * other[e15])
                - 2.0 * (other[e42] * other[e25])
                - 2.0 * (other[e43] * other[e35]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e12345
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e12345]]),
        );
    }
}
impl std::ops::Div<geometric_quotient> for Dipole {
    type Output = geometric_quotient_partial<Dipole>;
    fn div(self, _rhs: geometric_quotient) -> Self::Output {
        geometric_quotient_partial(self)
    }
}
impl GeometricQuotient<AntiCircleRotor> for Dipole {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        6        1
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        7        9        1
    //  no simd        7       16        1
    fn geometric_quotient(self, other: AntiCircleRotor) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e45], 2) + f32::powi(other[scalar], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - 2.0 * (other[e41] * other[e15])
                - 2.0 * (other[e42] * other[e25])
                - 2.0 * (other[e43] * other[e35]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e15], self[e25], self[e35]]),
        );
    }
}
impl GeometricQuotient<AntiDipoleInversion> for Dipole {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10        8        1
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       10       11        1
    //  no simd       10       18        1
    fn geometric_quotient(self, other: AntiDipoleInversion) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                + f32::powi(other[e1], 2)
                + f32::powi(other[e2], 2)
                + f32::powi(other[e3], 2)
                - f32::powi(other[e321], 2)
                - 2.0 * (other[e4] * other[e5]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e15], self[e25], self[e35]]),
        );
    }
}
impl GeometricQuotient<AntiDualNum> for Dipole {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        1
    //  no simd        0       10        1
    fn geometric_quotient(self, other: AntiDualNum) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e15], self[e25], self[e35]]),
        );
    }
}
impl GeometricQuotient<AntiFlatPoint> for Dipole {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        4        1
    //  no simd        0       11        1
    fn geometric_quotient(self, other: AntiFlatPoint) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e321], 2) * -1.0);
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e15], self[e25], self[e35]]),
        );
    }
}
impl GeometricQuotient<AntiFlector> for Dipole {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        3        1
    //  no simd        3       10        1
    fn geometric_quotient(self, other: AntiFlector) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2) - f32::powi(other[e321], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e15], self[e25], self[e35]]),
        );
    }
}
impl GeometricQuotient<AntiLine> for Dipole {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        3        1
    //  no simd        2       10        1
    fn geometric_quotient(self, other: AntiLine) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ -f32::powi(other[e23], 2) - f32::powi(other[e31], 2) - f32::powi(other[e12], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e15], self[e25], self[e35]]),
        );
    }
}
impl GeometricQuotient<AntiMotor> for Dipole {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        3        1
    //  no simd        3       10        1
    fn geometric_quotient(self, other: AntiMotor) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[scalar], 2) - f32::powi(other[e23], 2) - f32::powi(other[e31], 2) - f32::powi(other[e12], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e15], self[e25], self[e35]]),
        );
    }
}
impl GeometricQuotient<AntiPlane> for Dipole {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        3        1
    //  no simd        2       10        1
    fn geometric_quotient(self, other: AntiPlane) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e15], self[e25], self[e35]]),
        );
    }
}
impl GeometricQuotient<AntiScalar> for Dipole {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        4        1
    //  no simd        0       11        1
    fn geometric_quotient(self, other: AntiScalar) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e12345], 2) * -1.0);
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e15], self[e25], self[e35]]),
        );
    }
}
impl GeometricQuotient<Circle> for Dipole {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        6        1
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        6        9        1
    //  no simd        6       16        1
    fn geometric_quotient(self, other: Circle) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                - f32::powi(other[e321], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e15], self[e25], self[e35]]),
        );
    }
}
impl GeometricQuotient<CircleRotor> for Dipole {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        6        1
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        7        9        1
    //  no simd        7       16        1
    fn geometric_quotient(self, other: CircleRotor) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                - f32::powi(other[e321], 2)
                - f32::powi(other[e12345], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e15], self[e25], self[e35]]),
        );
    }
}
impl GeometricQuotient<Dipole> for Dipole {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        6        1
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        6        9        1
    //  no simd        6       16        1
    fn geometric_quotient(self, other: Dipole) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e45], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - 2.0 * (other[e41] * other[e15])
                - 2.0 * (other[e42] * other[e25])
                - 2.0 * (other[e43] * other[e35]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e15], self[e25], self[e35]]),
        );
    }
}
impl GeometricQuotient<DipoleInversion> for Dipole {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10        8        1
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       10       11        1
    //  no simd       10       18        1
    fn geometric_quotient(self, other: DipoleInversion) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e1234] * other[e3215]) + f32::powi(other[e45], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - f32::powi(other[e4235], 2)
                - f32::powi(other[e4315], 2)
                - f32::powi(other[e4125], 2)
                - 2.0 * (other[e41] * other[e15])
                - 2.0 * (other[e42] * other[e25])
                - 2.0 * (other[e43] * other[e35]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e15], self[e25], self[e35]]),
        );
    }
}
impl GeometricQuotient<DualNum> for Dipole {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        4        1
    //  no simd        0       11        1
    fn geometric_quotient(self, other: DualNum) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e12345], 2) * -1.0);
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e15], self[e25], self[e35]]),
        );
    }
}
impl GeometricQuotient<FlatPoint> for Dipole {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        1
    //  no simd        0       10        1
    fn geometric_quotient(self, other: FlatPoint) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e45], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e15], self[e25], self[e35]]),
        );
    }
}
impl GeometricQuotient<Flector> for Dipole {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        3        1
    //  no simd        3       10        1
    fn geometric_quotient(self, other: Flector) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e45], 2) - f32::powi(other[e4235], 2) - f32::powi(other[e4315], 2) - f32::powi(other[e4125], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e15], self[e25], self[e35]]),
        );
    }
}
impl GeometricQuotient<Line> for Dipole {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        3        1
    //  no simd        2       10        1
    fn geometric_quotient(self, other: Line) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e415], 2) + f32::powi(other[e425], 2) + f32::powi(other[e435], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e15], self[e25], self[e35]]),
        );
    }
}
impl GeometricQuotient<Motor> for Dipole {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        3        1
    //  no simd        3       10        1
    fn geometric_quotient(self, other: Motor) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e415], 2) + f32::powi(other[e425], 2) + f32::powi(other[e435], 2) - f32::powi(other[e12345], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e15], self[e25], self[e35]]),
        );
    }
}
impl GeometricQuotient<MultiVector> for Dipole {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       23       16        1
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       23       19        1
    //  no simd       23       26        1
    fn geometric_quotient(self, other: MultiVector) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + 2.0 * (other[e3215] * other[e1234])
                + f32::powi(other[scalar], 2)
                + f32::powi(other[e1], 2)
                + f32::powi(other[e2], 2)
                + f32::powi(other[e3], 2)
                + f32::powi(other[e45], 2)
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                - f32::powi(other[e12345], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - f32::powi(other[e321], 2)
                - f32::powi(other[e4235], 2)
                - f32::powi(other[e4315], 2)
                - f32::powi(other[e4125], 2)
                - 2.0 * (other[e4] * other[e5])
                - 2.0 * (other[e15] * other[e41])
                - 2.0 * (other[e25] * other[e42])
                - 2.0 * (other[e35] * other[e43]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e15], self[e25], self[e35]]),
        );
    }
}
impl GeometricQuotient<Plane> for Dipole {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        3        1
    //  no simd        2       10        1
    fn geometric_quotient(self, other: Plane) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ -f32::powi(other[e4235], 2) - f32::powi(other[e4315], 2) - f32::powi(other[e4125], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e15], self[e25], self[e35]]),
        );
    }
}
impl GeometricQuotient<RoundPoint> for Dipole {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        2        1
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        1
    //  no simd        3       12        1
    fn geometric_quotient(self, other: RoundPoint) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2) - 2.0 * (other[e4] * other[e5]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e15], self[e25], self[e35]]),
        );
    }
}
impl GeometricQuotient<Scalar> for Dipole {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        1
    //  no simd        0       10        1
    fn geometric_quotient(self, other: Scalar) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e15], self[e25], self[e35]]),
        );
    }
}
impl GeometricQuotient<Sphere> for Dipole {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        2        1
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        1
    //  no simd        3       12        1
    fn geometric_quotient(self, other: Sphere) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e3215] * other[e1234]) - f32::powi(other[e4235], 2) - f32::powi(other[e4315], 2) - f32::powi(other[e4125], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e15], self[e25], self[e35]]),
        );
    }
}
impl GeometricQuotient<VersorEven> for Dipole {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11        8        1
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       11       11        1
    //  no simd       11       18        1
    fn geometric_quotient(self, other: VersorEven) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                + f32::powi(other[e1], 2)
                + f32::powi(other[e2], 2)
                + f32::powi(other[e3], 2)
                - f32::powi(other[e12345], 2)
                - f32::powi(other[e321], 2)
                - 2.0 * (other[e5] * other[e4]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e15], self[e25], self[e35]]),
        );
    }
}
impl GeometricQuotient<VersorOdd> for Dipole {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11        8        1
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       11       11        1
    //  no simd       11       18        1
    fn geometric_quotient(self, other: VersorOdd) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e1234] * other[e3215]) + f32::powi(other[scalar], 2) + f32::powi(other[e45], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - f32::powi(other[e4235], 2)
                - f32::powi(other[e4315], 2)
                - f32::powi(other[e4125], 2)
                - 2.0 * (other[e41] * other[e15])
                - 2.0 * (other[e42] * other[e25])
                - 2.0 * (other[e43] * other[e35]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e15], self[e25], self[e35]]),
        );
    }
}
impl std::ops::Div<geometric_quotient> for DipoleInversion {
    type Output = geometric_quotient_partial<DipoleInversion>;
    fn div(self, _rhs: geometric_quotient) -> Self::Output {
        geometric_quotient_partial(self)
    }
}
impl GeometricQuotient<AntiCircleRotor> for DipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        6        1
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        7       10        1
    //  no simd        7       21        1
    fn geometric_quotient(self, other: AntiCircleRotor) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e45], 2) + f32::powi(other[scalar], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - 2.0 * (other[e41] * other[e15])
                - 2.0 * (other[e42] * other[e25])
                - 2.0 * (other[e43] * other[e35]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricQuotient<AntiDipoleInversion> for DipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10        8        1
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd       10       12        1
    //  no simd       10       23        1
    fn geometric_quotient(self, other: AntiDipoleInversion) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                + f32::powi(other[e1], 2)
                + f32::powi(other[e2], 2)
                + f32::powi(other[e3], 2)
                - f32::powi(other[e321], 2)
                - 2.0 * (other[e4] * other[e5]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricQuotient<AntiDualNum> for DipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        4        1
    //  no simd        0       15        1
    fn geometric_quotient(self, other: AntiDualNum) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricQuotient<AntiFlatPoint> for DipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        5        1
    //  no simd        0       16        1
    fn geometric_quotient(self, other: AntiFlatPoint) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e321], 2) * -1.0);
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricQuotient<AntiFlector> for DipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        3        4        1
    //  no simd        3       15        1
    fn geometric_quotient(self, other: AntiFlector) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2) - f32::powi(other[e321], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricQuotient<AntiLine> for DipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        2        4        1
    //  no simd        2       15        1
    fn geometric_quotient(self, other: AntiLine) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ -f32::powi(other[e23], 2) - f32::powi(other[e31], 2) - f32::powi(other[e12], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricQuotient<AntiMotor> for DipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        3        4        1
    //  no simd        3       15        1
    fn geometric_quotient(self, other: AntiMotor) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[scalar], 2) - f32::powi(other[e23], 2) - f32::powi(other[e31], 2) - f32::powi(other[e12], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricQuotient<AntiPlane> for DipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        2        4        1
    //  no simd        2       15        1
    fn geometric_quotient(self, other: AntiPlane) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricQuotient<AntiScalar> for DipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        5        1
    //  no simd        0       16        1
    fn geometric_quotient(self, other: AntiScalar) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e12345], 2) * -1.0);
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricQuotient<Circle> for DipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        6        1
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        6       10        1
    //  no simd        6       21        1
    fn geometric_quotient(self, other: Circle) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                - f32::powi(other[e321], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricQuotient<CircleRotor> for DipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        6        1
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        7       10        1
    //  no simd        7       21        1
    fn geometric_quotient(self, other: CircleRotor) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                - f32::powi(other[e321], 2)
                - f32::powi(other[e12345], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricQuotient<Dipole> for DipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        6        1
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        6       10        1
    //  no simd        6       21        1
    fn geometric_quotient(self, other: Dipole) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e45], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - 2.0 * (other[e41] * other[e15])
                - 2.0 * (other[e42] * other[e25])
                - 2.0 * (other[e43] * other[e35]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricQuotient<DipoleInversion> for DipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10        8        1
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd       10       12        1
    //  no simd       10       23        1
    fn geometric_quotient(self, other: DipoleInversion) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e1234] * other[e3215]) + f32::powi(other[e45], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - f32::powi(other[e4235], 2)
                - f32::powi(other[e4315], 2)
                - f32::powi(other[e4125], 2)
                - 2.0 * (other[e41] * other[e15])
                - 2.0 * (other[e42] * other[e25])
                - 2.0 * (other[e43] * other[e35]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricQuotient<DualNum> for DipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        5        1
    //  no simd        0       16        1
    fn geometric_quotient(self, other: DualNum) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e12345], 2) * -1.0);
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricQuotient<FlatPoint> for DipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        4        1
    //  no simd        0       15        1
    fn geometric_quotient(self, other: FlatPoint) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e45], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricQuotient<Flector> for DipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        3        4        1
    //  no simd        3       15        1
    fn geometric_quotient(self, other: Flector) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e45], 2) - f32::powi(other[e4235], 2) - f32::powi(other[e4315], 2) - f32::powi(other[e4125], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricQuotient<Line> for DipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        2        4        1
    //  no simd        2       15        1
    fn geometric_quotient(self, other: Line) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e415], 2) + f32::powi(other[e425], 2) + f32::powi(other[e435], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricQuotient<Motor> for DipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        3        4        1
    //  no simd        3       15        1
    fn geometric_quotient(self, other: Motor) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e415], 2) + f32::powi(other[e425], 2) + f32::powi(other[e435], 2) - f32::powi(other[e12345], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricQuotient<MultiVector> for DipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       23       16        1
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd       23       20        1
    //  no simd       23       31        1
    fn geometric_quotient(self, other: MultiVector) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + 2.0 * (other[e3215] * other[e1234])
                + f32::powi(other[scalar], 2)
                + f32::powi(other[e1], 2)
                + f32::powi(other[e2], 2)
                + f32::powi(other[e3], 2)
                + f32::powi(other[e45], 2)
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                - f32::powi(other[e12345], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - f32::powi(other[e321], 2)
                - f32::powi(other[e4235], 2)
                - f32::powi(other[e4315], 2)
                - f32::powi(other[e4125], 2)
                - 2.0 * (other[e4] * other[e5])
                - 2.0 * (other[e15] * other[e41])
                - 2.0 * (other[e25] * other[e42])
                - 2.0 * (other[e35] * other[e43]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricQuotient<Plane> for DipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        2        4        1
    //  no simd        2       15        1
    fn geometric_quotient(self, other: Plane) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ -f32::powi(other[e4235], 2) - f32::powi(other[e4315], 2) - f32::powi(other[e4125], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricQuotient<RoundPoint> for DipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        2        1
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        3        6        1
    //  no simd        3       17        1
    fn geometric_quotient(self, other: RoundPoint) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2) - 2.0 * (other[e4] * other[e5]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricQuotient<Scalar> for DipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        4        1
    //  no simd        0       15        1
    fn geometric_quotient(self, other: Scalar) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricQuotient<Sphere> for DipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        2        1
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        3        6        1
    //  no simd        3       17        1
    fn geometric_quotient(self, other: Sphere) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e3215] * other[e1234]) - f32::powi(other[e4235], 2) - f32::powi(other[e4315], 2) - f32::powi(other[e4125], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricQuotient<VersorEven> for DipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11        8        1
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd       11       12        1
    //  no simd       11       23        1
    fn geometric_quotient(self, other: VersorEven) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                + f32::powi(other[e1], 2)
                + f32::powi(other[e2], 2)
                + f32::powi(other[e3], 2)
                - f32::powi(other[e12345], 2)
                - f32::powi(other[e321], 2)
                - 2.0 * (other[e5] * other[e4]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricQuotient<VersorOdd> for DipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11        8        1
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd       11       12        1
    //  no simd       11       23        1
    fn geometric_quotient(self, other: VersorOdd) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e1234] * other[e3215]) + f32::powi(other[scalar], 2) + f32::powi(other[e45], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - f32::powi(other[e4235], 2)
                - f32::powi(other[e4315], 2)
                - f32::powi(other[e4125], 2)
                - 2.0 * (other[e41] * other[e15])
                - 2.0 * (other[e42] * other[e25])
                - 2.0 * (other[e43] * other[e35]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl std::ops::Div<geometric_quotient> for DualNum {
    type Output = geometric_quotient_partial<DualNum>;
    fn div(self, _rhs: geometric_quotient) -> Self::Output {
        geometric_quotient_partial(self)
    }
}
impl GeometricQuotient<AntiCircleRotor> for DualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        6        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        7        7        1
    //  no simd        7        8        1
    fn geometric_quotient(self, other: AntiCircleRotor) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e45], 2) + f32::powi(other[scalar], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - 2.0 * (other[e41] * other[e15])
                - 2.0 * (other[e42] * other[e25])
                - 2.0 * (other[e43] * other[e35]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return DualNum::from_groups(/* e5, e12345 */ Simd32x2::from(inverse[scalar]) * Simd32x2::from([self[e5], self[e12345]]));
    }
}
impl GeometricQuotient<AntiDipoleInversion> for DualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10        8        1
    //    simd2        0        1        0
    // Totals...
    // yes simd       10        9        1
    //  no simd       10       10        1
    fn geometric_quotient(self, other: AntiDipoleInversion) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                + f32::powi(other[e1], 2)
                + f32::powi(other[e2], 2)
                + f32::powi(other[e3], 2)
                - f32::powi(other[e321], 2)
                - 2.0 * (other[e4] * other[e5]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return DualNum::from_groups(/* e5, e12345 */ Simd32x2::from(inverse[scalar]) * Simd32x2::from([self[e5], self[e12345]]));
    }
}
impl GeometricQuotient<AntiDualNum> for DualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        1        1
    //  no simd        0        2        1
    fn geometric_quotient(self, other: AntiDualNum) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return DualNum::from_groups(/* e5, e12345 */ Simd32x2::from(inverse[scalar]) * Simd32x2::from([self[e5], self[e12345]]));
    }
}
impl GeometricQuotient<AntiFlatPoint> for DualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        3        1
    fn geometric_quotient(self, other: AntiFlatPoint) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e321], 2) * -1.0);
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return DualNum::from_groups(/* e5, e12345 */ Simd32x2::from(inverse[scalar]) * Simd32x2::from([self[e5], self[e12345]]));
    }
}
impl GeometricQuotient<AntiFlector> for DualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        3        1        1
    //  no simd        3        2        1
    fn geometric_quotient(self, other: AntiFlector) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2) - f32::powi(other[e321], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return DualNum::from_groups(/* e5, e12345 */ Simd32x2::from(inverse[scalar]) * Simd32x2::from([self[e5], self[e12345]]));
    }
}
impl GeometricQuotient<AntiLine> for DualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        2        1        1
    //  no simd        2        2        1
    fn geometric_quotient(self, other: AntiLine) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ -f32::powi(other[e23], 2) - f32::powi(other[e31], 2) - f32::powi(other[e12], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return DualNum::from_groups(/* e5, e12345 */ Simd32x2::from(inverse[scalar]) * Simd32x2::from([self[e5], self[e12345]]));
    }
}
impl GeometricQuotient<AntiMotor> for DualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        3        1        1
    //  no simd        3        2        1
    fn geometric_quotient(self, other: AntiMotor) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[scalar], 2) - f32::powi(other[e23], 2) - f32::powi(other[e31], 2) - f32::powi(other[e12], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return DualNum::from_groups(/* e5, e12345 */ Simd32x2::from(inverse[scalar]) * Simd32x2::from([self[e5], self[e12345]]));
    }
}
impl GeometricQuotient<AntiPlane> for DualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        2        1        1
    //  no simd        2        2        1
    fn geometric_quotient(self, other: AntiPlane) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return DualNum::from_groups(/* e5, e12345 */ Simd32x2::from(inverse[scalar]) * Simd32x2::from([self[e5], self[e12345]]));
    }
}
impl GeometricQuotient<AntiScalar> for DualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        3        1
    fn geometric_quotient(self, other: AntiScalar) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e12345], 2) * -1.0);
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return DualNum::from_groups(/* e5, e12345 */ Simd32x2::from(inverse[scalar]) * Simd32x2::from([self[e5], self[e12345]]));
    }
}
impl GeometricQuotient<Circle> for DualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        6        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        6        7        1
    //  no simd        6        8        1
    fn geometric_quotient(self, other: Circle) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                - f32::powi(other[e321], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return DualNum::from_groups(/* e5, e12345 */ Simd32x2::from(inverse[scalar]) * Simd32x2::from([self[e5], self[e12345]]));
    }
}
impl GeometricQuotient<CircleRotor> for DualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        6        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        7        7        1
    //  no simd        7        8        1
    fn geometric_quotient(self, other: CircleRotor) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                - f32::powi(other[e321], 2)
                - f32::powi(other[e12345], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return DualNum::from_groups(/* e5, e12345 */ Simd32x2::from(inverse[scalar]) * Simd32x2::from([self[e5], self[e12345]]));
    }
}
impl GeometricQuotient<Dipole> for DualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        6        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        6        7        1
    //  no simd        6        8        1
    fn geometric_quotient(self, other: Dipole) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e45], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - 2.0 * (other[e41] * other[e15])
                - 2.0 * (other[e42] * other[e25])
                - 2.0 * (other[e43] * other[e35]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return DualNum::from_groups(/* e5, e12345 */ Simd32x2::from(inverse[scalar]) * Simd32x2::from([self[e5], self[e12345]]));
    }
}
impl GeometricQuotient<DipoleInversion> for DualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10        8        1
    //    simd2        0        1        0
    // Totals...
    // yes simd       10        9        1
    //  no simd       10       10        1
    fn geometric_quotient(self, other: DipoleInversion) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e1234] * other[e3215]) + f32::powi(other[e45], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - f32::powi(other[e4235], 2)
                - f32::powi(other[e4315], 2)
                - f32::powi(other[e4125], 2)
                - 2.0 * (other[e41] * other[e15])
                - 2.0 * (other[e42] * other[e25])
                - 2.0 * (other[e43] * other[e35]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return DualNum::from_groups(/* e5, e12345 */ Simd32x2::from(inverse[scalar]) * Simd32x2::from([self[e5], self[e12345]]));
    }
}
impl GeometricQuotient<DualNum> for DualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        3        1
    fn geometric_quotient(self, other: DualNum) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e12345], 2) * -1.0);
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return DualNum::from_groups(/* e5, e12345 */ Simd32x2::from(inverse[scalar]) * Simd32x2::from([self[e5], self[e12345]]));
    }
}
impl GeometricQuotient<FlatPoint> for DualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        1        1
    //  no simd        0        2        1
    fn geometric_quotient(self, other: FlatPoint) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e45], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return DualNum::from_groups(/* e5, e12345 */ Simd32x2::from(inverse[scalar]) * Simd32x2::from([self[e5], self[e12345]]));
    }
}
impl GeometricQuotient<Flector> for DualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        3        1        1
    //  no simd        3        2        1
    fn geometric_quotient(self, other: Flector) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e45], 2) - f32::powi(other[e4235], 2) - f32::powi(other[e4315], 2) - f32::powi(other[e4125], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return DualNum::from_groups(/* e5, e12345 */ Simd32x2::from(inverse[scalar]) * Simd32x2::from([self[e5], self[e12345]]));
    }
}
impl GeometricQuotient<Line> for DualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        2        1        1
    //  no simd        2        2        1
    fn geometric_quotient(self, other: Line) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e415], 2) + f32::powi(other[e425], 2) + f32::powi(other[e435], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return DualNum::from_groups(/* e5, e12345 */ Simd32x2::from(inverse[scalar]) * Simd32x2::from([self[e5], self[e12345]]));
    }
}
impl GeometricQuotient<Motor> for DualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        3        1        1
    //  no simd        3        2        1
    fn geometric_quotient(self, other: Motor) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e415], 2) + f32::powi(other[e425], 2) + f32::powi(other[e435], 2) - f32::powi(other[e12345], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return DualNum::from_groups(/* e5, e12345 */ Simd32x2::from(inverse[scalar]) * Simd32x2::from([self[e5], self[e12345]]));
    }
}
impl GeometricQuotient<MultiVector> for DualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       23       16        1
    //    simd2        0        1        0
    // Totals...
    // yes simd       23       17        1
    //  no simd       23       18        1
    fn geometric_quotient(self, other: MultiVector) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + 2.0 * (other[e3215] * other[e1234])
                + f32::powi(other[scalar], 2)
                + f32::powi(other[e1], 2)
                + f32::powi(other[e2], 2)
                + f32::powi(other[e3], 2)
                + f32::powi(other[e45], 2)
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                - f32::powi(other[e12345], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - f32::powi(other[e321], 2)
                - f32::powi(other[e4235], 2)
                - f32::powi(other[e4315], 2)
                - f32::powi(other[e4125], 2)
                - 2.0 * (other[e4] * other[e5])
                - 2.0 * (other[e15] * other[e41])
                - 2.0 * (other[e25] * other[e42])
                - 2.0 * (other[e35] * other[e43]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return DualNum::from_groups(/* e5, e12345 */ Simd32x2::from(inverse[scalar]) * Simd32x2::from([self[e5], self[e12345]]));
    }
}
impl GeometricQuotient<Plane> for DualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        2        1        1
    //  no simd        2        2        1
    fn geometric_quotient(self, other: Plane) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ -f32::powi(other[e4235], 2) - f32::powi(other[e4315], 2) - f32::powi(other[e4125], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return DualNum::from_groups(/* e5, e12345 */ Simd32x2::from(inverse[scalar]) * Simd32x2::from([self[e5], self[e12345]]));
    }
}
impl GeometricQuotient<RoundPoint> for DualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        2        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        3        3        1
    //  no simd        3        4        1
    fn geometric_quotient(self, other: RoundPoint) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2) - 2.0 * (other[e4] * other[e5]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return DualNum::from_groups(/* e5, e12345 */ Simd32x2::from(inverse[scalar]) * Simd32x2::from([self[e5], self[e12345]]));
    }
}
impl GeometricQuotient<Scalar> for DualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        1        1
    //  no simd        0        2        1
    fn geometric_quotient(self, other: Scalar) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return DualNum::from_groups(/* e5, e12345 */ Simd32x2::from(inverse[scalar]) * Simd32x2::from([self[e5], self[e12345]]));
    }
}
impl GeometricQuotient<Sphere> for DualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        2        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        3        3        1
    //  no simd        3        4        1
    fn geometric_quotient(self, other: Sphere) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e3215] * other[e1234]) - f32::powi(other[e4235], 2) - f32::powi(other[e4315], 2) - f32::powi(other[e4125], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return DualNum::from_groups(/* e5, e12345 */ Simd32x2::from(inverse[scalar]) * Simd32x2::from([self[e5], self[e12345]]));
    }
}
impl GeometricQuotient<VersorEven> for DualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11        8        1
    //    simd2        0        1        0
    // Totals...
    // yes simd       11        9        1
    //  no simd       11       10        1
    fn geometric_quotient(self, other: VersorEven) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                + f32::powi(other[e1], 2)
                + f32::powi(other[e2], 2)
                + f32::powi(other[e3], 2)
                - f32::powi(other[e12345], 2)
                - f32::powi(other[e321], 2)
                - 2.0 * (other[e5] * other[e4]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return DualNum::from_groups(/* e5, e12345 */ Simd32x2::from(inverse[scalar]) * Simd32x2::from([self[e5], self[e12345]]));
    }
}
impl GeometricQuotient<VersorOdd> for DualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11        8        1
    //    simd2        0        1        0
    // Totals...
    // yes simd       11        9        1
    //  no simd       11       10        1
    fn geometric_quotient(self, other: VersorOdd) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e1234] * other[e3215]) + f32::powi(other[scalar], 2) + f32::powi(other[e45], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - f32::powi(other[e4235], 2)
                - f32::powi(other[e4315], 2)
                - f32::powi(other[e4125], 2)
                - 2.0 * (other[e41] * other[e15])
                - 2.0 * (other[e42] * other[e25])
                - 2.0 * (other[e43] * other[e35]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return DualNum::from_groups(/* e5, e12345 */ Simd32x2::from(inverse[scalar]) * Simd32x2::from([self[e5], self[e12345]]));
    }
}
impl std::ops::Div<geometric_quotient> for FlatPoint {
    type Output = geometric_quotient_partial<FlatPoint>;
    fn div(self, _rhs: geometric_quotient) -> Self::Output {
        geometric_quotient_partial(self)
    }
}
impl GeometricQuotient<AntiCircleRotor> for FlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        6        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        7        7        1
    //  no simd        7       10        1
    fn geometric_quotient(self, other: AntiCircleRotor) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e45], 2) + f32::powi(other[scalar], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - 2.0 * (other[e41] * other[e15])
                - 2.0 * (other[e42] * other[e25])
                - 2.0 * (other[e43] * other[e35]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return FlatPoint::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
        );
    }
}
impl GeometricQuotient<AntiDipoleInversion> for FlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10        8        1
    //    simd4        0        1        0
    // Totals...
    // yes simd       10        9        1
    //  no simd       10       12        1
    fn geometric_quotient(self, other: AntiDipoleInversion) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                + f32::powi(other[e1], 2)
                + f32::powi(other[e2], 2)
                + f32::powi(other[e3], 2)
                - f32::powi(other[e321], 2)
                - 2.0 * (other[e4] * other[e5]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return FlatPoint::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
        );
    }
}
impl GeometricQuotient<AntiDualNum> for FlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        1        1
    //  no simd        0        4        1
    fn geometric_quotient(self, other: AntiDualNum) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return FlatPoint::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
        );
    }
}
impl GeometricQuotient<AntiFlatPoint> for FlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        5        1
    fn geometric_quotient(self, other: AntiFlatPoint) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e321], 2) * -1.0);
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return FlatPoint::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
        );
    }
}
impl GeometricQuotient<AntiFlector> for FlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        1        1
    //  no simd        3        4        1
    fn geometric_quotient(self, other: AntiFlector) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2) - f32::powi(other[e321], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return FlatPoint::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
        );
    }
}
impl GeometricQuotient<AntiLine> for FlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        1        1
    //  no simd        2        4        1
    fn geometric_quotient(self, other: AntiLine) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ -f32::powi(other[e23], 2) - f32::powi(other[e31], 2) - f32::powi(other[e12], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return FlatPoint::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
        );
    }
}
impl GeometricQuotient<AntiMotor> for FlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        1        1
    //  no simd        3        4        1
    fn geometric_quotient(self, other: AntiMotor) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[scalar], 2) - f32::powi(other[e23], 2) - f32::powi(other[e31], 2) - f32::powi(other[e12], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return FlatPoint::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
        );
    }
}
impl GeometricQuotient<AntiPlane> for FlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        1        1
    //  no simd        2        4        1
    fn geometric_quotient(self, other: AntiPlane) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return FlatPoint::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
        );
    }
}
impl GeometricQuotient<AntiScalar> for FlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        5        1
    fn geometric_quotient(self, other: AntiScalar) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e12345], 2) * -1.0);
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return FlatPoint::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
        );
    }
}
impl GeometricQuotient<Circle> for FlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        6        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        6        7        1
    //  no simd        6       10        1
    fn geometric_quotient(self, other: Circle) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                - f32::powi(other[e321], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return FlatPoint::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
        );
    }
}
impl GeometricQuotient<CircleRotor> for FlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        6        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        7        7        1
    //  no simd        7       10        1
    fn geometric_quotient(self, other: CircleRotor) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                - f32::powi(other[e321], 2)
                - f32::powi(other[e12345], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return FlatPoint::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
        );
    }
}
impl GeometricQuotient<Dipole> for FlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        6        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        6        7        1
    //  no simd        6       10        1
    fn geometric_quotient(self, other: Dipole) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e45], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - 2.0 * (other[e41] * other[e15])
                - 2.0 * (other[e42] * other[e25])
                - 2.0 * (other[e43] * other[e35]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return FlatPoint::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
        );
    }
}
impl GeometricQuotient<DipoleInversion> for FlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10        8        1
    //    simd4        0        1        0
    // Totals...
    // yes simd       10        9        1
    //  no simd       10       12        1
    fn geometric_quotient(self, other: DipoleInversion) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e1234] * other[e3215]) + f32::powi(other[e45], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - f32::powi(other[e4235], 2)
                - f32::powi(other[e4315], 2)
                - f32::powi(other[e4125], 2)
                - 2.0 * (other[e41] * other[e15])
                - 2.0 * (other[e42] * other[e25])
                - 2.0 * (other[e43] * other[e35]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return FlatPoint::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
        );
    }
}
impl GeometricQuotient<DualNum> for FlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        5        1
    fn geometric_quotient(self, other: DualNum) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e12345], 2) * -1.0);
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return FlatPoint::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
        );
    }
}
impl GeometricQuotient<FlatPoint> for FlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        1        1
    //  no simd        0        4        1
    fn geometric_quotient(self, other: FlatPoint) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e45], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return FlatPoint::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
        );
    }
}
impl GeometricQuotient<Flector> for FlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        1        1
    //  no simd        3        4        1
    fn geometric_quotient(self, other: Flector) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e45], 2) - f32::powi(other[e4235], 2) - f32::powi(other[e4315], 2) - f32::powi(other[e4125], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return FlatPoint::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
        );
    }
}
impl GeometricQuotient<Line> for FlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        1        1
    //  no simd        2        4        1
    fn geometric_quotient(self, other: Line) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e415], 2) + f32::powi(other[e425], 2) + f32::powi(other[e435], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return FlatPoint::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
        );
    }
}
impl GeometricQuotient<Motor> for FlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        1        1
    //  no simd        3        4        1
    fn geometric_quotient(self, other: Motor) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e415], 2) + f32::powi(other[e425], 2) + f32::powi(other[e435], 2) - f32::powi(other[e12345], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return FlatPoint::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
        );
    }
}
impl GeometricQuotient<MultiVector> for FlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       23       16        1
    //    simd4        0        1        0
    // Totals...
    // yes simd       23       17        1
    //  no simd       23       20        1
    fn geometric_quotient(self, other: MultiVector) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + 2.0 * (other[e3215] * other[e1234])
                + f32::powi(other[scalar], 2)
                + f32::powi(other[e1], 2)
                + f32::powi(other[e2], 2)
                + f32::powi(other[e3], 2)
                + f32::powi(other[e45], 2)
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                - f32::powi(other[e12345], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - f32::powi(other[e321], 2)
                - f32::powi(other[e4235], 2)
                - f32::powi(other[e4315], 2)
                - f32::powi(other[e4125], 2)
                - 2.0 * (other[e4] * other[e5])
                - 2.0 * (other[e15] * other[e41])
                - 2.0 * (other[e25] * other[e42])
                - 2.0 * (other[e35] * other[e43]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return FlatPoint::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
        );
    }
}
impl GeometricQuotient<Plane> for FlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        1        1
    //  no simd        2        4        1
    fn geometric_quotient(self, other: Plane) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ -f32::powi(other[e4235], 2) - f32::powi(other[e4315], 2) - f32::powi(other[e4125], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return FlatPoint::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
        );
    }
}
impl GeometricQuotient<RoundPoint> for FlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        2        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        3        1
    //  no simd        3        6        1
    fn geometric_quotient(self, other: RoundPoint) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2) - 2.0 * (other[e4] * other[e5]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return FlatPoint::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
        );
    }
}
impl GeometricQuotient<Scalar> for FlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        1        1
    //  no simd        0        4        1
    fn geometric_quotient(self, other: Scalar) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return FlatPoint::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
        );
    }
}
impl GeometricQuotient<Sphere> for FlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        2        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        3        1
    //  no simd        3        6        1
    fn geometric_quotient(self, other: Sphere) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e3215] * other[e1234]) - f32::powi(other[e4235], 2) - f32::powi(other[e4315], 2) - f32::powi(other[e4125], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return FlatPoint::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
        );
    }
}
impl GeometricQuotient<VersorEven> for FlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11        8        1
    //    simd4        0        1        0
    // Totals...
    // yes simd       11        9        1
    //  no simd       11       12        1
    fn geometric_quotient(self, other: VersorEven) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                + f32::powi(other[e1], 2)
                + f32::powi(other[e2], 2)
                + f32::powi(other[e3], 2)
                - f32::powi(other[e12345], 2)
                - f32::powi(other[e321], 2)
                - 2.0 * (other[e5] * other[e4]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return FlatPoint::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
        );
    }
}
impl GeometricQuotient<VersorOdd> for FlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11        8        1
    //    simd4        0        1        0
    // Totals...
    // yes simd       11        9        1
    //  no simd       11       12        1
    fn geometric_quotient(self, other: VersorOdd) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e1234] * other[e3215]) + f32::powi(other[scalar], 2) + f32::powi(other[e45], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - f32::powi(other[e4235], 2)
                - f32::powi(other[e4315], 2)
                - f32::powi(other[e4125], 2)
                - 2.0 * (other[e41] * other[e15])
                - 2.0 * (other[e42] * other[e25])
                - 2.0 * (other[e43] * other[e35]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return FlatPoint::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
        );
    }
}
impl std::ops::Div<geometric_quotient> for Flector {
    type Output = geometric_quotient_partial<Flector>;
    fn div(self, _rhs: geometric_quotient) -> Self::Output {
        geometric_quotient_partial(self)
    }
}
impl GeometricQuotient<AntiCircleRotor> for Flector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        6        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        7        8        1
    //  no simd        7       14        1
    fn geometric_quotient(self, other: AntiCircleRotor) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e45], 2) + f32::powi(other[scalar], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - 2.0 * (other[e41] * other[e15])
                - 2.0 * (other[e42] * other[e25])
                - 2.0 * (other[e43] * other[e35]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricQuotient<AntiDipoleInversion> for Flector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10        8        1
    //    simd4        0        2        0
    // Totals...
    // yes simd       10       10        1
    //  no simd       10       16        1
    fn geometric_quotient(self, other: AntiDipoleInversion) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                + f32::powi(other[e1], 2)
                + f32::powi(other[e2], 2)
                + f32::powi(other[e3], 2)
                - f32::powi(other[e321], 2)
                - 2.0 * (other[e4] * other[e5]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricQuotient<AntiDualNum> for Flector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        8        1
    fn geometric_quotient(self, other: AntiDualNum) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricQuotient<AntiFlatPoint> for Flector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        1
    //  no simd        0        9        1
    fn geometric_quotient(self, other: AntiFlatPoint) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e321], 2) * -1.0);
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricQuotient<AntiFlector> for Flector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        2        1
    //  no simd        3        8        1
    fn geometric_quotient(self, other: AntiFlector) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2) - f32::powi(other[e321], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricQuotient<AntiLine> for Flector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        2        2        1
    //  no simd        2        8        1
    fn geometric_quotient(self, other: AntiLine) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ -f32::powi(other[e23], 2) - f32::powi(other[e31], 2) - f32::powi(other[e12], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricQuotient<AntiMotor> for Flector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        2        1
    //  no simd        3        8        1
    fn geometric_quotient(self, other: AntiMotor) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[scalar], 2) - f32::powi(other[e23], 2) - f32::powi(other[e31], 2) - f32::powi(other[e12], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricQuotient<AntiPlane> for Flector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        2        2        1
    //  no simd        2        8        1
    fn geometric_quotient(self, other: AntiPlane) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricQuotient<AntiScalar> for Flector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        1
    //  no simd        0        9        1
    fn geometric_quotient(self, other: AntiScalar) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e12345], 2) * -1.0);
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricQuotient<Circle> for Flector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        6        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        6        8        1
    //  no simd        6       14        1
    fn geometric_quotient(self, other: Circle) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                - f32::powi(other[e321], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricQuotient<CircleRotor> for Flector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        6        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        7        8        1
    //  no simd        7       14        1
    fn geometric_quotient(self, other: CircleRotor) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                - f32::powi(other[e321], 2)
                - f32::powi(other[e12345], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricQuotient<Dipole> for Flector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        6        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        6        8        1
    //  no simd        6       14        1
    fn geometric_quotient(self, other: Dipole) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e45], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - 2.0 * (other[e41] * other[e15])
                - 2.0 * (other[e42] * other[e25])
                - 2.0 * (other[e43] * other[e35]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricQuotient<DipoleInversion> for Flector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10        8        1
    //    simd4        0        2        0
    // Totals...
    // yes simd       10       10        1
    //  no simd       10       16        1
    fn geometric_quotient(self, other: DipoleInversion) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e1234] * other[e3215]) + f32::powi(other[e45], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - f32::powi(other[e4235], 2)
                - f32::powi(other[e4315], 2)
                - f32::powi(other[e4125], 2)
                - 2.0 * (other[e41] * other[e15])
                - 2.0 * (other[e42] * other[e25])
                - 2.0 * (other[e43] * other[e35]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricQuotient<DualNum> for Flector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        1
    //  no simd        0        9        1
    fn geometric_quotient(self, other: DualNum) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e12345], 2) * -1.0);
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricQuotient<FlatPoint> for Flector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        8        1
    fn geometric_quotient(self, other: FlatPoint) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e45], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricQuotient<Flector> for Flector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        2        1
    //  no simd        3        8        1
    fn geometric_quotient(self, other: Flector) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e45], 2) - f32::powi(other[e4235], 2) - f32::powi(other[e4315], 2) - f32::powi(other[e4125], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricQuotient<Line> for Flector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        2        2        1
    //  no simd        2        8        1
    fn geometric_quotient(self, other: Line) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e415], 2) + f32::powi(other[e425], 2) + f32::powi(other[e435], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricQuotient<Motor> for Flector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        2        1
    //  no simd        3        8        1
    fn geometric_quotient(self, other: Motor) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e415], 2) + f32::powi(other[e425], 2) + f32::powi(other[e435], 2) - f32::powi(other[e12345], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricQuotient<MultiVector> for Flector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       23       16        1
    //    simd4        0        2        0
    // Totals...
    // yes simd       23       18        1
    //  no simd       23       24        1
    fn geometric_quotient(self, other: MultiVector) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + 2.0 * (other[e3215] * other[e1234])
                + f32::powi(other[scalar], 2)
                + f32::powi(other[e1], 2)
                + f32::powi(other[e2], 2)
                + f32::powi(other[e3], 2)
                + f32::powi(other[e45], 2)
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                - f32::powi(other[e12345], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - f32::powi(other[e321], 2)
                - f32::powi(other[e4235], 2)
                - f32::powi(other[e4315], 2)
                - f32::powi(other[e4125], 2)
                - 2.0 * (other[e4] * other[e5])
                - 2.0 * (other[e15] * other[e41])
                - 2.0 * (other[e25] * other[e42])
                - 2.0 * (other[e35] * other[e43]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricQuotient<Plane> for Flector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        2        2        1
    //  no simd        2        8        1
    fn geometric_quotient(self, other: Plane) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ -f32::powi(other[e4235], 2) - f32::powi(other[e4315], 2) - f32::powi(other[e4125], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricQuotient<RoundPoint> for Flector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        2        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        4        1
    //  no simd        3       10        1
    fn geometric_quotient(self, other: RoundPoint) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2) - 2.0 * (other[e4] * other[e5]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricQuotient<Scalar> for Flector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        8        1
    fn geometric_quotient(self, other: Scalar) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricQuotient<Sphere> for Flector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        2        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        4        1
    //  no simd        3       10        1
    fn geometric_quotient(self, other: Sphere) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e3215] * other[e1234]) - f32::powi(other[e4235], 2) - f32::powi(other[e4315], 2) - f32::powi(other[e4125], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricQuotient<VersorEven> for Flector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11        8        1
    //    simd4        0        2        0
    // Totals...
    // yes simd       11       10        1
    //  no simd       11       16        1
    fn geometric_quotient(self, other: VersorEven) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                + f32::powi(other[e1], 2)
                + f32::powi(other[e2], 2)
                + f32::powi(other[e3], 2)
                - f32::powi(other[e12345], 2)
                - f32::powi(other[e321], 2)
                - 2.0 * (other[e5] * other[e4]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricQuotient<VersorOdd> for Flector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11        8        1
    //    simd4        0        2        0
    // Totals...
    // yes simd       11       10        1
    //  no simd       11       16        1
    fn geometric_quotient(self, other: VersorOdd) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e1234] * other[e3215]) + f32::powi(other[scalar], 2) + f32::powi(other[e45], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - f32::powi(other[e4235], 2)
                - f32::powi(other[e4315], 2)
                - f32::powi(other[e4125], 2)
                - 2.0 * (other[e41] * other[e15])
                - 2.0 * (other[e42] * other[e25])
                - 2.0 * (other[e43] * other[e35]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl std::ops::Div<geometric_quotient> for Line {
    type Output = geometric_quotient_partial<Line>;
    fn div(self, _rhs: geometric_quotient) -> Self::Output {
        geometric_quotient_partial(self)
    }
}
impl GeometricQuotient<AntiCircleRotor> for Line {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        6        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        7        8        1
    //  no simd        7       12        1
    fn geometric_quotient(self, other: AntiCircleRotor) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e45], 2) + f32::powi(other[scalar], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - 2.0 * (other[e41] * other[e15])
                - 2.0 * (other[e42] * other[e25])
                - 2.0 * (other[e43] * other[e35]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Line::from_groups(
            // e415, e425, e435
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e415], self[e425], self[e435]]),
            // e235, e315, e125
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
        );
    }
}
impl GeometricQuotient<AntiDipoleInversion> for Line {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10        8        1
    //    simd3        0        2        0
    // Totals...
    // yes simd       10       10        1
    //  no simd       10       14        1
    fn geometric_quotient(self, other: AntiDipoleInversion) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                + f32::powi(other[e1], 2)
                + f32::powi(other[e2], 2)
                + f32::powi(other[e3], 2)
                - f32::powi(other[e321], 2)
                - 2.0 * (other[e4] * other[e5]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Line::from_groups(
            // e415, e425, e435
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e415], self[e425], self[e435]]),
            // e235, e315, e125
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
        );
    }
}
impl GeometricQuotient<AntiDualNum> for Line {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        6        1
    fn geometric_quotient(self, other: AntiDualNum) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Line::from_groups(
            // e415, e425, e435
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e415], self[e425], self[e435]]),
            // e235, e315, e125
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
        );
    }
}
impl GeometricQuotient<AntiFlatPoint> for Line {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        3        1
    //  no simd        0        7        1
    fn geometric_quotient(self, other: AntiFlatPoint) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e321], 2) * -1.0);
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Line::from_groups(
            // e415, e425, e435
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e415], self[e425], self[e435]]),
            // e235, e315, e125
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
        );
    }
}
impl GeometricQuotient<AntiFlector> for Line {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        3        2        1
    //  no simd        3        6        1
    fn geometric_quotient(self, other: AntiFlector) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2) - f32::powi(other[e321], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Line::from_groups(
            // e415, e425, e435
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e415], self[e425], self[e435]]),
            // e235, e315, e125
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
        );
    }
}
impl GeometricQuotient<AntiLine> for Line {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        2        2        1
    //  no simd        2        6        1
    fn geometric_quotient(self, other: AntiLine) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ -f32::powi(other[e23], 2) - f32::powi(other[e31], 2) - f32::powi(other[e12], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Line::from_groups(
            // e415, e425, e435
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e415], self[e425], self[e435]]),
            // e235, e315, e125
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
        );
    }
}
impl GeometricQuotient<AntiMotor> for Line {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        3        2        1
    //  no simd        3        6        1
    fn geometric_quotient(self, other: AntiMotor) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[scalar], 2) - f32::powi(other[e23], 2) - f32::powi(other[e31], 2) - f32::powi(other[e12], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Line::from_groups(
            // e415, e425, e435
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e415], self[e425], self[e435]]),
            // e235, e315, e125
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
        );
    }
}
impl GeometricQuotient<AntiPlane> for Line {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        2        2        1
    //  no simd        2        6        1
    fn geometric_quotient(self, other: AntiPlane) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Line::from_groups(
            // e415, e425, e435
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e415], self[e425], self[e435]]),
            // e235, e315, e125
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
        );
    }
}
impl GeometricQuotient<AntiScalar> for Line {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        3        1
    //  no simd        0        7        1
    fn geometric_quotient(self, other: AntiScalar) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e12345], 2) * -1.0);
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Line::from_groups(
            // e415, e425, e435
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e415], self[e425], self[e435]]),
            // e235, e315, e125
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
        );
    }
}
impl GeometricQuotient<Circle> for Line {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        6        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        6        8        1
    //  no simd        6       12        1
    fn geometric_quotient(self, other: Circle) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                - f32::powi(other[e321], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Line::from_groups(
            // e415, e425, e435
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e415], self[e425], self[e435]]),
            // e235, e315, e125
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
        );
    }
}
impl GeometricQuotient<CircleRotor> for Line {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        6        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        7        8        1
    //  no simd        7       12        1
    fn geometric_quotient(self, other: CircleRotor) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                - f32::powi(other[e321], 2)
                - f32::powi(other[e12345], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Line::from_groups(
            // e415, e425, e435
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e415], self[e425], self[e435]]),
            // e235, e315, e125
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
        );
    }
}
impl GeometricQuotient<Dipole> for Line {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        6        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        6        8        1
    //  no simd        6       12        1
    fn geometric_quotient(self, other: Dipole) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e45], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - 2.0 * (other[e41] * other[e15])
                - 2.0 * (other[e42] * other[e25])
                - 2.0 * (other[e43] * other[e35]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Line::from_groups(
            // e415, e425, e435
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e415], self[e425], self[e435]]),
            // e235, e315, e125
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
        );
    }
}
impl GeometricQuotient<DipoleInversion> for Line {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10        8        1
    //    simd3        0        2        0
    // Totals...
    // yes simd       10       10        1
    //  no simd       10       14        1
    fn geometric_quotient(self, other: DipoleInversion) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e1234] * other[e3215]) + f32::powi(other[e45], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - f32::powi(other[e4235], 2)
                - f32::powi(other[e4315], 2)
                - f32::powi(other[e4125], 2)
                - 2.0 * (other[e41] * other[e15])
                - 2.0 * (other[e42] * other[e25])
                - 2.0 * (other[e43] * other[e35]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Line::from_groups(
            // e415, e425, e435
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e415], self[e425], self[e435]]),
            // e235, e315, e125
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
        );
    }
}
impl GeometricQuotient<DualNum> for Line {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        3        1
    //  no simd        0        7        1
    fn geometric_quotient(self, other: DualNum) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e12345], 2) * -1.0);
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Line::from_groups(
            // e415, e425, e435
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e415], self[e425], self[e435]]),
            // e235, e315, e125
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
        );
    }
}
impl GeometricQuotient<FlatPoint> for Line {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        6        1
    fn geometric_quotient(self, other: FlatPoint) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e45], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Line::from_groups(
            // e415, e425, e435
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e415], self[e425], self[e435]]),
            // e235, e315, e125
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
        );
    }
}
impl GeometricQuotient<Flector> for Line {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        3        2        1
    //  no simd        3        6        1
    fn geometric_quotient(self, other: Flector) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e45], 2) - f32::powi(other[e4235], 2) - f32::powi(other[e4315], 2) - f32::powi(other[e4125], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Line::from_groups(
            // e415, e425, e435
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e415], self[e425], self[e435]]),
            // e235, e315, e125
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
        );
    }
}
impl GeometricQuotient<Line> for Line {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        2        2        1
    //  no simd        2        6        1
    fn geometric_quotient(self, other: Line) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e415], 2) + f32::powi(other[e425], 2) + f32::powi(other[e435], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Line::from_groups(
            // e415, e425, e435
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e415], self[e425], self[e435]]),
            // e235, e315, e125
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
        );
    }
}
impl GeometricQuotient<Motor> for Line {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        3        2        1
    //  no simd        3        6        1
    fn geometric_quotient(self, other: Motor) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e415], 2) + f32::powi(other[e425], 2) + f32::powi(other[e435], 2) - f32::powi(other[e12345], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Line::from_groups(
            // e415, e425, e435
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e415], self[e425], self[e435]]),
            // e235, e315, e125
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
        );
    }
}
impl GeometricQuotient<MultiVector> for Line {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       23       16        1
    //    simd3        0        2        0
    // Totals...
    // yes simd       23       18        1
    //  no simd       23       22        1
    fn geometric_quotient(self, other: MultiVector) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + 2.0 * (other[e3215] * other[e1234])
                + f32::powi(other[scalar], 2)
                + f32::powi(other[e1], 2)
                + f32::powi(other[e2], 2)
                + f32::powi(other[e3], 2)
                + f32::powi(other[e45], 2)
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                - f32::powi(other[e12345], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - f32::powi(other[e321], 2)
                - f32::powi(other[e4235], 2)
                - f32::powi(other[e4315], 2)
                - f32::powi(other[e4125], 2)
                - 2.0 * (other[e4] * other[e5])
                - 2.0 * (other[e15] * other[e41])
                - 2.0 * (other[e25] * other[e42])
                - 2.0 * (other[e35] * other[e43]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Line::from_groups(
            // e415, e425, e435
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e415], self[e425], self[e435]]),
            // e235, e315, e125
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
        );
    }
}
impl GeometricQuotient<Plane> for Line {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        2        2        1
    //  no simd        2        6        1
    fn geometric_quotient(self, other: Plane) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ -f32::powi(other[e4235], 2) - f32::powi(other[e4315], 2) - f32::powi(other[e4125], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Line::from_groups(
            // e415, e425, e435
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e415], self[e425], self[e435]]),
            // e235, e315, e125
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
        );
    }
}
impl GeometricQuotient<RoundPoint> for Line {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        2        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        3        4        1
    //  no simd        3        8        1
    fn geometric_quotient(self, other: RoundPoint) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2) - 2.0 * (other[e4] * other[e5]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Line::from_groups(
            // e415, e425, e435
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e415], self[e425], self[e435]]),
            // e235, e315, e125
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
        );
    }
}
impl GeometricQuotient<Scalar> for Line {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        6        1
    fn geometric_quotient(self, other: Scalar) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Line::from_groups(
            // e415, e425, e435
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e415], self[e425], self[e435]]),
            // e235, e315, e125
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
        );
    }
}
impl GeometricQuotient<Sphere> for Line {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        2        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        3        4        1
    //  no simd        3        8        1
    fn geometric_quotient(self, other: Sphere) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e3215] * other[e1234]) - f32::powi(other[e4235], 2) - f32::powi(other[e4315], 2) - f32::powi(other[e4125], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Line::from_groups(
            // e415, e425, e435
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e415], self[e425], self[e435]]),
            // e235, e315, e125
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
        );
    }
}
impl GeometricQuotient<VersorEven> for Line {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11        8        1
    //    simd3        0        2        0
    // Totals...
    // yes simd       11       10        1
    //  no simd       11       14        1
    fn geometric_quotient(self, other: VersorEven) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                + f32::powi(other[e1], 2)
                + f32::powi(other[e2], 2)
                + f32::powi(other[e3], 2)
                - f32::powi(other[e12345], 2)
                - f32::powi(other[e321], 2)
                - 2.0 * (other[e5] * other[e4]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Line::from_groups(
            // e415, e425, e435
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e415], self[e425], self[e435]]),
            // e235, e315, e125
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
        );
    }
}
impl GeometricQuotient<VersorOdd> for Line {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11        8        1
    //    simd3        0        2        0
    // Totals...
    // yes simd       11       10        1
    //  no simd       11       14        1
    fn geometric_quotient(self, other: VersorOdd) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e1234] * other[e3215]) + f32::powi(other[scalar], 2) + f32::powi(other[e45], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - f32::powi(other[e4235], 2)
                - f32::powi(other[e4315], 2)
                - f32::powi(other[e4125], 2)
                - 2.0 * (other[e41] * other[e15])
                - 2.0 * (other[e42] * other[e25])
                - 2.0 * (other[e43] * other[e35]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Line::from_groups(
            // e415, e425, e435
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e415], self[e425], self[e435]]),
            // e235, e315, e125
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
        );
    }
}
impl std::ops::Div<geometric_quotient> for Motor {
    type Output = geometric_quotient_partial<Motor>;
    fn div(self, _rhs: geometric_quotient) -> Self::Output {
        geometric_quotient_partial(self)
    }
}
impl GeometricQuotient<AntiCircleRotor> for Motor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        6        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        7        8        1
    //  no simd        7       14        1
    fn geometric_quotient(self, other: AntiCircleRotor) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e45], 2) + f32::powi(other[scalar], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - 2.0 * (other[e41] * other[e15])
                - 2.0 * (other[e42] * other[e25])
                - 2.0 * (other[e43] * other[e35]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e12345]]),
            // e235, e315, e125, e5
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]),
        );
    }
}
impl GeometricQuotient<AntiDipoleInversion> for Motor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10        8        1
    //    simd4        0        2        0
    // Totals...
    // yes simd       10       10        1
    //  no simd       10       16        1
    fn geometric_quotient(self, other: AntiDipoleInversion) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                + f32::powi(other[e1], 2)
                + f32::powi(other[e2], 2)
                + f32::powi(other[e3], 2)
                - f32::powi(other[e321], 2)
                - 2.0 * (other[e4] * other[e5]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e12345]]),
            // e235, e315, e125, e5
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]),
        );
    }
}
impl GeometricQuotient<AntiDualNum> for Motor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        8        1
    fn geometric_quotient(self, other: AntiDualNum) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e12345]]),
            // e235, e315, e125, e5
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]),
        );
    }
}
impl GeometricQuotient<AntiFlatPoint> for Motor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        1
    //  no simd        0        9        1
    fn geometric_quotient(self, other: AntiFlatPoint) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e321], 2) * -1.0);
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e12345]]),
            // e235, e315, e125, e5
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]),
        );
    }
}
impl GeometricQuotient<AntiFlector> for Motor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        2        1
    //  no simd        3        8        1
    fn geometric_quotient(self, other: AntiFlector) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2) - f32::powi(other[e321], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e12345]]),
            // e235, e315, e125, e5
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]),
        );
    }
}
impl GeometricQuotient<AntiLine> for Motor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        2        2        1
    //  no simd        2        8        1
    fn geometric_quotient(self, other: AntiLine) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ -f32::powi(other[e23], 2) - f32::powi(other[e31], 2) - f32::powi(other[e12], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e12345]]),
            // e235, e315, e125, e5
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]),
        );
    }
}
impl GeometricQuotient<AntiMotor> for Motor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        2        1
    //  no simd        3        8        1
    fn geometric_quotient(self, other: AntiMotor) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[scalar], 2) - f32::powi(other[e23], 2) - f32::powi(other[e31], 2) - f32::powi(other[e12], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e12345]]),
            // e235, e315, e125, e5
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]),
        );
    }
}
impl GeometricQuotient<AntiPlane> for Motor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        2        2        1
    //  no simd        2        8        1
    fn geometric_quotient(self, other: AntiPlane) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e12345]]),
            // e235, e315, e125, e5
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]),
        );
    }
}
impl GeometricQuotient<AntiScalar> for Motor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        1
    //  no simd        0        9        1
    fn geometric_quotient(self, other: AntiScalar) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e12345], 2) * -1.0);
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e12345]]),
            // e235, e315, e125, e5
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]),
        );
    }
}
impl GeometricQuotient<Circle> for Motor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        6        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        6        8        1
    //  no simd        6       14        1
    fn geometric_quotient(self, other: Circle) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                - f32::powi(other[e321], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e12345]]),
            // e235, e315, e125, e5
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]),
        );
    }
}
impl GeometricQuotient<CircleRotor> for Motor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        6        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        7        8        1
    //  no simd        7       14        1
    fn geometric_quotient(self, other: CircleRotor) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                - f32::powi(other[e321], 2)
                - f32::powi(other[e12345], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e12345]]),
            // e235, e315, e125, e5
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]),
        );
    }
}
impl GeometricQuotient<Dipole> for Motor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        6        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        6        8        1
    //  no simd        6       14        1
    fn geometric_quotient(self, other: Dipole) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e45], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - 2.0 * (other[e41] * other[e15])
                - 2.0 * (other[e42] * other[e25])
                - 2.0 * (other[e43] * other[e35]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e12345]]),
            // e235, e315, e125, e5
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]),
        );
    }
}
impl GeometricQuotient<DipoleInversion> for Motor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10        8        1
    //    simd4        0        2        0
    // Totals...
    // yes simd       10       10        1
    //  no simd       10       16        1
    fn geometric_quotient(self, other: DipoleInversion) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e1234] * other[e3215]) + f32::powi(other[e45], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - f32::powi(other[e4235], 2)
                - f32::powi(other[e4315], 2)
                - f32::powi(other[e4125], 2)
                - 2.0 * (other[e41] * other[e15])
                - 2.0 * (other[e42] * other[e25])
                - 2.0 * (other[e43] * other[e35]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e12345]]),
            // e235, e315, e125, e5
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]),
        );
    }
}
impl GeometricQuotient<DualNum> for Motor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        1
    //  no simd        0        9        1
    fn geometric_quotient(self, other: DualNum) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e12345], 2) * -1.0);
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e12345]]),
            // e235, e315, e125, e5
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]),
        );
    }
}
impl GeometricQuotient<FlatPoint> for Motor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        8        1
    fn geometric_quotient(self, other: FlatPoint) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e45], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e12345]]),
            // e235, e315, e125, e5
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]),
        );
    }
}
impl GeometricQuotient<Flector> for Motor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        2        1
    //  no simd        3        8        1
    fn geometric_quotient(self, other: Flector) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e45], 2) - f32::powi(other[e4235], 2) - f32::powi(other[e4315], 2) - f32::powi(other[e4125], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e12345]]),
            // e235, e315, e125, e5
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]),
        );
    }
}
impl GeometricQuotient<Line> for Motor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        2        2        1
    //  no simd        2        8        1
    fn geometric_quotient(self, other: Line) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e415], 2) + f32::powi(other[e425], 2) + f32::powi(other[e435], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e12345]]),
            // e235, e315, e125, e5
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]),
        );
    }
}
impl GeometricQuotient<Motor> for Motor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        2        1
    //  no simd        3        8        1
    fn geometric_quotient(self, other: Motor) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e415], 2) + f32::powi(other[e425], 2) + f32::powi(other[e435], 2) - f32::powi(other[e12345], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e12345]]),
            // e235, e315, e125, e5
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]),
        );
    }
}
impl GeometricQuotient<MultiVector> for Motor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       23       16        1
    //    simd4        0        2        0
    // Totals...
    // yes simd       23       18        1
    //  no simd       23       24        1
    fn geometric_quotient(self, other: MultiVector) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + 2.0 * (other[e3215] * other[e1234])
                + f32::powi(other[scalar], 2)
                + f32::powi(other[e1], 2)
                + f32::powi(other[e2], 2)
                + f32::powi(other[e3], 2)
                + f32::powi(other[e45], 2)
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                - f32::powi(other[e12345], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - f32::powi(other[e321], 2)
                - f32::powi(other[e4235], 2)
                - f32::powi(other[e4315], 2)
                - f32::powi(other[e4125], 2)
                - 2.0 * (other[e4] * other[e5])
                - 2.0 * (other[e15] * other[e41])
                - 2.0 * (other[e25] * other[e42])
                - 2.0 * (other[e35] * other[e43]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e12345]]),
            // e235, e315, e125, e5
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]),
        );
    }
}
impl GeometricQuotient<Plane> for Motor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        2        2        1
    //  no simd        2        8        1
    fn geometric_quotient(self, other: Plane) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ -f32::powi(other[e4235], 2) - f32::powi(other[e4315], 2) - f32::powi(other[e4125], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e12345]]),
            // e235, e315, e125, e5
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]),
        );
    }
}
impl GeometricQuotient<RoundPoint> for Motor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        2        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        4        1
    //  no simd        3       10        1
    fn geometric_quotient(self, other: RoundPoint) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2) - 2.0 * (other[e4] * other[e5]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e12345]]),
            // e235, e315, e125, e5
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]),
        );
    }
}
impl GeometricQuotient<Scalar> for Motor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        8        1
    fn geometric_quotient(self, other: Scalar) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e12345]]),
            // e235, e315, e125, e5
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]),
        );
    }
}
impl GeometricQuotient<Sphere> for Motor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        2        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        4        1
    //  no simd        3       10        1
    fn geometric_quotient(self, other: Sphere) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e3215] * other[e1234]) - f32::powi(other[e4235], 2) - f32::powi(other[e4315], 2) - f32::powi(other[e4125], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e12345]]),
            // e235, e315, e125, e5
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]),
        );
    }
}
impl GeometricQuotient<VersorEven> for Motor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11        8        1
    //    simd4        0        2        0
    // Totals...
    // yes simd       11       10        1
    //  no simd       11       16        1
    fn geometric_quotient(self, other: VersorEven) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                + f32::powi(other[e1], 2)
                + f32::powi(other[e2], 2)
                + f32::powi(other[e3], 2)
                - f32::powi(other[e12345], 2)
                - f32::powi(other[e321], 2)
                - 2.0 * (other[e5] * other[e4]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e12345]]),
            // e235, e315, e125, e5
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]),
        );
    }
}
impl GeometricQuotient<VersorOdd> for Motor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11        8        1
    //    simd4        0        2        0
    // Totals...
    // yes simd       11       10        1
    //  no simd       11       16        1
    fn geometric_quotient(self, other: VersorOdd) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e1234] * other[e3215]) + f32::powi(other[scalar], 2) + f32::powi(other[e45], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - f32::powi(other[e4235], 2)
                - f32::powi(other[e4315], 2)
                - f32::powi(other[e4125], 2)
                - 2.0 * (other[e41] * other[e15])
                - 2.0 * (other[e42] * other[e25])
                - 2.0 * (other[e43] * other[e35]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e12345]]),
            // e235, e315, e125, e5
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]),
        );
    }
}
impl std::ops::Div<geometric_quotient> for MultiVector {
    type Output = geometric_quotient_partial<MultiVector>;
    fn div(self, _rhs: geometric_quotient) -> Self::Output {
        geometric_quotient_partial(self)
    }
}
impl GeometricQuotient<AntiCircleRotor> for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        8        1
    //    simd2        0        1        0
    //    simd3        0        4        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        7       17        1
    //  no simd        7       38        1
    fn geometric_quotient(self, other: AntiCircleRotor) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e45], 2) + f32::powi(other[scalar], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - 2.0 * (other[e41] * other[e15])
                - 2.0 * (other[e42] * other[e25])
                - 2.0 * (other[e43] * other[e35]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(inverse[scalar]) * Simd32x2::from([self[scalar], self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            self[e5] * inverse[scalar],
            // e15, e25, e35, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e423, e431, e412
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
            // e1234
            self[e1234] * inverse[scalar],
        );
    }
}
impl GeometricQuotient<AntiDipoleInversion> for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       10        1
    //    simd2        0        1        0
    //    simd3        0        4        0
    //    simd4        0        4        0
    // Totals...
    // yes simd       10       19        1
    //  no simd       10       40        1
    fn geometric_quotient(self, other: AntiDipoleInversion) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                + f32::powi(other[e1], 2)
                + f32::powi(other[e2], 2)
                + f32::powi(other[e3], 2)
                - f32::powi(other[e321], 2)
                - 2.0 * (other[e4] * other[e5]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(inverse[scalar]) * Simd32x2::from([self[scalar], self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            self[e5] * inverse[scalar],
            // e15, e25, e35, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e423, e431, e412
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
            // e1234
            self[e1234] * inverse[scalar],
        );
    }
}
impl GeometricQuotient<AntiDualNum> for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        1
    //    simd2        0        1        0
    //    simd3        0        4        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        0       11        1
    //  no simd        0       32        1
    fn geometric_quotient(self, other: AntiDualNum) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(inverse[scalar]) * Simd32x2::from([self[scalar], self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            self[e5] * inverse[scalar],
            // e15, e25, e35, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e423, e431, e412
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
            // e1234
            self[e1234] * inverse[scalar],
        );
    }
}
impl GeometricQuotient<AntiFlatPoint> for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        1
    //    simd2        0        1        0
    //    simd3        0        4        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        0       12        1
    //  no simd        0       33        1
    fn geometric_quotient(self, other: AntiFlatPoint) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e321], 2) * -1.0);
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(inverse[scalar]) * Simd32x2::from([self[scalar], self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            self[e5] * inverse[scalar],
            // e15, e25, e35, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e423, e431, e412
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
            // e1234
            self[e1234] * inverse[scalar],
        );
    }
}
impl GeometricQuotient<AntiFlector> for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        2        1
    //    simd2        0        1        0
    //    simd3        0        4        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        3       11        1
    //  no simd        3       32        1
    fn geometric_quotient(self, other: AntiFlector) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2) - f32::powi(other[e321], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(inverse[scalar]) * Simd32x2::from([self[scalar], self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            self[e5] * inverse[scalar],
            // e15, e25, e35, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e423, e431, e412
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
            // e1234
            self[e1234] * inverse[scalar],
        );
    }
}
impl GeometricQuotient<AntiLine> for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        2        1
    //    simd2        0        1        0
    //    simd3        0        4        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        2       11        1
    //  no simd        2       32        1
    fn geometric_quotient(self, other: AntiLine) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ -f32::powi(other[e23], 2) - f32::powi(other[e31], 2) - f32::powi(other[e12], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(inverse[scalar]) * Simd32x2::from([self[scalar], self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            self[e5] * inverse[scalar],
            // e15, e25, e35, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e423, e431, e412
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
            // e1234
            self[e1234] * inverse[scalar],
        );
    }
}
impl GeometricQuotient<AntiMotor> for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        2        1
    //    simd2        0        1        0
    //    simd3        0        4        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        3       11        1
    //  no simd        3       32        1
    fn geometric_quotient(self, other: AntiMotor) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[scalar], 2) - f32::powi(other[e23], 2) - f32::powi(other[e31], 2) - f32::powi(other[e12], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(inverse[scalar]) * Simd32x2::from([self[scalar], self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            self[e5] * inverse[scalar],
            // e15, e25, e35, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e423, e431, e412
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
            // e1234
            self[e1234] * inverse[scalar],
        );
    }
}
impl GeometricQuotient<AntiPlane> for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        2        1
    //    simd2        0        1        0
    //    simd3        0        4        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        2       11        1
    //  no simd        2       32        1
    fn geometric_quotient(self, other: AntiPlane) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(inverse[scalar]) * Simd32x2::from([self[scalar], self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            self[e5] * inverse[scalar],
            // e15, e25, e35, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e423, e431, e412
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
            // e1234
            self[e1234] * inverse[scalar],
        );
    }
}
impl GeometricQuotient<AntiScalar> for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        1
    //    simd2        0        1        0
    //    simd3        0        4        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        0       12        1
    //  no simd        0       33        1
    fn geometric_quotient(self, other: AntiScalar) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e12345], 2) * -1.0);
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(inverse[scalar]) * Simd32x2::from([self[scalar], self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            self[e5] * inverse[scalar],
            // e15, e25, e35, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e423, e431, e412
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
            // e1234
            self[e1234] * inverse[scalar],
        );
    }
}
impl GeometricQuotient<Circle> for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        8        1
    //    simd2        0        1        0
    //    simd3        0        4        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        6       17        1
    //  no simd        6       38        1
    fn geometric_quotient(self, other: Circle) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                - f32::powi(other[e321], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(inverse[scalar]) * Simd32x2::from([self[scalar], self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            self[e5] * inverse[scalar],
            // e15, e25, e35, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e423, e431, e412
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
            // e1234
            self[e1234] * inverse[scalar],
        );
    }
}
impl GeometricQuotient<CircleRotor> for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        8        1
    //    simd2        0        1        0
    //    simd3        0        4        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        7       17        1
    //  no simd        7       38        1
    fn geometric_quotient(self, other: CircleRotor) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                - f32::powi(other[e321], 2)
                - f32::powi(other[e12345], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(inverse[scalar]) * Simd32x2::from([self[scalar], self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            self[e5] * inverse[scalar],
            // e15, e25, e35, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e423, e431, e412
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
            // e1234
            self[e1234] * inverse[scalar],
        );
    }
}
impl GeometricQuotient<Dipole> for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        8        1
    //    simd2        0        1        0
    //    simd3        0        4        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        6       17        1
    //  no simd        6       38        1
    fn geometric_quotient(self, other: Dipole) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e45], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - 2.0 * (other[e41] * other[e15])
                - 2.0 * (other[e42] * other[e25])
                - 2.0 * (other[e43] * other[e35]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(inverse[scalar]) * Simd32x2::from([self[scalar], self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            self[e5] * inverse[scalar],
            // e15, e25, e35, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e423, e431, e412
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
            // e1234
            self[e1234] * inverse[scalar],
        );
    }
}
impl GeometricQuotient<DipoleInversion> for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       10        1
    //    simd2        0        1        0
    //    simd3        0        4        0
    //    simd4        0        4        0
    // Totals...
    // yes simd       10       19        1
    //  no simd       10       40        1
    fn geometric_quotient(self, other: DipoleInversion) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e1234] * other[e3215]) + f32::powi(other[e45], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - f32::powi(other[e4235], 2)
                - f32::powi(other[e4315], 2)
                - f32::powi(other[e4125], 2)
                - 2.0 * (other[e41] * other[e15])
                - 2.0 * (other[e42] * other[e25])
                - 2.0 * (other[e43] * other[e35]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(inverse[scalar]) * Simd32x2::from([self[scalar], self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            self[e5] * inverse[scalar],
            // e15, e25, e35, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e423, e431, e412
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
            // e1234
            self[e1234] * inverse[scalar],
        );
    }
}
impl GeometricQuotient<DualNum> for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        1
    //    simd2        0        1        0
    //    simd3        0        4        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        0       12        1
    //  no simd        0       33        1
    fn geometric_quotient(self, other: DualNum) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e12345], 2) * -1.0);
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(inverse[scalar]) * Simd32x2::from([self[scalar], self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            self[e5] * inverse[scalar],
            // e15, e25, e35, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e423, e431, e412
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
            // e1234
            self[e1234] * inverse[scalar],
        );
    }
}
impl GeometricQuotient<FlatPoint> for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        1
    //    simd2        0        1        0
    //    simd3        0        4        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        0       11        1
    //  no simd        0       32        1
    fn geometric_quotient(self, other: FlatPoint) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e45], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(inverse[scalar]) * Simd32x2::from([self[scalar], self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            self[e5] * inverse[scalar],
            // e15, e25, e35, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e423, e431, e412
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
            // e1234
            self[e1234] * inverse[scalar],
        );
    }
}
impl GeometricQuotient<Flector> for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        2        1
    //    simd2        0        1        0
    //    simd3        0        4        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        3       11        1
    //  no simd        3       32        1
    fn geometric_quotient(self, other: Flector) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e45], 2) - f32::powi(other[e4235], 2) - f32::powi(other[e4315], 2) - f32::powi(other[e4125], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(inverse[scalar]) * Simd32x2::from([self[scalar], self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            self[e5] * inverse[scalar],
            // e15, e25, e35, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e423, e431, e412
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
            // e1234
            self[e1234] * inverse[scalar],
        );
    }
}
impl GeometricQuotient<Line> for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        2        1
    //    simd2        0        1        0
    //    simd3        0        4        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        2       11        1
    //  no simd        2       32        1
    fn geometric_quotient(self, other: Line) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e415], 2) + f32::powi(other[e425], 2) + f32::powi(other[e435], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(inverse[scalar]) * Simd32x2::from([self[scalar], self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            self[e5] * inverse[scalar],
            // e15, e25, e35, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e423, e431, e412
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
            // e1234
            self[e1234] * inverse[scalar],
        );
    }
}
impl GeometricQuotient<Motor> for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        2        1
    //    simd2        0        1        0
    //    simd3        0        4        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        3       11        1
    //  no simd        3       32        1
    fn geometric_quotient(self, other: Motor) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e415], 2) + f32::powi(other[e425], 2) + f32::powi(other[e435], 2) - f32::powi(other[e12345], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(inverse[scalar]) * Simd32x2::from([self[scalar], self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            self[e5] * inverse[scalar],
            // e15, e25, e35, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e423, e431, e412
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
            // e1234
            self[e1234] * inverse[scalar],
        );
    }
}
impl GeometricQuotient<MultiVector> for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       23       18        1
    //    simd2        0        1        0
    //    simd3        0        4        0
    //    simd4        0        4        0
    // Totals...
    // yes simd       23       27        1
    //  no simd       23       48        1
    fn geometric_quotient(self, other: MultiVector) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + 2.0 * (other[e3215] * other[e1234])
                + f32::powi(other[scalar], 2)
                + f32::powi(other[e1], 2)
                + f32::powi(other[e2], 2)
                + f32::powi(other[e3], 2)
                + f32::powi(other[e45], 2)
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                - f32::powi(other[e12345], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - f32::powi(other[e321], 2)
                - f32::powi(other[e4235], 2)
                - f32::powi(other[e4315], 2)
                - f32::powi(other[e4125], 2)
                - 2.0 * (other[e4] * other[e5])
                - 2.0 * (other[e15] * other[e41])
                - 2.0 * (other[e25] * other[e42])
                - 2.0 * (other[e35] * other[e43]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(inverse[scalar]) * Simd32x2::from([self[scalar], self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            self[e5] * inverse[scalar],
            // e15, e25, e35, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e423, e431, e412
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
            // e1234
            self[e1234] * inverse[scalar],
        );
    }
}
impl GeometricQuotient<Plane> for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        2        1
    //    simd2        0        1        0
    //    simd3        0        4        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        2       11        1
    //  no simd        2       32        1
    fn geometric_quotient(self, other: Plane) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ -f32::powi(other[e4235], 2) - f32::powi(other[e4315], 2) - f32::powi(other[e4125], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(inverse[scalar]) * Simd32x2::from([self[scalar], self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            self[e5] * inverse[scalar],
            // e15, e25, e35, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e423, e431, e412
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
            // e1234
            self[e1234] * inverse[scalar],
        );
    }
}
impl GeometricQuotient<RoundPoint> for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        1
    //    simd2        0        1        0
    //    simd3        0        4        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        3       13        1
    //  no simd        3       34        1
    fn geometric_quotient(self, other: RoundPoint) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2) - 2.0 * (other[e4] * other[e5]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(inverse[scalar]) * Simd32x2::from([self[scalar], self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            self[e5] * inverse[scalar],
            // e15, e25, e35, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e423, e431, e412
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
            // e1234
            self[e1234] * inverse[scalar],
        );
    }
}
impl GeometricQuotient<Scalar> for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        1
    //    simd2        0        1        0
    //    simd3        0        4        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        0       11        1
    //  no simd        0       32        1
    fn geometric_quotient(self, other: Scalar) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(inverse[scalar]) * Simd32x2::from([self[scalar], self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            self[e5] * inverse[scalar],
            // e15, e25, e35, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e423, e431, e412
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
            // e1234
            self[e1234] * inverse[scalar],
        );
    }
}
impl GeometricQuotient<Sphere> for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        1
    //    simd2        0        1        0
    //    simd3        0        4        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        3       13        1
    //  no simd        3       34        1
    fn geometric_quotient(self, other: Sphere) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e3215] * other[e1234]) - f32::powi(other[e4235], 2) - f32::powi(other[e4315], 2) - f32::powi(other[e4125], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(inverse[scalar]) * Simd32x2::from([self[scalar], self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            self[e5] * inverse[scalar],
            // e15, e25, e35, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e423, e431, e412
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
            // e1234
            self[e1234] * inverse[scalar],
        );
    }
}
impl GeometricQuotient<VersorEven> for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       10        1
    //    simd2        0        1        0
    //    simd3        0        4        0
    //    simd4        0        4        0
    // Totals...
    // yes simd       11       19        1
    //  no simd       11       40        1
    fn geometric_quotient(self, other: VersorEven) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                + f32::powi(other[e1], 2)
                + f32::powi(other[e2], 2)
                + f32::powi(other[e3], 2)
                - f32::powi(other[e12345], 2)
                - f32::powi(other[e321], 2)
                - 2.0 * (other[e5] * other[e4]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(inverse[scalar]) * Simd32x2::from([self[scalar], self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            self[e5] * inverse[scalar],
            // e15, e25, e35, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e423, e431, e412
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
            // e1234
            self[e1234] * inverse[scalar],
        );
    }
}
impl GeometricQuotient<VersorOdd> for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       10        1
    //    simd2        0        1        0
    //    simd3        0        4        0
    //    simd4        0        4        0
    // Totals...
    // yes simd       11       19        1
    //  no simd       11       40        1
    fn geometric_quotient(self, other: VersorOdd) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e1234] * other[e3215]) + f32::powi(other[scalar], 2) + f32::powi(other[e45], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - f32::powi(other[e4235], 2)
                - f32::powi(other[e4315], 2)
                - f32::powi(other[e4125], 2)
                - 2.0 * (other[e41] * other[e15])
                - 2.0 * (other[e42] * other[e25])
                - 2.0 * (other[e43] * other[e35]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(inverse[scalar]) * Simd32x2::from([self[scalar], self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            self[e5] * inverse[scalar],
            // e15, e25, e35, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e423, e431, e412
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
            // e1234
            self[e1234] * inverse[scalar],
        );
    }
}
impl std::ops::Div<geometric_quotient> for Plane {
    type Output = geometric_quotient_partial<Plane>;
    fn div(self, _rhs: geometric_quotient) -> Self::Output {
        geometric_quotient_partial(self)
    }
}
impl GeometricQuotient<AntiCircleRotor> for Plane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        6        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        7        7        1
    //  no simd        7       10        1
    fn geometric_quotient(self, other: AntiCircleRotor) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e45], 2) + f32::powi(other[scalar], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - 2.0 * (other[e41] * other[e15])
                - 2.0 * (other[e42] * other[e25])
                - 2.0 * (other[e43] * other[e35]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricQuotient<AntiDipoleInversion> for Plane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10        8        1
    //    simd4        0        1        0
    // Totals...
    // yes simd       10        9        1
    //  no simd       10       12        1
    fn geometric_quotient(self, other: AntiDipoleInversion) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                + f32::powi(other[e1], 2)
                + f32::powi(other[e2], 2)
                + f32::powi(other[e3], 2)
                - f32::powi(other[e321], 2)
                - 2.0 * (other[e4] * other[e5]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricQuotient<AntiDualNum> for Plane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        1        1
    //  no simd        0        4        1
    fn geometric_quotient(self, other: AntiDualNum) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricQuotient<AntiFlatPoint> for Plane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        5        1
    fn geometric_quotient(self, other: AntiFlatPoint) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e321], 2) * -1.0);
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricQuotient<AntiFlector> for Plane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        1        1
    //  no simd        3        4        1
    fn geometric_quotient(self, other: AntiFlector) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2) - f32::powi(other[e321], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricQuotient<AntiLine> for Plane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        1        1
    //  no simd        2        4        1
    fn geometric_quotient(self, other: AntiLine) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ -f32::powi(other[e23], 2) - f32::powi(other[e31], 2) - f32::powi(other[e12], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricQuotient<AntiMotor> for Plane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        1        1
    //  no simd        3        4        1
    fn geometric_quotient(self, other: AntiMotor) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[scalar], 2) - f32::powi(other[e23], 2) - f32::powi(other[e31], 2) - f32::powi(other[e12], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricQuotient<AntiPlane> for Plane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        1        1
    //  no simd        2        4        1
    fn geometric_quotient(self, other: AntiPlane) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricQuotient<AntiScalar> for Plane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        5        1
    fn geometric_quotient(self, other: AntiScalar) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e12345], 2) * -1.0);
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricQuotient<Circle> for Plane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        6        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        6        7        1
    //  no simd        6       10        1
    fn geometric_quotient(self, other: Circle) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                - f32::powi(other[e321], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricQuotient<CircleRotor> for Plane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        6        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        7        7        1
    //  no simd        7       10        1
    fn geometric_quotient(self, other: CircleRotor) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                - f32::powi(other[e321], 2)
                - f32::powi(other[e12345], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricQuotient<Dipole> for Plane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        6        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        6        7        1
    //  no simd        6       10        1
    fn geometric_quotient(self, other: Dipole) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e45], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - 2.0 * (other[e41] * other[e15])
                - 2.0 * (other[e42] * other[e25])
                - 2.0 * (other[e43] * other[e35]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricQuotient<DipoleInversion> for Plane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10        8        1
    //    simd4        0        1        0
    // Totals...
    // yes simd       10        9        1
    //  no simd       10       12        1
    fn geometric_quotient(self, other: DipoleInversion) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e1234] * other[e3215]) + f32::powi(other[e45], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - f32::powi(other[e4235], 2)
                - f32::powi(other[e4315], 2)
                - f32::powi(other[e4125], 2)
                - 2.0 * (other[e41] * other[e15])
                - 2.0 * (other[e42] * other[e25])
                - 2.0 * (other[e43] * other[e35]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricQuotient<DualNum> for Plane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        5        1
    fn geometric_quotient(self, other: DualNum) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e12345], 2) * -1.0);
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricQuotient<FlatPoint> for Plane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        1        1
    //  no simd        0        4        1
    fn geometric_quotient(self, other: FlatPoint) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e45], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricQuotient<Flector> for Plane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        1        1
    //  no simd        3        4        1
    fn geometric_quotient(self, other: Flector) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e45], 2) - f32::powi(other[e4235], 2) - f32::powi(other[e4315], 2) - f32::powi(other[e4125], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricQuotient<Line> for Plane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        1        1
    //  no simd        2        4        1
    fn geometric_quotient(self, other: Line) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e415], 2) + f32::powi(other[e425], 2) + f32::powi(other[e435], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricQuotient<Motor> for Plane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        1        1
    //  no simd        3        4        1
    fn geometric_quotient(self, other: Motor) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e415], 2) + f32::powi(other[e425], 2) + f32::powi(other[e435], 2) - f32::powi(other[e12345], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricQuotient<MultiVector> for Plane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       23       16        1
    //    simd4        0        1        0
    // Totals...
    // yes simd       23       17        1
    //  no simd       23       20        1
    fn geometric_quotient(self, other: MultiVector) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + 2.0 * (other[e3215] * other[e1234])
                + f32::powi(other[scalar], 2)
                + f32::powi(other[e1], 2)
                + f32::powi(other[e2], 2)
                + f32::powi(other[e3], 2)
                + f32::powi(other[e45], 2)
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                - f32::powi(other[e12345], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - f32::powi(other[e321], 2)
                - f32::powi(other[e4235], 2)
                - f32::powi(other[e4315], 2)
                - f32::powi(other[e4125], 2)
                - 2.0 * (other[e4] * other[e5])
                - 2.0 * (other[e15] * other[e41])
                - 2.0 * (other[e25] * other[e42])
                - 2.0 * (other[e35] * other[e43]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricQuotient<Plane> for Plane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        1        1
    //  no simd        2        4        1
    fn geometric_quotient(self, other: Plane) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ -f32::powi(other[e4235], 2) - f32::powi(other[e4315], 2) - f32::powi(other[e4125], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricQuotient<RoundPoint> for Plane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        2        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        3        1
    //  no simd        3        6        1
    fn geometric_quotient(self, other: RoundPoint) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2) - 2.0 * (other[e4] * other[e5]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricQuotient<Scalar> for Plane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        1        1
    //  no simd        0        4        1
    fn geometric_quotient(self, other: Scalar) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricQuotient<Sphere> for Plane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        2        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        3        1
    //  no simd        3        6        1
    fn geometric_quotient(self, other: Sphere) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e3215] * other[e1234]) - f32::powi(other[e4235], 2) - f32::powi(other[e4315], 2) - f32::powi(other[e4125], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricQuotient<VersorEven> for Plane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11        8        1
    //    simd4        0        1        0
    // Totals...
    // yes simd       11        9        1
    //  no simd       11       12        1
    fn geometric_quotient(self, other: VersorEven) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                + f32::powi(other[e1], 2)
                + f32::powi(other[e2], 2)
                + f32::powi(other[e3], 2)
                - f32::powi(other[e12345], 2)
                - f32::powi(other[e321], 2)
                - 2.0 * (other[e5] * other[e4]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricQuotient<VersorOdd> for Plane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11        8        1
    //    simd4        0        1        0
    // Totals...
    // yes simd       11        9        1
    //  no simd       11       12        1
    fn geometric_quotient(self, other: VersorOdd) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e1234] * other[e3215]) + f32::powi(other[scalar], 2) + f32::powi(other[e45], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - f32::powi(other[e4235], 2)
                - f32::powi(other[e4315], 2)
                - f32::powi(other[e4125], 2)
                - 2.0 * (other[e41] * other[e15])
                - 2.0 * (other[e42] * other[e25])
                - 2.0 * (other[e43] * other[e35]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl std::ops::Div<geometric_quotient> for RoundPoint {
    type Output = geometric_quotient_partial<RoundPoint>;
    fn div(self, _rhs: geometric_quotient) -> Self::Output {
        geometric_quotient_partial(self)
    }
}
impl GeometricQuotient<AntiCircleRotor> for RoundPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        7        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        7        8        1
    //  no simd        7       11        1
    fn geometric_quotient(self, other: AntiCircleRotor) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e45], 2) + f32::powi(other[scalar], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - 2.0 * (other[e41] * other[e15])
                - 2.0 * (other[e42] * other[e25])
                - 2.0 * (other[e43] * other[e35]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            self[e5] * inverse[scalar],
        );
    }
}
impl GeometricQuotient<AntiDipoleInversion> for RoundPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10        9        1
    //    simd4        0        1        0
    // Totals...
    // yes simd       10       10        1
    //  no simd       10       13        1
    fn geometric_quotient(self, other: AntiDipoleInversion) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                + f32::powi(other[e1], 2)
                + f32::powi(other[e2], 2)
                + f32::powi(other[e3], 2)
                - f32::powi(other[e321], 2)
                - 2.0 * (other[e4] * other[e5]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            self[e5] * inverse[scalar],
        );
    }
}
impl GeometricQuotient<AntiDualNum> for RoundPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        5        1
    fn geometric_quotient(self, other: AntiDualNum) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            self[e5] * inverse[scalar],
        );
    }
}
impl GeometricQuotient<AntiFlatPoint> for RoundPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        1
    //  no simd        0        6        1
    fn geometric_quotient(self, other: AntiFlatPoint) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e321], 2) * -1.0);
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            self[e5] * inverse[scalar],
        );
    }
}
impl GeometricQuotient<AntiFlector> for RoundPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        1        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        2        1
    //  no simd        3        5        1
    fn geometric_quotient(self, other: AntiFlector) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2) - f32::powi(other[e321], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            self[e5] * inverse[scalar],
        );
    }
}
impl GeometricQuotient<AntiLine> for RoundPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        1        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        2        1
    //  no simd        2        5        1
    fn geometric_quotient(self, other: AntiLine) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ -f32::powi(other[e23], 2) - f32::powi(other[e31], 2) - f32::powi(other[e12], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            self[e5] * inverse[scalar],
        );
    }
}
impl GeometricQuotient<AntiMotor> for RoundPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        1        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        2        1
    //  no simd        3        5        1
    fn geometric_quotient(self, other: AntiMotor) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[scalar], 2) - f32::powi(other[e23], 2) - f32::powi(other[e31], 2) - f32::powi(other[e12], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            self[e5] * inverse[scalar],
        );
    }
}
impl GeometricQuotient<AntiPlane> for RoundPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        1        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        2        1
    //  no simd        2        5        1
    fn geometric_quotient(self, other: AntiPlane) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            self[e5] * inverse[scalar],
        );
    }
}
impl GeometricQuotient<AntiScalar> for RoundPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        1
    //  no simd        0        6        1
    fn geometric_quotient(self, other: AntiScalar) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e12345], 2) * -1.0);
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            self[e5] * inverse[scalar],
        );
    }
}
impl GeometricQuotient<Circle> for RoundPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        7        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        6        8        1
    //  no simd        6       11        1
    fn geometric_quotient(self, other: Circle) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                - f32::powi(other[e321], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            self[e5] * inverse[scalar],
        );
    }
}
impl GeometricQuotient<CircleRotor> for RoundPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        7        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        7        8        1
    //  no simd        7       11        1
    fn geometric_quotient(self, other: CircleRotor) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                - f32::powi(other[e321], 2)
                - f32::powi(other[e12345], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            self[e5] * inverse[scalar],
        );
    }
}
impl GeometricQuotient<Dipole> for RoundPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        7        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        6        8        1
    //  no simd        6       11        1
    fn geometric_quotient(self, other: Dipole) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e45], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - 2.0 * (other[e41] * other[e15])
                - 2.0 * (other[e42] * other[e25])
                - 2.0 * (other[e43] * other[e35]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            self[e5] * inverse[scalar],
        );
    }
}
impl GeometricQuotient<DipoleInversion> for RoundPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10        9        1
    //    simd4        0        1        0
    // Totals...
    // yes simd       10       10        1
    //  no simd       10       13        1
    fn geometric_quotient(self, other: DipoleInversion) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e1234] * other[e3215]) + f32::powi(other[e45], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - f32::powi(other[e4235], 2)
                - f32::powi(other[e4315], 2)
                - f32::powi(other[e4125], 2)
                - 2.0 * (other[e41] * other[e15])
                - 2.0 * (other[e42] * other[e25])
                - 2.0 * (other[e43] * other[e35]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            self[e5] * inverse[scalar],
        );
    }
}
impl GeometricQuotient<DualNum> for RoundPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        1
    //  no simd        0        6        1
    fn geometric_quotient(self, other: DualNum) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e12345], 2) * -1.0);
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            self[e5] * inverse[scalar],
        );
    }
}
impl GeometricQuotient<FlatPoint> for RoundPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        5        1
    fn geometric_quotient(self, other: FlatPoint) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e45], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            self[e5] * inverse[scalar],
        );
    }
}
impl GeometricQuotient<Flector> for RoundPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        1        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        2        1
    //  no simd        3        5        1
    fn geometric_quotient(self, other: Flector) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e45], 2) - f32::powi(other[e4235], 2) - f32::powi(other[e4315], 2) - f32::powi(other[e4125], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            self[e5] * inverse[scalar],
        );
    }
}
impl GeometricQuotient<Line> for RoundPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        1        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        2        1
    //  no simd        2        5        1
    fn geometric_quotient(self, other: Line) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e415], 2) + f32::powi(other[e425], 2) + f32::powi(other[e435], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            self[e5] * inverse[scalar],
        );
    }
}
impl GeometricQuotient<Motor> for RoundPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        1        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        2        1
    //  no simd        3        5        1
    fn geometric_quotient(self, other: Motor) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e415], 2) + f32::powi(other[e425], 2) + f32::powi(other[e435], 2) - f32::powi(other[e12345], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            self[e5] * inverse[scalar],
        );
    }
}
impl GeometricQuotient<MultiVector> for RoundPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       23       17        1
    //    simd4        0        1        0
    // Totals...
    // yes simd       23       18        1
    //  no simd       23       21        1
    fn geometric_quotient(self, other: MultiVector) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + 2.0 * (other[e3215] * other[e1234])
                + f32::powi(other[scalar], 2)
                + f32::powi(other[e1], 2)
                + f32::powi(other[e2], 2)
                + f32::powi(other[e3], 2)
                + f32::powi(other[e45], 2)
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                - f32::powi(other[e12345], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - f32::powi(other[e321], 2)
                - f32::powi(other[e4235], 2)
                - f32::powi(other[e4315], 2)
                - f32::powi(other[e4125], 2)
                - 2.0 * (other[e4] * other[e5])
                - 2.0 * (other[e15] * other[e41])
                - 2.0 * (other[e25] * other[e42])
                - 2.0 * (other[e35] * other[e43]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            self[e5] * inverse[scalar],
        );
    }
}
impl GeometricQuotient<Plane> for RoundPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        1        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        2        1
    //  no simd        2        5        1
    fn geometric_quotient(self, other: Plane) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ -f32::powi(other[e4235], 2) - f32::powi(other[e4315], 2) - f32::powi(other[e4125], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            self[e5] * inverse[scalar],
        );
    }
}
impl GeometricQuotient<RoundPoint> for RoundPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        3        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        4        1
    //  no simd        3        7        1
    fn geometric_quotient(self, other: RoundPoint) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2) - 2.0 * (other[e4] * other[e5]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            self[e5] * inverse[scalar],
        );
    }
}
impl GeometricQuotient<Scalar> for RoundPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        5        1
    fn geometric_quotient(self, other: Scalar) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            self[e5] * inverse[scalar],
        );
    }
}
impl GeometricQuotient<Sphere> for RoundPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        3        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        4        1
    //  no simd        3        7        1
    fn geometric_quotient(self, other: Sphere) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e3215] * other[e1234]) - f32::powi(other[e4235], 2) - f32::powi(other[e4315], 2) - f32::powi(other[e4125], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            self[e5] * inverse[scalar],
        );
    }
}
impl GeometricQuotient<VersorEven> for RoundPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11        9        1
    //    simd4        0        1        0
    // Totals...
    // yes simd       11       10        1
    //  no simd       11       13        1
    fn geometric_quotient(self, other: VersorEven) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                + f32::powi(other[e1], 2)
                + f32::powi(other[e2], 2)
                + f32::powi(other[e3], 2)
                - f32::powi(other[e12345], 2)
                - f32::powi(other[e321], 2)
                - 2.0 * (other[e5] * other[e4]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            self[e5] * inverse[scalar],
        );
    }
}
impl GeometricQuotient<VersorOdd> for RoundPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11        9        1
    //    simd4        0        1        0
    // Totals...
    // yes simd       11       10        1
    //  no simd       11       13        1
    fn geometric_quotient(self, other: VersorOdd) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e1234] * other[e3215]) + f32::powi(other[scalar], 2) + f32::powi(other[e45], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - f32::powi(other[e4235], 2)
                - f32::powi(other[e4315], 2)
                - f32::powi(other[e4125], 2)
                - 2.0 * (other[e41] * other[e15])
                - 2.0 * (other[e42] * other[e25])
                - 2.0 * (other[e43] * other[e35]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            self[e5] * inverse[scalar],
        );
    }
}
impl std::ops::Div<geometric_quotient> for Scalar {
    type Output = geometric_quotient_partial<Scalar>;
    fn div(self, _rhs: geometric_quotient) -> Self::Output {
        geometric_quotient_partial(self)
    }
}
impl GeometricQuotient<AntiCircleRotor> for Scalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        7        1
    fn geometric_quotient(self, other: AntiCircleRotor) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e45], 2) + f32::powi(other[scalar], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - 2.0 * (other[e41] * other[e15])
                - 2.0 * (other[e42] * other[e25])
                - 2.0 * (other[e43] * other[e35]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Scalar::from_groups(/* scalar */ inverse[scalar] * self[scalar]);
    }
}
impl GeometricQuotient<AntiDipoleInversion> for Scalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10        9        1
    fn geometric_quotient(self, other: AntiDipoleInversion) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                + f32::powi(other[e1], 2)
                + f32::powi(other[e2], 2)
                + f32::powi(other[e3], 2)
                - f32::powi(other[e321], 2)
                - 2.0 * (other[e4] * other[e5]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Scalar::from_groups(/* scalar */ inverse[scalar] * self[scalar]);
    }
}
impl GeometricQuotient<AntiDualNum> for Scalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        1
    fn geometric_quotient(self, other: AntiDualNum) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Scalar::from_groups(/* scalar */ inverse[scalar] * self[scalar]);
    }
}
impl GeometricQuotient<AntiFlatPoint> for Scalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        1
    fn geometric_quotient(self, other: AntiFlatPoint) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e321], 2) * -1.0);
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Scalar::from_groups(/* scalar */ inverse[scalar] * self[scalar]);
    }
}
impl GeometricQuotient<AntiFlector> for Scalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        1        1
    fn geometric_quotient(self, other: AntiFlector) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2) - f32::powi(other[e321], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Scalar::from_groups(/* scalar */ inverse[scalar] * self[scalar]);
    }
}
impl GeometricQuotient<AntiLine> for Scalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        1        1
    fn geometric_quotient(self, other: AntiLine) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ -f32::powi(other[e23], 2) - f32::powi(other[e31], 2) - f32::powi(other[e12], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Scalar::from_groups(/* scalar */ inverse[scalar] * self[scalar]);
    }
}
impl GeometricQuotient<AntiMotor> for Scalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        1        1
    fn geometric_quotient(self, other: AntiMotor) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[scalar], 2) - f32::powi(other[e23], 2) - f32::powi(other[e31], 2) - f32::powi(other[e12], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Scalar::from_groups(/* scalar */ inverse[scalar] * self[scalar]);
    }
}
impl GeometricQuotient<AntiPlane> for Scalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        1        1
    fn geometric_quotient(self, other: AntiPlane) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Scalar::from_groups(/* scalar */ inverse[scalar] * self[scalar]);
    }
}
impl GeometricQuotient<AntiScalar> for Scalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        1
    fn geometric_quotient(self, other: AntiScalar) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e12345], 2) * -1.0);
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Scalar::from_groups(/* scalar */ inverse[scalar] * self[scalar]);
    }
}
impl GeometricQuotient<Circle> for Scalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        7        1
    fn geometric_quotient(self, other: Circle) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                - f32::powi(other[e321], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Scalar::from_groups(/* scalar */ inverse[scalar] * self[scalar]);
    }
}
impl GeometricQuotient<CircleRotor> for Scalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        7        1
    fn geometric_quotient(self, other: CircleRotor) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                - f32::powi(other[e321], 2)
                - f32::powi(other[e12345], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Scalar::from_groups(/* scalar */ inverse[scalar] * self[scalar]);
    }
}
impl GeometricQuotient<Dipole> for Scalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        7        1
    fn geometric_quotient(self, other: Dipole) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e45], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - 2.0 * (other[e41] * other[e15])
                - 2.0 * (other[e42] * other[e25])
                - 2.0 * (other[e43] * other[e35]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Scalar::from_groups(/* scalar */ inverse[scalar] * self[scalar]);
    }
}
impl GeometricQuotient<DipoleInversion> for Scalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10        9        1
    fn geometric_quotient(self, other: DipoleInversion) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e1234] * other[e3215]) + f32::powi(other[e45], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - f32::powi(other[e4235], 2)
                - f32::powi(other[e4315], 2)
                - f32::powi(other[e4125], 2)
                - 2.0 * (other[e41] * other[e15])
                - 2.0 * (other[e42] * other[e25])
                - 2.0 * (other[e43] * other[e35]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Scalar::from_groups(/* scalar */ inverse[scalar] * self[scalar]);
    }
}
impl GeometricQuotient<DualNum> for Scalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        1
    fn geometric_quotient(self, other: DualNum) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e12345], 2) * -1.0);
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Scalar::from_groups(/* scalar */ inverse[scalar] * self[scalar]);
    }
}
impl GeometricQuotient<FlatPoint> for Scalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        1
    fn geometric_quotient(self, other: FlatPoint) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e45], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Scalar::from_groups(/* scalar */ inverse[scalar] * self[scalar]);
    }
}
impl GeometricQuotient<Flector> for Scalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        1        1
    fn geometric_quotient(self, other: Flector) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e45], 2) - f32::powi(other[e4235], 2) - f32::powi(other[e4315], 2) - f32::powi(other[e4125], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Scalar::from_groups(/* scalar */ inverse[scalar] * self[scalar]);
    }
}
impl GeometricQuotient<Line> for Scalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        1        1
    fn geometric_quotient(self, other: Line) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e415], 2) + f32::powi(other[e425], 2) + f32::powi(other[e435], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Scalar::from_groups(/* scalar */ inverse[scalar] * self[scalar]);
    }
}
impl GeometricQuotient<Motor> for Scalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        1        1
    fn geometric_quotient(self, other: Motor) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e415], 2) + f32::powi(other[e425], 2) + f32::powi(other[e435], 2) - f32::powi(other[e12345], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Scalar::from_groups(/* scalar */ inverse[scalar] * self[scalar]);
    }
}
impl GeometricQuotient<MultiVector> for Scalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       23       17        1
    fn geometric_quotient(self, other: MultiVector) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + 2.0 * (other[e3215] * other[e1234])
                + f32::powi(other[scalar], 2)
                + f32::powi(other[e1], 2)
                + f32::powi(other[e2], 2)
                + f32::powi(other[e3], 2)
                + f32::powi(other[e45], 2)
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                - f32::powi(other[e12345], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - f32::powi(other[e321], 2)
                - f32::powi(other[e4235], 2)
                - f32::powi(other[e4315], 2)
                - f32::powi(other[e4125], 2)
                - 2.0 * (other[e4] * other[e5])
                - 2.0 * (other[e15] * other[e41])
                - 2.0 * (other[e25] * other[e42])
                - 2.0 * (other[e35] * other[e43]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Scalar::from_groups(/* scalar */ inverse[scalar] * self[scalar]);
    }
}
impl GeometricQuotient<Plane> for Scalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        1        1
    fn geometric_quotient(self, other: Plane) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ -f32::powi(other[e4235], 2) - f32::powi(other[e4315], 2) - f32::powi(other[e4125], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Scalar::from_groups(/* scalar */ inverse[scalar] * self[scalar]);
    }
}
impl GeometricQuotient<RoundPoint> for Scalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        3        1
    fn geometric_quotient(self, other: RoundPoint) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2) - 2.0 * (other[e4] * other[e5]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Scalar::from_groups(/* scalar */ inverse[scalar] * self[scalar]);
    }
}
impl GeometricQuotient<Scalar> for Scalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        1
    fn geometric_quotient(self, other: Scalar) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Scalar::from_groups(/* scalar */ inverse[scalar] * self[scalar]);
    }
}
impl GeometricQuotient<Sphere> for Scalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        3        1
    fn geometric_quotient(self, other: Sphere) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e3215] * other[e1234]) - f32::powi(other[e4235], 2) - f32::powi(other[e4315], 2) - f32::powi(other[e4125], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Scalar::from_groups(/* scalar */ inverse[scalar] * self[scalar]);
    }
}
impl GeometricQuotient<VersorEven> for Scalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       11        9        1
    fn geometric_quotient(self, other: VersorEven) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                + f32::powi(other[e1], 2)
                + f32::powi(other[e2], 2)
                + f32::powi(other[e3], 2)
                - f32::powi(other[e12345], 2)
                - f32::powi(other[e321], 2)
                - 2.0 * (other[e5] * other[e4]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Scalar::from_groups(/* scalar */ inverse[scalar] * self[scalar]);
    }
}
impl GeometricQuotient<VersorOdd> for Scalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       11        9        1
    fn geometric_quotient(self, other: VersorOdd) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e1234] * other[e3215]) + f32::powi(other[scalar], 2) + f32::powi(other[e45], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - f32::powi(other[e4235], 2)
                - f32::powi(other[e4315], 2)
                - f32::powi(other[e4125], 2)
                - 2.0 * (other[e41] * other[e15])
                - 2.0 * (other[e42] * other[e25])
                - 2.0 * (other[e43] * other[e35]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Scalar::from_groups(/* scalar */ inverse[scalar] * self[scalar]);
    }
}
impl std::ops::Div<geometric_quotient> for Sphere {
    type Output = geometric_quotient_partial<Sphere>;
    fn div(self, _rhs: geometric_quotient) -> Self::Output {
        geometric_quotient_partial(self)
    }
}
impl GeometricQuotient<AntiCircleRotor> for Sphere {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        7        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        7        8        1
    //  no simd        7       11        1
    fn geometric_quotient(self, other: AntiCircleRotor) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e45], 2) + f32::powi(other[scalar], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - 2.0 * (other[e41] * other[e15])
                - 2.0 * (other[e42] * other[e25])
                - 2.0 * (other[e43] * other[e35]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
            // e1234
            inverse[scalar] * self[e1234],
        );
    }
}
impl GeometricQuotient<AntiDipoleInversion> for Sphere {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10        9        1
    //    simd4        0        1        0
    // Totals...
    // yes simd       10       10        1
    //  no simd       10       13        1
    fn geometric_quotient(self, other: AntiDipoleInversion) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                + f32::powi(other[e1], 2)
                + f32::powi(other[e2], 2)
                + f32::powi(other[e3], 2)
                - f32::powi(other[e321], 2)
                - 2.0 * (other[e4] * other[e5]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
            // e1234
            inverse[scalar] * self[e1234],
        );
    }
}
impl GeometricQuotient<AntiDualNum> for Sphere {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        5        1
    fn geometric_quotient(self, other: AntiDualNum) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
            // e1234
            inverse[scalar] * self[e1234],
        );
    }
}
impl GeometricQuotient<AntiFlatPoint> for Sphere {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        1
    //  no simd        0        6        1
    fn geometric_quotient(self, other: AntiFlatPoint) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e321], 2) * -1.0);
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
            // e1234
            inverse[scalar] * self[e1234],
        );
    }
}
impl GeometricQuotient<AntiFlector> for Sphere {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        1        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        2        1
    //  no simd        3        5        1
    fn geometric_quotient(self, other: AntiFlector) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2) - f32::powi(other[e321], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
            // e1234
            inverse[scalar] * self[e1234],
        );
    }
}
impl GeometricQuotient<AntiLine> for Sphere {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        1        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        2        1
    //  no simd        2        5        1
    fn geometric_quotient(self, other: AntiLine) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ -f32::powi(other[e23], 2) - f32::powi(other[e31], 2) - f32::powi(other[e12], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
            // e1234
            inverse[scalar] * self[e1234],
        );
    }
}
impl GeometricQuotient<AntiMotor> for Sphere {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        1        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        2        1
    //  no simd        3        5        1
    fn geometric_quotient(self, other: AntiMotor) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[scalar], 2) - f32::powi(other[e23], 2) - f32::powi(other[e31], 2) - f32::powi(other[e12], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
            // e1234
            inverse[scalar] * self[e1234],
        );
    }
}
impl GeometricQuotient<AntiPlane> for Sphere {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        1        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        2        1
    //  no simd        2        5        1
    fn geometric_quotient(self, other: AntiPlane) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
            // e1234
            inverse[scalar] * self[e1234],
        );
    }
}
impl GeometricQuotient<AntiScalar> for Sphere {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        1
    //  no simd        0        6        1
    fn geometric_quotient(self, other: AntiScalar) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e12345], 2) * -1.0);
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
            // e1234
            inverse[scalar] * self[e1234],
        );
    }
}
impl GeometricQuotient<Circle> for Sphere {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        7        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        6        8        1
    //  no simd        6       11        1
    fn geometric_quotient(self, other: Circle) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                - f32::powi(other[e321], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
            // e1234
            inverse[scalar] * self[e1234],
        );
    }
}
impl GeometricQuotient<CircleRotor> for Sphere {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        7        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        7        8        1
    //  no simd        7       11        1
    fn geometric_quotient(self, other: CircleRotor) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                - f32::powi(other[e321], 2)
                - f32::powi(other[e12345], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
            // e1234
            inverse[scalar] * self[e1234],
        );
    }
}
impl GeometricQuotient<Dipole> for Sphere {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        7        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        6        8        1
    //  no simd        6       11        1
    fn geometric_quotient(self, other: Dipole) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e45], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - 2.0 * (other[e41] * other[e15])
                - 2.0 * (other[e42] * other[e25])
                - 2.0 * (other[e43] * other[e35]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
            // e1234
            inverse[scalar] * self[e1234],
        );
    }
}
impl GeometricQuotient<DipoleInversion> for Sphere {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10        9        1
    //    simd4        0        1        0
    // Totals...
    // yes simd       10       10        1
    //  no simd       10       13        1
    fn geometric_quotient(self, other: DipoleInversion) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e1234] * other[e3215]) + f32::powi(other[e45], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - f32::powi(other[e4235], 2)
                - f32::powi(other[e4315], 2)
                - f32::powi(other[e4125], 2)
                - 2.0 * (other[e41] * other[e15])
                - 2.0 * (other[e42] * other[e25])
                - 2.0 * (other[e43] * other[e35]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
            // e1234
            inverse[scalar] * self[e1234],
        );
    }
}
impl GeometricQuotient<DualNum> for Sphere {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        1
    //  no simd        0        6        1
    fn geometric_quotient(self, other: DualNum) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e12345], 2) * -1.0);
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
            // e1234
            inverse[scalar] * self[e1234],
        );
    }
}
impl GeometricQuotient<FlatPoint> for Sphere {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        5        1
    fn geometric_quotient(self, other: FlatPoint) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e45], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
            // e1234
            inverse[scalar] * self[e1234],
        );
    }
}
impl GeometricQuotient<Flector> for Sphere {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        1        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        2        1
    //  no simd        3        5        1
    fn geometric_quotient(self, other: Flector) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e45], 2) - f32::powi(other[e4235], 2) - f32::powi(other[e4315], 2) - f32::powi(other[e4125], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
            // e1234
            inverse[scalar] * self[e1234],
        );
    }
}
impl GeometricQuotient<Line> for Sphere {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        1        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        2        1
    //  no simd        2        5        1
    fn geometric_quotient(self, other: Line) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e415], 2) + f32::powi(other[e425], 2) + f32::powi(other[e435], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
            // e1234
            inverse[scalar] * self[e1234],
        );
    }
}
impl GeometricQuotient<Motor> for Sphere {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        1        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        2        1
    //  no simd        3        5        1
    fn geometric_quotient(self, other: Motor) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e415], 2) + f32::powi(other[e425], 2) + f32::powi(other[e435], 2) - f32::powi(other[e12345], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
            // e1234
            inverse[scalar] * self[e1234],
        );
    }
}
impl GeometricQuotient<MultiVector> for Sphere {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       23       17        1
    //    simd4        0        1        0
    // Totals...
    // yes simd       23       18        1
    //  no simd       23       21        1
    fn geometric_quotient(self, other: MultiVector) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + 2.0 * (other[e3215] * other[e1234])
                + f32::powi(other[scalar], 2)
                + f32::powi(other[e1], 2)
                + f32::powi(other[e2], 2)
                + f32::powi(other[e3], 2)
                + f32::powi(other[e45], 2)
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                - f32::powi(other[e12345], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - f32::powi(other[e321], 2)
                - f32::powi(other[e4235], 2)
                - f32::powi(other[e4315], 2)
                - f32::powi(other[e4125], 2)
                - 2.0 * (other[e4] * other[e5])
                - 2.0 * (other[e15] * other[e41])
                - 2.0 * (other[e25] * other[e42])
                - 2.0 * (other[e35] * other[e43]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
            // e1234
            inverse[scalar] * self[e1234],
        );
    }
}
impl GeometricQuotient<Plane> for Sphere {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        1        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        2        1
    //  no simd        2        5        1
    fn geometric_quotient(self, other: Plane) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ -f32::powi(other[e4235], 2) - f32::powi(other[e4315], 2) - f32::powi(other[e4125], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
            // e1234
            inverse[scalar] * self[e1234],
        );
    }
}
impl GeometricQuotient<RoundPoint> for Sphere {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        3        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        4        1
    //  no simd        3        7        1
    fn geometric_quotient(self, other: RoundPoint) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2) - 2.0 * (other[e4] * other[e5]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
            // e1234
            inverse[scalar] * self[e1234],
        );
    }
}
impl GeometricQuotient<Scalar> for Sphere {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        5        1
    fn geometric_quotient(self, other: Scalar) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
            // e1234
            inverse[scalar] * self[e1234],
        );
    }
}
impl GeometricQuotient<Sphere> for Sphere {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        3        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        4        1
    //  no simd        3        7        1
    fn geometric_quotient(self, other: Sphere) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e3215] * other[e1234]) - f32::powi(other[e4235], 2) - f32::powi(other[e4315], 2) - f32::powi(other[e4125], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
            // e1234
            inverse[scalar] * self[e1234],
        );
    }
}
impl GeometricQuotient<VersorEven> for Sphere {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11        9        1
    //    simd4        0        1        0
    // Totals...
    // yes simd       11       10        1
    //  no simd       11       13        1
    fn geometric_quotient(self, other: VersorEven) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                + f32::powi(other[e1], 2)
                + f32::powi(other[e2], 2)
                + f32::powi(other[e3], 2)
                - f32::powi(other[e12345], 2)
                - f32::powi(other[e321], 2)
                - 2.0 * (other[e5] * other[e4]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
            // e1234
            inverse[scalar] * self[e1234],
        );
    }
}
impl GeometricQuotient<VersorOdd> for Sphere {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11        9        1
    //    simd4        0        1        0
    // Totals...
    // yes simd       11       10        1
    //  no simd       11       13        1
    fn geometric_quotient(self, other: VersorOdd) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e1234] * other[e3215]) + f32::powi(other[scalar], 2) + f32::powi(other[e45], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - f32::powi(other[e4235], 2)
                - f32::powi(other[e4315], 2)
                - f32::powi(other[e4125], 2)
                - 2.0 * (other[e41] * other[e15])
                - 2.0 * (other[e42] * other[e25])
                - 2.0 * (other[e43] * other[e35]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
            // e1234
            inverse[scalar] * self[e1234],
        );
    }
}
impl std::ops::Div<geometric_quotient> for VersorEven {
    type Output = geometric_quotient_partial<VersorEven>;
    fn div(self, _rhs: geometric_quotient) -> Self::Output {
        geometric_quotient_partial(self)
    }
}
impl GeometricQuotient<AntiCircleRotor> for VersorEven {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        6        1
    //    simd4        0        4        0
    // Totals...
    // yes simd        7       10        1
    //  no simd        7       22        1
    fn geometric_quotient(self, other: AntiCircleRotor) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e45], 2) + f32::powi(other[scalar], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - 2.0 * (other[e41] * other[e15])
                - 2.0 * (other[e42] * other[e25])
                - 2.0 * (other[e43] * other[e35]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e423], self[e431], self[e412], self[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
        );
    }
}
impl GeometricQuotient<AntiDipoleInversion> for VersorEven {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10        8        1
    //    simd4        0        4        0
    // Totals...
    // yes simd       10       12        1
    //  no simd       10       24        1
    fn geometric_quotient(self, other: AntiDipoleInversion) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                + f32::powi(other[e1], 2)
                + f32::powi(other[e2], 2)
                + f32::powi(other[e3], 2)
                - f32::powi(other[e321], 2)
                - 2.0 * (other[e4] * other[e5]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e423], self[e431], self[e412], self[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
        );
    }
}
impl GeometricQuotient<AntiDualNum> for VersorEven {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd4        0        4        0
    // Totals...
    // yes simd        0        4        1
    //  no simd        0       16        1
    fn geometric_quotient(self, other: AntiDualNum) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e423], self[e431], self[e412], self[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
        );
    }
}
impl GeometricQuotient<AntiFlatPoint> for VersorEven {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd4        0        4        0
    // Totals...
    // yes simd        0        5        1
    //  no simd        0       17        1
    fn geometric_quotient(self, other: AntiFlatPoint) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e321], 2) * -1.0);
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e423], self[e431], self[e412], self[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
        );
    }
}
impl GeometricQuotient<AntiFlector> for VersorEven {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd4        0        4        0
    // Totals...
    // yes simd        3        4        1
    //  no simd        3       16        1
    fn geometric_quotient(self, other: AntiFlector) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2) - f32::powi(other[e321], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e423], self[e431], self[e412], self[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
        );
    }
}
impl GeometricQuotient<AntiLine> for VersorEven {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd4        0        4        0
    // Totals...
    // yes simd        2        4        1
    //  no simd        2       16        1
    fn geometric_quotient(self, other: AntiLine) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ -f32::powi(other[e23], 2) - f32::powi(other[e31], 2) - f32::powi(other[e12], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e423], self[e431], self[e412], self[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
        );
    }
}
impl GeometricQuotient<AntiMotor> for VersorEven {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd4        0        4        0
    // Totals...
    // yes simd        3        4        1
    //  no simd        3       16        1
    fn geometric_quotient(self, other: AntiMotor) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[scalar], 2) - f32::powi(other[e23], 2) - f32::powi(other[e31], 2) - f32::powi(other[e12], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e423], self[e431], self[e412], self[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
        );
    }
}
impl GeometricQuotient<AntiPlane> for VersorEven {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd4        0        4        0
    // Totals...
    // yes simd        2        4        1
    //  no simd        2       16        1
    fn geometric_quotient(self, other: AntiPlane) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e423], self[e431], self[e412], self[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
        );
    }
}
impl GeometricQuotient<AntiScalar> for VersorEven {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd4        0        4        0
    // Totals...
    // yes simd        0        5        1
    //  no simd        0       17        1
    fn geometric_quotient(self, other: AntiScalar) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e12345], 2) * -1.0);
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e423], self[e431], self[e412], self[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
        );
    }
}
impl GeometricQuotient<Circle> for VersorEven {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        6        1
    //    simd4        0        4        0
    // Totals...
    // yes simd        6       10        1
    //  no simd        6       22        1
    fn geometric_quotient(self, other: Circle) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                - f32::powi(other[e321], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e423], self[e431], self[e412], self[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
        );
    }
}
impl GeometricQuotient<CircleRotor> for VersorEven {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        6        1
    //    simd4        0        4        0
    // Totals...
    // yes simd        7       10        1
    //  no simd        7       22        1
    fn geometric_quotient(self, other: CircleRotor) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                - f32::powi(other[e321], 2)
                - f32::powi(other[e12345], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e423], self[e431], self[e412], self[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
        );
    }
}
impl GeometricQuotient<Dipole> for VersorEven {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        6        1
    //    simd4        0        4        0
    // Totals...
    // yes simd        6       10        1
    //  no simd        6       22        1
    fn geometric_quotient(self, other: Dipole) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e45], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - 2.0 * (other[e41] * other[e15])
                - 2.0 * (other[e42] * other[e25])
                - 2.0 * (other[e43] * other[e35]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e423], self[e431], self[e412], self[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
        );
    }
}
impl GeometricQuotient<DipoleInversion> for VersorEven {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10        8        1
    //    simd4        0        4        0
    // Totals...
    // yes simd       10       12        1
    //  no simd       10       24        1
    fn geometric_quotient(self, other: DipoleInversion) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e1234] * other[e3215]) + f32::powi(other[e45], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - f32::powi(other[e4235], 2)
                - f32::powi(other[e4315], 2)
                - f32::powi(other[e4125], 2)
                - 2.0 * (other[e41] * other[e15])
                - 2.0 * (other[e42] * other[e25])
                - 2.0 * (other[e43] * other[e35]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e423], self[e431], self[e412], self[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
        );
    }
}
impl GeometricQuotient<DualNum> for VersorEven {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd4        0        4        0
    // Totals...
    // yes simd        0        5        1
    //  no simd        0       17        1
    fn geometric_quotient(self, other: DualNum) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e12345], 2) * -1.0);
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e423], self[e431], self[e412], self[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
        );
    }
}
impl GeometricQuotient<FlatPoint> for VersorEven {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd4        0        4        0
    // Totals...
    // yes simd        0        4        1
    //  no simd        0       16        1
    fn geometric_quotient(self, other: FlatPoint) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e45], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e423], self[e431], self[e412], self[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
        );
    }
}
impl GeometricQuotient<Flector> for VersorEven {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd4        0        4        0
    // Totals...
    // yes simd        3        4        1
    //  no simd        3       16        1
    fn geometric_quotient(self, other: Flector) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e45], 2) - f32::powi(other[e4235], 2) - f32::powi(other[e4315], 2) - f32::powi(other[e4125], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e423], self[e431], self[e412], self[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
        );
    }
}
impl GeometricQuotient<Line> for VersorEven {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd4        0        4        0
    // Totals...
    // yes simd        2        4        1
    //  no simd        2       16        1
    fn geometric_quotient(self, other: Line) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e415], 2) + f32::powi(other[e425], 2) + f32::powi(other[e435], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e423], self[e431], self[e412], self[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
        );
    }
}
impl GeometricQuotient<Motor> for VersorEven {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd4        0        4        0
    // Totals...
    // yes simd        3        4        1
    //  no simd        3       16        1
    fn geometric_quotient(self, other: Motor) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e415], 2) + f32::powi(other[e425], 2) + f32::powi(other[e435], 2) - f32::powi(other[e12345], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e423], self[e431], self[e412], self[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
        );
    }
}
impl GeometricQuotient<MultiVector> for VersorEven {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       23       16        1
    //    simd4        0        4        0
    // Totals...
    // yes simd       23       20        1
    //  no simd       23       32        1
    fn geometric_quotient(self, other: MultiVector) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + 2.0 * (other[e3215] * other[e1234])
                + f32::powi(other[scalar], 2)
                + f32::powi(other[e1], 2)
                + f32::powi(other[e2], 2)
                + f32::powi(other[e3], 2)
                + f32::powi(other[e45], 2)
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                - f32::powi(other[e12345], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - f32::powi(other[e321], 2)
                - f32::powi(other[e4235], 2)
                - f32::powi(other[e4315], 2)
                - f32::powi(other[e4125], 2)
                - 2.0 * (other[e4] * other[e5])
                - 2.0 * (other[e15] * other[e41])
                - 2.0 * (other[e25] * other[e42])
                - 2.0 * (other[e35] * other[e43]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e423], self[e431], self[e412], self[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
        );
    }
}
impl GeometricQuotient<Plane> for VersorEven {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd4        0        4        0
    // Totals...
    // yes simd        2        4        1
    //  no simd        2       16        1
    fn geometric_quotient(self, other: Plane) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ -f32::powi(other[e4235], 2) - f32::powi(other[e4315], 2) - f32::powi(other[e4125], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e423], self[e431], self[e412], self[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
        );
    }
}
impl GeometricQuotient<RoundPoint> for VersorEven {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        2        1
    //    simd4        0        4        0
    // Totals...
    // yes simd        3        6        1
    //  no simd        3       18        1
    fn geometric_quotient(self, other: RoundPoint) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2) - 2.0 * (other[e4] * other[e5]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e423], self[e431], self[e412], self[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
        );
    }
}
impl GeometricQuotient<Scalar> for VersorEven {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd4        0        4        0
    // Totals...
    // yes simd        0        4        1
    //  no simd        0       16        1
    fn geometric_quotient(self, other: Scalar) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e423], self[e431], self[e412], self[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
        );
    }
}
impl GeometricQuotient<Sphere> for VersorEven {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        2        1
    //    simd4        0        4        0
    // Totals...
    // yes simd        3        6        1
    //  no simd        3       18        1
    fn geometric_quotient(self, other: Sphere) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e3215] * other[e1234]) - f32::powi(other[e4235], 2) - f32::powi(other[e4315], 2) - f32::powi(other[e4125], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e423], self[e431], self[e412], self[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
        );
    }
}
impl GeometricQuotient<VersorEven> for VersorEven {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11        8        1
    //    simd4        0        4        0
    // Totals...
    // yes simd       11       12        1
    //  no simd       11       24        1
    fn geometric_quotient(self, other: VersorEven) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                + f32::powi(other[e1], 2)
                + f32::powi(other[e2], 2)
                + f32::powi(other[e3], 2)
                - f32::powi(other[e12345], 2)
                - f32::powi(other[e321], 2)
                - 2.0 * (other[e5] * other[e4]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e423], self[e431], self[e412], self[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
        );
    }
}
impl GeometricQuotient<VersorOdd> for VersorEven {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11        8        1
    //    simd4        0        4        0
    // Totals...
    // yes simd       11       12        1
    //  no simd       11       24        1
    fn geometric_quotient(self, other: VersorOdd) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e1234] * other[e3215]) + f32::powi(other[scalar], 2) + f32::powi(other[e45], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - f32::powi(other[e4235], 2)
                - f32::powi(other[e4315], 2)
                - f32::powi(other[e4125], 2)
                - 2.0 * (other[e41] * other[e15])
                - 2.0 * (other[e42] * other[e25])
                - 2.0 * (other[e43] * other[e35]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e423], self[e431], self[e412], self[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
        );
    }
}
impl std::ops::Div<geometric_quotient> for VersorOdd {
    type Output = geometric_quotient_partial<VersorOdd>;
    fn div(self, _rhs: geometric_quotient) -> Self::Output {
        geometric_quotient_partial(self)
    }
}
impl GeometricQuotient<AntiCircleRotor> for VersorOdd {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        6        1
    //    simd4        0        4        0
    // Totals...
    // yes simd        7       10        1
    //  no simd        7       22        1
    fn geometric_quotient(self, other: AntiCircleRotor) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e45], 2) + f32::powi(other[scalar], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - 2.0 * (other[e41] * other[e15])
                - 2.0 * (other[e42] * other[e25])
                - 2.0 * (other[e43] * other[e35]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e41], self[e42], self[e43], self[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricQuotient<AntiDipoleInversion> for VersorOdd {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10        8        1
    //    simd4        0        4        0
    // Totals...
    // yes simd       10       12        1
    //  no simd       10       24        1
    fn geometric_quotient(self, other: AntiDipoleInversion) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                + f32::powi(other[e1], 2)
                + f32::powi(other[e2], 2)
                + f32::powi(other[e3], 2)
                - f32::powi(other[e321], 2)
                - 2.0 * (other[e4] * other[e5]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e41], self[e42], self[e43], self[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricQuotient<AntiDualNum> for VersorOdd {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd4        0        4        0
    // Totals...
    // yes simd        0        4        1
    //  no simd        0       16        1
    fn geometric_quotient(self, other: AntiDualNum) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e41], self[e42], self[e43], self[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricQuotient<AntiFlatPoint> for VersorOdd {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd4        0        4        0
    // Totals...
    // yes simd        0        5        1
    //  no simd        0       17        1
    fn geometric_quotient(self, other: AntiFlatPoint) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e321], 2) * -1.0);
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e41], self[e42], self[e43], self[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricQuotient<AntiFlector> for VersorOdd {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd4        0        4        0
    // Totals...
    // yes simd        3        4        1
    //  no simd        3       16        1
    fn geometric_quotient(self, other: AntiFlector) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2) - f32::powi(other[e321], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e41], self[e42], self[e43], self[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricQuotient<AntiLine> for VersorOdd {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd4        0        4        0
    // Totals...
    // yes simd        2        4        1
    //  no simd        2       16        1
    fn geometric_quotient(self, other: AntiLine) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ -f32::powi(other[e23], 2) - f32::powi(other[e31], 2) - f32::powi(other[e12], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e41], self[e42], self[e43], self[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricQuotient<AntiMotor> for VersorOdd {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd4        0        4        0
    // Totals...
    // yes simd        3        4        1
    //  no simd        3       16        1
    fn geometric_quotient(self, other: AntiMotor) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[scalar], 2) - f32::powi(other[e23], 2) - f32::powi(other[e31], 2) - f32::powi(other[e12], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e41], self[e42], self[e43], self[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricQuotient<AntiPlane> for VersorOdd {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd4        0        4        0
    // Totals...
    // yes simd        2        4        1
    //  no simd        2       16        1
    fn geometric_quotient(self, other: AntiPlane) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e41], self[e42], self[e43], self[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricQuotient<AntiScalar> for VersorOdd {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd4        0        4        0
    // Totals...
    // yes simd        0        5        1
    //  no simd        0       17        1
    fn geometric_quotient(self, other: AntiScalar) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e12345], 2) * -1.0);
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e41], self[e42], self[e43], self[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricQuotient<Circle> for VersorOdd {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        6        1
    //    simd4        0        4        0
    // Totals...
    // yes simd        6       10        1
    //  no simd        6       22        1
    fn geometric_quotient(self, other: Circle) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                - f32::powi(other[e321], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e41], self[e42], self[e43], self[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricQuotient<CircleRotor> for VersorOdd {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        6        1
    //    simd4        0        4        0
    // Totals...
    // yes simd        7       10        1
    //  no simd        7       22        1
    fn geometric_quotient(self, other: CircleRotor) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                - f32::powi(other[e321], 2)
                - f32::powi(other[e12345], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e41], self[e42], self[e43], self[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricQuotient<Dipole> for VersorOdd {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        6        1
    //    simd4        0        4        0
    // Totals...
    // yes simd        6       10        1
    //  no simd        6       22        1
    fn geometric_quotient(self, other: Dipole) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e45], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - 2.0 * (other[e41] * other[e15])
                - 2.0 * (other[e42] * other[e25])
                - 2.0 * (other[e43] * other[e35]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e41], self[e42], self[e43], self[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricQuotient<DipoleInversion> for VersorOdd {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10        8        1
    //    simd4        0        4        0
    // Totals...
    // yes simd       10       12        1
    //  no simd       10       24        1
    fn geometric_quotient(self, other: DipoleInversion) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e1234] * other[e3215]) + f32::powi(other[e45], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - f32::powi(other[e4235], 2)
                - f32::powi(other[e4315], 2)
                - f32::powi(other[e4125], 2)
                - 2.0 * (other[e41] * other[e15])
                - 2.0 * (other[e42] * other[e25])
                - 2.0 * (other[e43] * other[e35]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e41], self[e42], self[e43], self[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricQuotient<DualNum> for VersorOdd {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd4        0        4        0
    // Totals...
    // yes simd        0        5        1
    //  no simd        0       17        1
    fn geometric_quotient(self, other: DualNum) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e12345], 2) * -1.0);
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e41], self[e42], self[e43], self[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricQuotient<FlatPoint> for VersorOdd {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd4        0        4        0
    // Totals...
    // yes simd        0        4        1
    //  no simd        0       16        1
    fn geometric_quotient(self, other: FlatPoint) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e45], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e41], self[e42], self[e43], self[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricQuotient<Flector> for VersorOdd {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd4        0        4        0
    // Totals...
    // yes simd        3        4        1
    //  no simd        3       16        1
    fn geometric_quotient(self, other: Flector) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e45], 2) - f32::powi(other[e4235], 2) - f32::powi(other[e4315], 2) - f32::powi(other[e4125], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e41], self[e42], self[e43], self[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricQuotient<Line> for VersorOdd {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd4        0        4        0
    // Totals...
    // yes simd        2        4        1
    //  no simd        2       16        1
    fn geometric_quotient(self, other: Line) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e415], 2) + f32::powi(other[e425], 2) + f32::powi(other[e435], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e41], self[e42], self[e43], self[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricQuotient<Motor> for VersorOdd {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd4        0        4        0
    // Totals...
    // yes simd        3        4        1
    //  no simd        3       16        1
    fn geometric_quotient(self, other: Motor) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e415], 2) + f32::powi(other[e425], 2) + f32::powi(other[e435], 2) - f32::powi(other[e12345], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e41], self[e42], self[e43], self[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricQuotient<MultiVector> for VersorOdd {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       23       16        1
    //    simd4        0        4        0
    // Totals...
    // yes simd       23       20        1
    //  no simd       23       32        1
    fn geometric_quotient(self, other: MultiVector) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + 2.0 * (other[e3215] * other[e1234])
                + f32::powi(other[scalar], 2)
                + f32::powi(other[e1], 2)
                + f32::powi(other[e2], 2)
                + f32::powi(other[e3], 2)
                + f32::powi(other[e45], 2)
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                - f32::powi(other[e12345], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - f32::powi(other[e321], 2)
                - f32::powi(other[e4235], 2)
                - f32::powi(other[e4315], 2)
                - f32::powi(other[e4125], 2)
                - 2.0 * (other[e4] * other[e5])
                - 2.0 * (other[e15] * other[e41])
                - 2.0 * (other[e25] * other[e42])
                - 2.0 * (other[e35] * other[e43]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e41], self[e42], self[e43], self[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricQuotient<Plane> for VersorOdd {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd4        0        4        0
    // Totals...
    // yes simd        2        4        1
    //  no simd        2       16        1
    fn geometric_quotient(self, other: Plane) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ -f32::powi(other[e4235], 2) - f32::powi(other[e4315], 2) - f32::powi(other[e4125], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e41], self[e42], self[e43], self[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricQuotient<RoundPoint> for VersorOdd {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        2        1
    //    simd4        0        4        0
    // Totals...
    // yes simd        3        6        1
    //  no simd        3       18        1
    fn geometric_quotient(self, other: RoundPoint) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2) - 2.0 * (other[e4] * other[e5]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e41], self[e42], self[e43], self[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricQuotient<Scalar> for VersorOdd {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd4        0        4        0
    // Totals...
    // yes simd        0        4        1
    //  no simd        0       16        1
    fn geometric_quotient(self, other: Scalar) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e41], self[e42], self[e43], self[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricQuotient<Sphere> for VersorOdd {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        2        1
    //    simd4        0        4        0
    // Totals...
    // yes simd        3        6        1
    //  no simd        3       18        1
    fn geometric_quotient(self, other: Sphere) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e3215] * other[e1234]) - f32::powi(other[e4235], 2) - f32::powi(other[e4315], 2) - f32::powi(other[e4125], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e41], self[e42], self[e43], self[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricQuotient<VersorEven> for VersorOdd {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11        8        1
    //    simd4        0        4        0
    // Totals...
    // yes simd       11       12        1
    //  no simd       11       24        1
    fn geometric_quotient(self, other: VersorEven) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e423] * other[e235])
                + 2.0 * (other[e431] * other[e315])
                + 2.0 * (other[e412] * other[e125])
                + f32::powi(other[e415], 2)
                + f32::powi(other[e425], 2)
                + f32::powi(other[e435], 2)
                + f32::powi(other[e1], 2)
                + f32::powi(other[e2], 2)
                + f32::powi(other[e3], 2)
                - f32::powi(other[e12345], 2)
                - f32::powi(other[e321], 2)
                - 2.0 * (other[e5] * other[e4]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e41], self[e42], self[e43], self[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricQuotient<VersorOdd> for VersorOdd {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11        8        1
    //    simd4        0        4        0
    // Totals...
    // yes simd       11       12        1
    //  no simd       11       24        1
    fn geometric_quotient(self, other: VersorOdd) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (other[e1234] * other[e3215]) + f32::powi(other[scalar], 2) + f32::powi(other[e45], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - f32::powi(other[e4235], 2)
                - f32::powi(other[e4315], 2)
                - f32::powi(other[e4125], 2)
                - 2.0 * (other[e41] * other[e15])
                - 2.0 * (other[e42] * other[e25])
                - 2.0 * (other[e43] * other[e35]),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e41], self[e42], self[e43], self[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
