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
//  Minimum:         0       0       0
//   Median:         0       0       0
//  Average:         0       0       0
//  Maximum:         0       0       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       0       0
//  Average:         0       0       0
//  Maximum:         0       0       0
impl AntiGrade for AntiScalar {
    fn anti_grade() -> usize {
        return 0;
    }
}
impl AntiGrade for Circle {
    fn anti_grade() -> usize {
        return 2;
    }
}
impl AntiGrade for Dipole {
    fn anti_grade() -> usize {
        return 3;
    }
}
impl AntiGrade for FlatPoint {
    fn anti_grade() -> usize {
        return 3;
    }
}
impl AntiGrade for Line {
    fn anti_grade() -> usize {
        return 2;
    }
}
impl AntiGrade for Plane {
    fn anti_grade() -> usize {
        return 1;
    }
}
impl AntiGrade for RoundPoint {
    fn anti_grade() -> usize {
        return 4;
    }
}
impl AntiGrade for Scalar {
    fn anti_grade() -> usize {
        return 5;
    }
}
impl AntiGrade for Sphere {
    fn anti_grade() -> usize {
        return 1;
    }
}
