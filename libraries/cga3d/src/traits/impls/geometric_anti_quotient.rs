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
impl std::ops::Div<geometric_anti_quotient> for AntiCircleRotor {
    type Output = geometric_anti_quotient_partial<AntiCircleRotor>;
    fn div(self, _rhs: geometric_anti_quotient) -> Self::Output {
        geometric_anti_quotient_partial(self)
    }
}
impl GeometricAntiQuotient<AntiCircleRotor> for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        6        1
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        7        9        1
    //  no simd        7       17        1
    fn geometric_anti_quotient(self, other: AntiCircleRotor) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e41] * other[e15])
                + 2.0 * (other[e42] * other[e25])
                + 2.0 * (other[e43] * other[e35])
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                - f32::powi(other[e45], 2)
                - f32::powi(other[scalar], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, scalar
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[scalar]]),
        );
    }
}
impl GeometricAntiQuotient<AntiDipoleInversion> for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10        8        1
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       10       11        1
    //  no simd       10       19        1
    fn geometric_anti_quotient(self, other: AntiDipoleInversion) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e4] * other[e5]) + f32::powi(other[e321], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - f32::powi(other[e1], 2)
                - f32::powi(other[e2], 2)
                - f32::powi(other[e3], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, scalar
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[scalar]]),
        );
    }
}
impl GeometricAntiQuotient<AntiDualNum> for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        4        1
    //  no simd        0       12        1
    fn geometric_anti_quotient(self, other: AntiDualNum) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[scalar], 2) * -1.0);
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, scalar
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[scalar]]),
        );
    }
}
impl GeometricAntiQuotient<AntiFlatPoint> for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        1
    //  no simd        0       11        1
    fn geometric_anti_quotient(self, other: AntiFlatPoint) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e321], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, scalar
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[scalar]]),
        );
    }
}
impl GeometricAntiQuotient<AntiFlector> for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        3        1
    //  no simd        3       11        1
    fn geometric_anti_quotient(self, other: AntiFlector) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e321], 2) - f32::powi(other[e1], 2) - f32::powi(other[e2], 2) - f32::powi(other[e3], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, scalar
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[scalar]]),
        );
    }
}
impl GeometricAntiQuotient<AntiLine> for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        2        3        1
    //  no simd        2       11        1
    fn geometric_anti_quotient(self, other: AntiLine) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e23], 2) + f32::powi(other[e31], 2) + f32::powi(other[e12], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, scalar
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[scalar]]),
        );
    }
}
impl GeometricAntiQuotient<AntiMotor> for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        3        1
    //  no simd        3       11        1
    fn geometric_anti_quotient(self, other: AntiMotor) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e23], 2) + f32::powi(other[e31], 2) + f32::powi(other[e12], 2) - f32::powi(other[scalar], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, scalar
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[scalar]]),
        );
    }
}
impl GeometricAntiQuotient<AntiPlane> for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        2        3        1
    //  no simd        2       11        1
    fn geometric_anti_quotient(self, other: AntiPlane) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ -f32::powi(other[e1], 2) - f32::powi(other[e2], 2) - f32::powi(other[e3], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, scalar
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[scalar]]),
        );
    }
}
impl GeometricAntiQuotient<AntiScalar> for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        1
    //  no simd        0       11        1
    fn geometric_anti_quotient(self, other: AntiScalar) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e12345], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, scalar
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[scalar]]),
        );
    }
}
impl GeometricAntiQuotient<Circle> for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        6        1
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        6        9        1
    //  no simd        6       17        1
    fn geometric_anti_quotient(self, other: Circle) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e321], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, scalar
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[scalar]]),
        );
    }
}
impl GeometricAntiQuotient<CircleRotor> for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        6        1
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        7        9        1
    //  no simd        7       17        1
    fn geometric_anti_quotient(self, other: CircleRotor) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e321], 2) + f32::powi(other[e12345], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, scalar
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[scalar]]),
        );
    }
}
impl GeometricAntiQuotient<Dipole> for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        6        1
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        6        9        1
    //  no simd        6       17        1
    fn geometric_anti_quotient(self, other: Dipole) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e41] * other[e15])
                + 2.0 * (other[e42] * other[e25])
                + 2.0 * (other[e43] * other[e35])
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                - f32::powi(other[e45], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, scalar
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[scalar]]),
        );
    }
}
impl GeometricAntiQuotient<DipoleInversion> for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10        8        1
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       10       11        1
    //  no simd       10       19        1
    fn geometric_anti_quotient(self, other: DipoleInversion) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e41] * other[e15])
                + 2.0 * (other[e42] * other[e25])
                + 2.0 * (other[e43] * other[e35])
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                + f32::powi(other[e4235], 2)
                + f32::powi(other[e4315], 2)
                + f32::powi(other[e4125], 2)
                - f32::powi(other[e45], 2)
                - 2.0 * (other[e1234] * other[e3215]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, scalar
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[scalar]]),
        );
    }
}
impl GeometricAntiQuotient<DualNum> for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        1
    //  no simd        0       11        1
    fn geometric_anti_quotient(self, other: DualNum) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e12345], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, scalar
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[scalar]]),
        );
    }
}
impl GeometricAntiQuotient<FlatPoint> for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        4        1
    //  no simd        0       12        1
    fn geometric_anti_quotient(self, other: FlatPoint) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e45], 2) * -1.0);
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, scalar
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[scalar]]),
        );
    }
}
impl GeometricAntiQuotient<Flector> for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        3        1
    //  no simd        3       11        1
    fn geometric_anti_quotient(self, other: Flector) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e4235], 2) + f32::powi(other[e4315], 2) + f32::powi(other[e4125], 2) - f32::powi(other[e45], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, scalar
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[scalar]]),
        );
    }
}
impl GeometricAntiQuotient<Line> for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        2        3        1
    //  no simd        2       11        1
    fn geometric_anti_quotient(self, other: Line) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ -f32::powi(other[e415], 2) - f32::powi(other[e425], 2) - f32::powi(other[e435], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, scalar
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[scalar]]),
        );
    }
}
impl GeometricAntiQuotient<Motor> for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        3        1
    //  no simd        3       11        1
    fn geometric_anti_quotient(self, other: Motor) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e12345], 2) - f32::powi(other[e415], 2) - f32::powi(other[e425], 2) - f32::powi(other[e435], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, scalar
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[scalar]]),
        );
    }
}
impl GeometricAntiQuotient<MultiVector> for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       23       16        1
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       23       19        1
    //  no simd       23       27        1
    fn geometric_anti_quotient(self, other: MultiVector) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e4] * other[e5])
                + 2.0 * (other[e15] * other[e41])
                + 2.0 * (other[e25] * other[e42])
                + 2.0 * (other[e35] * other[e43])
                + f32::powi(other[e12345], 2)
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                + f32::powi(other[e321], 2)
                + f32::powi(other[e4235], 2)
                + f32::powi(other[e4315], 2)
                + f32::powi(other[e4125], 2)
                - f32::powi(other[scalar], 2)
                - f32::powi(other[e1], 2)
                - f32::powi(other[e2], 2)
                - f32::powi(other[e3], 2)
                - f32::powi(other[e45], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125])
                - 2.0 * (other[e3215] * other[e1234]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, scalar
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[scalar]]),
        );
    }
}
impl GeometricAntiQuotient<Plane> for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        2        3        1
    //  no simd        2       11        1
    fn geometric_anti_quotient(self, other: Plane) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e4235], 2) + f32::powi(other[e4315], 2) + f32::powi(other[e4125], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, scalar
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[scalar]]),
        );
    }
}
impl GeometricAntiQuotient<RoundPoint> for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        2        1
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        5        1
    //  no simd        3       13        1
    fn geometric_anti_quotient(self, other: RoundPoint) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e4] * other[e5]) - f32::powi(other[e1], 2) - f32::powi(other[e2], 2) - f32::powi(other[e3], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, scalar
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[scalar]]),
        );
    }
}
impl GeometricAntiQuotient<Scalar> for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        4        1
    //  no simd        0       12        1
    fn geometric_anti_quotient(self, other: Scalar) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[scalar], 2) * -1.0);
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, scalar
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[scalar]]),
        );
    }
}
impl GeometricAntiQuotient<Sphere> for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        2        1
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        5        1
    //  no simd        3       13        1
    fn geometric_anti_quotient(self, other: Sphere) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e4235], 2) + f32::powi(other[e4315], 2) + f32::powi(other[e4125], 2) - 2.0 * (other[e3215] * other[e1234]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, scalar
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[scalar]]),
        );
    }
}
impl GeometricAntiQuotient<VersorEven> for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11        8        1
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       11       11        1
    //  no simd       11       19        1
    fn geometric_anti_quotient(self, other: VersorEven) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e5] * other[e4]) + f32::powi(other[e12345], 2) + f32::powi(other[e321], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - f32::powi(other[e1], 2)
                - f32::powi(other[e2], 2)
                - f32::powi(other[e3], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, scalar
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[scalar]]),
        );
    }
}
impl GeometricAntiQuotient<VersorOdd> for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11        8        1
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       11       11        1
    //  no simd       11       19        1
    fn geometric_anti_quotient(self, other: VersorOdd) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e41] * other[e15])
                + 2.0 * (other[e42] * other[e25])
                + 2.0 * (other[e43] * other[e35])
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                + f32::powi(other[e4235], 2)
                + f32::powi(other[e4315], 2)
                + f32::powi(other[e4125], 2)
                - f32::powi(other[scalar], 2)
                - f32::powi(other[e45], 2)
                - 2.0 * (other[e1234] * other[e3215]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, scalar
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[scalar]]),
        );
    }
}
impl std::ops::Div<geometric_anti_quotient> for AntiDipoleInversion {
    type Output = geometric_anti_quotient_partial<AntiDipoleInversion>;
    fn div(self, _rhs: geometric_anti_quotient) -> Self::Output {
        geometric_anti_quotient_partial(self)
    }
}
impl GeometricAntiQuotient<AntiCircleRotor> for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        6        1
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        7       10        1
    //  no simd        7       21        1
    fn geometric_anti_quotient(self, other: AntiCircleRotor) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e41] * other[e15])
                + 2.0 * (other[e42] * other[e25])
                + 2.0 * (other[e43] * other[e35])
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                - f32::powi(other[e45], 2)
                - f32::powi(other[scalar], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e4
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e4]]),
            // e1, e2, e3, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricAntiQuotient<AntiDipoleInversion> for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10        8        1
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd       10       12        1
    //  no simd       10       23        1
    fn geometric_anti_quotient(self, other: AntiDipoleInversion) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e4] * other[e5]) + f32::powi(other[e321], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - f32::powi(other[e1], 2)
                - f32::powi(other[e2], 2)
                - f32::powi(other[e3], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e4
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e4]]),
            // e1, e2, e3, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricAntiQuotient<AntiDualNum> for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        5        1
    //  no simd        0       16        1
    fn geometric_anti_quotient(self, other: AntiDualNum) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[scalar], 2) * -1.0);
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e4
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e4]]),
            // e1, e2, e3, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricAntiQuotient<AntiFlatPoint> for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        4        1
    //  no simd        0       15        1
    fn geometric_anti_quotient(self, other: AntiFlatPoint) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e321], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e4
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e4]]),
            // e1, e2, e3, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricAntiQuotient<AntiFlector> for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        3        4        1
    //  no simd        3       15        1
    fn geometric_anti_quotient(self, other: AntiFlector) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e321], 2) - f32::powi(other[e1], 2) - f32::powi(other[e2], 2) - f32::powi(other[e3], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e4
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e4]]),
            // e1, e2, e3, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricAntiQuotient<AntiLine> for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        2        4        1
    //  no simd        2       15        1
    fn geometric_anti_quotient(self, other: AntiLine) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e23], 2) + f32::powi(other[e31], 2) + f32::powi(other[e12], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e4
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e4]]),
            // e1, e2, e3, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricAntiQuotient<AntiMotor> for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        3        4        1
    //  no simd        3       15        1
    fn geometric_anti_quotient(self, other: AntiMotor) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e23], 2) + f32::powi(other[e31], 2) + f32::powi(other[e12], 2) - f32::powi(other[scalar], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e4
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e4]]),
            // e1, e2, e3, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricAntiQuotient<AntiPlane> for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        2        4        1
    //  no simd        2       15        1
    fn geometric_anti_quotient(self, other: AntiPlane) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ -f32::powi(other[e1], 2) - f32::powi(other[e2], 2) - f32::powi(other[e3], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e4
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e4]]),
            // e1, e2, e3, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricAntiQuotient<AntiScalar> for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        4        1
    //  no simd        0       15        1
    fn geometric_anti_quotient(self, other: AntiScalar) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e12345], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e4
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e4]]),
            // e1, e2, e3, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricAntiQuotient<Circle> for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        6        1
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        6       10        1
    //  no simd        6       21        1
    fn geometric_anti_quotient(self, other: Circle) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e321], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e4
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e4]]),
            // e1, e2, e3, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricAntiQuotient<CircleRotor> for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        6        1
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        7       10        1
    //  no simd        7       21        1
    fn geometric_anti_quotient(self, other: CircleRotor) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e321], 2) + f32::powi(other[e12345], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e4
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e4]]),
            // e1, e2, e3, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricAntiQuotient<Dipole> for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        6        1
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        6       10        1
    //  no simd        6       21        1
    fn geometric_anti_quotient(self, other: Dipole) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e41] * other[e15])
                + 2.0 * (other[e42] * other[e25])
                + 2.0 * (other[e43] * other[e35])
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                - f32::powi(other[e45], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e4
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e4]]),
            // e1, e2, e3, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricAntiQuotient<DipoleInversion> for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10        8        1
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd       10       12        1
    //  no simd       10       23        1
    fn geometric_anti_quotient(self, other: DipoleInversion) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e41] * other[e15])
                + 2.0 * (other[e42] * other[e25])
                + 2.0 * (other[e43] * other[e35])
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                + f32::powi(other[e4235], 2)
                + f32::powi(other[e4315], 2)
                + f32::powi(other[e4125], 2)
                - f32::powi(other[e45], 2)
                - 2.0 * (other[e1234] * other[e3215]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e4
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e4]]),
            // e1, e2, e3, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricAntiQuotient<DualNum> for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        4        1
    //  no simd        0       15        1
    fn geometric_anti_quotient(self, other: DualNum) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e12345], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e4
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e4]]),
            // e1, e2, e3, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricAntiQuotient<FlatPoint> for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        5        1
    //  no simd        0       16        1
    fn geometric_anti_quotient(self, other: FlatPoint) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e45], 2) * -1.0);
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e4
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e4]]),
            // e1, e2, e3, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricAntiQuotient<Flector> for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        3        4        1
    //  no simd        3       15        1
    fn geometric_anti_quotient(self, other: Flector) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e4235], 2) + f32::powi(other[e4315], 2) + f32::powi(other[e4125], 2) - f32::powi(other[e45], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e4
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e4]]),
            // e1, e2, e3, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricAntiQuotient<Line> for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        2        4        1
    //  no simd        2       15        1
    fn geometric_anti_quotient(self, other: Line) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ -f32::powi(other[e415], 2) - f32::powi(other[e425], 2) - f32::powi(other[e435], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e4
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e4]]),
            // e1, e2, e3, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricAntiQuotient<Motor> for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        3        4        1
    //  no simd        3       15        1
    fn geometric_anti_quotient(self, other: Motor) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e12345], 2) - f32::powi(other[e415], 2) - f32::powi(other[e425], 2) - f32::powi(other[e435], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e4
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e4]]),
            // e1, e2, e3, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricAntiQuotient<MultiVector> for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       23       16        1
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd       23       20        1
    //  no simd       23       31        1
    fn geometric_anti_quotient(self, other: MultiVector) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e4] * other[e5])
                + 2.0 * (other[e15] * other[e41])
                + 2.0 * (other[e25] * other[e42])
                + 2.0 * (other[e35] * other[e43])
                + f32::powi(other[e12345], 2)
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                + f32::powi(other[e321], 2)
                + f32::powi(other[e4235], 2)
                + f32::powi(other[e4315], 2)
                + f32::powi(other[e4125], 2)
                - f32::powi(other[scalar], 2)
                - f32::powi(other[e1], 2)
                - f32::powi(other[e2], 2)
                - f32::powi(other[e3], 2)
                - f32::powi(other[e45], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125])
                - 2.0 * (other[e3215] * other[e1234]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e4
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e4]]),
            // e1, e2, e3, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricAntiQuotient<Plane> for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        2        4        1
    //  no simd        2       15        1
    fn geometric_anti_quotient(self, other: Plane) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e4235], 2) + f32::powi(other[e4315], 2) + f32::powi(other[e4125], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e4
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e4]]),
            // e1, e2, e3, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricAntiQuotient<RoundPoint> for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        2        1
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        3        6        1
    //  no simd        3       17        1
    fn geometric_anti_quotient(self, other: RoundPoint) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e4] * other[e5]) - f32::powi(other[e1], 2) - f32::powi(other[e2], 2) - f32::powi(other[e3], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e4
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e4]]),
            // e1, e2, e3, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricAntiQuotient<Scalar> for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        5        1
    //  no simd        0       16        1
    fn geometric_anti_quotient(self, other: Scalar) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[scalar], 2) * -1.0);
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e4
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e4]]),
            // e1, e2, e3, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricAntiQuotient<Sphere> for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        2        1
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        3        6        1
    //  no simd        3       17        1
    fn geometric_anti_quotient(self, other: Sphere) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e4235], 2) + f32::powi(other[e4315], 2) + f32::powi(other[e4125], 2) - 2.0 * (other[e3215] * other[e1234]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e4
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e4]]),
            // e1, e2, e3, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricAntiQuotient<VersorEven> for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11        8        1
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd       11       12        1
    //  no simd       11       23        1
    fn geometric_anti_quotient(self, other: VersorEven) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e5] * other[e4]) + f32::powi(other[e12345], 2) + f32::powi(other[e321], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - f32::powi(other[e1], 2)
                - f32::powi(other[e2], 2)
                - f32::powi(other[e3], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e4
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e4]]),
            // e1, e2, e3, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricAntiQuotient<VersorOdd> for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11        8        1
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd       11       12        1
    //  no simd       11       23        1
    fn geometric_anti_quotient(self, other: VersorOdd) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e41] * other[e15])
                + 2.0 * (other[e42] * other[e25])
                + 2.0 * (other[e43] * other[e35])
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                + f32::powi(other[e4235], 2)
                + f32::powi(other[e4315], 2)
                + f32::powi(other[e4125], 2)
                - f32::powi(other[scalar], 2)
                - f32::powi(other[e45], 2)
                - 2.0 * (other[e1234] * other[e3215]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e4
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e4]]),
            // e1, e2, e3, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl std::ops::Div<geometric_anti_quotient> for AntiDualNum {
    type Output = geometric_anti_quotient_partial<AntiDualNum>;
    fn div(self, _rhs: geometric_anti_quotient) -> Self::Output {
        geometric_anti_quotient_partial(self)
    }
}
impl GeometricAntiQuotient<AntiCircleRotor> for AntiDualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        6        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        7        7        1
    //  no simd        7        8        1
    fn geometric_anti_quotient(self, other: AntiCircleRotor) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e41] * other[e15])
                + 2.0 * (other[e42] * other[e25])
                + 2.0 * (other[e43] * other[e35])
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                - f32::powi(other[e45], 2)
                - f32::powi(other[scalar], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiDualNum::from_groups(/* e1234, scalar */ Simd32x2::from(anti_inverse[e12345]) * Simd32x2::from([self[e1234], self[scalar]]));
    }
}
impl GeometricAntiQuotient<AntiDipoleInversion> for AntiDualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10        8        1
    //    simd2        0        1        0
    // Totals...
    // yes simd       10        9        1
    //  no simd       10       10        1
    fn geometric_anti_quotient(self, other: AntiDipoleInversion) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e4] * other[e5]) + f32::powi(other[e321], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - f32::powi(other[e1], 2)
                - f32::powi(other[e2], 2)
                - f32::powi(other[e3], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiDualNum::from_groups(/* e1234, scalar */ Simd32x2::from(anti_inverse[e12345]) * Simd32x2::from([self[e1234], self[scalar]]));
    }
}
impl GeometricAntiQuotient<AntiDualNum> for AntiDualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        3        1
    fn geometric_anti_quotient(self, other: AntiDualNum) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[scalar], 2) * -1.0);
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiDualNum::from_groups(/* e1234, scalar */ Simd32x2::from(anti_inverse[e12345]) * Simd32x2::from([self[e1234], self[scalar]]));
    }
}
impl GeometricAntiQuotient<AntiFlatPoint> for AntiDualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        1        1
    //  no simd        0        2        1
    fn geometric_anti_quotient(self, other: AntiFlatPoint) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e321], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiDualNum::from_groups(/* e1234, scalar */ Simd32x2::from(anti_inverse[e12345]) * Simd32x2::from([self[e1234], self[scalar]]));
    }
}
impl GeometricAntiQuotient<AntiFlector> for AntiDualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        3        1        1
    //  no simd        3        2        1
    fn geometric_anti_quotient(self, other: AntiFlector) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e321], 2) - f32::powi(other[e1], 2) - f32::powi(other[e2], 2) - f32::powi(other[e3], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiDualNum::from_groups(/* e1234, scalar */ Simd32x2::from(anti_inverse[e12345]) * Simd32x2::from([self[e1234], self[scalar]]));
    }
}
impl GeometricAntiQuotient<AntiLine> for AntiDualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        2        1        1
    //  no simd        2        2        1
    fn geometric_anti_quotient(self, other: AntiLine) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e23], 2) + f32::powi(other[e31], 2) + f32::powi(other[e12], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiDualNum::from_groups(/* e1234, scalar */ Simd32x2::from(anti_inverse[e12345]) * Simd32x2::from([self[e1234], self[scalar]]));
    }
}
impl GeometricAntiQuotient<AntiMotor> for AntiDualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        3        1        1
    //  no simd        3        2        1
    fn geometric_anti_quotient(self, other: AntiMotor) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e23], 2) + f32::powi(other[e31], 2) + f32::powi(other[e12], 2) - f32::powi(other[scalar], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiDualNum::from_groups(/* e1234, scalar */ Simd32x2::from(anti_inverse[e12345]) * Simd32x2::from([self[e1234], self[scalar]]));
    }
}
impl GeometricAntiQuotient<AntiPlane> for AntiDualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        2        1        1
    //  no simd        2        2        1
    fn geometric_anti_quotient(self, other: AntiPlane) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ -f32::powi(other[e1], 2) - f32::powi(other[e2], 2) - f32::powi(other[e3], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiDualNum::from_groups(/* e1234, scalar */ Simd32x2::from(anti_inverse[e12345]) * Simd32x2::from([self[e1234], self[scalar]]));
    }
}
impl GeometricAntiQuotient<AntiScalar> for AntiDualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        1        1
    //  no simd        0        2        1
    fn geometric_anti_quotient(self, other: AntiScalar) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e12345], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiDualNum::from_groups(/* e1234, scalar */ Simd32x2::from(anti_inverse[e12345]) * Simd32x2::from([self[e1234], self[scalar]]));
    }
}
impl GeometricAntiQuotient<Circle> for AntiDualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        6        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        6        7        1
    //  no simd        6        8        1
    fn geometric_anti_quotient(self, other: Circle) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e321], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiDualNum::from_groups(/* e1234, scalar */ Simd32x2::from(anti_inverse[e12345]) * Simd32x2::from([self[e1234], self[scalar]]));
    }
}
impl GeometricAntiQuotient<CircleRotor> for AntiDualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        6        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        7        7        1
    //  no simd        7        8        1
    fn geometric_anti_quotient(self, other: CircleRotor) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e321], 2) + f32::powi(other[e12345], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiDualNum::from_groups(/* e1234, scalar */ Simd32x2::from(anti_inverse[e12345]) * Simd32x2::from([self[e1234], self[scalar]]));
    }
}
impl GeometricAntiQuotient<Dipole> for AntiDualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        6        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        6        7        1
    //  no simd        6        8        1
    fn geometric_anti_quotient(self, other: Dipole) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e41] * other[e15])
                + 2.0 * (other[e42] * other[e25])
                + 2.0 * (other[e43] * other[e35])
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                - f32::powi(other[e45], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiDualNum::from_groups(/* e1234, scalar */ Simd32x2::from(anti_inverse[e12345]) * Simd32x2::from([self[e1234], self[scalar]]));
    }
}
impl GeometricAntiQuotient<DipoleInversion> for AntiDualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10        8        1
    //    simd2        0        1        0
    // Totals...
    // yes simd       10        9        1
    //  no simd       10       10        1
    fn geometric_anti_quotient(self, other: DipoleInversion) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e41] * other[e15])
                + 2.0 * (other[e42] * other[e25])
                + 2.0 * (other[e43] * other[e35])
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                + f32::powi(other[e4235], 2)
                + f32::powi(other[e4315], 2)
                + f32::powi(other[e4125], 2)
                - f32::powi(other[e45], 2)
                - 2.0 * (other[e1234] * other[e3215]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiDualNum::from_groups(/* e1234, scalar */ Simd32x2::from(anti_inverse[e12345]) * Simd32x2::from([self[e1234], self[scalar]]));
    }
}
impl GeometricAntiQuotient<DualNum> for AntiDualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        1        1
    //  no simd        0        2        1
    fn geometric_anti_quotient(self, other: DualNum) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e12345], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiDualNum::from_groups(/* e1234, scalar */ Simd32x2::from(anti_inverse[e12345]) * Simd32x2::from([self[e1234], self[scalar]]));
    }
}
impl GeometricAntiQuotient<FlatPoint> for AntiDualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        3        1
    fn geometric_anti_quotient(self, other: FlatPoint) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e45], 2) * -1.0);
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiDualNum::from_groups(/* e1234, scalar */ Simd32x2::from(anti_inverse[e12345]) * Simd32x2::from([self[e1234], self[scalar]]));
    }
}
impl GeometricAntiQuotient<Flector> for AntiDualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        3        1        1
    //  no simd        3        2        1
    fn geometric_anti_quotient(self, other: Flector) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e4235], 2) + f32::powi(other[e4315], 2) + f32::powi(other[e4125], 2) - f32::powi(other[e45], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiDualNum::from_groups(/* e1234, scalar */ Simd32x2::from(anti_inverse[e12345]) * Simd32x2::from([self[e1234], self[scalar]]));
    }
}
impl GeometricAntiQuotient<Line> for AntiDualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        2        1        1
    //  no simd        2        2        1
    fn geometric_anti_quotient(self, other: Line) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ -f32::powi(other[e415], 2) - f32::powi(other[e425], 2) - f32::powi(other[e435], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiDualNum::from_groups(/* e1234, scalar */ Simd32x2::from(anti_inverse[e12345]) * Simd32x2::from([self[e1234], self[scalar]]));
    }
}
impl GeometricAntiQuotient<Motor> for AntiDualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        3        1        1
    //  no simd        3        2        1
    fn geometric_anti_quotient(self, other: Motor) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e12345], 2) - f32::powi(other[e415], 2) - f32::powi(other[e425], 2) - f32::powi(other[e435], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiDualNum::from_groups(/* e1234, scalar */ Simd32x2::from(anti_inverse[e12345]) * Simd32x2::from([self[e1234], self[scalar]]));
    }
}
impl GeometricAntiQuotient<MultiVector> for AntiDualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       23       16        1
    //    simd2        0        1        0
    // Totals...
    // yes simd       23       17        1
    //  no simd       23       18        1
    fn geometric_anti_quotient(self, other: MultiVector) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e4] * other[e5])
                + 2.0 * (other[e15] * other[e41])
                + 2.0 * (other[e25] * other[e42])
                + 2.0 * (other[e35] * other[e43])
                + f32::powi(other[e12345], 2)
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                + f32::powi(other[e321], 2)
                + f32::powi(other[e4235], 2)
                + f32::powi(other[e4315], 2)
                + f32::powi(other[e4125], 2)
                - f32::powi(other[scalar], 2)
                - f32::powi(other[e1], 2)
                - f32::powi(other[e2], 2)
                - f32::powi(other[e3], 2)
                - f32::powi(other[e45], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125])
                - 2.0 * (other[e3215] * other[e1234]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiDualNum::from_groups(/* e1234, scalar */ Simd32x2::from(anti_inverse[e12345]) * Simd32x2::from([self[e1234], self[scalar]]));
    }
}
impl GeometricAntiQuotient<Plane> for AntiDualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        2        1        1
    //  no simd        2        2        1
    fn geometric_anti_quotient(self, other: Plane) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e4235], 2) + f32::powi(other[e4315], 2) + f32::powi(other[e4125], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiDualNum::from_groups(/* e1234, scalar */ Simd32x2::from(anti_inverse[e12345]) * Simd32x2::from([self[e1234], self[scalar]]));
    }
}
impl GeometricAntiQuotient<RoundPoint> for AntiDualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        2        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        3        3        1
    //  no simd        3        4        1
    fn geometric_anti_quotient(self, other: RoundPoint) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e4] * other[e5]) - f32::powi(other[e1], 2) - f32::powi(other[e2], 2) - f32::powi(other[e3], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiDualNum::from_groups(/* e1234, scalar */ Simd32x2::from(anti_inverse[e12345]) * Simd32x2::from([self[e1234], self[scalar]]));
    }
}
impl GeometricAntiQuotient<Scalar> for AntiDualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        3        1
    fn geometric_anti_quotient(self, other: Scalar) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[scalar], 2) * -1.0);
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiDualNum::from_groups(/* e1234, scalar */ Simd32x2::from(anti_inverse[e12345]) * Simd32x2::from([self[e1234], self[scalar]]));
    }
}
impl GeometricAntiQuotient<Sphere> for AntiDualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        2        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        3        3        1
    //  no simd        3        4        1
    fn geometric_anti_quotient(self, other: Sphere) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e4235], 2) + f32::powi(other[e4315], 2) + f32::powi(other[e4125], 2) - 2.0 * (other[e3215] * other[e1234]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiDualNum::from_groups(/* e1234, scalar */ Simd32x2::from(anti_inverse[e12345]) * Simd32x2::from([self[e1234], self[scalar]]));
    }
}
impl GeometricAntiQuotient<VersorEven> for AntiDualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11        8        1
    //    simd2        0        1        0
    // Totals...
    // yes simd       11        9        1
    //  no simd       11       10        1
    fn geometric_anti_quotient(self, other: VersorEven) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e5] * other[e4]) + f32::powi(other[e12345], 2) + f32::powi(other[e321], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - f32::powi(other[e1], 2)
                - f32::powi(other[e2], 2)
                - f32::powi(other[e3], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiDualNum::from_groups(/* e1234, scalar */ Simd32x2::from(anti_inverse[e12345]) * Simd32x2::from([self[e1234], self[scalar]]));
    }
}
impl GeometricAntiQuotient<VersorOdd> for AntiDualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11        8        1
    //    simd2        0        1        0
    // Totals...
    // yes simd       11        9        1
    //  no simd       11       10        1
    fn geometric_anti_quotient(self, other: VersorOdd) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e41] * other[e15])
                + 2.0 * (other[e42] * other[e25])
                + 2.0 * (other[e43] * other[e35])
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                + f32::powi(other[e4235], 2)
                + f32::powi(other[e4315], 2)
                + f32::powi(other[e4125], 2)
                - f32::powi(other[scalar], 2)
                - f32::powi(other[e45], 2)
                - 2.0 * (other[e1234] * other[e3215]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiDualNum::from_groups(/* e1234, scalar */ Simd32x2::from(anti_inverse[e12345]) * Simd32x2::from([self[e1234], self[scalar]]));
    }
}
impl std::ops::Div<geometric_anti_quotient> for AntiFlatPoint {
    type Output = geometric_anti_quotient_partial<AntiFlatPoint>;
    fn div(self, _rhs: geometric_anti_quotient) -> Self::Output {
        geometric_anti_quotient_partial(self)
    }
}
impl GeometricAntiQuotient<AntiCircleRotor> for AntiFlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        6        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        7        7        1
    //  no simd        7       10        1
    fn geometric_anti_quotient(self, other: AntiCircleRotor) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e41] * other[e15])
                + 2.0 * (other[e42] * other[e25])
                + 2.0 * (other[e43] * other[e35])
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                - f32::powi(other[e45], 2)
                - f32::powi(other[scalar], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiFlatPoint::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e321]]),
        );
    }
}
impl GeometricAntiQuotient<AntiDipoleInversion> for AntiFlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10        8        1
    //    simd4        0        1        0
    // Totals...
    // yes simd       10        9        1
    //  no simd       10       12        1
    fn geometric_anti_quotient(self, other: AntiDipoleInversion) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e4] * other[e5]) + f32::powi(other[e321], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - f32::powi(other[e1], 2)
                - f32::powi(other[e2], 2)
                - f32::powi(other[e3], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiFlatPoint::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e321]]),
        );
    }
}
impl GeometricAntiQuotient<AntiDualNum> for AntiFlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        5        1
    fn geometric_anti_quotient(self, other: AntiDualNum) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[scalar], 2) * -1.0);
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiFlatPoint::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e321]]),
        );
    }
}
impl GeometricAntiQuotient<AntiFlatPoint> for AntiFlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        1        1
    //  no simd        0        4        1
    fn geometric_anti_quotient(self, other: AntiFlatPoint) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e321], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiFlatPoint::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e321]]),
        );
    }
}
impl GeometricAntiQuotient<AntiFlector> for AntiFlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        1        1
    //  no simd        3        4        1
    fn geometric_anti_quotient(self, other: AntiFlector) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e321], 2) - f32::powi(other[e1], 2) - f32::powi(other[e2], 2) - f32::powi(other[e3], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiFlatPoint::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e321]]),
        );
    }
}
impl GeometricAntiQuotient<AntiLine> for AntiFlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        1        1
    //  no simd        2        4        1
    fn geometric_anti_quotient(self, other: AntiLine) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e23], 2) + f32::powi(other[e31], 2) + f32::powi(other[e12], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiFlatPoint::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e321]]),
        );
    }
}
impl GeometricAntiQuotient<AntiMotor> for AntiFlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        1        1
    //  no simd        3        4        1
    fn geometric_anti_quotient(self, other: AntiMotor) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e23], 2) + f32::powi(other[e31], 2) + f32::powi(other[e12], 2) - f32::powi(other[scalar], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiFlatPoint::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e321]]),
        );
    }
}
impl GeometricAntiQuotient<AntiPlane> for AntiFlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        1        1
    //  no simd        2        4        1
    fn geometric_anti_quotient(self, other: AntiPlane) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ -f32::powi(other[e1], 2) - f32::powi(other[e2], 2) - f32::powi(other[e3], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiFlatPoint::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e321]]),
        );
    }
}
impl GeometricAntiQuotient<AntiScalar> for AntiFlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        1        1
    //  no simd        0        4        1
    fn geometric_anti_quotient(self, other: AntiScalar) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e12345], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiFlatPoint::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e321]]),
        );
    }
}
impl GeometricAntiQuotient<Circle> for AntiFlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        6        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        6        7        1
    //  no simd        6       10        1
    fn geometric_anti_quotient(self, other: Circle) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e321], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiFlatPoint::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e321]]),
        );
    }
}
impl GeometricAntiQuotient<CircleRotor> for AntiFlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        6        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        7        7        1
    //  no simd        7       10        1
    fn geometric_anti_quotient(self, other: CircleRotor) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e321], 2) + f32::powi(other[e12345], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiFlatPoint::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e321]]),
        );
    }
}
impl GeometricAntiQuotient<Dipole> for AntiFlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        6        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        6        7        1
    //  no simd        6       10        1
    fn geometric_anti_quotient(self, other: Dipole) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e41] * other[e15])
                + 2.0 * (other[e42] * other[e25])
                + 2.0 * (other[e43] * other[e35])
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                - f32::powi(other[e45], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiFlatPoint::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e321]]),
        );
    }
}
impl GeometricAntiQuotient<DipoleInversion> for AntiFlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10        8        1
    //    simd4        0        1        0
    // Totals...
    // yes simd       10        9        1
    //  no simd       10       12        1
    fn geometric_anti_quotient(self, other: DipoleInversion) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e41] * other[e15])
                + 2.0 * (other[e42] * other[e25])
                + 2.0 * (other[e43] * other[e35])
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                + f32::powi(other[e4235], 2)
                + f32::powi(other[e4315], 2)
                + f32::powi(other[e4125], 2)
                - f32::powi(other[e45], 2)
                - 2.0 * (other[e1234] * other[e3215]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiFlatPoint::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e321]]),
        );
    }
}
impl GeometricAntiQuotient<DualNum> for AntiFlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        1        1
    //  no simd        0        4        1
    fn geometric_anti_quotient(self, other: DualNum) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e12345], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiFlatPoint::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e321]]),
        );
    }
}
impl GeometricAntiQuotient<FlatPoint> for AntiFlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        5        1
    fn geometric_anti_quotient(self, other: FlatPoint) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e45], 2) * -1.0);
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiFlatPoint::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e321]]),
        );
    }
}
impl GeometricAntiQuotient<Flector> for AntiFlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        1        1
    //  no simd        3        4        1
    fn geometric_anti_quotient(self, other: Flector) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e4235], 2) + f32::powi(other[e4315], 2) + f32::powi(other[e4125], 2) - f32::powi(other[e45], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiFlatPoint::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e321]]),
        );
    }
}
impl GeometricAntiQuotient<Line> for AntiFlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        1        1
    //  no simd        2        4        1
    fn geometric_anti_quotient(self, other: Line) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ -f32::powi(other[e415], 2) - f32::powi(other[e425], 2) - f32::powi(other[e435], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiFlatPoint::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e321]]),
        );
    }
}
impl GeometricAntiQuotient<Motor> for AntiFlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        1        1
    //  no simd        3        4        1
    fn geometric_anti_quotient(self, other: Motor) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e12345], 2) - f32::powi(other[e415], 2) - f32::powi(other[e425], 2) - f32::powi(other[e435], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiFlatPoint::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e321]]),
        );
    }
}
impl GeometricAntiQuotient<MultiVector> for AntiFlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       23       16        1
    //    simd4        0        1        0
    // Totals...
    // yes simd       23       17        1
    //  no simd       23       20        1
    fn geometric_anti_quotient(self, other: MultiVector) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e4] * other[e5])
                + 2.0 * (other[e15] * other[e41])
                + 2.0 * (other[e25] * other[e42])
                + 2.0 * (other[e35] * other[e43])
                + f32::powi(other[e12345], 2)
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                + f32::powi(other[e321], 2)
                + f32::powi(other[e4235], 2)
                + f32::powi(other[e4315], 2)
                + f32::powi(other[e4125], 2)
                - f32::powi(other[scalar], 2)
                - f32::powi(other[e1], 2)
                - f32::powi(other[e2], 2)
                - f32::powi(other[e3], 2)
                - f32::powi(other[e45], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125])
                - 2.0 * (other[e3215] * other[e1234]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiFlatPoint::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e321]]),
        );
    }
}
impl GeometricAntiQuotient<Plane> for AntiFlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        1        1
    //  no simd        2        4        1
    fn geometric_anti_quotient(self, other: Plane) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e4235], 2) + f32::powi(other[e4315], 2) + f32::powi(other[e4125], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiFlatPoint::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e321]]),
        );
    }
}
impl GeometricAntiQuotient<RoundPoint> for AntiFlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        2        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        3        1
    //  no simd        3        6        1
    fn geometric_anti_quotient(self, other: RoundPoint) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e4] * other[e5]) - f32::powi(other[e1], 2) - f32::powi(other[e2], 2) - f32::powi(other[e3], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiFlatPoint::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e321]]),
        );
    }
}
impl GeometricAntiQuotient<Scalar> for AntiFlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        5        1
    fn geometric_anti_quotient(self, other: Scalar) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[scalar], 2) * -1.0);
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiFlatPoint::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e321]]),
        );
    }
}
impl GeometricAntiQuotient<Sphere> for AntiFlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        2        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        3        1
    //  no simd        3        6        1
    fn geometric_anti_quotient(self, other: Sphere) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e4235], 2) + f32::powi(other[e4315], 2) + f32::powi(other[e4125], 2) - 2.0 * (other[e3215] * other[e1234]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiFlatPoint::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e321]]),
        );
    }
}
impl GeometricAntiQuotient<VersorEven> for AntiFlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11        8        1
    //    simd4        0        1        0
    // Totals...
    // yes simd       11        9        1
    //  no simd       11       12        1
    fn geometric_anti_quotient(self, other: VersorEven) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e5] * other[e4]) + f32::powi(other[e12345], 2) + f32::powi(other[e321], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - f32::powi(other[e1], 2)
                - f32::powi(other[e2], 2)
                - f32::powi(other[e3], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiFlatPoint::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e321]]),
        );
    }
}
impl GeometricAntiQuotient<VersorOdd> for AntiFlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11        8        1
    //    simd4        0        1        0
    // Totals...
    // yes simd       11        9        1
    //  no simd       11       12        1
    fn geometric_anti_quotient(self, other: VersorOdd) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e41] * other[e15])
                + 2.0 * (other[e42] * other[e25])
                + 2.0 * (other[e43] * other[e35])
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                + f32::powi(other[e4235], 2)
                + f32::powi(other[e4315], 2)
                + f32::powi(other[e4125], 2)
                - f32::powi(other[scalar], 2)
                - f32::powi(other[e45], 2)
                - 2.0 * (other[e1234] * other[e3215]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiFlatPoint::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e321]]),
        );
    }
}
impl std::ops::Div<geometric_anti_quotient> for AntiFlector {
    type Output = geometric_anti_quotient_partial<AntiFlector>;
    fn div(self, _rhs: geometric_anti_quotient) -> Self::Output {
        geometric_anti_quotient_partial(self)
    }
}
impl GeometricAntiQuotient<AntiCircleRotor> for AntiFlector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        6        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        7        8        1
    //  no simd        7       14        1
    fn geometric_anti_quotient(self, other: AntiCircleRotor) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e41] * other[e15])
                + 2.0 * (other[e42] * other[e25])
                + 2.0 * (other[e43] * other[e35])
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                - f32::powi(other[e45], 2)
                - f32::powi(other[scalar], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e321]]),
            // e1, e2, e3, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricAntiQuotient<AntiDipoleInversion> for AntiFlector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10        8        1
    //    simd4        0        2        0
    // Totals...
    // yes simd       10       10        1
    //  no simd       10       16        1
    fn geometric_anti_quotient(self, other: AntiDipoleInversion) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e4] * other[e5]) + f32::powi(other[e321], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - f32::powi(other[e1], 2)
                - f32::powi(other[e2], 2)
                - f32::powi(other[e3], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e321]]),
            // e1, e2, e3, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricAntiQuotient<AntiDualNum> for AntiFlector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        1
    //  no simd        0        9        1
    fn geometric_anti_quotient(self, other: AntiDualNum) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[scalar], 2) * -1.0);
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e321]]),
            // e1, e2, e3, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricAntiQuotient<AntiFlatPoint> for AntiFlector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        8        1
    fn geometric_anti_quotient(self, other: AntiFlatPoint) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e321], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e321]]),
            // e1, e2, e3, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricAntiQuotient<AntiFlector> for AntiFlector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        2        1
    //  no simd        3        8        1
    fn geometric_anti_quotient(self, other: AntiFlector) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e321], 2) - f32::powi(other[e1], 2) - f32::powi(other[e2], 2) - f32::powi(other[e3], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e321]]),
            // e1, e2, e3, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricAntiQuotient<AntiLine> for AntiFlector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        2        2        1
    //  no simd        2        8        1
    fn geometric_anti_quotient(self, other: AntiLine) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e23], 2) + f32::powi(other[e31], 2) + f32::powi(other[e12], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e321]]),
            // e1, e2, e3, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricAntiQuotient<AntiMotor> for AntiFlector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        2        1
    //  no simd        3        8        1
    fn geometric_anti_quotient(self, other: AntiMotor) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e23], 2) + f32::powi(other[e31], 2) + f32::powi(other[e12], 2) - f32::powi(other[scalar], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e321]]),
            // e1, e2, e3, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricAntiQuotient<AntiPlane> for AntiFlector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        2        2        1
    //  no simd        2        8        1
    fn geometric_anti_quotient(self, other: AntiPlane) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ -f32::powi(other[e1], 2) - f32::powi(other[e2], 2) - f32::powi(other[e3], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e321]]),
            // e1, e2, e3, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricAntiQuotient<AntiScalar> for AntiFlector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        8        1
    fn geometric_anti_quotient(self, other: AntiScalar) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e12345], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e321]]),
            // e1, e2, e3, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricAntiQuotient<Circle> for AntiFlector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        6        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        6        8        1
    //  no simd        6       14        1
    fn geometric_anti_quotient(self, other: Circle) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e321], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e321]]),
            // e1, e2, e3, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricAntiQuotient<CircleRotor> for AntiFlector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        6        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        7        8        1
    //  no simd        7       14        1
    fn geometric_anti_quotient(self, other: CircleRotor) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e321], 2) + f32::powi(other[e12345], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e321]]),
            // e1, e2, e3, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricAntiQuotient<Dipole> for AntiFlector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        6        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        6        8        1
    //  no simd        6       14        1
    fn geometric_anti_quotient(self, other: Dipole) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e41] * other[e15])
                + 2.0 * (other[e42] * other[e25])
                + 2.0 * (other[e43] * other[e35])
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                - f32::powi(other[e45], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e321]]),
            // e1, e2, e3, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricAntiQuotient<DipoleInversion> for AntiFlector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10        8        1
    //    simd4        0        2        0
    // Totals...
    // yes simd       10       10        1
    //  no simd       10       16        1
    fn geometric_anti_quotient(self, other: DipoleInversion) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e41] * other[e15])
                + 2.0 * (other[e42] * other[e25])
                + 2.0 * (other[e43] * other[e35])
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                + f32::powi(other[e4235], 2)
                + f32::powi(other[e4315], 2)
                + f32::powi(other[e4125], 2)
                - f32::powi(other[e45], 2)
                - 2.0 * (other[e1234] * other[e3215]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e321]]),
            // e1, e2, e3, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricAntiQuotient<DualNum> for AntiFlector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        8        1
    fn geometric_anti_quotient(self, other: DualNum) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e12345], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e321]]),
            // e1, e2, e3, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricAntiQuotient<FlatPoint> for AntiFlector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        1
    //  no simd        0        9        1
    fn geometric_anti_quotient(self, other: FlatPoint) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e45], 2) * -1.0);
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e321]]),
            // e1, e2, e3, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricAntiQuotient<Flector> for AntiFlector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        2        1
    //  no simd        3        8        1
    fn geometric_anti_quotient(self, other: Flector) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e4235], 2) + f32::powi(other[e4315], 2) + f32::powi(other[e4125], 2) - f32::powi(other[e45], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e321]]),
            // e1, e2, e3, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricAntiQuotient<Line> for AntiFlector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        2        2        1
    //  no simd        2        8        1
    fn geometric_anti_quotient(self, other: Line) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ -f32::powi(other[e415], 2) - f32::powi(other[e425], 2) - f32::powi(other[e435], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e321]]),
            // e1, e2, e3, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricAntiQuotient<Motor> for AntiFlector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        2        1
    //  no simd        3        8        1
    fn geometric_anti_quotient(self, other: Motor) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e12345], 2) - f32::powi(other[e415], 2) - f32::powi(other[e425], 2) - f32::powi(other[e435], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e321]]),
            // e1, e2, e3, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricAntiQuotient<MultiVector> for AntiFlector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       23       16        1
    //    simd4        0        2        0
    // Totals...
    // yes simd       23       18        1
    //  no simd       23       24        1
    fn geometric_anti_quotient(self, other: MultiVector) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e4] * other[e5])
                + 2.0 * (other[e15] * other[e41])
                + 2.0 * (other[e25] * other[e42])
                + 2.0 * (other[e35] * other[e43])
                + f32::powi(other[e12345], 2)
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                + f32::powi(other[e321], 2)
                + f32::powi(other[e4235], 2)
                + f32::powi(other[e4315], 2)
                + f32::powi(other[e4125], 2)
                - f32::powi(other[scalar], 2)
                - f32::powi(other[e1], 2)
                - f32::powi(other[e2], 2)
                - f32::powi(other[e3], 2)
                - f32::powi(other[e45], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125])
                - 2.0 * (other[e3215] * other[e1234]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e321]]),
            // e1, e2, e3, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricAntiQuotient<Plane> for AntiFlector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        2        2        1
    //  no simd        2        8        1
    fn geometric_anti_quotient(self, other: Plane) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e4235], 2) + f32::powi(other[e4315], 2) + f32::powi(other[e4125], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e321]]),
            // e1, e2, e3, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricAntiQuotient<RoundPoint> for AntiFlector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        2        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        4        1
    //  no simd        3       10        1
    fn geometric_anti_quotient(self, other: RoundPoint) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e4] * other[e5]) - f32::powi(other[e1], 2) - f32::powi(other[e2], 2) - f32::powi(other[e3], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e321]]),
            // e1, e2, e3, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricAntiQuotient<Scalar> for AntiFlector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        1
    //  no simd        0        9        1
    fn geometric_anti_quotient(self, other: Scalar) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[scalar], 2) * -1.0);
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e321]]),
            // e1, e2, e3, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricAntiQuotient<Sphere> for AntiFlector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        2        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        4        1
    //  no simd        3       10        1
    fn geometric_anti_quotient(self, other: Sphere) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e4235], 2) + f32::powi(other[e4315], 2) + f32::powi(other[e4125], 2) - 2.0 * (other[e3215] * other[e1234]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e321]]),
            // e1, e2, e3, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricAntiQuotient<VersorEven> for AntiFlector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11        8        1
    //    simd4        0        2        0
    // Totals...
    // yes simd       11       10        1
    //  no simd       11       16        1
    fn geometric_anti_quotient(self, other: VersorEven) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e5] * other[e4]) + f32::powi(other[e12345], 2) + f32::powi(other[e321], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - f32::powi(other[e1], 2)
                - f32::powi(other[e2], 2)
                - f32::powi(other[e3], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e321]]),
            // e1, e2, e3, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricAntiQuotient<VersorOdd> for AntiFlector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11        8        1
    //    simd4        0        2        0
    // Totals...
    // yes simd       11       10        1
    //  no simd       11       16        1
    fn geometric_anti_quotient(self, other: VersorOdd) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e41] * other[e15])
                + 2.0 * (other[e42] * other[e25])
                + 2.0 * (other[e43] * other[e35])
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                + f32::powi(other[e4235], 2)
                + f32::powi(other[e4315], 2)
                + f32::powi(other[e4125], 2)
                - f32::powi(other[scalar], 2)
                - f32::powi(other[e45], 2)
                - 2.0 * (other[e1234] * other[e3215]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e321]]),
            // e1, e2, e3, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl std::ops::Div<geometric_anti_quotient> for AntiLine {
    type Output = geometric_anti_quotient_partial<AntiLine>;
    fn div(self, _rhs: geometric_anti_quotient) -> Self::Output {
        geometric_anti_quotient_partial(self)
    }
}
impl GeometricAntiQuotient<AntiCircleRotor> for AntiLine {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        6        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        7        8        1
    //  no simd        7       12        1
    fn geometric_anti_quotient(self, other: AntiCircleRotor) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e41] * other[e15])
                + 2.0 * (other[e42] * other[e25])
                + 2.0 * (other[e43] * other[e35])
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                - f32::powi(other[e45], 2)
                - f32::powi(other[scalar], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e15, e25, e35
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e15], self[e25], self[e35]]),
        );
    }
}
impl GeometricAntiQuotient<AntiDipoleInversion> for AntiLine {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10        8        1
    //    simd3        0        2        0
    // Totals...
    // yes simd       10       10        1
    //  no simd       10       14        1
    fn geometric_anti_quotient(self, other: AntiDipoleInversion) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e4] * other[e5]) + f32::powi(other[e321], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - f32::powi(other[e1], 2)
                - f32::powi(other[e2], 2)
                - f32::powi(other[e3], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e15, e25, e35
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e15], self[e25], self[e35]]),
        );
    }
}
impl GeometricAntiQuotient<AntiDualNum> for AntiLine {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        3        1
    //  no simd        0        7        1
    fn geometric_anti_quotient(self, other: AntiDualNum) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[scalar], 2) * -1.0);
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e15, e25, e35
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e15], self[e25], self[e35]]),
        );
    }
}
impl GeometricAntiQuotient<AntiFlatPoint> for AntiLine {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        6        1
    fn geometric_anti_quotient(self, other: AntiFlatPoint) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e321], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e15, e25, e35
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e15], self[e25], self[e35]]),
        );
    }
}
impl GeometricAntiQuotient<AntiFlector> for AntiLine {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        3        2        1
    //  no simd        3        6        1
    fn geometric_anti_quotient(self, other: AntiFlector) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e321], 2) - f32::powi(other[e1], 2) - f32::powi(other[e2], 2) - f32::powi(other[e3], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e15, e25, e35
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e15], self[e25], self[e35]]),
        );
    }
}
impl GeometricAntiQuotient<AntiLine> for AntiLine {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        2        2        1
    //  no simd        2        6        1
    fn geometric_anti_quotient(self, other: AntiLine) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e23], 2) + f32::powi(other[e31], 2) + f32::powi(other[e12], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e15, e25, e35
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e15], self[e25], self[e35]]),
        );
    }
}
impl GeometricAntiQuotient<AntiMotor> for AntiLine {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        3        2        1
    //  no simd        3        6        1
    fn geometric_anti_quotient(self, other: AntiMotor) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e23], 2) + f32::powi(other[e31], 2) + f32::powi(other[e12], 2) - f32::powi(other[scalar], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e15, e25, e35
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e15], self[e25], self[e35]]),
        );
    }
}
impl GeometricAntiQuotient<AntiPlane> for AntiLine {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        2        2        1
    //  no simd        2        6        1
    fn geometric_anti_quotient(self, other: AntiPlane) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ -f32::powi(other[e1], 2) - f32::powi(other[e2], 2) - f32::powi(other[e3], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e15, e25, e35
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e15], self[e25], self[e35]]),
        );
    }
}
impl GeometricAntiQuotient<AntiScalar> for AntiLine {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        6        1
    fn geometric_anti_quotient(self, other: AntiScalar) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e12345], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e15, e25, e35
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e15], self[e25], self[e35]]),
        );
    }
}
impl GeometricAntiQuotient<Circle> for AntiLine {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        6        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        6        8        1
    //  no simd        6       12        1
    fn geometric_anti_quotient(self, other: Circle) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e321], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e15, e25, e35
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e15], self[e25], self[e35]]),
        );
    }
}
impl GeometricAntiQuotient<CircleRotor> for AntiLine {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        6        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        7        8        1
    //  no simd        7       12        1
    fn geometric_anti_quotient(self, other: CircleRotor) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e321], 2) + f32::powi(other[e12345], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e15, e25, e35
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e15], self[e25], self[e35]]),
        );
    }
}
impl GeometricAntiQuotient<Dipole> for AntiLine {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        6        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        6        8        1
    //  no simd        6       12        1
    fn geometric_anti_quotient(self, other: Dipole) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e41] * other[e15])
                + 2.0 * (other[e42] * other[e25])
                + 2.0 * (other[e43] * other[e35])
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                - f32::powi(other[e45], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e15, e25, e35
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e15], self[e25], self[e35]]),
        );
    }
}
impl GeometricAntiQuotient<DipoleInversion> for AntiLine {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10        8        1
    //    simd3        0        2        0
    // Totals...
    // yes simd       10       10        1
    //  no simd       10       14        1
    fn geometric_anti_quotient(self, other: DipoleInversion) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e41] * other[e15])
                + 2.0 * (other[e42] * other[e25])
                + 2.0 * (other[e43] * other[e35])
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                + f32::powi(other[e4235], 2)
                + f32::powi(other[e4315], 2)
                + f32::powi(other[e4125], 2)
                - f32::powi(other[e45], 2)
                - 2.0 * (other[e1234] * other[e3215]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e15, e25, e35
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e15], self[e25], self[e35]]),
        );
    }
}
impl GeometricAntiQuotient<DualNum> for AntiLine {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        6        1
    fn geometric_anti_quotient(self, other: DualNum) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e12345], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e15, e25, e35
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e15], self[e25], self[e35]]),
        );
    }
}
impl GeometricAntiQuotient<FlatPoint> for AntiLine {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        3        1
    //  no simd        0        7        1
    fn geometric_anti_quotient(self, other: FlatPoint) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e45], 2) * -1.0);
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e15, e25, e35
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e15], self[e25], self[e35]]),
        );
    }
}
impl GeometricAntiQuotient<Flector> for AntiLine {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        3        2        1
    //  no simd        3        6        1
    fn geometric_anti_quotient(self, other: Flector) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e4235], 2) + f32::powi(other[e4315], 2) + f32::powi(other[e4125], 2) - f32::powi(other[e45], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e15, e25, e35
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e15], self[e25], self[e35]]),
        );
    }
}
impl GeometricAntiQuotient<Line> for AntiLine {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        2        2        1
    //  no simd        2        6        1
    fn geometric_anti_quotient(self, other: Line) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ -f32::powi(other[e415], 2) - f32::powi(other[e425], 2) - f32::powi(other[e435], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e15, e25, e35
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e15], self[e25], self[e35]]),
        );
    }
}
impl GeometricAntiQuotient<Motor> for AntiLine {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        3        2        1
    //  no simd        3        6        1
    fn geometric_anti_quotient(self, other: Motor) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e12345], 2) - f32::powi(other[e415], 2) - f32::powi(other[e425], 2) - f32::powi(other[e435], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e15, e25, e35
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e15], self[e25], self[e35]]),
        );
    }
}
impl GeometricAntiQuotient<MultiVector> for AntiLine {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       23       16        1
    //    simd3        0        2        0
    // Totals...
    // yes simd       23       18        1
    //  no simd       23       22        1
    fn geometric_anti_quotient(self, other: MultiVector) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e4] * other[e5])
                + 2.0 * (other[e15] * other[e41])
                + 2.0 * (other[e25] * other[e42])
                + 2.0 * (other[e35] * other[e43])
                + f32::powi(other[e12345], 2)
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                + f32::powi(other[e321], 2)
                + f32::powi(other[e4235], 2)
                + f32::powi(other[e4315], 2)
                + f32::powi(other[e4125], 2)
                - f32::powi(other[scalar], 2)
                - f32::powi(other[e1], 2)
                - f32::powi(other[e2], 2)
                - f32::powi(other[e3], 2)
                - f32::powi(other[e45], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125])
                - 2.0 * (other[e3215] * other[e1234]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e15, e25, e35
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e15], self[e25], self[e35]]),
        );
    }
}
impl GeometricAntiQuotient<Plane> for AntiLine {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        2        2        1
    //  no simd        2        6        1
    fn geometric_anti_quotient(self, other: Plane) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e4235], 2) + f32::powi(other[e4315], 2) + f32::powi(other[e4125], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e15, e25, e35
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e15], self[e25], self[e35]]),
        );
    }
}
impl GeometricAntiQuotient<RoundPoint> for AntiLine {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        2        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        3        4        1
    //  no simd        3        8        1
    fn geometric_anti_quotient(self, other: RoundPoint) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e4] * other[e5]) - f32::powi(other[e1], 2) - f32::powi(other[e2], 2) - f32::powi(other[e3], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e15, e25, e35
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e15], self[e25], self[e35]]),
        );
    }
}
impl GeometricAntiQuotient<Scalar> for AntiLine {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        3        1
    //  no simd        0        7        1
    fn geometric_anti_quotient(self, other: Scalar) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[scalar], 2) * -1.0);
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e15, e25, e35
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e15], self[e25], self[e35]]),
        );
    }
}
impl GeometricAntiQuotient<Sphere> for AntiLine {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        2        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        3        4        1
    //  no simd        3        8        1
    fn geometric_anti_quotient(self, other: Sphere) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e4235], 2) + f32::powi(other[e4315], 2) + f32::powi(other[e4125], 2) - 2.0 * (other[e3215] * other[e1234]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e15, e25, e35
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e15], self[e25], self[e35]]),
        );
    }
}
impl GeometricAntiQuotient<VersorEven> for AntiLine {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11        8        1
    //    simd3        0        2        0
    // Totals...
    // yes simd       11       10        1
    //  no simd       11       14        1
    fn geometric_anti_quotient(self, other: VersorEven) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e5] * other[e4]) + f32::powi(other[e12345], 2) + f32::powi(other[e321], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - f32::powi(other[e1], 2)
                - f32::powi(other[e2], 2)
                - f32::powi(other[e3], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e15, e25, e35
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e15], self[e25], self[e35]]),
        );
    }
}
impl GeometricAntiQuotient<VersorOdd> for AntiLine {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11        8        1
    //    simd3        0        2        0
    // Totals...
    // yes simd       11       10        1
    //  no simd       11       14        1
    fn geometric_anti_quotient(self, other: VersorOdd) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e41] * other[e15])
                + 2.0 * (other[e42] * other[e25])
                + 2.0 * (other[e43] * other[e35])
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                + f32::powi(other[e4235], 2)
                + f32::powi(other[e4315], 2)
                + f32::powi(other[e4125], 2)
                - f32::powi(other[scalar], 2)
                - f32::powi(other[e45], 2)
                - 2.0 * (other[e1234] * other[e3215]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e15, e25, e35
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e15], self[e25], self[e35]]),
        );
    }
}
impl std::ops::Div<geometric_anti_quotient> for AntiMotor {
    type Output = geometric_anti_quotient_partial<AntiMotor>;
    fn div(self, _rhs: geometric_anti_quotient) -> Self::Output {
        geometric_anti_quotient_partial(self)
    }
}
impl GeometricAntiQuotient<AntiCircleRotor> for AntiMotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        6        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        7        8        1
    //  no simd        7       14        1
    fn geometric_anti_quotient(self, other: AntiCircleRotor) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e41] * other[e15])
                + 2.0 * (other[e42] * other[e25])
                + 2.0 * (other[e43] * other[e35])
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                - f32::powi(other[e45], 2)
                - f32::powi(other[scalar], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]),
            // e15, e25, e35, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<AntiDipoleInversion> for AntiMotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10        8        1
    //    simd4        0        2        0
    // Totals...
    // yes simd       10       10        1
    //  no simd       10       16        1
    fn geometric_anti_quotient(self, other: AntiDipoleInversion) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e4] * other[e5]) + f32::powi(other[e321], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - f32::powi(other[e1], 2)
                - f32::powi(other[e2], 2)
                - f32::powi(other[e3], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]),
            // e15, e25, e35, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<AntiDualNum> for AntiMotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        1
    //  no simd        0        9        1
    fn geometric_anti_quotient(self, other: AntiDualNum) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[scalar], 2) * -1.0);
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]),
            // e15, e25, e35, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<AntiFlatPoint> for AntiMotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        8        1
    fn geometric_anti_quotient(self, other: AntiFlatPoint) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e321], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]),
            // e15, e25, e35, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<AntiFlector> for AntiMotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        2        1
    //  no simd        3        8        1
    fn geometric_anti_quotient(self, other: AntiFlector) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e321], 2) - f32::powi(other[e1], 2) - f32::powi(other[e2], 2) - f32::powi(other[e3], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]),
            // e15, e25, e35, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<AntiLine> for AntiMotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        2        2        1
    //  no simd        2        8        1
    fn geometric_anti_quotient(self, other: AntiLine) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e23], 2) + f32::powi(other[e31], 2) + f32::powi(other[e12], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]),
            // e15, e25, e35, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<AntiMotor> for AntiMotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        2        1
    //  no simd        3        8        1
    fn geometric_anti_quotient(self, other: AntiMotor) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e23], 2) + f32::powi(other[e31], 2) + f32::powi(other[e12], 2) - f32::powi(other[scalar], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]),
            // e15, e25, e35, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<AntiPlane> for AntiMotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        2        2        1
    //  no simd        2        8        1
    fn geometric_anti_quotient(self, other: AntiPlane) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ -f32::powi(other[e1], 2) - f32::powi(other[e2], 2) - f32::powi(other[e3], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]),
            // e15, e25, e35, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<AntiScalar> for AntiMotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        8        1
    fn geometric_anti_quotient(self, other: AntiScalar) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e12345], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]),
            // e15, e25, e35, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<Circle> for AntiMotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        6        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        6        8        1
    //  no simd        6       14        1
    fn geometric_anti_quotient(self, other: Circle) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e321], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]),
            // e15, e25, e35, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<CircleRotor> for AntiMotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        6        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        7        8        1
    //  no simd        7       14        1
    fn geometric_anti_quotient(self, other: CircleRotor) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e321], 2) + f32::powi(other[e12345], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]),
            // e15, e25, e35, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<Dipole> for AntiMotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        6        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        6        8        1
    //  no simd        6       14        1
    fn geometric_anti_quotient(self, other: Dipole) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e41] * other[e15])
                + 2.0 * (other[e42] * other[e25])
                + 2.0 * (other[e43] * other[e35])
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                - f32::powi(other[e45], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]),
            // e15, e25, e35, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<DipoleInversion> for AntiMotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10        8        1
    //    simd4        0        2        0
    // Totals...
    // yes simd       10       10        1
    //  no simd       10       16        1
    fn geometric_anti_quotient(self, other: DipoleInversion) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e41] * other[e15])
                + 2.0 * (other[e42] * other[e25])
                + 2.0 * (other[e43] * other[e35])
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                + f32::powi(other[e4235], 2)
                + f32::powi(other[e4315], 2)
                + f32::powi(other[e4125], 2)
                - f32::powi(other[e45], 2)
                - 2.0 * (other[e1234] * other[e3215]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]),
            // e15, e25, e35, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<DualNum> for AntiMotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        8        1
    fn geometric_anti_quotient(self, other: DualNum) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e12345], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]),
            // e15, e25, e35, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<FlatPoint> for AntiMotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        1
    //  no simd        0        9        1
    fn geometric_anti_quotient(self, other: FlatPoint) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e45], 2) * -1.0);
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]),
            // e15, e25, e35, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<Flector> for AntiMotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        2        1
    //  no simd        3        8        1
    fn geometric_anti_quotient(self, other: Flector) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e4235], 2) + f32::powi(other[e4315], 2) + f32::powi(other[e4125], 2) - f32::powi(other[e45], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]),
            // e15, e25, e35, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<Line> for AntiMotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        2        2        1
    //  no simd        2        8        1
    fn geometric_anti_quotient(self, other: Line) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ -f32::powi(other[e415], 2) - f32::powi(other[e425], 2) - f32::powi(other[e435], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]),
            // e15, e25, e35, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<Motor> for AntiMotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        2        1
    //  no simd        3        8        1
    fn geometric_anti_quotient(self, other: Motor) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e12345], 2) - f32::powi(other[e415], 2) - f32::powi(other[e425], 2) - f32::powi(other[e435], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]),
            // e15, e25, e35, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<MultiVector> for AntiMotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       23       16        1
    //    simd4        0        2        0
    // Totals...
    // yes simd       23       18        1
    //  no simd       23       24        1
    fn geometric_anti_quotient(self, other: MultiVector) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e4] * other[e5])
                + 2.0 * (other[e15] * other[e41])
                + 2.0 * (other[e25] * other[e42])
                + 2.0 * (other[e35] * other[e43])
                + f32::powi(other[e12345], 2)
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                + f32::powi(other[e321], 2)
                + f32::powi(other[e4235], 2)
                + f32::powi(other[e4315], 2)
                + f32::powi(other[e4125], 2)
                - f32::powi(other[scalar], 2)
                - f32::powi(other[e1], 2)
                - f32::powi(other[e2], 2)
                - f32::powi(other[e3], 2)
                - f32::powi(other[e45], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125])
                - 2.0 * (other[e3215] * other[e1234]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]),
            // e15, e25, e35, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<Plane> for AntiMotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        2        2        1
    //  no simd        2        8        1
    fn geometric_anti_quotient(self, other: Plane) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e4235], 2) + f32::powi(other[e4315], 2) + f32::powi(other[e4125], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]),
            // e15, e25, e35, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<RoundPoint> for AntiMotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        2        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        4        1
    //  no simd        3       10        1
    fn geometric_anti_quotient(self, other: RoundPoint) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e4] * other[e5]) - f32::powi(other[e1], 2) - f32::powi(other[e2], 2) - f32::powi(other[e3], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]),
            // e15, e25, e35, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<Scalar> for AntiMotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        1
    //  no simd        0        9        1
    fn geometric_anti_quotient(self, other: Scalar) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[scalar], 2) * -1.0);
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]),
            // e15, e25, e35, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<Sphere> for AntiMotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        2        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        4        1
    //  no simd        3       10        1
    fn geometric_anti_quotient(self, other: Sphere) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e4235], 2) + f32::powi(other[e4315], 2) + f32::powi(other[e4125], 2) - 2.0 * (other[e3215] * other[e1234]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]),
            // e15, e25, e35, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<VersorEven> for AntiMotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11        8        1
    //    simd4        0        2        0
    // Totals...
    // yes simd       11       10        1
    //  no simd       11       16        1
    fn geometric_anti_quotient(self, other: VersorEven) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e5] * other[e4]) + f32::powi(other[e12345], 2) + f32::powi(other[e321], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - f32::powi(other[e1], 2)
                - f32::powi(other[e2], 2)
                - f32::powi(other[e3], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]),
            // e15, e25, e35, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<VersorOdd> for AntiMotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11        8        1
    //    simd4        0        2        0
    // Totals...
    // yes simd       11       10        1
    //  no simd       11       16        1
    fn geometric_anti_quotient(self, other: VersorOdd) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e41] * other[e15])
                + 2.0 * (other[e42] * other[e25])
                + 2.0 * (other[e43] * other[e35])
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                + f32::powi(other[e4235], 2)
                + f32::powi(other[e4315], 2)
                + f32::powi(other[e4125], 2)
                - f32::powi(other[scalar], 2)
                - f32::powi(other[e45], 2)
                - 2.0 * (other[e1234] * other[e3215]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]),
            // e15, e25, e35, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e3215]]),
        );
    }
}
impl std::ops::Div<geometric_anti_quotient> for AntiPlane {
    type Output = geometric_anti_quotient_partial<AntiPlane>;
    fn div(self, _rhs: geometric_anti_quotient) -> Self::Output {
        geometric_anti_quotient_partial(self)
    }
}
impl GeometricAntiQuotient<AntiCircleRotor> for AntiPlane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        6        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        7        7        1
    //  no simd        7       10        1
    fn geometric_anti_quotient(self, other: AntiCircleRotor) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e41] * other[e15])
                + 2.0 * (other[e42] * other[e25])
                + 2.0 * (other[e43] * other[e35])
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                - f32::powi(other[e45], 2)
                - f32::powi(other[scalar], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricAntiQuotient<AntiDipoleInversion> for AntiPlane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10        8        1
    //    simd4        0        1        0
    // Totals...
    // yes simd       10        9        1
    //  no simd       10       12        1
    fn geometric_anti_quotient(self, other: AntiDipoleInversion) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e4] * other[e5]) + f32::powi(other[e321], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - f32::powi(other[e1], 2)
                - f32::powi(other[e2], 2)
                - f32::powi(other[e3], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricAntiQuotient<AntiDualNum> for AntiPlane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        5        1
    fn geometric_anti_quotient(self, other: AntiDualNum) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[scalar], 2) * -1.0);
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricAntiQuotient<AntiFlatPoint> for AntiPlane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        1        1
    //  no simd        0        4        1
    fn geometric_anti_quotient(self, other: AntiFlatPoint) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e321], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricAntiQuotient<AntiFlector> for AntiPlane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        1        1
    //  no simd        3        4        1
    fn geometric_anti_quotient(self, other: AntiFlector) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e321], 2) - f32::powi(other[e1], 2) - f32::powi(other[e2], 2) - f32::powi(other[e3], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricAntiQuotient<AntiLine> for AntiPlane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        1        1
    //  no simd        2        4        1
    fn geometric_anti_quotient(self, other: AntiLine) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e23], 2) + f32::powi(other[e31], 2) + f32::powi(other[e12], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricAntiQuotient<AntiMotor> for AntiPlane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        1        1
    //  no simd        3        4        1
    fn geometric_anti_quotient(self, other: AntiMotor) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e23], 2) + f32::powi(other[e31], 2) + f32::powi(other[e12], 2) - f32::powi(other[scalar], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricAntiQuotient<AntiPlane> for AntiPlane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        1        1
    //  no simd        2        4        1
    fn geometric_anti_quotient(self, other: AntiPlane) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ -f32::powi(other[e1], 2) - f32::powi(other[e2], 2) - f32::powi(other[e3], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricAntiQuotient<AntiScalar> for AntiPlane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        1        1
    //  no simd        0        4        1
    fn geometric_anti_quotient(self, other: AntiScalar) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e12345], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricAntiQuotient<Circle> for AntiPlane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        6        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        6        7        1
    //  no simd        6       10        1
    fn geometric_anti_quotient(self, other: Circle) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e321], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricAntiQuotient<CircleRotor> for AntiPlane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        6        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        7        7        1
    //  no simd        7       10        1
    fn geometric_anti_quotient(self, other: CircleRotor) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e321], 2) + f32::powi(other[e12345], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricAntiQuotient<Dipole> for AntiPlane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        6        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        6        7        1
    //  no simd        6       10        1
    fn geometric_anti_quotient(self, other: Dipole) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e41] * other[e15])
                + 2.0 * (other[e42] * other[e25])
                + 2.0 * (other[e43] * other[e35])
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                - f32::powi(other[e45], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricAntiQuotient<DipoleInversion> for AntiPlane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10        8        1
    //    simd4        0        1        0
    // Totals...
    // yes simd       10        9        1
    //  no simd       10       12        1
    fn geometric_anti_quotient(self, other: DipoleInversion) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e41] * other[e15])
                + 2.0 * (other[e42] * other[e25])
                + 2.0 * (other[e43] * other[e35])
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                + f32::powi(other[e4235], 2)
                + f32::powi(other[e4315], 2)
                + f32::powi(other[e4125], 2)
                - f32::powi(other[e45], 2)
                - 2.0 * (other[e1234] * other[e3215]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricAntiQuotient<DualNum> for AntiPlane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        1        1
    //  no simd        0        4        1
    fn geometric_anti_quotient(self, other: DualNum) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e12345], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricAntiQuotient<FlatPoint> for AntiPlane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        5        1
    fn geometric_anti_quotient(self, other: FlatPoint) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e45], 2) * -1.0);
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricAntiQuotient<Flector> for AntiPlane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        1        1
    //  no simd        3        4        1
    fn geometric_anti_quotient(self, other: Flector) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e4235], 2) + f32::powi(other[e4315], 2) + f32::powi(other[e4125], 2) - f32::powi(other[e45], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricAntiQuotient<Line> for AntiPlane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        1        1
    //  no simd        2        4        1
    fn geometric_anti_quotient(self, other: Line) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ -f32::powi(other[e415], 2) - f32::powi(other[e425], 2) - f32::powi(other[e435], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricAntiQuotient<Motor> for AntiPlane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        1        1
    //  no simd        3        4        1
    fn geometric_anti_quotient(self, other: Motor) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e12345], 2) - f32::powi(other[e415], 2) - f32::powi(other[e425], 2) - f32::powi(other[e435], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricAntiQuotient<MultiVector> for AntiPlane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       23       16        1
    //    simd4        0        1        0
    // Totals...
    // yes simd       23       17        1
    //  no simd       23       20        1
    fn geometric_anti_quotient(self, other: MultiVector) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e4] * other[e5])
                + 2.0 * (other[e15] * other[e41])
                + 2.0 * (other[e25] * other[e42])
                + 2.0 * (other[e35] * other[e43])
                + f32::powi(other[e12345], 2)
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                + f32::powi(other[e321], 2)
                + f32::powi(other[e4235], 2)
                + f32::powi(other[e4315], 2)
                + f32::powi(other[e4125], 2)
                - f32::powi(other[scalar], 2)
                - f32::powi(other[e1], 2)
                - f32::powi(other[e2], 2)
                - f32::powi(other[e3], 2)
                - f32::powi(other[e45], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125])
                - 2.0 * (other[e3215] * other[e1234]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricAntiQuotient<Plane> for AntiPlane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        1        1
    //  no simd        2        4        1
    fn geometric_anti_quotient(self, other: Plane) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e4235], 2) + f32::powi(other[e4315], 2) + f32::powi(other[e4125], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricAntiQuotient<RoundPoint> for AntiPlane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        2        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        3        1
    //  no simd        3        6        1
    fn geometric_anti_quotient(self, other: RoundPoint) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e4] * other[e5]) - f32::powi(other[e1], 2) - f32::powi(other[e2], 2) - f32::powi(other[e3], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricAntiQuotient<Scalar> for AntiPlane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        5        1
    fn geometric_anti_quotient(self, other: Scalar) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[scalar], 2) * -1.0);
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricAntiQuotient<Sphere> for AntiPlane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        2        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        3        1
    //  no simd        3        6        1
    fn geometric_anti_quotient(self, other: Sphere) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e4235], 2) + f32::powi(other[e4315], 2) + f32::powi(other[e4125], 2) - 2.0 * (other[e3215] * other[e1234]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricAntiQuotient<VersorEven> for AntiPlane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11        8        1
    //    simd4        0        1        0
    // Totals...
    // yes simd       11        9        1
    //  no simd       11       12        1
    fn geometric_anti_quotient(self, other: VersorEven) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e5] * other[e4]) + f32::powi(other[e12345], 2) + f32::powi(other[e321], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - f32::powi(other[e1], 2)
                - f32::powi(other[e2], 2)
                - f32::powi(other[e3], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl GeometricAntiQuotient<VersorOdd> for AntiPlane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11        8        1
    //    simd4        0        1        0
    // Totals...
    // yes simd       11        9        1
    //  no simd       11       12        1
    fn geometric_anti_quotient(self, other: VersorOdd) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e41] * other[e15])
                + 2.0 * (other[e42] * other[e25])
                + 2.0 * (other[e43] * other[e35])
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                + f32::powi(other[e4235], 2)
                + f32::powi(other[e4315], 2)
                + f32::powi(other[e4125], 2)
                - f32::powi(other[scalar], 2)
                - f32::powi(other[e45], 2)
                - 2.0 * (other[e1234] * other[e3215]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl std::ops::Div<geometric_anti_quotient> for AntiScalar {
    type Output = geometric_anti_quotient_partial<AntiScalar>;
    fn div(self, _rhs: geometric_anti_quotient) -> Self::Output {
        geometric_anti_quotient_partial(self)
    }
}
impl GeometricAntiQuotient<AntiCircleRotor> for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        7        1
    fn geometric_anti_quotient(self, other: AntiCircleRotor) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e41] * other[e15])
                + 2.0 * (other[e42] * other[e25])
                + 2.0 * (other[e43] * other[e35])
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                - f32::powi(other[e45], 2)
                - f32::powi(other[scalar], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiScalar::from_groups(/* e12345 */ anti_inverse[e12345] * self[e12345]);
    }
}
impl GeometricAntiQuotient<AntiDipoleInversion> for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10        9        1
    fn geometric_anti_quotient(self, other: AntiDipoleInversion) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e4] * other[e5]) + f32::powi(other[e321], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - f32::powi(other[e1], 2)
                - f32::powi(other[e2], 2)
                - f32::powi(other[e3], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiScalar::from_groups(/* e12345 */ anti_inverse[e12345] * self[e12345]);
    }
}
impl GeometricAntiQuotient<AntiDualNum> for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        1
    fn geometric_anti_quotient(self, other: AntiDualNum) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[scalar], 2) * -1.0);
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiScalar::from_groups(/* e12345 */ anti_inverse[e12345] * self[e12345]);
    }
}
impl GeometricAntiQuotient<AntiFlatPoint> for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        1
    fn geometric_anti_quotient(self, other: AntiFlatPoint) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e321], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiScalar::from_groups(/* e12345 */ anti_inverse[e12345] * self[e12345]);
    }
}
impl GeometricAntiQuotient<AntiFlector> for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        1        1
    fn geometric_anti_quotient(self, other: AntiFlector) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e321], 2) - f32::powi(other[e1], 2) - f32::powi(other[e2], 2) - f32::powi(other[e3], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiScalar::from_groups(/* e12345 */ anti_inverse[e12345] * self[e12345]);
    }
}
impl GeometricAntiQuotient<AntiLine> for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        1        1
    fn geometric_anti_quotient(self, other: AntiLine) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e23], 2) + f32::powi(other[e31], 2) + f32::powi(other[e12], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiScalar::from_groups(/* e12345 */ anti_inverse[e12345] * self[e12345]);
    }
}
impl GeometricAntiQuotient<AntiMotor> for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        1        1
    fn geometric_anti_quotient(self, other: AntiMotor) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e23], 2) + f32::powi(other[e31], 2) + f32::powi(other[e12], 2) - f32::powi(other[scalar], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiScalar::from_groups(/* e12345 */ anti_inverse[e12345] * self[e12345]);
    }
}
impl GeometricAntiQuotient<AntiPlane> for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        1        1
    fn geometric_anti_quotient(self, other: AntiPlane) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ -f32::powi(other[e1], 2) - f32::powi(other[e2], 2) - f32::powi(other[e3], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiScalar::from_groups(/* e12345 */ anti_inverse[e12345] * self[e12345]);
    }
}
impl GeometricAntiQuotient<AntiScalar> for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        1
    fn geometric_anti_quotient(self, other: AntiScalar) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e12345], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiScalar::from_groups(/* e12345 */ anti_inverse[e12345] * self[e12345]);
    }
}
impl GeometricAntiQuotient<Circle> for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        7        1
    fn geometric_anti_quotient(self, other: Circle) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e321], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiScalar::from_groups(/* e12345 */ anti_inverse[e12345] * self[e12345]);
    }
}
impl GeometricAntiQuotient<CircleRotor> for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        7        1
    fn geometric_anti_quotient(self, other: CircleRotor) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e321], 2) + f32::powi(other[e12345], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiScalar::from_groups(/* e12345 */ anti_inverse[e12345] * self[e12345]);
    }
}
impl GeometricAntiQuotient<Dipole> for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        7        1
    fn geometric_anti_quotient(self, other: Dipole) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e41] * other[e15])
                + 2.0 * (other[e42] * other[e25])
                + 2.0 * (other[e43] * other[e35])
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                - f32::powi(other[e45], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiScalar::from_groups(/* e12345 */ anti_inverse[e12345] * self[e12345]);
    }
}
impl GeometricAntiQuotient<DipoleInversion> for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10        9        1
    fn geometric_anti_quotient(self, other: DipoleInversion) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e41] * other[e15])
                + 2.0 * (other[e42] * other[e25])
                + 2.0 * (other[e43] * other[e35])
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                + f32::powi(other[e4235], 2)
                + f32::powi(other[e4315], 2)
                + f32::powi(other[e4125], 2)
                - f32::powi(other[e45], 2)
                - 2.0 * (other[e1234] * other[e3215]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiScalar::from_groups(/* e12345 */ anti_inverse[e12345] * self[e12345]);
    }
}
impl GeometricAntiQuotient<DualNum> for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        1
    fn geometric_anti_quotient(self, other: DualNum) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e12345], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiScalar::from_groups(/* e12345 */ anti_inverse[e12345] * self[e12345]);
    }
}
impl GeometricAntiQuotient<FlatPoint> for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        1
    fn geometric_anti_quotient(self, other: FlatPoint) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e45], 2) * -1.0);
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiScalar::from_groups(/* e12345 */ anti_inverse[e12345] * self[e12345]);
    }
}
impl GeometricAntiQuotient<Flector> for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        1        1
    fn geometric_anti_quotient(self, other: Flector) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e4235], 2) + f32::powi(other[e4315], 2) + f32::powi(other[e4125], 2) - f32::powi(other[e45], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiScalar::from_groups(/* e12345 */ anti_inverse[e12345] * self[e12345]);
    }
}
impl GeometricAntiQuotient<Line> for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        1        1
    fn geometric_anti_quotient(self, other: Line) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ -f32::powi(other[e415], 2) - f32::powi(other[e425], 2) - f32::powi(other[e435], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiScalar::from_groups(/* e12345 */ anti_inverse[e12345] * self[e12345]);
    }
}
impl GeometricAntiQuotient<Motor> for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        1        1
    fn geometric_anti_quotient(self, other: Motor) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e12345], 2) - f32::powi(other[e415], 2) - f32::powi(other[e425], 2) - f32::powi(other[e435], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiScalar::from_groups(/* e12345 */ anti_inverse[e12345] * self[e12345]);
    }
}
impl GeometricAntiQuotient<MultiVector> for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       23       17        1
    fn geometric_anti_quotient(self, other: MultiVector) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e4] * other[e5])
                + 2.0 * (other[e15] * other[e41])
                + 2.0 * (other[e25] * other[e42])
                + 2.0 * (other[e35] * other[e43])
                + f32::powi(other[e12345], 2)
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                + f32::powi(other[e321], 2)
                + f32::powi(other[e4235], 2)
                + f32::powi(other[e4315], 2)
                + f32::powi(other[e4125], 2)
                - f32::powi(other[scalar], 2)
                - f32::powi(other[e1], 2)
                - f32::powi(other[e2], 2)
                - f32::powi(other[e3], 2)
                - f32::powi(other[e45], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125])
                - 2.0 * (other[e3215] * other[e1234]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiScalar::from_groups(/* e12345 */ anti_inverse[e12345] * self[e12345]);
    }
}
impl GeometricAntiQuotient<Plane> for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        1        1
    fn geometric_anti_quotient(self, other: Plane) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e4235], 2) + f32::powi(other[e4315], 2) + f32::powi(other[e4125], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiScalar::from_groups(/* e12345 */ anti_inverse[e12345] * self[e12345]);
    }
}
impl GeometricAntiQuotient<RoundPoint> for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        3        1
    fn geometric_anti_quotient(self, other: RoundPoint) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e4] * other[e5]) - f32::powi(other[e1], 2) - f32::powi(other[e2], 2) - f32::powi(other[e3], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiScalar::from_groups(/* e12345 */ anti_inverse[e12345] * self[e12345]);
    }
}
impl GeometricAntiQuotient<Scalar> for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        1
    fn geometric_anti_quotient(self, other: Scalar) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[scalar], 2) * -1.0);
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiScalar::from_groups(/* e12345 */ anti_inverse[e12345] * self[e12345]);
    }
}
impl GeometricAntiQuotient<Sphere> for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        3        1
    fn geometric_anti_quotient(self, other: Sphere) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e4235], 2) + f32::powi(other[e4315], 2) + f32::powi(other[e4125], 2) - 2.0 * (other[e3215] * other[e1234]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiScalar::from_groups(/* e12345 */ anti_inverse[e12345] * self[e12345]);
    }
}
impl GeometricAntiQuotient<VersorEven> for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       11        9        1
    fn geometric_anti_quotient(self, other: VersorEven) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e5] * other[e4]) + f32::powi(other[e12345], 2) + f32::powi(other[e321], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - f32::powi(other[e1], 2)
                - f32::powi(other[e2], 2)
                - f32::powi(other[e3], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiScalar::from_groups(/* e12345 */ anti_inverse[e12345] * self[e12345]);
    }
}
impl GeometricAntiQuotient<VersorOdd> for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       11        9        1
    fn geometric_anti_quotient(self, other: VersorOdd) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e41] * other[e15])
                + 2.0 * (other[e42] * other[e25])
                + 2.0 * (other[e43] * other[e35])
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                + f32::powi(other[e4235], 2)
                + f32::powi(other[e4315], 2)
                + f32::powi(other[e4125], 2)
                - f32::powi(other[scalar], 2)
                - f32::powi(other[e45], 2)
                - 2.0 * (other[e1234] * other[e3215]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiScalar::from_groups(/* e12345 */ anti_inverse[e12345] * self[e12345]);
    }
}
impl std::ops::Div<geometric_anti_quotient> for Circle {
    type Output = geometric_anti_quotient_partial<Circle>;
    fn div(self, _rhs: geometric_anti_quotient) -> Self::Output {
        geometric_anti_quotient_partial(self)
    }
}
impl GeometricAntiQuotient<AntiCircleRotor> for Circle {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        6        1
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        7        9        1
    //  no simd        7       16        1
    fn geometric_anti_quotient(self, other: AntiCircleRotor) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e41] * other[e15])
                + 2.0 * (other[e42] * other[e25])
                + 2.0 * (other[e43] * other[e35])
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                - f32::powi(other[e45], 2)
                - f32::powi(other[scalar], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
        );
    }
}
impl GeometricAntiQuotient<AntiDipoleInversion> for Circle {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10        8        1
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       10       11        1
    //  no simd       10       18        1
    fn geometric_anti_quotient(self, other: AntiDipoleInversion) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e4] * other[e5]) + f32::powi(other[e321], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - f32::powi(other[e1], 2)
                - f32::powi(other[e2], 2)
                - f32::powi(other[e3], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
        );
    }
}
impl GeometricAntiQuotient<AntiDualNum> for Circle {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        4        1
    //  no simd        0       11        1
    fn geometric_anti_quotient(self, other: AntiDualNum) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[scalar], 2) * -1.0);
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
        );
    }
}
impl GeometricAntiQuotient<AntiFlatPoint> for Circle {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        1
    //  no simd        0       10        1
    fn geometric_anti_quotient(self, other: AntiFlatPoint) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e321], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
        );
    }
}
impl GeometricAntiQuotient<AntiFlector> for Circle {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        3        1
    //  no simd        3       10        1
    fn geometric_anti_quotient(self, other: AntiFlector) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e321], 2) - f32::powi(other[e1], 2) - f32::powi(other[e2], 2) - f32::powi(other[e3], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
        );
    }
}
impl GeometricAntiQuotient<AntiLine> for Circle {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        3        1
    //  no simd        2       10        1
    fn geometric_anti_quotient(self, other: AntiLine) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e23], 2) + f32::powi(other[e31], 2) + f32::powi(other[e12], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
        );
    }
}
impl GeometricAntiQuotient<AntiMotor> for Circle {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        3        1
    //  no simd        3       10        1
    fn geometric_anti_quotient(self, other: AntiMotor) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e23], 2) + f32::powi(other[e31], 2) + f32::powi(other[e12], 2) - f32::powi(other[scalar], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
        );
    }
}
impl GeometricAntiQuotient<AntiPlane> for Circle {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        3        1
    //  no simd        2       10        1
    fn geometric_anti_quotient(self, other: AntiPlane) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ -f32::powi(other[e1], 2) - f32::powi(other[e2], 2) - f32::powi(other[e3], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
        );
    }
}
impl GeometricAntiQuotient<AntiScalar> for Circle {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        1
    //  no simd        0       10        1
    fn geometric_anti_quotient(self, other: AntiScalar) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e12345], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
        );
    }
}
impl GeometricAntiQuotient<Circle> for Circle {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        6        1
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        6        9        1
    //  no simd        6       16        1
    fn geometric_anti_quotient(self, other: Circle) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e321], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
        );
    }
}
impl GeometricAntiQuotient<CircleRotor> for Circle {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        6        1
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        7        9        1
    //  no simd        7       16        1
    fn geometric_anti_quotient(self, other: CircleRotor) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e321], 2) + f32::powi(other[e12345], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
        );
    }
}
impl GeometricAntiQuotient<Dipole> for Circle {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        6        1
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        6        9        1
    //  no simd        6       16        1
    fn geometric_anti_quotient(self, other: Dipole) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e41] * other[e15])
                + 2.0 * (other[e42] * other[e25])
                + 2.0 * (other[e43] * other[e35])
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                - f32::powi(other[e45], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
        );
    }
}
impl GeometricAntiQuotient<DipoleInversion> for Circle {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10        8        1
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       10       11        1
    //  no simd       10       18        1
    fn geometric_anti_quotient(self, other: DipoleInversion) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e41] * other[e15])
                + 2.0 * (other[e42] * other[e25])
                + 2.0 * (other[e43] * other[e35])
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                + f32::powi(other[e4235], 2)
                + f32::powi(other[e4315], 2)
                + f32::powi(other[e4125], 2)
                - f32::powi(other[e45], 2)
                - 2.0 * (other[e1234] * other[e3215]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
        );
    }
}
impl GeometricAntiQuotient<DualNum> for Circle {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        1
    //  no simd        0       10        1
    fn geometric_anti_quotient(self, other: DualNum) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e12345], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
        );
    }
}
impl GeometricAntiQuotient<FlatPoint> for Circle {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        4        1
    //  no simd        0       11        1
    fn geometric_anti_quotient(self, other: FlatPoint) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e45], 2) * -1.0);
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
        );
    }
}
impl GeometricAntiQuotient<Flector> for Circle {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        3        1
    //  no simd        3       10        1
    fn geometric_anti_quotient(self, other: Flector) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e4235], 2) + f32::powi(other[e4315], 2) + f32::powi(other[e4125], 2) - f32::powi(other[e45], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
        );
    }
}
impl GeometricAntiQuotient<Line> for Circle {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        3        1
    //  no simd        2       10        1
    fn geometric_anti_quotient(self, other: Line) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ -f32::powi(other[e415], 2) - f32::powi(other[e425], 2) - f32::powi(other[e435], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
        );
    }
}
impl GeometricAntiQuotient<Motor> for Circle {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        3        1
    //  no simd        3       10        1
    fn geometric_anti_quotient(self, other: Motor) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e12345], 2) - f32::powi(other[e415], 2) - f32::powi(other[e425], 2) - f32::powi(other[e435], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
        );
    }
}
impl GeometricAntiQuotient<MultiVector> for Circle {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       23       16        1
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       23       19        1
    //  no simd       23       26        1
    fn geometric_anti_quotient(self, other: MultiVector) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e4] * other[e5])
                + 2.0 * (other[e15] * other[e41])
                + 2.0 * (other[e25] * other[e42])
                + 2.0 * (other[e35] * other[e43])
                + f32::powi(other[e12345], 2)
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                + f32::powi(other[e321], 2)
                + f32::powi(other[e4235], 2)
                + f32::powi(other[e4315], 2)
                + f32::powi(other[e4125], 2)
                - f32::powi(other[scalar], 2)
                - f32::powi(other[e1], 2)
                - f32::powi(other[e2], 2)
                - f32::powi(other[e3], 2)
                - f32::powi(other[e45], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125])
                - 2.0 * (other[e3215] * other[e1234]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
        );
    }
}
impl GeometricAntiQuotient<Plane> for Circle {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        3        1
    //  no simd        2       10        1
    fn geometric_anti_quotient(self, other: Plane) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e4235], 2) + f32::powi(other[e4315], 2) + f32::powi(other[e4125], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
        );
    }
}
impl GeometricAntiQuotient<RoundPoint> for Circle {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        2        1
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        1
    //  no simd        3       12        1
    fn geometric_anti_quotient(self, other: RoundPoint) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e4] * other[e5]) - f32::powi(other[e1], 2) - f32::powi(other[e2], 2) - f32::powi(other[e3], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
        );
    }
}
impl GeometricAntiQuotient<Scalar> for Circle {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        4        1
    //  no simd        0       11        1
    fn geometric_anti_quotient(self, other: Scalar) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[scalar], 2) * -1.0);
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
        );
    }
}
impl GeometricAntiQuotient<Sphere> for Circle {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        2        1
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        1
    //  no simd        3       12        1
    fn geometric_anti_quotient(self, other: Sphere) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e4235], 2) + f32::powi(other[e4315], 2) + f32::powi(other[e4125], 2) - 2.0 * (other[e3215] * other[e1234]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
        );
    }
}
impl GeometricAntiQuotient<VersorEven> for Circle {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11        8        1
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       11       11        1
    //  no simd       11       18        1
    fn geometric_anti_quotient(self, other: VersorEven) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e5] * other[e4]) + f32::powi(other[e12345], 2) + f32::powi(other[e321], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - f32::powi(other[e1], 2)
                - f32::powi(other[e2], 2)
                - f32::powi(other[e3], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
        );
    }
}
impl GeometricAntiQuotient<VersorOdd> for Circle {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11        8        1
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       11       11        1
    //  no simd       11       18        1
    fn geometric_anti_quotient(self, other: VersorOdd) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e41] * other[e15])
                + 2.0 * (other[e42] * other[e25])
                + 2.0 * (other[e43] * other[e35])
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                + f32::powi(other[e4235], 2)
                + f32::powi(other[e4315], 2)
                + f32::powi(other[e4125], 2)
                - f32::powi(other[scalar], 2)
                - f32::powi(other[e45], 2)
                - 2.0 * (other[e1234] * other[e3215]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
        );
    }
}
impl std::ops::Div<geometric_anti_quotient> for CircleRotor {
    type Output = geometric_anti_quotient_partial<CircleRotor>;
    fn div(self, _rhs: geometric_anti_quotient) -> Self::Output {
        geometric_anti_quotient_partial(self)
    }
}
impl GeometricAntiQuotient<AntiCircleRotor> for CircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        6        1
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        7        9        1
    //  no simd        7       17        1
    fn geometric_anti_quotient(self, other: AntiCircleRotor) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e41] * other[e15])
                + 2.0 * (other[e42] * other[e25])
                + 2.0 * (other[e43] * other[e35])
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                - f32::powi(other[e45], 2)
                - f32::powi(other[scalar], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e12345
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e12345]]),
        );
    }
}
impl GeometricAntiQuotient<AntiDipoleInversion> for CircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10        8        1
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       10       11        1
    //  no simd       10       19        1
    fn geometric_anti_quotient(self, other: AntiDipoleInversion) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e4] * other[e5]) + f32::powi(other[e321], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - f32::powi(other[e1], 2)
                - f32::powi(other[e2], 2)
                - f32::powi(other[e3], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e12345
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e12345]]),
        );
    }
}
impl GeometricAntiQuotient<AntiDualNum> for CircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        4        1
    //  no simd        0       12        1
    fn geometric_anti_quotient(self, other: AntiDualNum) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[scalar], 2) * -1.0);
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e12345
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e12345]]),
        );
    }
}
impl GeometricAntiQuotient<AntiFlatPoint> for CircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        1
    //  no simd        0       11        1
    fn geometric_anti_quotient(self, other: AntiFlatPoint) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e321], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e12345
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e12345]]),
        );
    }
}
impl GeometricAntiQuotient<AntiFlector> for CircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        3        1
    //  no simd        3       11        1
    fn geometric_anti_quotient(self, other: AntiFlector) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e321], 2) - f32::powi(other[e1], 2) - f32::powi(other[e2], 2) - f32::powi(other[e3], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e12345
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e12345]]),
        );
    }
}
impl GeometricAntiQuotient<AntiLine> for CircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        2        3        1
    //  no simd        2       11        1
    fn geometric_anti_quotient(self, other: AntiLine) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e23], 2) + f32::powi(other[e31], 2) + f32::powi(other[e12], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e12345
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e12345]]),
        );
    }
}
impl GeometricAntiQuotient<AntiMotor> for CircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        3        1
    //  no simd        3       11        1
    fn geometric_anti_quotient(self, other: AntiMotor) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e23], 2) + f32::powi(other[e31], 2) + f32::powi(other[e12], 2) - f32::powi(other[scalar], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e12345
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e12345]]),
        );
    }
}
impl GeometricAntiQuotient<AntiPlane> for CircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        2        3        1
    //  no simd        2       11        1
    fn geometric_anti_quotient(self, other: AntiPlane) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ -f32::powi(other[e1], 2) - f32::powi(other[e2], 2) - f32::powi(other[e3], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e12345
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e12345]]),
        );
    }
}
impl GeometricAntiQuotient<AntiScalar> for CircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        1
    //  no simd        0       11        1
    fn geometric_anti_quotient(self, other: AntiScalar) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e12345], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e12345
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e12345]]),
        );
    }
}
impl GeometricAntiQuotient<Circle> for CircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        6        1
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        6        9        1
    //  no simd        6       17        1
    fn geometric_anti_quotient(self, other: Circle) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e321], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e12345
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e12345]]),
        );
    }
}
impl GeometricAntiQuotient<CircleRotor> for CircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        6        1
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        7        9        1
    //  no simd        7       17        1
    fn geometric_anti_quotient(self, other: CircleRotor) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e321], 2) + f32::powi(other[e12345], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e12345
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e12345]]),
        );
    }
}
impl GeometricAntiQuotient<Dipole> for CircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        6        1
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        6        9        1
    //  no simd        6       17        1
    fn geometric_anti_quotient(self, other: Dipole) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e41] * other[e15])
                + 2.0 * (other[e42] * other[e25])
                + 2.0 * (other[e43] * other[e35])
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                - f32::powi(other[e45], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e12345
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e12345]]),
        );
    }
}
impl GeometricAntiQuotient<DipoleInversion> for CircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10        8        1
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       10       11        1
    //  no simd       10       19        1
    fn geometric_anti_quotient(self, other: DipoleInversion) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e41] * other[e15])
                + 2.0 * (other[e42] * other[e25])
                + 2.0 * (other[e43] * other[e35])
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                + f32::powi(other[e4235], 2)
                + f32::powi(other[e4315], 2)
                + f32::powi(other[e4125], 2)
                - f32::powi(other[e45], 2)
                - 2.0 * (other[e1234] * other[e3215]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e12345
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e12345]]),
        );
    }
}
impl GeometricAntiQuotient<DualNum> for CircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        1
    //  no simd        0       11        1
    fn geometric_anti_quotient(self, other: DualNum) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e12345], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e12345
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e12345]]),
        );
    }
}
impl GeometricAntiQuotient<FlatPoint> for CircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        4        1
    //  no simd        0       12        1
    fn geometric_anti_quotient(self, other: FlatPoint) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e45], 2) * -1.0);
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e12345
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e12345]]),
        );
    }
}
impl GeometricAntiQuotient<Flector> for CircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        3        1
    //  no simd        3       11        1
    fn geometric_anti_quotient(self, other: Flector) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e4235], 2) + f32::powi(other[e4315], 2) + f32::powi(other[e4125], 2) - f32::powi(other[e45], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e12345
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e12345]]),
        );
    }
}
impl GeometricAntiQuotient<Line> for CircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        2        3        1
    //  no simd        2       11        1
    fn geometric_anti_quotient(self, other: Line) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ -f32::powi(other[e415], 2) - f32::powi(other[e425], 2) - f32::powi(other[e435], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e12345
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e12345]]),
        );
    }
}
impl GeometricAntiQuotient<Motor> for CircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        3        1
    //  no simd        3       11        1
    fn geometric_anti_quotient(self, other: Motor) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e12345], 2) - f32::powi(other[e415], 2) - f32::powi(other[e425], 2) - f32::powi(other[e435], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e12345
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e12345]]),
        );
    }
}
impl GeometricAntiQuotient<MultiVector> for CircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       23       16        1
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       23       19        1
    //  no simd       23       27        1
    fn geometric_anti_quotient(self, other: MultiVector) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e4] * other[e5])
                + 2.0 * (other[e15] * other[e41])
                + 2.0 * (other[e25] * other[e42])
                + 2.0 * (other[e35] * other[e43])
                + f32::powi(other[e12345], 2)
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                + f32::powi(other[e321], 2)
                + f32::powi(other[e4235], 2)
                + f32::powi(other[e4315], 2)
                + f32::powi(other[e4125], 2)
                - f32::powi(other[scalar], 2)
                - f32::powi(other[e1], 2)
                - f32::powi(other[e2], 2)
                - f32::powi(other[e3], 2)
                - f32::powi(other[e45], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125])
                - 2.0 * (other[e3215] * other[e1234]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e12345
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e12345]]),
        );
    }
}
impl GeometricAntiQuotient<Plane> for CircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        2        3        1
    //  no simd        2       11        1
    fn geometric_anti_quotient(self, other: Plane) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e4235], 2) + f32::powi(other[e4315], 2) + f32::powi(other[e4125], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e12345
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e12345]]),
        );
    }
}
impl GeometricAntiQuotient<RoundPoint> for CircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        2        1
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        5        1
    //  no simd        3       13        1
    fn geometric_anti_quotient(self, other: RoundPoint) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e4] * other[e5]) - f32::powi(other[e1], 2) - f32::powi(other[e2], 2) - f32::powi(other[e3], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e12345
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e12345]]),
        );
    }
}
impl GeometricAntiQuotient<Scalar> for CircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        4        1
    //  no simd        0       12        1
    fn geometric_anti_quotient(self, other: Scalar) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[scalar], 2) * -1.0);
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e12345
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e12345]]),
        );
    }
}
impl GeometricAntiQuotient<Sphere> for CircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        2        1
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        5        1
    //  no simd        3       13        1
    fn geometric_anti_quotient(self, other: Sphere) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e4235], 2) + f32::powi(other[e4315], 2) + f32::powi(other[e4125], 2) - 2.0 * (other[e3215] * other[e1234]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e12345
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e12345]]),
        );
    }
}
impl GeometricAntiQuotient<VersorEven> for CircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11        8        1
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       11       11        1
    //  no simd       11       19        1
    fn geometric_anti_quotient(self, other: VersorEven) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e5] * other[e4]) + f32::powi(other[e12345], 2) + f32::powi(other[e321], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - f32::powi(other[e1], 2)
                - f32::powi(other[e2], 2)
                - f32::powi(other[e3], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e12345
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e12345]]),
        );
    }
}
impl GeometricAntiQuotient<VersorOdd> for CircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11        8        1
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       11       11        1
    //  no simd       11       19        1
    fn geometric_anti_quotient(self, other: VersorOdd) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e41] * other[e15])
                + 2.0 * (other[e42] * other[e25])
                + 2.0 * (other[e43] * other[e35])
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                + f32::powi(other[e4235], 2)
                + f32::powi(other[e4315], 2)
                + f32::powi(other[e4125], 2)
                - f32::powi(other[scalar], 2)
                - f32::powi(other[e45], 2)
                - 2.0 * (other[e1234] * other[e3215]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e12345
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e12345]]),
        );
    }
}
impl std::ops::Div<geometric_anti_quotient> for Dipole {
    type Output = geometric_anti_quotient_partial<Dipole>;
    fn div(self, _rhs: geometric_anti_quotient) -> Self::Output {
        geometric_anti_quotient_partial(self)
    }
}
impl GeometricAntiQuotient<AntiCircleRotor> for Dipole {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        6        1
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        7        9        1
    //  no simd        7       16        1
    fn geometric_anti_quotient(self, other: AntiCircleRotor) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e41] * other[e15])
                + 2.0 * (other[e42] * other[e25])
                + 2.0 * (other[e43] * other[e35])
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                - f32::powi(other[e45], 2)
                - f32::powi(other[scalar], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e15], self[e25], self[e35]]),
        );
    }
}
impl GeometricAntiQuotient<AntiDipoleInversion> for Dipole {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10        8        1
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       10       11        1
    //  no simd       10       18        1
    fn geometric_anti_quotient(self, other: AntiDipoleInversion) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e4] * other[e5]) + f32::powi(other[e321], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - f32::powi(other[e1], 2)
                - f32::powi(other[e2], 2)
                - f32::powi(other[e3], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e15], self[e25], self[e35]]),
        );
    }
}
impl GeometricAntiQuotient<AntiDualNum> for Dipole {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        4        1
    //  no simd        0       11        1
    fn geometric_anti_quotient(self, other: AntiDualNum) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[scalar], 2) * -1.0);
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e15], self[e25], self[e35]]),
        );
    }
}
impl GeometricAntiQuotient<AntiFlatPoint> for Dipole {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        1
    //  no simd        0       10        1
    fn geometric_anti_quotient(self, other: AntiFlatPoint) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e321], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e15], self[e25], self[e35]]),
        );
    }
}
impl GeometricAntiQuotient<AntiFlector> for Dipole {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        3        1
    //  no simd        3       10        1
    fn geometric_anti_quotient(self, other: AntiFlector) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e321], 2) - f32::powi(other[e1], 2) - f32::powi(other[e2], 2) - f32::powi(other[e3], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e15], self[e25], self[e35]]),
        );
    }
}
impl GeometricAntiQuotient<AntiLine> for Dipole {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        3        1
    //  no simd        2       10        1
    fn geometric_anti_quotient(self, other: AntiLine) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e23], 2) + f32::powi(other[e31], 2) + f32::powi(other[e12], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e15], self[e25], self[e35]]),
        );
    }
}
impl GeometricAntiQuotient<AntiMotor> for Dipole {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        3        1
    //  no simd        3       10        1
    fn geometric_anti_quotient(self, other: AntiMotor) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e23], 2) + f32::powi(other[e31], 2) + f32::powi(other[e12], 2) - f32::powi(other[scalar], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e15], self[e25], self[e35]]),
        );
    }
}
impl GeometricAntiQuotient<AntiPlane> for Dipole {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        3        1
    //  no simd        2       10        1
    fn geometric_anti_quotient(self, other: AntiPlane) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ -f32::powi(other[e1], 2) - f32::powi(other[e2], 2) - f32::powi(other[e3], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e15], self[e25], self[e35]]),
        );
    }
}
impl GeometricAntiQuotient<AntiScalar> for Dipole {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        1
    //  no simd        0       10        1
    fn geometric_anti_quotient(self, other: AntiScalar) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e12345], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e15], self[e25], self[e35]]),
        );
    }
}
impl GeometricAntiQuotient<Circle> for Dipole {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        6        1
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        6        9        1
    //  no simd        6       16        1
    fn geometric_anti_quotient(self, other: Circle) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e321], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e15], self[e25], self[e35]]),
        );
    }
}
impl GeometricAntiQuotient<CircleRotor> for Dipole {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        6        1
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        7        9        1
    //  no simd        7       16        1
    fn geometric_anti_quotient(self, other: CircleRotor) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e321], 2) + f32::powi(other[e12345], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e15], self[e25], self[e35]]),
        );
    }
}
impl GeometricAntiQuotient<Dipole> for Dipole {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        6        1
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        6        9        1
    //  no simd        6       16        1
    fn geometric_anti_quotient(self, other: Dipole) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e41] * other[e15])
                + 2.0 * (other[e42] * other[e25])
                + 2.0 * (other[e43] * other[e35])
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                - f32::powi(other[e45], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e15], self[e25], self[e35]]),
        );
    }
}
impl GeometricAntiQuotient<DipoleInversion> for Dipole {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10        8        1
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       10       11        1
    //  no simd       10       18        1
    fn geometric_anti_quotient(self, other: DipoleInversion) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e41] * other[e15])
                + 2.0 * (other[e42] * other[e25])
                + 2.0 * (other[e43] * other[e35])
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                + f32::powi(other[e4235], 2)
                + f32::powi(other[e4315], 2)
                + f32::powi(other[e4125], 2)
                - f32::powi(other[e45], 2)
                - 2.0 * (other[e1234] * other[e3215]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e15], self[e25], self[e35]]),
        );
    }
}
impl GeometricAntiQuotient<DualNum> for Dipole {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        1
    //  no simd        0       10        1
    fn geometric_anti_quotient(self, other: DualNum) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e12345], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e15], self[e25], self[e35]]),
        );
    }
}
impl GeometricAntiQuotient<FlatPoint> for Dipole {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        4        1
    //  no simd        0       11        1
    fn geometric_anti_quotient(self, other: FlatPoint) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e45], 2) * -1.0);
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e15], self[e25], self[e35]]),
        );
    }
}
impl GeometricAntiQuotient<Flector> for Dipole {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        3        1
    //  no simd        3       10        1
    fn geometric_anti_quotient(self, other: Flector) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e4235], 2) + f32::powi(other[e4315], 2) + f32::powi(other[e4125], 2) - f32::powi(other[e45], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e15], self[e25], self[e35]]),
        );
    }
}
impl GeometricAntiQuotient<Line> for Dipole {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        3        1
    //  no simd        2       10        1
    fn geometric_anti_quotient(self, other: Line) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ -f32::powi(other[e415], 2) - f32::powi(other[e425], 2) - f32::powi(other[e435], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e15], self[e25], self[e35]]),
        );
    }
}
impl GeometricAntiQuotient<Motor> for Dipole {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        3        1
    //  no simd        3       10        1
    fn geometric_anti_quotient(self, other: Motor) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e12345], 2) - f32::powi(other[e415], 2) - f32::powi(other[e425], 2) - f32::powi(other[e435], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e15], self[e25], self[e35]]),
        );
    }
}
impl GeometricAntiQuotient<MultiVector> for Dipole {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       23       16        1
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       23       19        1
    //  no simd       23       26        1
    fn geometric_anti_quotient(self, other: MultiVector) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e4] * other[e5])
                + 2.0 * (other[e15] * other[e41])
                + 2.0 * (other[e25] * other[e42])
                + 2.0 * (other[e35] * other[e43])
                + f32::powi(other[e12345], 2)
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                + f32::powi(other[e321], 2)
                + f32::powi(other[e4235], 2)
                + f32::powi(other[e4315], 2)
                + f32::powi(other[e4125], 2)
                - f32::powi(other[scalar], 2)
                - f32::powi(other[e1], 2)
                - f32::powi(other[e2], 2)
                - f32::powi(other[e3], 2)
                - f32::powi(other[e45], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125])
                - 2.0 * (other[e3215] * other[e1234]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e15], self[e25], self[e35]]),
        );
    }
}
impl GeometricAntiQuotient<Plane> for Dipole {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        3        1
    //  no simd        2       10        1
    fn geometric_anti_quotient(self, other: Plane) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e4235], 2) + f32::powi(other[e4315], 2) + f32::powi(other[e4125], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e15], self[e25], self[e35]]),
        );
    }
}
impl GeometricAntiQuotient<RoundPoint> for Dipole {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        2        1
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        1
    //  no simd        3       12        1
    fn geometric_anti_quotient(self, other: RoundPoint) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e4] * other[e5]) - f32::powi(other[e1], 2) - f32::powi(other[e2], 2) - f32::powi(other[e3], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e15], self[e25], self[e35]]),
        );
    }
}
impl GeometricAntiQuotient<Scalar> for Dipole {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        4        1
    //  no simd        0       11        1
    fn geometric_anti_quotient(self, other: Scalar) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[scalar], 2) * -1.0);
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e15], self[e25], self[e35]]),
        );
    }
}
impl GeometricAntiQuotient<Sphere> for Dipole {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        2        1
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        1
    //  no simd        3       12        1
    fn geometric_anti_quotient(self, other: Sphere) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e4235], 2) + f32::powi(other[e4315], 2) + f32::powi(other[e4125], 2) - 2.0 * (other[e3215] * other[e1234]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e15], self[e25], self[e35]]),
        );
    }
}
impl GeometricAntiQuotient<VersorEven> for Dipole {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11        8        1
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       11       11        1
    //  no simd       11       18        1
    fn geometric_anti_quotient(self, other: VersorEven) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e5] * other[e4]) + f32::powi(other[e12345], 2) + f32::powi(other[e321], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - f32::powi(other[e1], 2)
                - f32::powi(other[e2], 2)
                - f32::powi(other[e3], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e15], self[e25], self[e35]]),
        );
    }
}
impl GeometricAntiQuotient<VersorOdd> for Dipole {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11        8        1
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       11       11        1
    //  no simd       11       18        1
    fn geometric_anti_quotient(self, other: VersorOdd) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e41] * other[e15])
                + 2.0 * (other[e42] * other[e25])
                + 2.0 * (other[e43] * other[e35])
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                + f32::powi(other[e4235], 2)
                + f32::powi(other[e4315], 2)
                + f32::powi(other[e4125], 2)
                - f32::powi(other[scalar], 2)
                - f32::powi(other[e45], 2)
                - 2.0 * (other[e1234] * other[e3215]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e15], self[e25], self[e35]]),
        );
    }
}
impl std::ops::Div<geometric_anti_quotient> for DipoleInversion {
    type Output = geometric_anti_quotient_partial<DipoleInversion>;
    fn div(self, _rhs: geometric_anti_quotient) -> Self::Output {
        geometric_anti_quotient_partial(self)
    }
}
impl GeometricAntiQuotient<AntiCircleRotor> for DipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        6        1
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        7       10        1
    //  no simd        7       21        1
    fn geometric_anti_quotient(self, other: AntiCircleRotor) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e41] * other[e15])
                + 2.0 * (other[e42] * other[e25])
                + 2.0 * (other[e43] * other[e35])
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                - f32::powi(other[e45], 2)
                - f32::powi(other[scalar], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<AntiDipoleInversion> for DipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10        8        1
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd       10       12        1
    //  no simd       10       23        1
    fn geometric_anti_quotient(self, other: AntiDipoleInversion) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e4] * other[e5]) + f32::powi(other[e321], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - f32::powi(other[e1], 2)
                - f32::powi(other[e2], 2)
                - f32::powi(other[e3], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<AntiDualNum> for DipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        5        1
    //  no simd        0       16        1
    fn geometric_anti_quotient(self, other: AntiDualNum) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[scalar], 2) * -1.0);
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<AntiFlatPoint> for DipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        4        1
    //  no simd        0       15        1
    fn geometric_anti_quotient(self, other: AntiFlatPoint) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e321], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<AntiFlector> for DipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        3        4        1
    //  no simd        3       15        1
    fn geometric_anti_quotient(self, other: AntiFlector) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e321], 2) - f32::powi(other[e1], 2) - f32::powi(other[e2], 2) - f32::powi(other[e3], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<AntiLine> for DipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        2        4        1
    //  no simd        2       15        1
    fn geometric_anti_quotient(self, other: AntiLine) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e23], 2) + f32::powi(other[e31], 2) + f32::powi(other[e12], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<AntiMotor> for DipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        3        4        1
    //  no simd        3       15        1
    fn geometric_anti_quotient(self, other: AntiMotor) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e23], 2) + f32::powi(other[e31], 2) + f32::powi(other[e12], 2) - f32::powi(other[scalar], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<AntiPlane> for DipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        2        4        1
    //  no simd        2       15        1
    fn geometric_anti_quotient(self, other: AntiPlane) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ -f32::powi(other[e1], 2) - f32::powi(other[e2], 2) - f32::powi(other[e3], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<AntiScalar> for DipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        4        1
    //  no simd        0       15        1
    fn geometric_anti_quotient(self, other: AntiScalar) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e12345], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<Circle> for DipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        6        1
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        6       10        1
    //  no simd        6       21        1
    fn geometric_anti_quotient(self, other: Circle) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e321], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<CircleRotor> for DipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        6        1
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        7       10        1
    //  no simd        7       21        1
    fn geometric_anti_quotient(self, other: CircleRotor) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e321], 2) + f32::powi(other[e12345], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<Dipole> for DipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        6        1
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        6       10        1
    //  no simd        6       21        1
    fn geometric_anti_quotient(self, other: Dipole) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e41] * other[e15])
                + 2.0 * (other[e42] * other[e25])
                + 2.0 * (other[e43] * other[e35])
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                - f32::powi(other[e45], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<DipoleInversion> for DipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10        8        1
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd       10       12        1
    //  no simd       10       23        1
    fn geometric_anti_quotient(self, other: DipoleInversion) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e41] * other[e15])
                + 2.0 * (other[e42] * other[e25])
                + 2.0 * (other[e43] * other[e35])
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                + f32::powi(other[e4235], 2)
                + f32::powi(other[e4315], 2)
                + f32::powi(other[e4125], 2)
                - f32::powi(other[e45], 2)
                - 2.0 * (other[e1234] * other[e3215]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<DualNum> for DipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        4        1
    //  no simd        0       15        1
    fn geometric_anti_quotient(self, other: DualNum) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e12345], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<FlatPoint> for DipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        5        1
    //  no simd        0       16        1
    fn geometric_anti_quotient(self, other: FlatPoint) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e45], 2) * -1.0);
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<Flector> for DipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        3        4        1
    //  no simd        3       15        1
    fn geometric_anti_quotient(self, other: Flector) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e4235], 2) + f32::powi(other[e4315], 2) + f32::powi(other[e4125], 2) - f32::powi(other[e45], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<Line> for DipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        2        4        1
    //  no simd        2       15        1
    fn geometric_anti_quotient(self, other: Line) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ -f32::powi(other[e415], 2) - f32::powi(other[e425], 2) - f32::powi(other[e435], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<Motor> for DipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        3        4        1
    //  no simd        3       15        1
    fn geometric_anti_quotient(self, other: Motor) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e12345], 2) - f32::powi(other[e415], 2) - f32::powi(other[e425], 2) - f32::powi(other[e435], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<MultiVector> for DipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       23       16        1
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd       23       20        1
    //  no simd       23       31        1
    fn geometric_anti_quotient(self, other: MultiVector) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e4] * other[e5])
                + 2.0 * (other[e15] * other[e41])
                + 2.0 * (other[e25] * other[e42])
                + 2.0 * (other[e35] * other[e43])
                + f32::powi(other[e12345], 2)
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                + f32::powi(other[e321], 2)
                + f32::powi(other[e4235], 2)
                + f32::powi(other[e4315], 2)
                + f32::powi(other[e4125], 2)
                - f32::powi(other[scalar], 2)
                - f32::powi(other[e1], 2)
                - f32::powi(other[e2], 2)
                - f32::powi(other[e3], 2)
                - f32::powi(other[e45], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125])
                - 2.0 * (other[e3215] * other[e1234]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<Plane> for DipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        2        4        1
    //  no simd        2       15        1
    fn geometric_anti_quotient(self, other: Plane) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e4235], 2) + f32::powi(other[e4315], 2) + f32::powi(other[e4125], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<RoundPoint> for DipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        2        1
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        3        6        1
    //  no simd        3       17        1
    fn geometric_anti_quotient(self, other: RoundPoint) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e4] * other[e5]) - f32::powi(other[e1], 2) - f32::powi(other[e2], 2) - f32::powi(other[e3], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<Scalar> for DipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        5        1
    //  no simd        0       16        1
    fn geometric_anti_quotient(self, other: Scalar) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[scalar], 2) * -1.0);
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<Sphere> for DipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        2        1
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        3        6        1
    //  no simd        3       17        1
    fn geometric_anti_quotient(self, other: Sphere) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e4235], 2) + f32::powi(other[e4315], 2) + f32::powi(other[e4125], 2) - 2.0 * (other[e3215] * other[e1234]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<VersorEven> for DipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11        8        1
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd       11       12        1
    //  no simd       11       23        1
    fn geometric_anti_quotient(self, other: VersorEven) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e5] * other[e4]) + f32::powi(other[e12345], 2) + f32::powi(other[e321], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - f32::powi(other[e1], 2)
                - f32::powi(other[e2], 2)
                - f32::powi(other[e3], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<VersorOdd> for DipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11        8        1
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd       11       12        1
    //  no simd       11       23        1
    fn geometric_anti_quotient(self, other: VersorOdd) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e41] * other[e15])
                + 2.0 * (other[e42] * other[e25])
                + 2.0 * (other[e43] * other[e35])
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                + f32::powi(other[e4235], 2)
                + f32::powi(other[e4315], 2)
                + f32::powi(other[e4125], 2)
                - f32::powi(other[scalar], 2)
                - f32::powi(other[e45], 2)
                - 2.0 * (other[e1234] * other[e3215]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl std::ops::Div<geometric_anti_quotient> for DualNum {
    type Output = geometric_anti_quotient_partial<DualNum>;
    fn div(self, _rhs: geometric_anti_quotient) -> Self::Output {
        geometric_anti_quotient_partial(self)
    }
}
impl GeometricAntiQuotient<AntiCircleRotor> for DualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        6        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        7        7        1
    //  no simd        7        8        1
    fn geometric_anti_quotient(self, other: AntiCircleRotor) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e41] * other[e15])
                + 2.0 * (other[e42] * other[e25])
                + 2.0 * (other[e43] * other[e35])
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                - f32::powi(other[e45], 2)
                - f32::powi(other[scalar], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return DualNum::from_groups(/* e4, e12345 */ Simd32x2::from(anti_inverse[e12345]) * Simd32x2::from([self[e4], self[e12345]]));
    }
}
impl GeometricAntiQuotient<AntiDipoleInversion> for DualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10        8        1
    //    simd2        0        1        0
    // Totals...
    // yes simd       10        9        1
    //  no simd       10       10        1
    fn geometric_anti_quotient(self, other: AntiDipoleInversion) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e4] * other[e5]) + f32::powi(other[e321], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - f32::powi(other[e1], 2)
                - f32::powi(other[e2], 2)
                - f32::powi(other[e3], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return DualNum::from_groups(/* e4, e12345 */ Simd32x2::from(anti_inverse[e12345]) * Simd32x2::from([self[e4], self[e12345]]));
    }
}
impl GeometricAntiQuotient<AntiDualNum> for DualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        3        1
    fn geometric_anti_quotient(self, other: AntiDualNum) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[scalar], 2) * -1.0);
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return DualNum::from_groups(/* e4, e12345 */ Simd32x2::from(anti_inverse[e12345]) * Simd32x2::from([self[e4], self[e12345]]));
    }
}
impl GeometricAntiQuotient<AntiFlatPoint> for DualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        1        1
    //  no simd        0        2        1
    fn geometric_anti_quotient(self, other: AntiFlatPoint) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e321], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return DualNum::from_groups(/* e4, e12345 */ Simd32x2::from(anti_inverse[e12345]) * Simd32x2::from([self[e4], self[e12345]]));
    }
}
impl GeometricAntiQuotient<AntiFlector> for DualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        3        1        1
    //  no simd        3        2        1
    fn geometric_anti_quotient(self, other: AntiFlector) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e321], 2) - f32::powi(other[e1], 2) - f32::powi(other[e2], 2) - f32::powi(other[e3], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return DualNum::from_groups(/* e4, e12345 */ Simd32x2::from(anti_inverse[e12345]) * Simd32x2::from([self[e4], self[e12345]]));
    }
}
impl GeometricAntiQuotient<AntiLine> for DualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        2        1        1
    //  no simd        2        2        1
    fn geometric_anti_quotient(self, other: AntiLine) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e23], 2) + f32::powi(other[e31], 2) + f32::powi(other[e12], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return DualNum::from_groups(/* e4, e12345 */ Simd32x2::from(anti_inverse[e12345]) * Simd32x2::from([self[e4], self[e12345]]));
    }
}
impl GeometricAntiQuotient<AntiMotor> for DualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        3        1        1
    //  no simd        3        2        1
    fn geometric_anti_quotient(self, other: AntiMotor) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e23], 2) + f32::powi(other[e31], 2) + f32::powi(other[e12], 2) - f32::powi(other[scalar], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return DualNum::from_groups(/* e4, e12345 */ Simd32x2::from(anti_inverse[e12345]) * Simd32x2::from([self[e4], self[e12345]]));
    }
}
impl GeometricAntiQuotient<AntiPlane> for DualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        2        1        1
    //  no simd        2        2        1
    fn geometric_anti_quotient(self, other: AntiPlane) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ -f32::powi(other[e1], 2) - f32::powi(other[e2], 2) - f32::powi(other[e3], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return DualNum::from_groups(/* e4, e12345 */ Simd32x2::from(anti_inverse[e12345]) * Simd32x2::from([self[e4], self[e12345]]));
    }
}
impl GeometricAntiQuotient<AntiScalar> for DualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        1        1
    //  no simd        0        2        1
    fn geometric_anti_quotient(self, other: AntiScalar) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e12345], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return DualNum::from_groups(/* e4, e12345 */ Simd32x2::from(anti_inverse[e12345]) * Simd32x2::from([self[e4], self[e12345]]));
    }
}
impl GeometricAntiQuotient<Circle> for DualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        6        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        6        7        1
    //  no simd        6        8        1
    fn geometric_anti_quotient(self, other: Circle) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e321], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return DualNum::from_groups(/* e4, e12345 */ Simd32x2::from(anti_inverse[e12345]) * Simd32x2::from([self[e4], self[e12345]]));
    }
}
impl GeometricAntiQuotient<CircleRotor> for DualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        6        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        7        7        1
    //  no simd        7        8        1
    fn geometric_anti_quotient(self, other: CircleRotor) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e321], 2) + f32::powi(other[e12345], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return DualNum::from_groups(/* e4, e12345 */ Simd32x2::from(anti_inverse[e12345]) * Simd32x2::from([self[e4], self[e12345]]));
    }
}
impl GeometricAntiQuotient<Dipole> for DualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        6        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        6        7        1
    //  no simd        6        8        1
    fn geometric_anti_quotient(self, other: Dipole) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e41] * other[e15])
                + 2.0 * (other[e42] * other[e25])
                + 2.0 * (other[e43] * other[e35])
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                - f32::powi(other[e45], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return DualNum::from_groups(/* e4, e12345 */ Simd32x2::from(anti_inverse[e12345]) * Simd32x2::from([self[e4], self[e12345]]));
    }
}
impl GeometricAntiQuotient<DipoleInversion> for DualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10        8        1
    //    simd2        0        1        0
    // Totals...
    // yes simd       10        9        1
    //  no simd       10       10        1
    fn geometric_anti_quotient(self, other: DipoleInversion) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e41] * other[e15])
                + 2.0 * (other[e42] * other[e25])
                + 2.0 * (other[e43] * other[e35])
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                + f32::powi(other[e4235], 2)
                + f32::powi(other[e4315], 2)
                + f32::powi(other[e4125], 2)
                - f32::powi(other[e45], 2)
                - 2.0 * (other[e1234] * other[e3215]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return DualNum::from_groups(/* e4, e12345 */ Simd32x2::from(anti_inverse[e12345]) * Simd32x2::from([self[e4], self[e12345]]));
    }
}
impl GeometricAntiQuotient<DualNum> for DualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        1        1
    //  no simd        0        2        1
    fn geometric_anti_quotient(self, other: DualNum) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e12345], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return DualNum::from_groups(/* e4, e12345 */ Simd32x2::from(anti_inverse[e12345]) * Simd32x2::from([self[e4], self[e12345]]));
    }
}
impl GeometricAntiQuotient<FlatPoint> for DualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        3        1
    fn geometric_anti_quotient(self, other: FlatPoint) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e45], 2) * -1.0);
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return DualNum::from_groups(/* e4, e12345 */ Simd32x2::from(anti_inverse[e12345]) * Simd32x2::from([self[e4], self[e12345]]));
    }
}
impl GeometricAntiQuotient<Flector> for DualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        3        1        1
    //  no simd        3        2        1
    fn geometric_anti_quotient(self, other: Flector) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e4235], 2) + f32::powi(other[e4315], 2) + f32::powi(other[e4125], 2) - f32::powi(other[e45], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return DualNum::from_groups(/* e4, e12345 */ Simd32x2::from(anti_inverse[e12345]) * Simd32x2::from([self[e4], self[e12345]]));
    }
}
impl GeometricAntiQuotient<Line> for DualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        2        1        1
    //  no simd        2        2        1
    fn geometric_anti_quotient(self, other: Line) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ -f32::powi(other[e415], 2) - f32::powi(other[e425], 2) - f32::powi(other[e435], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return DualNum::from_groups(/* e4, e12345 */ Simd32x2::from(anti_inverse[e12345]) * Simd32x2::from([self[e4], self[e12345]]));
    }
}
impl GeometricAntiQuotient<Motor> for DualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        3        1        1
    //  no simd        3        2        1
    fn geometric_anti_quotient(self, other: Motor) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e12345], 2) - f32::powi(other[e415], 2) - f32::powi(other[e425], 2) - f32::powi(other[e435], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return DualNum::from_groups(/* e4, e12345 */ Simd32x2::from(anti_inverse[e12345]) * Simd32x2::from([self[e4], self[e12345]]));
    }
}
impl GeometricAntiQuotient<MultiVector> for DualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       23       16        1
    //    simd2        0        1        0
    // Totals...
    // yes simd       23       17        1
    //  no simd       23       18        1
    fn geometric_anti_quotient(self, other: MultiVector) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e4] * other[e5])
                + 2.0 * (other[e15] * other[e41])
                + 2.0 * (other[e25] * other[e42])
                + 2.0 * (other[e35] * other[e43])
                + f32::powi(other[e12345], 2)
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                + f32::powi(other[e321], 2)
                + f32::powi(other[e4235], 2)
                + f32::powi(other[e4315], 2)
                + f32::powi(other[e4125], 2)
                - f32::powi(other[scalar], 2)
                - f32::powi(other[e1], 2)
                - f32::powi(other[e2], 2)
                - f32::powi(other[e3], 2)
                - f32::powi(other[e45], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125])
                - 2.0 * (other[e3215] * other[e1234]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return DualNum::from_groups(/* e4, e12345 */ Simd32x2::from(anti_inverse[e12345]) * Simd32x2::from([self[e4], self[e12345]]));
    }
}
impl GeometricAntiQuotient<Plane> for DualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        2        1        1
    //  no simd        2        2        1
    fn geometric_anti_quotient(self, other: Plane) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e4235], 2) + f32::powi(other[e4315], 2) + f32::powi(other[e4125], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return DualNum::from_groups(/* e4, e12345 */ Simd32x2::from(anti_inverse[e12345]) * Simd32x2::from([self[e4], self[e12345]]));
    }
}
impl GeometricAntiQuotient<RoundPoint> for DualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        2        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        3        3        1
    //  no simd        3        4        1
    fn geometric_anti_quotient(self, other: RoundPoint) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e4] * other[e5]) - f32::powi(other[e1], 2) - f32::powi(other[e2], 2) - f32::powi(other[e3], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return DualNum::from_groups(/* e4, e12345 */ Simd32x2::from(anti_inverse[e12345]) * Simd32x2::from([self[e4], self[e12345]]));
    }
}
impl GeometricAntiQuotient<Scalar> for DualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        3        1
    fn geometric_anti_quotient(self, other: Scalar) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[scalar], 2) * -1.0);
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return DualNum::from_groups(/* e4, e12345 */ Simd32x2::from(anti_inverse[e12345]) * Simd32x2::from([self[e4], self[e12345]]));
    }
}
impl GeometricAntiQuotient<Sphere> for DualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        2        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        3        3        1
    //  no simd        3        4        1
    fn geometric_anti_quotient(self, other: Sphere) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e4235], 2) + f32::powi(other[e4315], 2) + f32::powi(other[e4125], 2) - 2.0 * (other[e3215] * other[e1234]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return DualNum::from_groups(/* e4, e12345 */ Simd32x2::from(anti_inverse[e12345]) * Simd32x2::from([self[e4], self[e12345]]));
    }
}
impl GeometricAntiQuotient<VersorEven> for DualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11        8        1
    //    simd2        0        1        0
    // Totals...
    // yes simd       11        9        1
    //  no simd       11       10        1
    fn geometric_anti_quotient(self, other: VersorEven) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e5] * other[e4]) + f32::powi(other[e12345], 2) + f32::powi(other[e321], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - f32::powi(other[e1], 2)
                - f32::powi(other[e2], 2)
                - f32::powi(other[e3], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return DualNum::from_groups(/* e4, e12345 */ Simd32x2::from(anti_inverse[e12345]) * Simd32x2::from([self[e4], self[e12345]]));
    }
}
impl GeometricAntiQuotient<VersorOdd> for DualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11        8        1
    //    simd2        0        1        0
    // Totals...
    // yes simd       11        9        1
    //  no simd       11       10        1
    fn geometric_anti_quotient(self, other: VersorOdd) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e41] * other[e15])
                + 2.0 * (other[e42] * other[e25])
                + 2.0 * (other[e43] * other[e35])
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                + f32::powi(other[e4235], 2)
                + f32::powi(other[e4315], 2)
                + f32::powi(other[e4125], 2)
                - f32::powi(other[scalar], 2)
                - f32::powi(other[e45], 2)
                - 2.0 * (other[e1234] * other[e3215]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return DualNum::from_groups(/* e4, e12345 */ Simd32x2::from(anti_inverse[e12345]) * Simd32x2::from([self[e4], self[e12345]]));
    }
}
impl std::ops::Div<geometric_anti_quotient> for FlatPoint {
    type Output = geometric_anti_quotient_partial<FlatPoint>;
    fn div(self, _rhs: geometric_anti_quotient) -> Self::Output {
        geometric_anti_quotient_partial(self)
    }
}
impl GeometricAntiQuotient<AntiCircleRotor> for FlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        6        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        7        7        1
    //  no simd        7       10        1
    fn geometric_anti_quotient(self, other: AntiCircleRotor) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e41] * other[e15])
                + 2.0 * (other[e42] * other[e25])
                + 2.0 * (other[e43] * other[e35])
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                - f32::powi(other[e45], 2)
                - f32::powi(other[scalar], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return FlatPoint::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
        );
    }
}
impl GeometricAntiQuotient<AntiDipoleInversion> for FlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10        8        1
    //    simd4        0        1        0
    // Totals...
    // yes simd       10        9        1
    //  no simd       10       12        1
    fn geometric_anti_quotient(self, other: AntiDipoleInversion) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e4] * other[e5]) + f32::powi(other[e321], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - f32::powi(other[e1], 2)
                - f32::powi(other[e2], 2)
                - f32::powi(other[e3], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return FlatPoint::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
        );
    }
}
impl GeometricAntiQuotient<AntiDualNum> for FlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        5        1
    fn geometric_anti_quotient(self, other: AntiDualNum) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[scalar], 2) * -1.0);
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return FlatPoint::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
        );
    }
}
impl GeometricAntiQuotient<AntiFlatPoint> for FlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        1        1
    //  no simd        0        4        1
    fn geometric_anti_quotient(self, other: AntiFlatPoint) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e321], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return FlatPoint::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
        );
    }
}
impl GeometricAntiQuotient<AntiFlector> for FlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        1        1
    //  no simd        3        4        1
    fn geometric_anti_quotient(self, other: AntiFlector) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e321], 2) - f32::powi(other[e1], 2) - f32::powi(other[e2], 2) - f32::powi(other[e3], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return FlatPoint::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
        );
    }
}
impl GeometricAntiQuotient<AntiLine> for FlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        1        1
    //  no simd        2        4        1
    fn geometric_anti_quotient(self, other: AntiLine) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e23], 2) + f32::powi(other[e31], 2) + f32::powi(other[e12], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return FlatPoint::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
        );
    }
}
impl GeometricAntiQuotient<AntiMotor> for FlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        1        1
    //  no simd        3        4        1
    fn geometric_anti_quotient(self, other: AntiMotor) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e23], 2) + f32::powi(other[e31], 2) + f32::powi(other[e12], 2) - f32::powi(other[scalar], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return FlatPoint::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
        );
    }
}
impl GeometricAntiQuotient<AntiPlane> for FlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        1        1
    //  no simd        2        4        1
    fn geometric_anti_quotient(self, other: AntiPlane) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ -f32::powi(other[e1], 2) - f32::powi(other[e2], 2) - f32::powi(other[e3], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return FlatPoint::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
        );
    }
}
impl GeometricAntiQuotient<AntiScalar> for FlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        1        1
    //  no simd        0        4        1
    fn geometric_anti_quotient(self, other: AntiScalar) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e12345], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return FlatPoint::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
        );
    }
}
impl GeometricAntiQuotient<Circle> for FlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        6        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        6        7        1
    //  no simd        6       10        1
    fn geometric_anti_quotient(self, other: Circle) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e321], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return FlatPoint::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
        );
    }
}
impl GeometricAntiQuotient<CircleRotor> for FlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        6        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        7        7        1
    //  no simd        7       10        1
    fn geometric_anti_quotient(self, other: CircleRotor) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e321], 2) + f32::powi(other[e12345], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return FlatPoint::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
        );
    }
}
impl GeometricAntiQuotient<Dipole> for FlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        6        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        6        7        1
    //  no simd        6       10        1
    fn geometric_anti_quotient(self, other: Dipole) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e41] * other[e15])
                + 2.0 * (other[e42] * other[e25])
                + 2.0 * (other[e43] * other[e35])
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                - f32::powi(other[e45], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return FlatPoint::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
        );
    }
}
impl GeometricAntiQuotient<DipoleInversion> for FlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10        8        1
    //    simd4        0        1        0
    // Totals...
    // yes simd       10        9        1
    //  no simd       10       12        1
    fn geometric_anti_quotient(self, other: DipoleInversion) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e41] * other[e15])
                + 2.0 * (other[e42] * other[e25])
                + 2.0 * (other[e43] * other[e35])
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                + f32::powi(other[e4235], 2)
                + f32::powi(other[e4315], 2)
                + f32::powi(other[e4125], 2)
                - f32::powi(other[e45], 2)
                - 2.0 * (other[e1234] * other[e3215]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return FlatPoint::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
        );
    }
}
impl GeometricAntiQuotient<DualNum> for FlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        1        1
    //  no simd        0        4        1
    fn geometric_anti_quotient(self, other: DualNum) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e12345], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return FlatPoint::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
        );
    }
}
impl GeometricAntiQuotient<FlatPoint> for FlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        5        1
    fn geometric_anti_quotient(self, other: FlatPoint) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e45], 2) * -1.0);
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return FlatPoint::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
        );
    }
}
impl GeometricAntiQuotient<Flector> for FlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        1        1
    //  no simd        3        4        1
    fn geometric_anti_quotient(self, other: Flector) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e4235], 2) + f32::powi(other[e4315], 2) + f32::powi(other[e4125], 2) - f32::powi(other[e45], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return FlatPoint::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
        );
    }
}
impl GeometricAntiQuotient<Line> for FlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        1        1
    //  no simd        2        4        1
    fn geometric_anti_quotient(self, other: Line) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ -f32::powi(other[e415], 2) - f32::powi(other[e425], 2) - f32::powi(other[e435], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return FlatPoint::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
        );
    }
}
impl GeometricAntiQuotient<Motor> for FlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        1        1
    //  no simd        3        4        1
    fn geometric_anti_quotient(self, other: Motor) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e12345], 2) - f32::powi(other[e415], 2) - f32::powi(other[e425], 2) - f32::powi(other[e435], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return FlatPoint::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
        );
    }
}
impl GeometricAntiQuotient<MultiVector> for FlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       23       16        1
    //    simd4        0        1        0
    // Totals...
    // yes simd       23       17        1
    //  no simd       23       20        1
    fn geometric_anti_quotient(self, other: MultiVector) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e4] * other[e5])
                + 2.0 * (other[e15] * other[e41])
                + 2.0 * (other[e25] * other[e42])
                + 2.0 * (other[e35] * other[e43])
                + f32::powi(other[e12345], 2)
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                + f32::powi(other[e321], 2)
                + f32::powi(other[e4235], 2)
                + f32::powi(other[e4315], 2)
                + f32::powi(other[e4125], 2)
                - f32::powi(other[scalar], 2)
                - f32::powi(other[e1], 2)
                - f32::powi(other[e2], 2)
                - f32::powi(other[e3], 2)
                - f32::powi(other[e45], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125])
                - 2.0 * (other[e3215] * other[e1234]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return FlatPoint::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
        );
    }
}
impl GeometricAntiQuotient<Plane> for FlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        1        1
    //  no simd        2        4        1
    fn geometric_anti_quotient(self, other: Plane) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e4235], 2) + f32::powi(other[e4315], 2) + f32::powi(other[e4125], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return FlatPoint::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
        );
    }
}
impl GeometricAntiQuotient<RoundPoint> for FlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        2        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        3        1
    //  no simd        3        6        1
    fn geometric_anti_quotient(self, other: RoundPoint) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e4] * other[e5]) - f32::powi(other[e1], 2) - f32::powi(other[e2], 2) - f32::powi(other[e3], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return FlatPoint::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
        );
    }
}
impl GeometricAntiQuotient<Scalar> for FlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        5        1
    fn geometric_anti_quotient(self, other: Scalar) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[scalar], 2) * -1.0);
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return FlatPoint::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
        );
    }
}
impl GeometricAntiQuotient<Sphere> for FlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        2        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        3        1
    //  no simd        3        6        1
    fn geometric_anti_quotient(self, other: Sphere) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e4235], 2) + f32::powi(other[e4315], 2) + f32::powi(other[e4125], 2) - 2.0 * (other[e3215] * other[e1234]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return FlatPoint::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
        );
    }
}
impl GeometricAntiQuotient<VersorEven> for FlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11        8        1
    //    simd4        0        1        0
    // Totals...
    // yes simd       11        9        1
    //  no simd       11       12        1
    fn geometric_anti_quotient(self, other: VersorEven) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e5] * other[e4]) + f32::powi(other[e12345], 2) + f32::powi(other[e321], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - f32::powi(other[e1], 2)
                - f32::powi(other[e2], 2)
                - f32::powi(other[e3], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return FlatPoint::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
        );
    }
}
impl GeometricAntiQuotient<VersorOdd> for FlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11        8        1
    //    simd4        0        1        0
    // Totals...
    // yes simd       11        9        1
    //  no simd       11       12        1
    fn geometric_anti_quotient(self, other: VersorOdd) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e41] * other[e15])
                + 2.0 * (other[e42] * other[e25])
                + 2.0 * (other[e43] * other[e35])
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                + f32::powi(other[e4235], 2)
                + f32::powi(other[e4315], 2)
                + f32::powi(other[e4125], 2)
                - f32::powi(other[scalar], 2)
                - f32::powi(other[e45], 2)
                - 2.0 * (other[e1234] * other[e3215]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return FlatPoint::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
        );
    }
}
impl std::ops::Div<geometric_anti_quotient> for Flector {
    type Output = geometric_anti_quotient_partial<Flector>;
    fn div(self, _rhs: geometric_anti_quotient) -> Self::Output {
        geometric_anti_quotient_partial(self)
    }
}
impl GeometricAntiQuotient<AntiCircleRotor> for Flector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        6        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        7        8        1
    //  no simd        7       14        1
    fn geometric_anti_quotient(self, other: AntiCircleRotor) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e41] * other[e15])
                + 2.0 * (other[e42] * other[e25])
                + 2.0 * (other[e43] * other[e35])
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                - f32::powi(other[e45], 2)
                - f32::powi(other[scalar], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<AntiDipoleInversion> for Flector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10        8        1
    //    simd4        0        2        0
    // Totals...
    // yes simd       10       10        1
    //  no simd       10       16        1
    fn geometric_anti_quotient(self, other: AntiDipoleInversion) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e4] * other[e5]) + f32::powi(other[e321], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - f32::powi(other[e1], 2)
                - f32::powi(other[e2], 2)
                - f32::powi(other[e3], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<AntiDualNum> for Flector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        1
    //  no simd        0        9        1
    fn geometric_anti_quotient(self, other: AntiDualNum) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[scalar], 2) * -1.0);
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<AntiFlatPoint> for Flector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        8        1
    fn geometric_anti_quotient(self, other: AntiFlatPoint) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e321], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<AntiFlector> for Flector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        2        1
    //  no simd        3        8        1
    fn geometric_anti_quotient(self, other: AntiFlector) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e321], 2) - f32::powi(other[e1], 2) - f32::powi(other[e2], 2) - f32::powi(other[e3], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<AntiLine> for Flector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        2        2        1
    //  no simd        2        8        1
    fn geometric_anti_quotient(self, other: AntiLine) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e23], 2) + f32::powi(other[e31], 2) + f32::powi(other[e12], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<AntiMotor> for Flector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        2        1
    //  no simd        3        8        1
    fn geometric_anti_quotient(self, other: AntiMotor) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e23], 2) + f32::powi(other[e31], 2) + f32::powi(other[e12], 2) - f32::powi(other[scalar], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<AntiPlane> for Flector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        2        2        1
    //  no simd        2        8        1
    fn geometric_anti_quotient(self, other: AntiPlane) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ -f32::powi(other[e1], 2) - f32::powi(other[e2], 2) - f32::powi(other[e3], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<AntiScalar> for Flector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        8        1
    fn geometric_anti_quotient(self, other: AntiScalar) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e12345], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<Circle> for Flector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        6        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        6        8        1
    //  no simd        6       14        1
    fn geometric_anti_quotient(self, other: Circle) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e321], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<CircleRotor> for Flector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        6        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        7        8        1
    //  no simd        7       14        1
    fn geometric_anti_quotient(self, other: CircleRotor) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e321], 2) + f32::powi(other[e12345], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<Dipole> for Flector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        6        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        6        8        1
    //  no simd        6       14        1
    fn geometric_anti_quotient(self, other: Dipole) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e41] * other[e15])
                + 2.0 * (other[e42] * other[e25])
                + 2.0 * (other[e43] * other[e35])
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                - f32::powi(other[e45], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<DipoleInversion> for Flector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10        8        1
    //    simd4        0        2        0
    // Totals...
    // yes simd       10       10        1
    //  no simd       10       16        1
    fn geometric_anti_quotient(self, other: DipoleInversion) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e41] * other[e15])
                + 2.0 * (other[e42] * other[e25])
                + 2.0 * (other[e43] * other[e35])
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                + f32::powi(other[e4235], 2)
                + f32::powi(other[e4315], 2)
                + f32::powi(other[e4125], 2)
                - f32::powi(other[e45], 2)
                - 2.0 * (other[e1234] * other[e3215]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<DualNum> for Flector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        8        1
    fn geometric_anti_quotient(self, other: DualNum) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e12345], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<FlatPoint> for Flector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        1
    //  no simd        0        9        1
    fn geometric_anti_quotient(self, other: FlatPoint) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e45], 2) * -1.0);
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<Flector> for Flector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        2        1
    //  no simd        3        8        1
    fn geometric_anti_quotient(self, other: Flector) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e4235], 2) + f32::powi(other[e4315], 2) + f32::powi(other[e4125], 2) - f32::powi(other[e45], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<Line> for Flector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        2        2        1
    //  no simd        2        8        1
    fn geometric_anti_quotient(self, other: Line) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ -f32::powi(other[e415], 2) - f32::powi(other[e425], 2) - f32::powi(other[e435], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<Motor> for Flector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        2        1
    //  no simd        3        8        1
    fn geometric_anti_quotient(self, other: Motor) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e12345], 2) - f32::powi(other[e415], 2) - f32::powi(other[e425], 2) - f32::powi(other[e435], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<MultiVector> for Flector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       23       16        1
    //    simd4        0        2        0
    // Totals...
    // yes simd       23       18        1
    //  no simd       23       24        1
    fn geometric_anti_quotient(self, other: MultiVector) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e4] * other[e5])
                + 2.0 * (other[e15] * other[e41])
                + 2.0 * (other[e25] * other[e42])
                + 2.0 * (other[e35] * other[e43])
                + f32::powi(other[e12345], 2)
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                + f32::powi(other[e321], 2)
                + f32::powi(other[e4235], 2)
                + f32::powi(other[e4315], 2)
                + f32::powi(other[e4125], 2)
                - f32::powi(other[scalar], 2)
                - f32::powi(other[e1], 2)
                - f32::powi(other[e2], 2)
                - f32::powi(other[e3], 2)
                - f32::powi(other[e45], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125])
                - 2.0 * (other[e3215] * other[e1234]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<Plane> for Flector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        2        2        1
    //  no simd        2        8        1
    fn geometric_anti_quotient(self, other: Plane) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e4235], 2) + f32::powi(other[e4315], 2) + f32::powi(other[e4125], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<RoundPoint> for Flector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        2        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        4        1
    //  no simd        3       10        1
    fn geometric_anti_quotient(self, other: RoundPoint) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e4] * other[e5]) - f32::powi(other[e1], 2) - f32::powi(other[e2], 2) - f32::powi(other[e3], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<Scalar> for Flector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        1
    //  no simd        0        9        1
    fn geometric_anti_quotient(self, other: Scalar) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[scalar], 2) * -1.0);
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<Sphere> for Flector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        2        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        4        1
    //  no simd        3       10        1
    fn geometric_anti_quotient(self, other: Sphere) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e4235], 2) + f32::powi(other[e4315], 2) + f32::powi(other[e4125], 2) - 2.0 * (other[e3215] * other[e1234]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<VersorEven> for Flector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11        8        1
    //    simd4        0        2        0
    // Totals...
    // yes simd       11       10        1
    //  no simd       11       16        1
    fn geometric_anti_quotient(self, other: VersorEven) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e5] * other[e4]) + f32::powi(other[e12345], 2) + f32::powi(other[e321], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - f32::powi(other[e1], 2)
                - f32::powi(other[e2], 2)
                - f32::powi(other[e3], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<VersorOdd> for Flector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11        8        1
    //    simd4        0        2        0
    // Totals...
    // yes simd       11       10        1
    //  no simd       11       16        1
    fn geometric_anti_quotient(self, other: VersorOdd) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e41] * other[e15])
                + 2.0 * (other[e42] * other[e25])
                + 2.0 * (other[e43] * other[e35])
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                + f32::powi(other[e4235], 2)
                + f32::powi(other[e4315], 2)
                + f32::powi(other[e4125], 2)
                - f32::powi(other[scalar], 2)
                - f32::powi(other[e45], 2)
                - 2.0 * (other[e1234] * other[e3215]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl std::ops::Div<geometric_anti_quotient> for Line {
    type Output = geometric_anti_quotient_partial<Line>;
    fn div(self, _rhs: geometric_anti_quotient) -> Self::Output {
        geometric_anti_quotient_partial(self)
    }
}
impl GeometricAntiQuotient<AntiCircleRotor> for Line {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        6        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        7        8        1
    //  no simd        7       12        1
    fn geometric_anti_quotient(self, other: AntiCircleRotor) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e41] * other[e15])
                + 2.0 * (other[e42] * other[e25])
                + 2.0 * (other[e43] * other[e35])
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                - f32::powi(other[e45], 2)
                - f32::powi(other[scalar], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Line::from_groups(
            // e415, e425, e435
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e415], self[e425], self[e435]]),
            // e235, e315, e125
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
        );
    }
}
impl GeometricAntiQuotient<AntiDipoleInversion> for Line {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10        8        1
    //    simd3        0        2        0
    // Totals...
    // yes simd       10       10        1
    //  no simd       10       14        1
    fn geometric_anti_quotient(self, other: AntiDipoleInversion) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e4] * other[e5]) + f32::powi(other[e321], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - f32::powi(other[e1], 2)
                - f32::powi(other[e2], 2)
                - f32::powi(other[e3], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Line::from_groups(
            // e415, e425, e435
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e415], self[e425], self[e435]]),
            // e235, e315, e125
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
        );
    }
}
impl GeometricAntiQuotient<AntiDualNum> for Line {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        3        1
    //  no simd        0        7        1
    fn geometric_anti_quotient(self, other: AntiDualNum) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[scalar], 2) * -1.0);
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Line::from_groups(
            // e415, e425, e435
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e415], self[e425], self[e435]]),
            // e235, e315, e125
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
        );
    }
}
impl GeometricAntiQuotient<AntiFlatPoint> for Line {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        6        1
    fn geometric_anti_quotient(self, other: AntiFlatPoint) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e321], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Line::from_groups(
            // e415, e425, e435
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e415], self[e425], self[e435]]),
            // e235, e315, e125
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
        );
    }
}
impl GeometricAntiQuotient<AntiFlector> for Line {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        3        2        1
    //  no simd        3        6        1
    fn geometric_anti_quotient(self, other: AntiFlector) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e321], 2) - f32::powi(other[e1], 2) - f32::powi(other[e2], 2) - f32::powi(other[e3], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Line::from_groups(
            // e415, e425, e435
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e415], self[e425], self[e435]]),
            // e235, e315, e125
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
        );
    }
}
impl GeometricAntiQuotient<AntiLine> for Line {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        2        2        1
    //  no simd        2        6        1
    fn geometric_anti_quotient(self, other: AntiLine) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e23], 2) + f32::powi(other[e31], 2) + f32::powi(other[e12], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Line::from_groups(
            // e415, e425, e435
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e415], self[e425], self[e435]]),
            // e235, e315, e125
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
        );
    }
}
impl GeometricAntiQuotient<AntiMotor> for Line {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        3        2        1
    //  no simd        3        6        1
    fn geometric_anti_quotient(self, other: AntiMotor) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e23], 2) + f32::powi(other[e31], 2) + f32::powi(other[e12], 2) - f32::powi(other[scalar], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Line::from_groups(
            // e415, e425, e435
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e415], self[e425], self[e435]]),
            // e235, e315, e125
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
        );
    }
}
impl GeometricAntiQuotient<AntiPlane> for Line {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        2        2        1
    //  no simd        2        6        1
    fn geometric_anti_quotient(self, other: AntiPlane) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ -f32::powi(other[e1], 2) - f32::powi(other[e2], 2) - f32::powi(other[e3], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Line::from_groups(
            // e415, e425, e435
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e415], self[e425], self[e435]]),
            // e235, e315, e125
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
        );
    }
}
impl GeometricAntiQuotient<AntiScalar> for Line {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        6        1
    fn geometric_anti_quotient(self, other: AntiScalar) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e12345], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Line::from_groups(
            // e415, e425, e435
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e415], self[e425], self[e435]]),
            // e235, e315, e125
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
        );
    }
}
impl GeometricAntiQuotient<Circle> for Line {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        6        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        6        8        1
    //  no simd        6       12        1
    fn geometric_anti_quotient(self, other: Circle) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e321], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Line::from_groups(
            // e415, e425, e435
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e415], self[e425], self[e435]]),
            // e235, e315, e125
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
        );
    }
}
impl GeometricAntiQuotient<CircleRotor> for Line {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        6        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        7        8        1
    //  no simd        7       12        1
    fn geometric_anti_quotient(self, other: CircleRotor) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e321], 2) + f32::powi(other[e12345], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Line::from_groups(
            // e415, e425, e435
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e415], self[e425], self[e435]]),
            // e235, e315, e125
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
        );
    }
}
impl GeometricAntiQuotient<Dipole> for Line {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        6        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        6        8        1
    //  no simd        6       12        1
    fn geometric_anti_quotient(self, other: Dipole) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e41] * other[e15])
                + 2.0 * (other[e42] * other[e25])
                + 2.0 * (other[e43] * other[e35])
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                - f32::powi(other[e45], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Line::from_groups(
            // e415, e425, e435
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e415], self[e425], self[e435]]),
            // e235, e315, e125
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
        );
    }
}
impl GeometricAntiQuotient<DipoleInversion> for Line {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10        8        1
    //    simd3        0        2        0
    // Totals...
    // yes simd       10       10        1
    //  no simd       10       14        1
    fn geometric_anti_quotient(self, other: DipoleInversion) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e41] * other[e15])
                + 2.0 * (other[e42] * other[e25])
                + 2.0 * (other[e43] * other[e35])
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                + f32::powi(other[e4235], 2)
                + f32::powi(other[e4315], 2)
                + f32::powi(other[e4125], 2)
                - f32::powi(other[e45], 2)
                - 2.0 * (other[e1234] * other[e3215]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Line::from_groups(
            // e415, e425, e435
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e415], self[e425], self[e435]]),
            // e235, e315, e125
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
        );
    }
}
impl GeometricAntiQuotient<DualNum> for Line {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        6        1
    fn geometric_anti_quotient(self, other: DualNum) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e12345], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Line::from_groups(
            // e415, e425, e435
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e415], self[e425], self[e435]]),
            // e235, e315, e125
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
        );
    }
}
impl GeometricAntiQuotient<FlatPoint> for Line {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        3        1
    //  no simd        0        7        1
    fn geometric_anti_quotient(self, other: FlatPoint) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e45], 2) * -1.0);
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Line::from_groups(
            // e415, e425, e435
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e415], self[e425], self[e435]]),
            // e235, e315, e125
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
        );
    }
}
impl GeometricAntiQuotient<Flector> for Line {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        3        2        1
    //  no simd        3        6        1
    fn geometric_anti_quotient(self, other: Flector) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e4235], 2) + f32::powi(other[e4315], 2) + f32::powi(other[e4125], 2) - f32::powi(other[e45], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Line::from_groups(
            // e415, e425, e435
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e415], self[e425], self[e435]]),
            // e235, e315, e125
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
        );
    }
}
impl GeometricAntiQuotient<Line> for Line {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        2        2        1
    //  no simd        2        6        1
    fn geometric_anti_quotient(self, other: Line) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ -f32::powi(other[e415], 2) - f32::powi(other[e425], 2) - f32::powi(other[e435], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Line::from_groups(
            // e415, e425, e435
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e415], self[e425], self[e435]]),
            // e235, e315, e125
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
        );
    }
}
impl GeometricAntiQuotient<Motor> for Line {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        3        2        1
    //  no simd        3        6        1
    fn geometric_anti_quotient(self, other: Motor) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e12345], 2) - f32::powi(other[e415], 2) - f32::powi(other[e425], 2) - f32::powi(other[e435], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Line::from_groups(
            // e415, e425, e435
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e415], self[e425], self[e435]]),
            // e235, e315, e125
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
        );
    }
}
impl GeometricAntiQuotient<MultiVector> for Line {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       23       16        1
    //    simd3        0        2        0
    // Totals...
    // yes simd       23       18        1
    //  no simd       23       22        1
    fn geometric_anti_quotient(self, other: MultiVector) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e4] * other[e5])
                + 2.0 * (other[e15] * other[e41])
                + 2.0 * (other[e25] * other[e42])
                + 2.0 * (other[e35] * other[e43])
                + f32::powi(other[e12345], 2)
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                + f32::powi(other[e321], 2)
                + f32::powi(other[e4235], 2)
                + f32::powi(other[e4315], 2)
                + f32::powi(other[e4125], 2)
                - f32::powi(other[scalar], 2)
                - f32::powi(other[e1], 2)
                - f32::powi(other[e2], 2)
                - f32::powi(other[e3], 2)
                - f32::powi(other[e45], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125])
                - 2.0 * (other[e3215] * other[e1234]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Line::from_groups(
            // e415, e425, e435
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e415], self[e425], self[e435]]),
            // e235, e315, e125
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
        );
    }
}
impl GeometricAntiQuotient<Plane> for Line {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        2        2        1
    //  no simd        2        6        1
    fn geometric_anti_quotient(self, other: Plane) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e4235], 2) + f32::powi(other[e4315], 2) + f32::powi(other[e4125], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Line::from_groups(
            // e415, e425, e435
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e415], self[e425], self[e435]]),
            // e235, e315, e125
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
        );
    }
}
impl GeometricAntiQuotient<RoundPoint> for Line {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        2        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        3        4        1
    //  no simd        3        8        1
    fn geometric_anti_quotient(self, other: RoundPoint) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e4] * other[e5]) - f32::powi(other[e1], 2) - f32::powi(other[e2], 2) - f32::powi(other[e3], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Line::from_groups(
            // e415, e425, e435
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e415], self[e425], self[e435]]),
            // e235, e315, e125
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
        );
    }
}
impl GeometricAntiQuotient<Scalar> for Line {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        3        1
    //  no simd        0        7        1
    fn geometric_anti_quotient(self, other: Scalar) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[scalar], 2) * -1.0);
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Line::from_groups(
            // e415, e425, e435
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e415], self[e425], self[e435]]),
            // e235, e315, e125
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
        );
    }
}
impl GeometricAntiQuotient<Sphere> for Line {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        2        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        3        4        1
    //  no simd        3        8        1
    fn geometric_anti_quotient(self, other: Sphere) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e4235], 2) + f32::powi(other[e4315], 2) + f32::powi(other[e4125], 2) - 2.0 * (other[e3215] * other[e1234]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Line::from_groups(
            // e415, e425, e435
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e415], self[e425], self[e435]]),
            // e235, e315, e125
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
        );
    }
}
impl GeometricAntiQuotient<VersorEven> for Line {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11        8        1
    //    simd3        0        2        0
    // Totals...
    // yes simd       11       10        1
    //  no simd       11       14        1
    fn geometric_anti_quotient(self, other: VersorEven) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e5] * other[e4]) + f32::powi(other[e12345], 2) + f32::powi(other[e321], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - f32::powi(other[e1], 2)
                - f32::powi(other[e2], 2)
                - f32::powi(other[e3], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Line::from_groups(
            // e415, e425, e435
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e415], self[e425], self[e435]]),
            // e235, e315, e125
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
        );
    }
}
impl GeometricAntiQuotient<VersorOdd> for Line {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11        8        1
    //    simd3        0        2        0
    // Totals...
    // yes simd       11       10        1
    //  no simd       11       14        1
    fn geometric_anti_quotient(self, other: VersorOdd) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e41] * other[e15])
                + 2.0 * (other[e42] * other[e25])
                + 2.0 * (other[e43] * other[e35])
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                + f32::powi(other[e4235], 2)
                + f32::powi(other[e4315], 2)
                + f32::powi(other[e4125], 2)
                - f32::powi(other[scalar], 2)
                - f32::powi(other[e45], 2)
                - 2.0 * (other[e1234] * other[e3215]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Line::from_groups(
            // e415, e425, e435
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e415], self[e425], self[e435]]),
            // e235, e315, e125
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
        );
    }
}
impl std::ops::Div<geometric_anti_quotient> for Motor {
    type Output = geometric_anti_quotient_partial<Motor>;
    fn div(self, _rhs: geometric_anti_quotient) -> Self::Output {
        geometric_anti_quotient_partial(self)
    }
}
impl GeometricAntiQuotient<AntiCircleRotor> for Motor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        6        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        7        8        1
    //  no simd        7       14        1
    fn geometric_anti_quotient(self, other: AntiCircleRotor) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e41] * other[e15])
                + 2.0 * (other[e42] * other[e25])
                + 2.0 * (other[e43] * other[e35])
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                - f32::powi(other[e45], 2)
                - f32::powi(other[scalar], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e12345]]),
            // e235, e315, e125, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]),
        );
    }
}
impl GeometricAntiQuotient<AntiDipoleInversion> for Motor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10        8        1
    //    simd4        0        2        0
    // Totals...
    // yes simd       10       10        1
    //  no simd       10       16        1
    fn geometric_anti_quotient(self, other: AntiDipoleInversion) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e4] * other[e5]) + f32::powi(other[e321], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - f32::powi(other[e1], 2)
                - f32::powi(other[e2], 2)
                - f32::powi(other[e3], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e12345]]),
            // e235, e315, e125, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]),
        );
    }
}
impl GeometricAntiQuotient<AntiDualNum> for Motor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        1
    //  no simd        0        9        1
    fn geometric_anti_quotient(self, other: AntiDualNum) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[scalar], 2) * -1.0);
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e12345]]),
            // e235, e315, e125, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]),
        );
    }
}
impl GeometricAntiQuotient<AntiFlatPoint> for Motor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        8        1
    fn geometric_anti_quotient(self, other: AntiFlatPoint) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e321], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e12345]]),
            // e235, e315, e125, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]),
        );
    }
}
impl GeometricAntiQuotient<AntiFlector> for Motor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        2        1
    //  no simd        3        8        1
    fn geometric_anti_quotient(self, other: AntiFlector) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e321], 2) - f32::powi(other[e1], 2) - f32::powi(other[e2], 2) - f32::powi(other[e3], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e12345]]),
            // e235, e315, e125, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]),
        );
    }
}
impl GeometricAntiQuotient<AntiLine> for Motor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        2        2        1
    //  no simd        2        8        1
    fn geometric_anti_quotient(self, other: AntiLine) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e23], 2) + f32::powi(other[e31], 2) + f32::powi(other[e12], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e12345]]),
            // e235, e315, e125, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]),
        );
    }
}
impl GeometricAntiQuotient<AntiMotor> for Motor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        2        1
    //  no simd        3        8        1
    fn geometric_anti_quotient(self, other: AntiMotor) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e23], 2) + f32::powi(other[e31], 2) + f32::powi(other[e12], 2) - f32::powi(other[scalar], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e12345]]),
            // e235, e315, e125, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]),
        );
    }
}
impl GeometricAntiQuotient<AntiPlane> for Motor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        2        2        1
    //  no simd        2        8        1
    fn geometric_anti_quotient(self, other: AntiPlane) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ -f32::powi(other[e1], 2) - f32::powi(other[e2], 2) - f32::powi(other[e3], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e12345]]),
            // e235, e315, e125, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]),
        );
    }
}
impl GeometricAntiQuotient<AntiScalar> for Motor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        8        1
    fn geometric_anti_quotient(self, other: AntiScalar) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e12345], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e12345]]),
            // e235, e315, e125, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]),
        );
    }
}
impl GeometricAntiQuotient<Circle> for Motor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        6        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        6        8        1
    //  no simd        6       14        1
    fn geometric_anti_quotient(self, other: Circle) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e321], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e12345]]),
            // e235, e315, e125, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]),
        );
    }
}
impl GeometricAntiQuotient<CircleRotor> for Motor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        6        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        7        8        1
    //  no simd        7       14        1
    fn geometric_anti_quotient(self, other: CircleRotor) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e321], 2) + f32::powi(other[e12345], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e12345]]),
            // e235, e315, e125, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]),
        );
    }
}
impl GeometricAntiQuotient<Dipole> for Motor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        6        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        6        8        1
    //  no simd        6       14        1
    fn geometric_anti_quotient(self, other: Dipole) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e41] * other[e15])
                + 2.0 * (other[e42] * other[e25])
                + 2.0 * (other[e43] * other[e35])
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                - f32::powi(other[e45], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e12345]]),
            // e235, e315, e125, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]),
        );
    }
}
impl GeometricAntiQuotient<DipoleInversion> for Motor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10        8        1
    //    simd4        0        2        0
    // Totals...
    // yes simd       10       10        1
    //  no simd       10       16        1
    fn geometric_anti_quotient(self, other: DipoleInversion) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e41] * other[e15])
                + 2.0 * (other[e42] * other[e25])
                + 2.0 * (other[e43] * other[e35])
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                + f32::powi(other[e4235], 2)
                + f32::powi(other[e4315], 2)
                + f32::powi(other[e4125], 2)
                - f32::powi(other[e45], 2)
                - 2.0 * (other[e1234] * other[e3215]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e12345]]),
            // e235, e315, e125, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]),
        );
    }
}
impl GeometricAntiQuotient<DualNum> for Motor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        8        1
    fn geometric_anti_quotient(self, other: DualNum) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e12345], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e12345]]),
            // e235, e315, e125, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]),
        );
    }
}
impl GeometricAntiQuotient<FlatPoint> for Motor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        1
    //  no simd        0        9        1
    fn geometric_anti_quotient(self, other: FlatPoint) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e45], 2) * -1.0);
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e12345]]),
            // e235, e315, e125, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]),
        );
    }
}
impl GeometricAntiQuotient<Flector> for Motor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        2        1
    //  no simd        3        8        1
    fn geometric_anti_quotient(self, other: Flector) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e4235], 2) + f32::powi(other[e4315], 2) + f32::powi(other[e4125], 2) - f32::powi(other[e45], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e12345]]),
            // e235, e315, e125, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]),
        );
    }
}
impl GeometricAntiQuotient<Line> for Motor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        2        2        1
    //  no simd        2        8        1
    fn geometric_anti_quotient(self, other: Line) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ -f32::powi(other[e415], 2) - f32::powi(other[e425], 2) - f32::powi(other[e435], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e12345]]),
            // e235, e315, e125, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]),
        );
    }
}
impl GeometricAntiQuotient<Motor> for Motor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        2        1
    //  no simd        3        8        1
    fn geometric_anti_quotient(self, other: Motor) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e12345], 2) - f32::powi(other[e415], 2) - f32::powi(other[e425], 2) - f32::powi(other[e435], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e12345]]),
            // e235, e315, e125, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]),
        );
    }
}
impl GeometricAntiQuotient<MultiVector> for Motor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       23       16        1
    //    simd4        0        2        0
    // Totals...
    // yes simd       23       18        1
    //  no simd       23       24        1
    fn geometric_anti_quotient(self, other: MultiVector) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e4] * other[e5])
                + 2.0 * (other[e15] * other[e41])
                + 2.0 * (other[e25] * other[e42])
                + 2.0 * (other[e35] * other[e43])
                + f32::powi(other[e12345], 2)
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                + f32::powi(other[e321], 2)
                + f32::powi(other[e4235], 2)
                + f32::powi(other[e4315], 2)
                + f32::powi(other[e4125], 2)
                - f32::powi(other[scalar], 2)
                - f32::powi(other[e1], 2)
                - f32::powi(other[e2], 2)
                - f32::powi(other[e3], 2)
                - f32::powi(other[e45], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125])
                - 2.0 * (other[e3215] * other[e1234]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e12345]]),
            // e235, e315, e125, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]),
        );
    }
}
impl GeometricAntiQuotient<Plane> for Motor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        2        2        1
    //  no simd        2        8        1
    fn geometric_anti_quotient(self, other: Plane) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e4235], 2) + f32::powi(other[e4315], 2) + f32::powi(other[e4125], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e12345]]),
            // e235, e315, e125, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]),
        );
    }
}
impl GeometricAntiQuotient<RoundPoint> for Motor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        2        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        4        1
    //  no simd        3       10        1
    fn geometric_anti_quotient(self, other: RoundPoint) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e4] * other[e5]) - f32::powi(other[e1], 2) - f32::powi(other[e2], 2) - f32::powi(other[e3], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e12345]]),
            // e235, e315, e125, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]),
        );
    }
}
impl GeometricAntiQuotient<Scalar> for Motor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        1
    //  no simd        0        9        1
    fn geometric_anti_quotient(self, other: Scalar) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[scalar], 2) * -1.0);
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e12345]]),
            // e235, e315, e125, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]),
        );
    }
}
impl GeometricAntiQuotient<Sphere> for Motor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        2        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        4        1
    //  no simd        3       10        1
    fn geometric_anti_quotient(self, other: Sphere) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e4235], 2) + f32::powi(other[e4315], 2) + f32::powi(other[e4125], 2) - 2.0 * (other[e3215] * other[e1234]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e12345]]),
            // e235, e315, e125, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]),
        );
    }
}
impl GeometricAntiQuotient<VersorEven> for Motor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11        8        1
    //    simd4        0        2        0
    // Totals...
    // yes simd       11       10        1
    //  no simd       11       16        1
    fn geometric_anti_quotient(self, other: VersorEven) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e5] * other[e4]) + f32::powi(other[e12345], 2) + f32::powi(other[e321], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - f32::powi(other[e1], 2)
                - f32::powi(other[e2], 2)
                - f32::powi(other[e3], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e12345]]),
            // e235, e315, e125, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]),
        );
    }
}
impl GeometricAntiQuotient<VersorOdd> for Motor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11        8        1
    //    simd4        0        2        0
    // Totals...
    // yes simd       11       10        1
    //  no simd       11       16        1
    fn geometric_anti_quotient(self, other: VersorOdd) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e41] * other[e15])
                + 2.0 * (other[e42] * other[e25])
                + 2.0 * (other[e43] * other[e35])
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                + f32::powi(other[e4235], 2)
                + f32::powi(other[e4315], 2)
                + f32::powi(other[e4125], 2)
                - f32::powi(other[scalar], 2)
                - f32::powi(other[e45], 2)
                - 2.0 * (other[e1234] * other[e3215]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e12345]]),
            // e235, e315, e125, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]),
        );
    }
}
impl std::ops::Div<geometric_anti_quotient> for MultiVector {
    type Output = geometric_anti_quotient_partial<MultiVector>;
    fn div(self, _rhs: geometric_anti_quotient) -> Self::Output {
        geometric_anti_quotient_partial(self)
    }
}
impl GeometricAntiQuotient<AntiCircleRotor> for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        8        1
    //    simd2        0        1        0
    //    simd3        0        4        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        7       17        1
    //  no simd        7       38        1
    fn geometric_anti_quotient(self, other: AntiCircleRotor) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e41] * other[e15])
                + 2.0 * (other[e42] * other[e25])
                + 2.0 * (other[e43] * other[e35])
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                - f32::powi(other[e45], 2)
                - f32::powi(other[scalar], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(anti_inverse[e12345]) * Simd32x2::from([self[scalar], self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            anti_inverse[e12345] * self[e5],
            // e15, e25, e35, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e423, e431, e412
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
            // e1234
            anti_inverse[e12345] * self[e1234],
        );
    }
}
impl GeometricAntiQuotient<AntiDipoleInversion> for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       10        1
    //    simd2        0        1        0
    //    simd3        0        4        0
    //    simd4        0        4        0
    // Totals...
    // yes simd       10       19        1
    //  no simd       10       40        1
    fn geometric_anti_quotient(self, other: AntiDipoleInversion) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e4] * other[e5]) + f32::powi(other[e321], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - f32::powi(other[e1], 2)
                - f32::powi(other[e2], 2)
                - f32::powi(other[e3], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(anti_inverse[e12345]) * Simd32x2::from([self[scalar], self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            anti_inverse[e12345] * self[e5],
            // e15, e25, e35, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e423, e431, e412
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
            // e1234
            anti_inverse[e12345] * self[e1234],
        );
    }
}
impl GeometricAntiQuotient<AntiDualNum> for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        1
    //    simd2        0        1        0
    //    simd3        0        4        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        0       12        1
    //  no simd        0       33        1
    fn geometric_anti_quotient(self, other: AntiDualNum) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[scalar], 2) * -1.0);
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(anti_inverse[e12345]) * Simd32x2::from([self[scalar], self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            anti_inverse[e12345] * self[e5],
            // e15, e25, e35, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e423, e431, e412
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
            // e1234
            anti_inverse[e12345] * self[e1234],
        );
    }
}
impl GeometricAntiQuotient<AntiFlatPoint> for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        1
    //    simd2        0        1        0
    //    simd3        0        4        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        0       11        1
    //  no simd        0       32        1
    fn geometric_anti_quotient(self, other: AntiFlatPoint) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e321], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(anti_inverse[e12345]) * Simd32x2::from([self[scalar], self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            anti_inverse[e12345] * self[e5],
            // e15, e25, e35, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e423, e431, e412
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
            // e1234
            anti_inverse[e12345] * self[e1234],
        );
    }
}
impl GeometricAntiQuotient<AntiFlector> for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        2        1
    //    simd2        0        1        0
    //    simd3        0        4        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        3       11        1
    //  no simd        3       32        1
    fn geometric_anti_quotient(self, other: AntiFlector) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e321], 2) - f32::powi(other[e1], 2) - f32::powi(other[e2], 2) - f32::powi(other[e3], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(anti_inverse[e12345]) * Simd32x2::from([self[scalar], self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            anti_inverse[e12345] * self[e5],
            // e15, e25, e35, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e423, e431, e412
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
            // e1234
            anti_inverse[e12345] * self[e1234],
        );
    }
}
impl GeometricAntiQuotient<AntiLine> for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        2        1
    //    simd2        0        1        0
    //    simd3        0        4        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        2       11        1
    //  no simd        2       32        1
    fn geometric_anti_quotient(self, other: AntiLine) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e23], 2) + f32::powi(other[e31], 2) + f32::powi(other[e12], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(anti_inverse[e12345]) * Simd32x2::from([self[scalar], self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            anti_inverse[e12345] * self[e5],
            // e15, e25, e35, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e423, e431, e412
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
            // e1234
            anti_inverse[e12345] * self[e1234],
        );
    }
}
impl GeometricAntiQuotient<AntiMotor> for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        2        1
    //    simd2        0        1        0
    //    simd3        0        4        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        3       11        1
    //  no simd        3       32        1
    fn geometric_anti_quotient(self, other: AntiMotor) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e23], 2) + f32::powi(other[e31], 2) + f32::powi(other[e12], 2) - f32::powi(other[scalar], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(anti_inverse[e12345]) * Simd32x2::from([self[scalar], self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            anti_inverse[e12345] * self[e5],
            // e15, e25, e35, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e423, e431, e412
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
            // e1234
            anti_inverse[e12345] * self[e1234],
        );
    }
}
impl GeometricAntiQuotient<AntiPlane> for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        2        1
    //    simd2        0        1        0
    //    simd3        0        4        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        2       11        1
    //  no simd        2       32        1
    fn geometric_anti_quotient(self, other: AntiPlane) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ -f32::powi(other[e1], 2) - f32::powi(other[e2], 2) - f32::powi(other[e3], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(anti_inverse[e12345]) * Simd32x2::from([self[scalar], self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            anti_inverse[e12345] * self[e5],
            // e15, e25, e35, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e423, e431, e412
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
            // e1234
            anti_inverse[e12345] * self[e1234],
        );
    }
}
impl GeometricAntiQuotient<AntiScalar> for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        1
    //    simd2        0        1        0
    //    simd3        0        4        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        0       11        1
    //  no simd        0       32        1
    fn geometric_anti_quotient(self, other: AntiScalar) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e12345], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(anti_inverse[e12345]) * Simd32x2::from([self[scalar], self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            anti_inverse[e12345] * self[e5],
            // e15, e25, e35, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e423, e431, e412
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
            // e1234
            anti_inverse[e12345] * self[e1234],
        );
    }
}
impl GeometricAntiQuotient<Circle> for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        8        1
    //    simd2        0        1        0
    //    simd3        0        4        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        6       17        1
    //  no simd        6       38        1
    fn geometric_anti_quotient(self, other: Circle) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e321], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(anti_inverse[e12345]) * Simd32x2::from([self[scalar], self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            anti_inverse[e12345] * self[e5],
            // e15, e25, e35, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e423, e431, e412
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
            // e1234
            anti_inverse[e12345] * self[e1234],
        );
    }
}
impl GeometricAntiQuotient<CircleRotor> for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        8        1
    //    simd2        0        1        0
    //    simd3        0        4        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        7       17        1
    //  no simd        7       38        1
    fn geometric_anti_quotient(self, other: CircleRotor) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e321], 2) + f32::powi(other[e12345], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(anti_inverse[e12345]) * Simd32x2::from([self[scalar], self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            anti_inverse[e12345] * self[e5],
            // e15, e25, e35, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e423, e431, e412
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
            // e1234
            anti_inverse[e12345] * self[e1234],
        );
    }
}
impl GeometricAntiQuotient<Dipole> for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        8        1
    //    simd2        0        1        0
    //    simd3        0        4        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        6       17        1
    //  no simd        6       38        1
    fn geometric_anti_quotient(self, other: Dipole) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e41] * other[e15])
                + 2.0 * (other[e42] * other[e25])
                + 2.0 * (other[e43] * other[e35])
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                - f32::powi(other[e45], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(anti_inverse[e12345]) * Simd32x2::from([self[scalar], self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            anti_inverse[e12345] * self[e5],
            // e15, e25, e35, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e423, e431, e412
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
            // e1234
            anti_inverse[e12345] * self[e1234],
        );
    }
}
impl GeometricAntiQuotient<DipoleInversion> for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       10        1
    //    simd2        0        1        0
    //    simd3        0        4        0
    //    simd4        0        4        0
    // Totals...
    // yes simd       10       19        1
    //  no simd       10       40        1
    fn geometric_anti_quotient(self, other: DipoleInversion) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e41] * other[e15])
                + 2.0 * (other[e42] * other[e25])
                + 2.0 * (other[e43] * other[e35])
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                + f32::powi(other[e4235], 2)
                + f32::powi(other[e4315], 2)
                + f32::powi(other[e4125], 2)
                - f32::powi(other[e45], 2)
                - 2.0 * (other[e1234] * other[e3215]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(anti_inverse[e12345]) * Simd32x2::from([self[scalar], self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            anti_inverse[e12345] * self[e5],
            // e15, e25, e35, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e423, e431, e412
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
            // e1234
            anti_inverse[e12345] * self[e1234],
        );
    }
}
impl GeometricAntiQuotient<DualNum> for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        1
    //    simd2        0        1        0
    //    simd3        0        4        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        0       11        1
    //  no simd        0       32        1
    fn geometric_anti_quotient(self, other: DualNum) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e12345], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(anti_inverse[e12345]) * Simd32x2::from([self[scalar], self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            anti_inverse[e12345] * self[e5],
            // e15, e25, e35, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e423, e431, e412
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
            // e1234
            anti_inverse[e12345] * self[e1234],
        );
    }
}
impl GeometricAntiQuotient<FlatPoint> for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        1
    //    simd2        0        1        0
    //    simd3        0        4        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        0       12        1
    //  no simd        0       33        1
    fn geometric_anti_quotient(self, other: FlatPoint) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e45], 2) * -1.0);
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(anti_inverse[e12345]) * Simd32x2::from([self[scalar], self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            anti_inverse[e12345] * self[e5],
            // e15, e25, e35, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e423, e431, e412
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
            // e1234
            anti_inverse[e12345] * self[e1234],
        );
    }
}
impl GeometricAntiQuotient<Flector> for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        2        1
    //    simd2        0        1        0
    //    simd3        0        4        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        3       11        1
    //  no simd        3       32        1
    fn geometric_anti_quotient(self, other: Flector) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e4235], 2) + f32::powi(other[e4315], 2) + f32::powi(other[e4125], 2) - f32::powi(other[e45], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(anti_inverse[e12345]) * Simd32x2::from([self[scalar], self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            anti_inverse[e12345] * self[e5],
            // e15, e25, e35, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e423, e431, e412
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
            // e1234
            anti_inverse[e12345] * self[e1234],
        );
    }
}
impl GeometricAntiQuotient<Line> for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        2        1
    //    simd2        0        1        0
    //    simd3        0        4        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        2       11        1
    //  no simd        2       32        1
    fn geometric_anti_quotient(self, other: Line) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ -f32::powi(other[e415], 2) - f32::powi(other[e425], 2) - f32::powi(other[e435], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(anti_inverse[e12345]) * Simd32x2::from([self[scalar], self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            anti_inverse[e12345] * self[e5],
            // e15, e25, e35, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e423, e431, e412
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
            // e1234
            anti_inverse[e12345] * self[e1234],
        );
    }
}
impl GeometricAntiQuotient<Motor> for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        2        1
    //    simd2        0        1        0
    //    simd3        0        4        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        3       11        1
    //  no simd        3       32        1
    fn geometric_anti_quotient(self, other: Motor) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e12345], 2) - f32::powi(other[e415], 2) - f32::powi(other[e425], 2) - f32::powi(other[e435], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(anti_inverse[e12345]) * Simd32x2::from([self[scalar], self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            anti_inverse[e12345] * self[e5],
            // e15, e25, e35, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e423, e431, e412
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
            // e1234
            anti_inverse[e12345] * self[e1234],
        );
    }
}
impl GeometricAntiQuotient<MultiVector> for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       23       18        1
    //    simd2        0        1        0
    //    simd3        0        4        0
    //    simd4        0        4        0
    // Totals...
    // yes simd       23       27        1
    //  no simd       23       48        1
    fn geometric_anti_quotient(self, other: MultiVector) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e4] * other[e5])
                + 2.0 * (other[e15] * other[e41])
                + 2.0 * (other[e25] * other[e42])
                + 2.0 * (other[e35] * other[e43])
                + f32::powi(other[e12345], 2)
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                + f32::powi(other[e321], 2)
                + f32::powi(other[e4235], 2)
                + f32::powi(other[e4315], 2)
                + f32::powi(other[e4125], 2)
                - f32::powi(other[scalar], 2)
                - f32::powi(other[e1], 2)
                - f32::powi(other[e2], 2)
                - f32::powi(other[e3], 2)
                - f32::powi(other[e45], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125])
                - 2.0 * (other[e3215] * other[e1234]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(anti_inverse[e12345]) * Simd32x2::from([self[scalar], self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            anti_inverse[e12345] * self[e5],
            // e15, e25, e35, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e423, e431, e412
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
            // e1234
            anti_inverse[e12345] * self[e1234],
        );
    }
}
impl GeometricAntiQuotient<Plane> for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        2        1
    //    simd2        0        1        0
    //    simd3        0        4        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        2       11        1
    //  no simd        2       32        1
    fn geometric_anti_quotient(self, other: Plane) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e4235], 2) + f32::powi(other[e4315], 2) + f32::powi(other[e4125], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(anti_inverse[e12345]) * Simd32x2::from([self[scalar], self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            anti_inverse[e12345] * self[e5],
            // e15, e25, e35, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e423, e431, e412
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
            // e1234
            anti_inverse[e12345] * self[e1234],
        );
    }
}
impl GeometricAntiQuotient<RoundPoint> for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        1
    //    simd2        0        1        0
    //    simd3        0        4        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        3       13        1
    //  no simd        3       34        1
    fn geometric_anti_quotient(self, other: RoundPoint) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e4] * other[e5]) - f32::powi(other[e1], 2) - f32::powi(other[e2], 2) - f32::powi(other[e3], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(anti_inverse[e12345]) * Simd32x2::from([self[scalar], self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            anti_inverse[e12345] * self[e5],
            // e15, e25, e35, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e423, e431, e412
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
            // e1234
            anti_inverse[e12345] * self[e1234],
        );
    }
}
impl GeometricAntiQuotient<Scalar> for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        1
    //    simd2        0        1        0
    //    simd3        0        4        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        0       12        1
    //  no simd        0       33        1
    fn geometric_anti_quotient(self, other: Scalar) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[scalar], 2) * -1.0);
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(anti_inverse[e12345]) * Simd32x2::from([self[scalar], self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            anti_inverse[e12345] * self[e5],
            // e15, e25, e35, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e423, e431, e412
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
            // e1234
            anti_inverse[e12345] * self[e1234],
        );
    }
}
impl GeometricAntiQuotient<Sphere> for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        1
    //    simd2        0        1        0
    //    simd3        0        4        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        3       13        1
    //  no simd        3       34        1
    fn geometric_anti_quotient(self, other: Sphere) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e4235], 2) + f32::powi(other[e4315], 2) + f32::powi(other[e4125], 2) - 2.0 * (other[e3215] * other[e1234]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(anti_inverse[e12345]) * Simd32x2::from([self[scalar], self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            anti_inverse[e12345] * self[e5],
            // e15, e25, e35, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e423, e431, e412
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
            // e1234
            anti_inverse[e12345] * self[e1234],
        );
    }
}
impl GeometricAntiQuotient<VersorEven> for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       10        1
    //    simd2        0        1        0
    //    simd3        0        4        0
    //    simd4        0        4        0
    // Totals...
    // yes simd       11       19        1
    //  no simd       11       40        1
    fn geometric_anti_quotient(self, other: VersorEven) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e5] * other[e4]) + f32::powi(other[e12345], 2) + f32::powi(other[e321], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - f32::powi(other[e1], 2)
                - f32::powi(other[e2], 2)
                - f32::powi(other[e3], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(anti_inverse[e12345]) * Simd32x2::from([self[scalar], self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            anti_inverse[e12345] * self[e5],
            // e15, e25, e35, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e423, e431, e412
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
            // e1234
            anti_inverse[e12345] * self[e1234],
        );
    }
}
impl GeometricAntiQuotient<VersorOdd> for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       10        1
    //    simd2        0        1        0
    //    simd3        0        4        0
    //    simd4        0        4        0
    // Totals...
    // yes simd       11       19        1
    //  no simd       11       40        1
    fn geometric_anti_quotient(self, other: VersorOdd) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e41] * other[e15])
                + 2.0 * (other[e42] * other[e25])
                + 2.0 * (other[e43] * other[e35])
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                + f32::powi(other[e4235], 2)
                + f32::powi(other[e4315], 2)
                + f32::powi(other[e4125], 2)
                - f32::powi(other[scalar], 2)
                - f32::powi(other[e45], 2)
                - 2.0 * (other[e1234] * other[e3215]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(anti_inverse[e12345]) * Simd32x2::from([self[scalar], self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            anti_inverse[e12345] * self[e5],
            // e15, e25, e35, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e423, e431, e412
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
            // e1234
            anti_inverse[e12345] * self[e1234],
        );
    }
}
impl std::ops::Div<geometric_anti_quotient> for Plane {
    type Output = geometric_anti_quotient_partial<Plane>;
    fn div(self, _rhs: geometric_anti_quotient) -> Self::Output {
        geometric_anti_quotient_partial(self)
    }
}
impl GeometricAntiQuotient<AntiCircleRotor> for Plane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        6        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        7        7        1
    //  no simd        7       10        1
    fn geometric_anti_quotient(self, other: AntiCircleRotor) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e41] * other[e15])
                + 2.0 * (other[e42] * other[e25])
                + 2.0 * (other[e43] * other[e35])
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                - f32::powi(other[e45], 2)
                - f32::powi(other[scalar], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<AntiDipoleInversion> for Plane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10        8        1
    //    simd4        0        1        0
    // Totals...
    // yes simd       10        9        1
    //  no simd       10       12        1
    fn geometric_anti_quotient(self, other: AntiDipoleInversion) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e4] * other[e5]) + f32::powi(other[e321], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - f32::powi(other[e1], 2)
                - f32::powi(other[e2], 2)
                - f32::powi(other[e3], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<AntiDualNum> for Plane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        5        1
    fn geometric_anti_quotient(self, other: AntiDualNum) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[scalar], 2) * -1.0);
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<AntiFlatPoint> for Plane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        1        1
    //  no simd        0        4        1
    fn geometric_anti_quotient(self, other: AntiFlatPoint) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e321], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<AntiFlector> for Plane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        1        1
    //  no simd        3        4        1
    fn geometric_anti_quotient(self, other: AntiFlector) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e321], 2) - f32::powi(other[e1], 2) - f32::powi(other[e2], 2) - f32::powi(other[e3], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<AntiLine> for Plane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        1        1
    //  no simd        2        4        1
    fn geometric_anti_quotient(self, other: AntiLine) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e23], 2) + f32::powi(other[e31], 2) + f32::powi(other[e12], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<AntiMotor> for Plane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        1        1
    //  no simd        3        4        1
    fn geometric_anti_quotient(self, other: AntiMotor) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e23], 2) + f32::powi(other[e31], 2) + f32::powi(other[e12], 2) - f32::powi(other[scalar], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<AntiPlane> for Plane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        1        1
    //  no simd        2        4        1
    fn geometric_anti_quotient(self, other: AntiPlane) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ -f32::powi(other[e1], 2) - f32::powi(other[e2], 2) - f32::powi(other[e3], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<AntiScalar> for Plane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        1        1
    //  no simd        0        4        1
    fn geometric_anti_quotient(self, other: AntiScalar) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e12345], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<Circle> for Plane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        6        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        6        7        1
    //  no simd        6       10        1
    fn geometric_anti_quotient(self, other: Circle) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e321], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<CircleRotor> for Plane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        6        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        7        7        1
    //  no simd        7       10        1
    fn geometric_anti_quotient(self, other: CircleRotor) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e321], 2) + f32::powi(other[e12345], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<Dipole> for Plane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        6        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        6        7        1
    //  no simd        6       10        1
    fn geometric_anti_quotient(self, other: Dipole) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e41] * other[e15])
                + 2.0 * (other[e42] * other[e25])
                + 2.0 * (other[e43] * other[e35])
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                - f32::powi(other[e45], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<DipoleInversion> for Plane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10        8        1
    //    simd4        0        1        0
    // Totals...
    // yes simd       10        9        1
    //  no simd       10       12        1
    fn geometric_anti_quotient(self, other: DipoleInversion) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e41] * other[e15])
                + 2.0 * (other[e42] * other[e25])
                + 2.0 * (other[e43] * other[e35])
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                + f32::powi(other[e4235], 2)
                + f32::powi(other[e4315], 2)
                + f32::powi(other[e4125], 2)
                - f32::powi(other[e45], 2)
                - 2.0 * (other[e1234] * other[e3215]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<DualNum> for Plane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        1        1
    //  no simd        0        4        1
    fn geometric_anti_quotient(self, other: DualNum) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e12345], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<FlatPoint> for Plane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        5        1
    fn geometric_anti_quotient(self, other: FlatPoint) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e45], 2) * -1.0);
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<Flector> for Plane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        1        1
    //  no simd        3        4        1
    fn geometric_anti_quotient(self, other: Flector) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e4235], 2) + f32::powi(other[e4315], 2) + f32::powi(other[e4125], 2) - f32::powi(other[e45], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<Line> for Plane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        1        1
    //  no simd        2        4        1
    fn geometric_anti_quotient(self, other: Line) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ -f32::powi(other[e415], 2) - f32::powi(other[e425], 2) - f32::powi(other[e435], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<Motor> for Plane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        1        1
    //  no simd        3        4        1
    fn geometric_anti_quotient(self, other: Motor) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e12345], 2) - f32::powi(other[e415], 2) - f32::powi(other[e425], 2) - f32::powi(other[e435], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<MultiVector> for Plane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       23       16        1
    //    simd4        0        1        0
    // Totals...
    // yes simd       23       17        1
    //  no simd       23       20        1
    fn geometric_anti_quotient(self, other: MultiVector) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e4] * other[e5])
                + 2.0 * (other[e15] * other[e41])
                + 2.0 * (other[e25] * other[e42])
                + 2.0 * (other[e35] * other[e43])
                + f32::powi(other[e12345], 2)
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                + f32::powi(other[e321], 2)
                + f32::powi(other[e4235], 2)
                + f32::powi(other[e4315], 2)
                + f32::powi(other[e4125], 2)
                - f32::powi(other[scalar], 2)
                - f32::powi(other[e1], 2)
                - f32::powi(other[e2], 2)
                - f32::powi(other[e3], 2)
                - f32::powi(other[e45], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125])
                - 2.0 * (other[e3215] * other[e1234]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<Plane> for Plane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        1        1
    //  no simd        2        4        1
    fn geometric_anti_quotient(self, other: Plane) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e4235], 2) + f32::powi(other[e4315], 2) + f32::powi(other[e4125], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<RoundPoint> for Plane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        2        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        3        1
    //  no simd        3        6        1
    fn geometric_anti_quotient(self, other: RoundPoint) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e4] * other[e5]) - f32::powi(other[e1], 2) - f32::powi(other[e2], 2) - f32::powi(other[e3], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<Scalar> for Plane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        5        1
    fn geometric_anti_quotient(self, other: Scalar) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[scalar], 2) * -1.0);
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<Sphere> for Plane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        2        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        3        1
    //  no simd        3        6        1
    fn geometric_anti_quotient(self, other: Sphere) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e4235], 2) + f32::powi(other[e4315], 2) + f32::powi(other[e4125], 2) - 2.0 * (other[e3215] * other[e1234]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<VersorEven> for Plane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11        8        1
    //    simd4        0        1        0
    // Totals...
    // yes simd       11        9        1
    //  no simd       11       12        1
    fn geometric_anti_quotient(self, other: VersorEven) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e5] * other[e4]) + f32::powi(other[e12345], 2) + f32::powi(other[e321], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - f32::powi(other[e1], 2)
                - f32::powi(other[e2], 2)
                - f32::powi(other[e3], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<VersorOdd> for Plane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11        8        1
    //    simd4        0        1        0
    // Totals...
    // yes simd       11        9        1
    //  no simd       11       12        1
    fn geometric_anti_quotient(self, other: VersorOdd) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e41] * other[e15])
                + 2.0 * (other[e42] * other[e25])
                + 2.0 * (other[e43] * other[e35])
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                + f32::powi(other[e4235], 2)
                + f32::powi(other[e4315], 2)
                + f32::powi(other[e4125], 2)
                - f32::powi(other[scalar], 2)
                - f32::powi(other[e45], 2)
                - 2.0 * (other[e1234] * other[e3215]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl std::ops::Div<geometric_anti_quotient> for RoundPoint {
    type Output = geometric_anti_quotient_partial<RoundPoint>;
    fn div(self, _rhs: geometric_anti_quotient) -> Self::Output {
        geometric_anti_quotient_partial(self)
    }
}
impl GeometricAntiQuotient<AntiCircleRotor> for RoundPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        7        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        7        8        1
    //  no simd        7       11        1
    fn geometric_anti_quotient(self, other: AntiCircleRotor) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e41] * other[e15])
                + 2.0 * (other[e42] * other[e25])
                + 2.0 * (other[e43] * other[e35])
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                - f32::powi(other[e45], 2)
                - f32::powi(other[scalar], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            anti_inverse[e12345] * self[e5],
        );
    }
}
impl GeometricAntiQuotient<AntiDipoleInversion> for RoundPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10        9        1
    //    simd4        0        1        0
    // Totals...
    // yes simd       10       10        1
    //  no simd       10       13        1
    fn geometric_anti_quotient(self, other: AntiDipoleInversion) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e4] * other[e5]) + f32::powi(other[e321], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - f32::powi(other[e1], 2)
                - f32::powi(other[e2], 2)
                - f32::powi(other[e3], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            anti_inverse[e12345] * self[e5],
        );
    }
}
impl GeometricAntiQuotient<AntiDualNum> for RoundPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        1
    //  no simd        0        6        1
    fn geometric_anti_quotient(self, other: AntiDualNum) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[scalar], 2) * -1.0);
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            anti_inverse[e12345] * self[e5],
        );
    }
}
impl GeometricAntiQuotient<AntiFlatPoint> for RoundPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        5        1
    fn geometric_anti_quotient(self, other: AntiFlatPoint) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e321], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            anti_inverse[e12345] * self[e5],
        );
    }
}
impl GeometricAntiQuotient<AntiFlector> for RoundPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        1        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        2        1
    //  no simd        3        5        1
    fn geometric_anti_quotient(self, other: AntiFlector) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e321], 2) - f32::powi(other[e1], 2) - f32::powi(other[e2], 2) - f32::powi(other[e3], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            anti_inverse[e12345] * self[e5],
        );
    }
}
impl GeometricAntiQuotient<AntiLine> for RoundPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        1        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        2        1
    //  no simd        2        5        1
    fn geometric_anti_quotient(self, other: AntiLine) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e23], 2) + f32::powi(other[e31], 2) + f32::powi(other[e12], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            anti_inverse[e12345] * self[e5],
        );
    }
}
impl GeometricAntiQuotient<AntiMotor> for RoundPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        1        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        2        1
    //  no simd        3        5        1
    fn geometric_anti_quotient(self, other: AntiMotor) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e23], 2) + f32::powi(other[e31], 2) + f32::powi(other[e12], 2) - f32::powi(other[scalar], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            anti_inverse[e12345] * self[e5],
        );
    }
}
impl GeometricAntiQuotient<AntiPlane> for RoundPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        1        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        2        1
    //  no simd        2        5        1
    fn geometric_anti_quotient(self, other: AntiPlane) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ -f32::powi(other[e1], 2) - f32::powi(other[e2], 2) - f32::powi(other[e3], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            anti_inverse[e12345] * self[e5],
        );
    }
}
impl GeometricAntiQuotient<AntiScalar> for RoundPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        5        1
    fn geometric_anti_quotient(self, other: AntiScalar) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e12345], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            anti_inverse[e12345] * self[e5],
        );
    }
}
impl GeometricAntiQuotient<Circle> for RoundPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        7        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        6        8        1
    //  no simd        6       11        1
    fn geometric_anti_quotient(self, other: Circle) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e321], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            anti_inverse[e12345] * self[e5],
        );
    }
}
impl GeometricAntiQuotient<CircleRotor> for RoundPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        7        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        7        8        1
    //  no simd        7       11        1
    fn geometric_anti_quotient(self, other: CircleRotor) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e321], 2) + f32::powi(other[e12345], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            anti_inverse[e12345] * self[e5],
        );
    }
}
impl GeometricAntiQuotient<Dipole> for RoundPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        7        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        6        8        1
    //  no simd        6       11        1
    fn geometric_anti_quotient(self, other: Dipole) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e41] * other[e15])
                + 2.0 * (other[e42] * other[e25])
                + 2.0 * (other[e43] * other[e35])
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                - f32::powi(other[e45], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            anti_inverse[e12345] * self[e5],
        );
    }
}
impl GeometricAntiQuotient<DipoleInversion> for RoundPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10        9        1
    //    simd4        0        1        0
    // Totals...
    // yes simd       10       10        1
    //  no simd       10       13        1
    fn geometric_anti_quotient(self, other: DipoleInversion) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e41] * other[e15])
                + 2.0 * (other[e42] * other[e25])
                + 2.0 * (other[e43] * other[e35])
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                + f32::powi(other[e4235], 2)
                + f32::powi(other[e4315], 2)
                + f32::powi(other[e4125], 2)
                - f32::powi(other[e45], 2)
                - 2.0 * (other[e1234] * other[e3215]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            anti_inverse[e12345] * self[e5],
        );
    }
}
impl GeometricAntiQuotient<DualNum> for RoundPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        5        1
    fn geometric_anti_quotient(self, other: DualNum) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e12345], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            anti_inverse[e12345] * self[e5],
        );
    }
}
impl GeometricAntiQuotient<FlatPoint> for RoundPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        1
    //  no simd        0        6        1
    fn geometric_anti_quotient(self, other: FlatPoint) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e45], 2) * -1.0);
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            anti_inverse[e12345] * self[e5],
        );
    }
}
impl GeometricAntiQuotient<Flector> for RoundPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        1        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        2        1
    //  no simd        3        5        1
    fn geometric_anti_quotient(self, other: Flector) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e4235], 2) + f32::powi(other[e4315], 2) + f32::powi(other[e4125], 2) - f32::powi(other[e45], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            anti_inverse[e12345] * self[e5],
        );
    }
}
impl GeometricAntiQuotient<Line> for RoundPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        1        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        2        1
    //  no simd        2        5        1
    fn geometric_anti_quotient(self, other: Line) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ -f32::powi(other[e415], 2) - f32::powi(other[e425], 2) - f32::powi(other[e435], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            anti_inverse[e12345] * self[e5],
        );
    }
}
impl GeometricAntiQuotient<Motor> for RoundPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        1        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        2        1
    //  no simd        3        5        1
    fn geometric_anti_quotient(self, other: Motor) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e12345], 2) - f32::powi(other[e415], 2) - f32::powi(other[e425], 2) - f32::powi(other[e435], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            anti_inverse[e12345] * self[e5],
        );
    }
}
impl GeometricAntiQuotient<MultiVector> for RoundPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       23       17        1
    //    simd4        0        1        0
    // Totals...
    // yes simd       23       18        1
    //  no simd       23       21        1
    fn geometric_anti_quotient(self, other: MultiVector) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e4] * other[e5])
                + 2.0 * (other[e15] * other[e41])
                + 2.0 * (other[e25] * other[e42])
                + 2.0 * (other[e35] * other[e43])
                + f32::powi(other[e12345], 2)
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                + f32::powi(other[e321], 2)
                + f32::powi(other[e4235], 2)
                + f32::powi(other[e4315], 2)
                + f32::powi(other[e4125], 2)
                - f32::powi(other[scalar], 2)
                - f32::powi(other[e1], 2)
                - f32::powi(other[e2], 2)
                - f32::powi(other[e3], 2)
                - f32::powi(other[e45], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125])
                - 2.0 * (other[e3215] * other[e1234]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            anti_inverse[e12345] * self[e5],
        );
    }
}
impl GeometricAntiQuotient<Plane> for RoundPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        1        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        2        1
    //  no simd        2        5        1
    fn geometric_anti_quotient(self, other: Plane) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e4235], 2) + f32::powi(other[e4315], 2) + f32::powi(other[e4125], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            anti_inverse[e12345] * self[e5],
        );
    }
}
impl GeometricAntiQuotient<RoundPoint> for RoundPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        3        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        4        1
    //  no simd        3        7        1
    fn geometric_anti_quotient(self, other: RoundPoint) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e4] * other[e5]) - f32::powi(other[e1], 2) - f32::powi(other[e2], 2) - f32::powi(other[e3], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            anti_inverse[e12345] * self[e5],
        );
    }
}
impl GeometricAntiQuotient<Scalar> for RoundPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        1
    //  no simd        0        6        1
    fn geometric_anti_quotient(self, other: Scalar) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[scalar], 2) * -1.0);
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            anti_inverse[e12345] * self[e5],
        );
    }
}
impl GeometricAntiQuotient<Sphere> for RoundPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        3        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        4        1
    //  no simd        3        7        1
    fn geometric_anti_quotient(self, other: Sphere) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e4235], 2) + f32::powi(other[e4315], 2) + f32::powi(other[e4125], 2) - 2.0 * (other[e3215] * other[e1234]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            anti_inverse[e12345] * self[e5],
        );
    }
}
impl GeometricAntiQuotient<VersorEven> for RoundPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11        9        1
    //    simd4        0        1        0
    // Totals...
    // yes simd       11       10        1
    //  no simd       11       13        1
    fn geometric_anti_quotient(self, other: VersorEven) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e5] * other[e4]) + f32::powi(other[e12345], 2) + f32::powi(other[e321], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - f32::powi(other[e1], 2)
                - f32::powi(other[e2], 2)
                - f32::powi(other[e3], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            anti_inverse[e12345] * self[e5],
        );
    }
}
impl GeometricAntiQuotient<VersorOdd> for RoundPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11        9        1
    //    simd4        0        1        0
    // Totals...
    // yes simd       11       10        1
    //  no simd       11       13        1
    fn geometric_anti_quotient(self, other: VersorOdd) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e41] * other[e15])
                + 2.0 * (other[e42] * other[e25])
                + 2.0 * (other[e43] * other[e35])
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                + f32::powi(other[e4235], 2)
                + f32::powi(other[e4315], 2)
                + f32::powi(other[e4125], 2)
                - f32::powi(other[scalar], 2)
                - f32::powi(other[e45], 2)
                - 2.0 * (other[e1234] * other[e3215]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            anti_inverse[e12345] * self[e5],
        );
    }
}
impl std::ops::Div<geometric_anti_quotient> for Scalar {
    type Output = geometric_anti_quotient_partial<Scalar>;
    fn div(self, _rhs: geometric_anti_quotient) -> Self::Output {
        geometric_anti_quotient_partial(self)
    }
}
impl GeometricAntiQuotient<AntiCircleRotor> for Scalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        7        1
    fn geometric_anti_quotient(self, other: AntiCircleRotor) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e41] * other[e15])
                + 2.0 * (other[e42] * other[e25])
                + 2.0 * (other[e43] * other[e35])
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                - f32::powi(other[e45], 2)
                - f32::powi(other[scalar], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Scalar::from_groups(/* scalar */ anti_inverse[e12345] * self[scalar]);
    }
}
impl GeometricAntiQuotient<AntiDipoleInversion> for Scalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10        9        1
    fn geometric_anti_quotient(self, other: AntiDipoleInversion) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e4] * other[e5]) + f32::powi(other[e321], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - f32::powi(other[e1], 2)
                - f32::powi(other[e2], 2)
                - f32::powi(other[e3], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Scalar::from_groups(/* scalar */ anti_inverse[e12345] * self[scalar]);
    }
}
impl GeometricAntiQuotient<AntiDualNum> for Scalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        1
    fn geometric_anti_quotient(self, other: AntiDualNum) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[scalar], 2) * -1.0);
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Scalar::from_groups(/* scalar */ anti_inverse[e12345] * self[scalar]);
    }
}
impl GeometricAntiQuotient<AntiFlatPoint> for Scalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        1
    fn geometric_anti_quotient(self, other: AntiFlatPoint) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e321], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Scalar::from_groups(/* scalar */ anti_inverse[e12345] * self[scalar]);
    }
}
impl GeometricAntiQuotient<AntiFlector> for Scalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        1        1
    fn geometric_anti_quotient(self, other: AntiFlector) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e321], 2) - f32::powi(other[e1], 2) - f32::powi(other[e2], 2) - f32::powi(other[e3], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Scalar::from_groups(/* scalar */ anti_inverse[e12345] * self[scalar]);
    }
}
impl GeometricAntiQuotient<AntiLine> for Scalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        1        1
    fn geometric_anti_quotient(self, other: AntiLine) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e23], 2) + f32::powi(other[e31], 2) + f32::powi(other[e12], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Scalar::from_groups(/* scalar */ anti_inverse[e12345] * self[scalar]);
    }
}
impl GeometricAntiQuotient<AntiMotor> for Scalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        1        1
    fn geometric_anti_quotient(self, other: AntiMotor) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e23], 2) + f32::powi(other[e31], 2) + f32::powi(other[e12], 2) - f32::powi(other[scalar], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Scalar::from_groups(/* scalar */ anti_inverse[e12345] * self[scalar]);
    }
}
impl GeometricAntiQuotient<AntiPlane> for Scalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        1        1
    fn geometric_anti_quotient(self, other: AntiPlane) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ -f32::powi(other[e1], 2) - f32::powi(other[e2], 2) - f32::powi(other[e3], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Scalar::from_groups(/* scalar */ anti_inverse[e12345] * self[scalar]);
    }
}
impl GeometricAntiQuotient<AntiScalar> for Scalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        1
    fn geometric_anti_quotient(self, other: AntiScalar) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e12345], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Scalar::from_groups(/* scalar */ anti_inverse[e12345] * self[scalar]);
    }
}
impl GeometricAntiQuotient<Circle> for Scalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        7        1
    fn geometric_anti_quotient(self, other: Circle) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e321], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Scalar::from_groups(/* scalar */ anti_inverse[e12345] * self[scalar]);
    }
}
impl GeometricAntiQuotient<CircleRotor> for Scalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        7        1
    fn geometric_anti_quotient(self, other: CircleRotor) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e321], 2) + f32::powi(other[e12345], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Scalar::from_groups(/* scalar */ anti_inverse[e12345] * self[scalar]);
    }
}
impl GeometricAntiQuotient<Dipole> for Scalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        7        1
    fn geometric_anti_quotient(self, other: Dipole) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e41] * other[e15])
                + 2.0 * (other[e42] * other[e25])
                + 2.0 * (other[e43] * other[e35])
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                - f32::powi(other[e45], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Scalar::from_groups(/* scalar */ anti_inverse[e12345] * self[scalar]);
    }
}
impl GeometricAntiQuotient<DipoleInversion> for Scalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10        9        1
    fn geometric_anti_quotient(self, other: DipoleInversion) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e41] * other[e15])
                + 2.0 * (other[e42] * other[e25])
                + 2.0 * (other[e43] * other[e35])
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                + f32::powi(other[e4235], 2)
                + f32::powi(other[e4315], 2)
                + f32::powi(other[e4125], 2)
                - f32::powi(other[e45], 2)
                - 2.0 * (other[e1234] * other[e3215]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Scalar::from_groups(/* scalar */ anti_inverse[e12345] * self[scalar]);
    }
}
impl GeometricAntiQuotient<DualNum> for Scalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        1
    fn geometric_anti_quotient(self, other: DualNum) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e12345], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Scalar::from_groups(/* scalar */ anti_inverse[e12345] * self[scalar]);
    }
}
impl GeometricAntiQuotient<FlatPoint> for Scalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        1
    fn geometric_anti_quotient(self, other: FlatPoint) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e45], 2) * -1.0);
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Scalar::from_groups(/* scalar */ anti_inverse[e12345] * self[scalar]);
    }
}
impl GeometricAntiQuotient<Flector> for Scalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        1        1
    fn geometric_anti_quotient(self, other: Flector) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e4235], 2) + f32::powi(other[e4315], 2) + f32::powi(other[e4125], 2) - f32::powi(other[e45], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Scalar::from_groups(/* scalar */ anti_inverse[e12345] * self[scalar]);
    }
}
impl GeometricAntiQuotient<Line> for Scalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        1        1
    fn geometric_anti_quotient(self, other: Line) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ -f32::powi(other[e415], 2) - f32::powi(other[e425], 2) - f32::powi(other[e435], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Scalar::from_groups(/* scalar */ anti_inverse[e12345] * self[scalar]);
    }
}
impl GeometricAntiQuotient<Motor> for Scalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        1        1
    fn geometric_anti_quotient(self, other: Motor) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e12345], 2) - f32::powi(other[e415], 2) - f32::powi(other[e425], 2) - f32::powi(other[e435], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Scalar::from_groups(/* scalar */ anti_inverse[e12345] * self[scalar]);
    }
}
impl GeometricAntiQuotient<MultiVector> for Scalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       23       17        1
    fn geometric_anti_quotient(self, other: MultiVector) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e4] * other[e5])
                + 2.0 * (other[e15] * other[e41])
                + 2.0 * (other[e25] * other[e42])
                + 2.0 * (other[e35] * other[e43])
                + f32::powi(other[e12345], 2)
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                + f32::powi(other[e321], 2)
                + f32::powi(other[e4235], 2)
                + f32::powi(other[e4315], 2)
                + f32::powi(other[e4125], 2)
                - f32::powi(other[scalar], 2)
                - f32::powi(other[e1], 2)
                - f32::powi(other[e2], 2)
                - f32::powi(other[e3], 2)
                - f32::powi(other[e45], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125])
                - 2.0 * (other[e3215] * other[e1234]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Scalar::from_groups(/* scalar */ anti_inverse[e12345] * self[scalar]);
    }
}
impl GeometricAntiQuotient<Plane> for Scalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        1        1
    fn geometric_anti_quotient(self, other: Plane) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e4235], 2) + f32::powi(other[e4315], 2) + f32::powi(other[e4125], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Scalar::from_groups(/* scalar */ anti_inverse[e12345] * self[scalar]);
    }
}
impl GeometricAntiQuotient<RoundPoint> for Scalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        3        1
    fn geometric_anti_quotient(self, other: RoundPoint) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e4] * other[e5]) - f32::powi(other[e1], 2) - f32::powi(other[e2], 2) - f32::powi(other[e3], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Scalar::from_groups(/* scalar */ anti_inverse[e12345] * self[scalar]);
    }
}
impl GeometricAntiQuotient<Scalar> for Scalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        1
    fn geometric_anti_quotient(self, other: Scalar) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[scalar], 2) * -1.0);
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Scalar::from_groups(/* scalar */ anti_inverse[e12345] * self[scalar]);
    }
}
impl GeometricAntiQuotient<Sphere> for Scalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        3        1
    fn geometric_anti_quotient(self, other: Sphere) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e4235], 2) + f32::powi(other[e4315], 2) + f32::powi(other[e4125], 2) - 2.0 * (other[e3215] * other[e1234]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Scalar::from_groups(/* scalar */ anti_inverse[e12345] * self[scalar]);
    }
}
impl GeometricAntiQuotient<VersorEven> for Scalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       11        9        1
    fn geometric_anti_quotient(self, other: VersorEven) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e5] * other[e4]) + f32::powi(other[e12345], 2) + f32::powi(other[e321], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - f32::powi(other[e1], 2)
                - f32::powi(other[e2], 2)
                - f32::powi(other[e3], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Scalar::from_groups(/* scalar */ anti_inverse[e12345] * self[scalar]);
    }
}
impl GeometricAntiQuotient<VersorOdd> for Scalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       11        9        1
    fn geometric_anti_quotient(self, other: VersorOdd) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e41] * other[e15])
                + 2.0 * (other[e42] * other[e25])
                + 2.0 * (other[e43] * other[e35])
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                + f32::powi(other[e4235], 2)
                + f32::powi(other[e4315], 2)
                + f32::powi(other[e4125], 2)
                - f32::powi(other[scalar], 2)
                - f32::powi(other[e45], 2)
                - 2.0 * (other[e1234] * other[e3215]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Scalar::from_groups(/* scalar */ anti_inverse[e12345] * self[scalar]);
    }
}
impl std::ops::Div<geometric_anti_quotient> for Sphere {
    type Output = geometric_anti_quotient_partial<Sphere>;
    fn div(self, _rhs: geometric_anti_quotient) -> Self::Output {
        geometric_anti_quotient_partial(self)
    }
}
impl GeometricAntiQuotient<AntiCircleRotor> for Sphere {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        7        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        7        8        1
    //  no simd        7       11        1
    fn geometric_anti_quotient(self, other: AntiCircleRotor) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e41] * other[e15])
                + 2.0 * (other[e42] * other[e25])
                + 2.0 * (other[e43] * other[e35])
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                - f32::powi(other[e45], 2)
                - f32::powi(other[scalar], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
            // e1234
            anti_inverse[e12345] * self[e1234],
        );
    }
}
impl GeometricAntiQuotient<AntiDipoleInversion> for Sphere {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10        9        1
    //    simd4        0        1        0
    // Totals...
    // yes simd       10       10        1
    //  no simd       10       13        1
    fn geometric_anti_quotient(self, other: AntiDipoleInversion) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e4] * other[e5]) + f32::powi(other[e321], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - f32::powi(other[e1], 2)
                - f32::powi(other[e2], 2)
                - f32::powi(other[e3], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
            // e1234
            anti_inverse[e12345] * self[e1234],
        );
    }
}
impl GeometricAntiQuotient<AntiDualNum> for Sphere {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        1
    //  no simd        0        6        1
    fn geometric_anti_quotient(self, other: AntiDualNum) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[scalar], 2) * -1.0);
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
            // e1234
            anti_inverse[e12345] * self[e1234],
        );
    }
}
impl GeometricAntiQuotient<AntiFlatPoint> for Sphere {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        5        1
    fn geometric_anti_quotient(self, other: AntiFlatPoint) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e321], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
            // e1234
            anti_inverse[e12345] * self[e1234],
        );
    }
}
impl GeometricAntiQuotient<AntiFlector> for Sphere {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        1        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        2        1
    //  no simd        3        5        1
    fn geometric_anti_quotient(self, other: AntiFlector) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e321], 2) - f32::powi(other[e1], 2) - f32::powi(other[e2], 2) - f32::powi(other[e3], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
            // e1234
            anti_inverse[e12345] * self[e1234],
        );
    }
}
impl GeometricAntiQuotient<AntiLine> for Sphere {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        1        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        2        1
    //  no simd        2        5        1
    fn geometric_anti_quotient(self, other: AntiLine) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e23], 2) + f32::powi(other[e31], 2) + f32::powi(other[e12], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
            // e1234
            anti_inverse[e12345] * self[e1234],
        );
    }
}
impl GeometricAntiQuotient<AntiMotor> for Sphere {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        1        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        2        1
    //  no simd        3        5        1
    fn geometric_anti_quotient(self, other: AntiMotor) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e23], 2) + f32::powi(other[e31], 2) + f32::powi(other[e12], 2) - f32::powi(other[scalar], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
            // e1234
            anti_inverse[e12345] * self[e1234],
        );
    }
}
impl GeometricAntiQuotient<AntiPlane> for Sphere {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        1        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        2        1
    //  no simd        2        5        1
    fn geometric_anti_quotient(self, other: AntiPlane) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ -f32::powi(other[e1], 2) - f32::powi(other[e2], 2) - f32::powi(other[e3], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
            // e1234
            anti_inverse[e12345] * self[e1234],
        );
    }
}
impl GeometricAntiQuotient<AntiScalar> for Sphere {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        5        1
    fn geometric_anti_quotient(self, other: AntiScalar) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e12345], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
            // e1234
            anti_inverse[e12345] * self[e1234],
        );
    }
}
impl GeometricAntiQuotient<Circle> for Sphere {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        7        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        6        8        1
    //  no simd        6       11        1
    fn geometric_anti_quotient(self, other: Circle) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e321], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
            // e1234
            anti_inverse[e12345] * self[e1234],
        );
    }
}
impl GeometricAntiQuotient<CircleRotor> for Sphere {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        7        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        7        8        1
    //  no simd        7       11        1
    fn geometric_anti_quotient(self, other: CircleRotor) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e321], 2) + f32::powi(other[e12345], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
            // e1234
            anti_inverse[e12345] * self[e1234],
        );
    }
}
impl GeometricAntiQuotient<Dipole> for Sphere {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        7        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        6        8        1
    //  no simd        6       11        1
    fn geometric_anti_quotient(self, other: Dipole) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e41] * other[e15])
                + 2.0 * (other[e42] * other[e25])
                + 2.0 * (other[e43] * other[e35])
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                - f32::powi(other[e45], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
            // e1234
            anti_inverse[e12345] * self[e1234],
        );
    }
}
impl GeometricAntiQuotient<DipoleInversion> for Sphere {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10        9        1
    //    simd4        0        1        0
    // Totals...
    // yes simd       10       10        1
    //  no simd       10       13        1
    fn geometric_anti_quotient(self, other: DipoleInversion) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e41] * other[e15])
                + 2.0 * (other[e42] * other[e25])
                + 2.0 * (other[e43] * other[e35])
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                + f32::powi(other[e4235], 2)
                + f32::powi(other[e4315], 2)
                + f32::powi(other[e4125], 2)
                - f32::powi(other[e45], 2)
                - 2.0 * (other[e1234] * other[e3215]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
            // e1234
            anti_inverse[e12345] * self[e1234],
        );
    }
}
impl GeometricAntiQuotient<DualNum> for Sphere {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        5        1
    fn geometric_anti_quotient(self, other: DualNum) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e12345], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
            // e1234
            anti_inverse[e12345] * self[e1234],
        );
    }
}
impl GeometricAntiQuotient<FlatPoint> for Sphere {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        1
    //  no simd        0        6        1
    fn geometric_anti_quotient(self, other: FlatPoint) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e45], 2) * -1.0);
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
            // e1234
            anti_inverse[e12345] * self[e1234],
        );
    }
}
impl GeometricAntiQuotient<Flector> for Sphere {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        1        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        2        1
    //  no simd        3        5        1
    fn geometric_anti_quotient(self, other: Flector) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e4235], 2) + f32::powi(other[e4315], 2) + f32::powi(other[e4125], 2) - f32::powi(other[e45], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
            // e1234
            anti_inverse[e12345] * self[e1234],
        );
    }
}
impl GeometricAntiQuotient<Line> for Sphere {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        1        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        2        1
    //  no simd        2        5        1
    fn geometric_anti_quotient(self, other: Line) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ -f32::powi(other[e415], 2) - f32::powi(other[e425], 2) - f32::powi(other[e435], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
            // e1234
            anti_inverse[e12345] * self[e1234],
        );
    }
}
impl GeometricAntiQuotient<Motor> for Sphere {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        1        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        2        1
    //  no simd        3        5        1
    fn geometric_anti_quotient(self, other: Motor) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e12345], 2) - f32::powi(other[e415], 2) - f32::powi(other[e425], 2) - f32::powi(other[e435], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
            // e1234
            anti_inverse[e12345] * self[e1234],
        );
    }
}
impl GeometricAntiQuotient<MultiVector> for Sphere {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       23       17        1
    //    simd4        0        1        0
    // Totals...
    // yes simd       23       18        1
    //  no simd       23       21        1
    fn geometric_anti_quotient(self, other: MultiVector) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e4] * other[e5])
                + 2.0 * (other[e15] * other[e41])
                + 2.0 * (other[e25] * other[e42])
                + 2.0 * (other[e35] * other[e43])
                + f32::powi(other[e12345], 2)
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                + f32::powi(other[e321], 2)
                + f32::powi(other[e4235], 2)
                + f32::powi(other[e4315], 2)
                + f32::powi(other[e4125], 2)
                - f32::powi(other[scalar], 2)
                - f32::powi(other[e1], 2)
                - f32::powi(other[e2], 2)
                - f32::powi(other[e3], 2)
                - f32::powi(other[e45], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125])
                - 2.0 * (other[e3215] * other[e1234]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
            // e1234
            anti_inverse[e12345] * self[e1234],
        );
    }
}
impl GeometricAntiQuotient<Plane> for Sphere {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        1        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        2        1
    //  no simd        2        5        1
    fn geometric_anti_quotient(self, other: Plane) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e4235], 2) + f32::powi(other[e4315], 2) + f32::powi(other[e4125], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
            // e1234
            anti_inverse[e12345] * self[e1234],
        );
    }
}
impl GeometricAntiQuotient<RoundPoint> for Sphere {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        3        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        4        1
    //  no simd        3        7        1
    fn geometric_anti_quotient(self, other: RoundPoint) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e4] * other[e5]) - f32::powi(other[e1], 2) - f32::powi(other[e2], 2) - f32::powi(other[e3], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
            // e1234
            anti_inverse[e12345] * self[e1234],
        );
    }
}
impl GeometricAntiQuotient<Scalar> for Sphere {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        1
    //  no simd        0        6        1
    fn geometric_anti_quotient(self, other: Scalar) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[scalar], 2) * -1.0);
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
            // e1234
            anti_inverse[e12345] * self[e1234],
        );
    }
}
impl GeometricAntiQuotient<Sphere> for Sphere {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        3        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        4        1
    //  no simd        3        7        1
    fn geometric_anti_quotient(self, other: Sphere) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e4235], 2) + f32::powi(other[e4315], 2) + f32::powi(other[e4125], 2) - 2.0 * (other[e3215] * other[e1234]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
            // e1234
            anti_inverse[e12345] * self[e1234],
        );
    }
}
impl GeometricAntiQuotient<VersorEven> for Sphere {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11        9        1
    //    simd4        0        1        0
    // Totals...
    // yes simd       11       10        1
    //  no simd       11       13        1
    fn geometric_anti_quotient(self, other: VersorEven) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e5] * other[e4]) + f32::powi(other[e12345], 2) + f32::powi(other[e321], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - f32::powi(other[e1], 2)
                - f32::powi(other[e2], 2)
                - f32::powi(other[e3], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
            // e1234
            anti_inverse[e12345] * self[e1234],
        );
    }
}
impl GeometricAntiQuotient<VersorOdd> for Sphere {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11        9        1
    //    simd4        0        1        0
    // Totals...
    // yes simd       11       10        1
    //  no simd       11       13        1
    fn geometric_anti_quotient(self, other: VersorOdd) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e41] * other[e15])
                + 2.0 * (other[e42] * other[e25])
                + 2.0 * (other[e43] * other[e35])
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                + f32::powi(other[e4235], 2)
                + f32::powi(other[e4315], 2)
                + f32::powi(other[e4125], 2)
                - f32::powi(other[scalar], 2)
                - f32::powi(other[e45], 2)
                - 2.0 * (other[e1234] * other[e3215]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
            // e1234
            anti_inverse[e12345] * self[e1234],
        );
    }
}
impl std::ops::Div<geometric_anti_quotient> for VersorEven {
    type Output = geometric_anti_quotient_partial<VersorEven>;
    fn div(self, _rhs: geometric_anti_quotient) -> Self::Output {
        geometric_anti_quotient_partial(self)
    }
}
impl GeometricAntiQuotient<AntiCircleRotor> for VersorEven {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        6        1
    //    simd4        0        4        0
    // Totals...
    // yes simd        7       10        1
    //  no simd        7       22        1
    fn geometric_anti_quotient(self, other: AntiCircleRotor) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e41] * other[e15])
                + 2.0 * (other[e42] * other[e25])
                + 2.0 * (other[e43] * other[e35])
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                - f32::powi(other[e45], 2)
                - f32::powi(other[scalar], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e423], self[e431], self[e412], self[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
        );
    }
}
impl GeometricAntiQuotient<AntiDipoleInversion> for VersorEven {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10        8        1
    //    simd4        0        4        0
    // Totals...
    // yes simd       10       12        1
    //  no simd       10       24        1
    fn geometric_anti_quotient(self, other: AntiDipoleInversion) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e4] * other[e5]) + f32::powi(other[e321], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - f32::powi(other[e1], 2)
                - f32::powi(other[e2], 2)
                - f32::powi(other[e3], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e423], self[e431], self[e412], self[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
        );
    }
}
impl GeometricAntiQuotient<AntiDualNum> for VersorEven {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd4        0        4        0
    // Totals...
    // yes simd        0        5        1
    //  no simd        0       17        1
    fn geometric_anti_quotient(self, other: AntiDualNum) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[scalar], 2) * -1.0);
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e423], self[e431], self[e412], self[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
        );
    }
}
impl GeometricAntiQuotient<AntiFlatPoint> for VersorEven {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd4        0        4        0
    // Totals...
    // yes simd        0        4        1
    //  no simd        0       16        1
    fn geometric_anti_quotient(self, other: AntiFlatPoint) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e321], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e423], self[e431], self[e412], self[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
        );
    }
}
impl GeometricAntiQuotient<AntiFlector> for VersorEven {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd4        0        4        0
    // Totals...
    // yes simd        3        4        1
    //  no simd        3       16        1
    fn geometric_anti_quotient(self, other: AntiFlector) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e321], 2) - f32::powi(other[e1], 2) - f32::powi(other[e2], 2) - f32::powi(other[e3], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e423], self[e431], self[e412], self[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
        );
    }
}
impl GeometricAntiQuotient<AntiLine> for VersorEven {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd4        0        4        0
    // Totals...
    // yes simd        2        4        1
    //  no simd        2       16        1
    fn geometric_anti_quotient(self, other: AntiLine) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e23], 2) + f32::powi(other[e31], 2) + f32::powi(other[e12], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e423], self[e431], self[e412], self[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
        );
    }
}
impl GeometricAntiQuotient<AntiMotor> for VersorEven {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd4        0        4        0
    // Totals...
    // yes simd        3        4        1
    //  no simd        3       16        1
    fn geometric_anti_quotient(self, other: AntiMotor) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e23], 2) + f32::powi(other[e31], 2) + f32::powi(other[e12], 2) - f32::powi(other[scalar], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e423], self[e431], self[e412], self[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
        );
    }
}
impl GeometricAntiQuotient<AntiPlane> for VersorEven {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd4        0        4        0
    // Totals...
    // yes simd        2        4        1
    //  no simd        2       16        1
    fn geometric_anti_quotient(self, other: AntiPlane) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ -f32::powi(other[e1], 2) - f32::powi(other[e2], 2) - f32::powi(other[e3], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e423], self[e431], self[e412], self[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
        );
    }
}
impl GeometricAntiQuotient<AntiScalar> for VersorEven {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd4        0        4        0
    // Totals...
    // yes simd        0        4        1
    //  no simd        0       16        1
    fn geometric_anti_quotient(self, other: AntiScalar) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e12345], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e423], self[e431], self[e412], self[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
        );
    }
}
impl GeometricAntiQuotient<Circle> for VersorEven {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        6        1
    //    simd4        0        4        0
    // Totals...
    // yes simd        6       10        1
    //  no simd        6       22        1
    fn geometric_anti_quotient(self, other: Circle) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e321], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e423], self[e431], self[e412], self[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
        );
    }
}
impl GeometricAntiQuotient<CircleRotor> for VersorEven {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        6        1
    //    simd4        0        4        0
    // Totals...
    // yes simd        7       10        1
    //  no simd        7       22        1
    fn geometric_anti_quotient(self, other: CircleRotor) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e321], 2) + f32::powi(other[e12345], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e423], self[e431], self[e412], self[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
        );
    }
}
impl GeometricAntiQuotient<Dipole> for VersorEven {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        6        1
    //    simd4        0        4        0
    // Totals...
    // yes simd        6       10        1
    //  no simd        6       22        1
    fn geometric_anti_quotient(self, other: Dipole) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e41] * other[e15])
                + 2.0 * (other[e42] * other[e25])
                + 2.0 * (other[e43] * other[e35])
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                - f32::powi(other[e45], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e423], self[e431], self[e412], self[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
        );
    }
}
impl GeometricAntiQuotient<DipoleInversion> for VersorEven {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10        8        1
    //    simd4        0        4        0
    // Totals...
    // yes simd       10       12        1
    //  no simd       10       24        1
    fn geometric_anti_quotient(self, other: DipoleInversion) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e41] * other[e15])
                + 2.0 * (other[e42] * other[e25])
                + 2.0 * (other[e43] * other[e35])
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                + f32::powi(other[e4235], 2)
                + f32::powi(other[e4315], 2)
                + f32::powi(other[e4125], 2)
                - f32::powi(other[e45], 2)
                - 2.0 * (other[e1234] * other[e3215]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e423], self[e431], self[e412], self[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
        );
    }
}
impl GeometricAntiQuotient<DualNum> for VersorEven {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd4        0        4        0
    // Totals...
    // yes simd        0        4        1
    //  no simd        0       16        1
    fn geometric_anti_quotient(self, other: DualNum) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e12345], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e423], self[e431], self[e412], self[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
        );
    }
}
impl GeometricAntiQuotient<FlatPoint> for VersorEven {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd4        0        4        0
    // Totals...
    // yes simd        0        5        1
    //  no simd        0       17        1
    fn geometric_anti_quotient(self, other: FlatPoint) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e45], 2) * -1.0);
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e423], self[e431], self[e412], self[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
        );
    }
}
impl GeometricAntiQuotient<Flector> for VersorEven {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd4        0        4        0
    // Totals...
    // yes simd        3        4        1
    //  no simd        3       16        1
    fn geometric_anti_quotient(self, other: Flector) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e4235], 2) + f32::powi(other[e4315], 2) + f32::powi(other[e4125], 2) - f32::powi(other[e45], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e423], self[e431], self[e412], self[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
        );
    }
}
impl GeometricAntiQuotient<Line> for VersorEven {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd4        0        4        0
    // Totals...
    // yes simd        2        4        1
    //  no simd        2       16        1
    fn geometric_anti_quotient(self, other: Line) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ -f32::powi(other[e415], 2) - f32::powi(other[e425], 2) - f32::powi(other[e435], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e423], self[e431], self[e412], self[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
        );
    }
}
impl GeometricAntiQuotient<Motor> for VersorEven {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd4        0        4        0
    // Totals...
    // yes simd        3        4        1
    //  no simd        3       16        1
    fn geometric_anti_quotient(self, other: Motor) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e12345], 2) - f32::powi(other[e415], 2) - f32::powi(other[e425], 2) - f32::powi(other[e435], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e423], self[e431], self[e412], self[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
        );
    }
}
impl GeometricAntiQuotient<MultiVector> for VersorEven {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       23       16        1
    //    simd4        0        4        0
    // Totals...
    // yes simd       23       20        1
    //  no simd       23       32        1
    fn geometric_anti_quotient(self, other: MultiVector) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e4] * other[e5])
                + 2.0 * (other[e15] * other[e41])
                + 2.0 * (other[e25] * other[e42])
                + 2.0 * (other[e35] * other[e43])
                + f32::powi(other[e12345], 2)
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                + f32::powi(other[e321], 2)
                + f32::powi(other[e4235], 2)
                + f32::powi(other[e4315], 2)
                + f32::powi(other[e4125], 2)
                - f32::powi(other[scalar], 2)
                - f32::powi(other[e1], 2)
                - f32::powi(other[e2], 2)
                - f32::powi(other[e3], 2)
                - f32::powi(other[e45], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125])
                - 2.0 * (other[e3215] * other[e1234]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e423], self[e431], self[e412], self[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
        );
    }
}
impl GeometricAntiQuotient<Plane> for VersorEven {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd4        0        4        0
    // Totals...
    // yes simd        2        4        1
    //  no simd        2       16        1
    fn geometric_anti_quotient(self, other: Plane) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e4235], 2) + f32::powi(other[e4315], 2) + f32::powi(other[e4125], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e423], self[e431], self[e412], self[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
        );
    }
}
impl GeometricAntiQuotient<RoundPoint> for VersorEven {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        2        1
    //    simd4        0        4        0
    // Totals...
    // yes simd        3        6        1
    //  no simd        3       18        1
    fn geometric_anti_quotient(self, other: RoundPoint) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e4] * other[e5]) - f32::powi(other[e1], 2) - f32::powi(other[e2], 2) - f32::powi(other[e3], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e423], self[e431], self[e412], self[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
        );
    }
}
impl GeometricAntiQuotient<Scalar> for VersorEven {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd4        0        4        0
    // Totals...
    // yes simd        0        5        1
    //  no simd        0       17        1
    fn geometric_anti_quotient(self, other: Scalar) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[scalar], 2) * -1.0);
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e423], self[e431], self[e412], self[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
        );
    }
}
impl GeometricAntiQuotient<Sphere> for VersorEven {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        2        1
    //    simd4        0        4        0
    // Totals...
    // yes simd        3        6        1
    //  no simd        3       18        1
    fn geometric_anti_quotient(self, other: Sphere) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e4235], 2) + f32::powi(other[e4315], 2) + f32::powi(other[e4125], 2) - 2.0 * (other[e3215] * other[e1234]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e423], self[e431], self[e412], self[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
        );
    }
}
impl GeometricAntiQuotient<VersorEven> for VersorEven {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11        8        1
    //    simd4        0        4        0
    // Totals...
    // yes simd       11       12        1
    //  no simd       11       24        1
    fn geometric_anti_quotient(self, other: VersorEven) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e5] * other[e4]) + f32::powi(other[e12345], 2) + f32::powi(other[e321], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - f32::powi(other[e1], 2)
                - f32::powi(other[e2], 2)
                - f32::powi(other[e3], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e423], self[e431], self[e412], self[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
        );
    }
}
impl GeometricAntiQuotient<VersorOdd> for VersorEven {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11        8        1
    //    simd4        0        4        0
    // Totals...
    // yes simd       11       12        1
    //  no simd       11       24        1
    fn geometric_anti_quotient(self, other: VersorOdd) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e41] * other[e15])
                + 2.0 * (other[e42] * other[e25])
                + 2.0 * (other[e43] * other[e35])
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                + f32::powi(other[e4235], 2)
                + f32::powi(other[e4315], 2)
                + f32::powi(other[e4125], 2)
                - f32::powi(other[scalar], 2)
                - f32::powi(other[e45], 2)
                - 2.0 * (other[e1234] * other[e3215]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e423], self[e431], self[e412], self[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
        );
    }
}
impl std::ops::Div<geometric_anti_quotient> for VersorOdd {
    type Output = geometric_anti_quotient_partial<VersorOdd>;
    fn div(self, _rhs: geometric_anti_quotient) -> Self::Output {
        geometric_anti_quotient_partial(self)
    }
}
impl GeometricAntiQuotient<AntiCircleRotor> for VersorOdd {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        6        1
    //    simd4        0        4        0
    // Totals...
    // yes simd        7       10        1
    //  no simd        7       22        1
    fn geometric_anti_quotient(self, other: AntiCircleRotor) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e41] * other[e15])
                + 2.0 * (other[e42] * other[e25])
                + 2.0 * (other[e43] * other[e35])
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                - f32::powi(other[e45], 2)
                - f32::powi(other[scalar], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e41], self[e42], self[e43], self[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<AntiDipoleInversion> for VersorOdd {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10        8        1
    //    simd4        0        4        0
    // Totals...
    // yes simd       10       12        1
    //  no simd       10       24        1
    fn geometric_anti_quotient(self, other: AntiDipoleInversion) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e4] * other[e5]) + f32::powi(other[e321], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - f32::powi(other[e1], 2)
                - f32::powi(other[e2], 2)
                - f32::powi(other[e3], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e41], self[e42], self[e43], self[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<AntiDualNum> for VersorOdd {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd4        0        4        0
    // Totals...
    // yes simd        0        5        1
    //  no simd        0       17        1
    fn geometric_anti_quotient(self, other: AntiDualNum) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[scalar], 2) * -1.0);
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e41], self[e42], self[e43], self[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<AntiFlatPoint> for VersorOdd {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd4        0        4        0
    // Totals...
    // yes simd        0        4        1
    //  no simd        0       16        1
    fn geometric_anti_quotient(self, other: AntiFlatPoint) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e321], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e41], self[e42], self[e43], self[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<AntiFlector> for VersorOdd {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd4        0        4        0
    // Totals...
    // yes simd        3        4        1
    //  no simd        3       16        1
    fn geometric_anti_quotient(self, other: AntiFlector) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e321], 2) - f32::powi(other[e1], 2) - f32::powi(other[e2], 2) - f32::powi(other[e3], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e41], self[e42], self[e43], self[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<AntiLine> for VersorOdd {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd4        0        4        0
    // Totals...
    // yes simd        2        4        1
    //  no simd        2       16        1
    fn geometric_anti_quotient(self, other: AntiLine) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e23], 2) + f32::powi(other[e31], 2) + f32::powi(other[e12], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e41], self[e42], self[e43], self[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<AntiMotor> for VersorOdd {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd4        0        4        0
    // Totals...
    // yes simd        3        4        1
    //  no simd        3       16        1
    fn geometric_anti_quotient(self, other: AntiMotor) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e23], 2) + f32::powi(other[e31], 2) + f32::powi(other[e12], 2) - f32::powi(other[scalar], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e41], self[e42], self[e43], self[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<AntiPlane> for VersorOdd {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd4        0        4        0
    // Totals...
    // yes simd        2        4        1
    //  no simd        2       16        1
    fn geometric_anti_quotient(self, other: AntiPlane) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ -f32::powi(other[e1], 2) - f32::powi(other[e2], 2) - f32::powi(other[e3], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e41], self[e42], self[e43], self[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<AntiScalar> for VersorOdd {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd4        0        4        0
    // Totals...
    // yes simd        0        4        1
    //  no simd        0       16        1
    fn geometric_anti_quotient(self, other: AntiScalar) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e12345], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e41], self[e42], self[e43], self[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<Circle> for VersorOdd {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        6        1
    //    simd4        0        4        0
    // Totals...
    // yes simd        6       10        1
    //  no simd        6       22        1
    fn geometric_anti_quotient(self, other: Circle) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e321], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e41], self[e42], self[e43], self[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<CircleRotor> for VersorOdd {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        6        1
    //    simd4        0        4        0
    // Totals...
    // yes simd        7       10        1
    //  no simd        7       22        1
    fn geometric_anti_quotient(self, other: CircleRotor) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e321], 2) + f32::powi(other[e12345], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e41], self[e42], self[e43], self[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<Dipole> for VersorOdd {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        6        1
    //    simd4        0        4        0
    // Totals...
    // yes simd        6       10        1
    //  no simd        6       22        1
    fn geometric_anti_quotient(self, other: Dipole) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e41] * other[e15])
                + 2.0 * (other[e42] * other[e25])
                + 2.0 * (other[e43] * other[e35])
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                - f32::powi(other[e45], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e41], self[e42], self[e43], self[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<DipoleInversion> for VersorOdd {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10        8        1
    //    simd4        0        4        0
    // Totals...
    // yes simd       10       12        1
    //  no simd       10       24        1
    fn geometric_anti_quotient(self, other: DipoleInversion) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e41] * other[e15])
                + 2.0 * (other[e42] * other[e25])
                + 2.0 * (other[e43] * other[e35])
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                + f32::powi(other[e4235], 2)
                + f32::powi(other[e4315], 2)
                + f32::powi(other[e4125], 2)
                - f32::powi(other[e45], 2)
                - 2.0 * (other[e1234] * other[e3215]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e41], self[e42], self[e43], self[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<DualNum> for VersorOdd {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd4        0        4        0
    // Totals...
    // yes simd        0        4        1
    //  no simd        0       16        1
    fn geometric_anti_quotient(self, other: DualNum) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e12345], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e41], self[e42], self[e43], self[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<FlatPoint> for VersorOdd {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd4        0        4        0
    // Totals...
    // yes simd        0        5        1
    //  no simd        0       17        1
    fn geometric_anti_quotient(self, other: FlatPoint) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e45], 2) * -1.0);
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e41], self[e42], self[e43], self[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<Flector> for VersorOdd {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd4        0        4        0
    // Totals...
    // yes simd        3        4        1
    //  no simd        3       16        1
    fn geometric_anti_quotient(self, other: Flector) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e4235], 2) + f32::powi(other[e4315], 2) + f32::powi(other[e4125], 2) - f32::powi(other[e45], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e41], self[e42], self[e43], self[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<Line> for VersorOdd {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd4        0        4        0
    // Totals...
    // yes simd        2        4        1
    //  no simd        2       16        1
    fn geometric_anti_quotient(self, other: Line) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ -f32::powi(other[e415], 2) - f32::powi(other[e425], 2) - f32::powi(other[e435], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e41], self[e42], self[e43], self[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<Motor> for VersorOdd {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd4        0        4        0
    // Totals...
    // yes simd        3        4        1
    //  no simd        3       16        1
    fn geometric_anti_quotient(self, other: Motor) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e12345], 2) - f32::powi(other[e415], 2) - f32::powi(other[e425], 2) - f32::powi(other[e435], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e41], self[e42], self[e43], self[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<MultiVector> for VersorOdd {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       23       16        1
    //    simd4        0        4        0
    // Totals...
    // yes simd       23       20        1
    //  no simd       23       32        1
    fn geometric_anti_quotient(self, other: MultiVector) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e4] * other[e5])
                + 2.0 * (other[e15] * other[e41])
                + 2.0 * (other[e25] * other[e42])
                + 2.0 * (other[e35] * other[e43])
                + f32::powi(other[e12345], 2)
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                + f32::powi(other[e321], 2)
                + f32::powi(other[e4235], 2)
                + f32::powi(other[e4315], 2)
                + f32::powi(other[e4125], 2)
                - f32::powi(other[scalar], 2)
                - f32::powi(other[e1], 2)
                - f32::powi(other[e2], 2)
                - f32::powi(other[e3], 2)
                - f32::powi(other[e45], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125])
                - 2.0 * (other[e3215] * other[e1234]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e41], self[e42], self[e43], self[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<Plane> for VersorOdd {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd4        0        4        0
    // Totals...
    // yes simd        2        4        1
    //  no simd        2       16        1
    fn geometric_anti_quotient(self, other: Plane) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[e4235], 2) + f32::powi(other[e4315], 2) + f32::powi(other[e4125], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e41], self[e42], self[e43], self[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<RoundPoint> for VersorOdd {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        2        1
    //    simd4        0        4        0
    // Totals...
    // yes simd        3        6        1
    //  no simd        3       18        1
    fn geometric_anti_quotient(self, other: RoundPoint) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e4] * other[e5]) - f32::powi(other[e1], 2) - f32::powi(other[e2], 2) - f32::powi(other[e3], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e41], self[e42], self[e43], self[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<Scalar> for VersorOdd {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd4        0        4        0
    // Totals...
    // yes simd        0        5        1
    //  no simd        0       17        1
    fn geometric_anti_quotient(self, other: Scalar) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(other[scalar], 2) * -1.0);
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e41], self[e42], self[e43], self[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<Sphere> for VersorOdd {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        2        1
    //    simd4        0        4        0
    // Totals...
    // yes simd        3        6        1
    //  no simd        3       18        1
    fn geometric_anti_quotient(self, other: Sphere) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(other[e4235], 2) + f32::powi(other[e4315], 2) + f32::powi(other[e4125], 2) - 2.0 * (other[e3215] * other[e1234]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e41], self[e42], self[e43], self[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<VersorEven> for VersorOdd {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11        8        1
    //    simd4        0        4        0
    // Totals...
    // yes simd       11       12        1
    //  no simd       11       24        1
    fn geometric_anti_quotient(self, other: VersorEven) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e5] * other[e4]) + f32::powi(other[e12345], 2) + f32::powi(other[e321], 2)
                - f32::powi(other[e415], 2)
                - f32::powi(other[e425], 2)
                - f32::powi(other[e435], 2)
                - f32::powi(other[e1], 2)
                - f32::powi(other[e2], 2)
                - f32::powi(other[e3], 2)
                - 2.0 * (other[e423] * other[e235])
                - 2.0 * (other[e431] * other[e315])
                - 2.0 * (other[e412] * other[e125]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e41], self[e42], self[e43], self[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl GeometricAntiQuotient<VersorOdd> for VersorOdd {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11        8        1
    //    simd4        0        4        0
    // Totals...
    // yes simd       11       12        1
    //  no simd       11       24        1
    fn geometric_anti_quotient(self, other: VersorOdd) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (other[e41] * other[e15])
                + 2.0 * (other[e42] * other[e25])
                + 2.0 * (other[e43] * other[e35])
                + f32::powi(other[e23], 2)
                + f32::powi(other[e31], 2)
                + f32::powi(other[e12], 2)
                + f32::powi(other[e4235], 2)
                + f32::powi(other[e4315], 2)
                + f32::powi(other[e4125], 2)
                - f32::powi(other[scalar], 2)
                - f32::powi(other[e45], 2)
                - 2.0 * (other[e1234] * other[e3215]),
        );
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e41], self[e42], self[e43], self[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
