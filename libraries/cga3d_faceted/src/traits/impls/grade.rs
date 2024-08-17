// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 42
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       0       0
//  Average:         0       0       0
//  Maximum:         0       0       0
//
//  No SIMD:   add/sub    mul    div
//  Minimum:         0       0       0
//   Median:         0       0       0
//  Average:         0       0       0
//  Maximum:         0       0       0
impl Grade for AntiCircleOnOrigin {
    fn grade() -> usize {
        return 2;
    }
}
impl Grade for AntiDipoleOnOrigin {
    fn grade() -> usize {
        return 3;
    }
}
impl Grade for AntiFlatOrigin {
    fn grade() -> usize {
        return 3;
    }
}
impl Grade for AntiFlatPoint {
    fn grade() -> usize {
        return 3;
    }
}
impl Grade for AntiLine {
    fn grade() -> usize {
        return 2;
    }
}
impl Grade for AntiLineOnOrigin {
    fn grade() -> usize {
        return 2;
    }
}
impl Grade for AntiPlane {
    fn grade() -> usize {
        return 1;
    }
}
impl Grade for AntiPlaneOnOrigin {
    fn grade() -> usize {
        return 1;
    }
}
impl Grade for AntiScalar {
    fn grade() -> usize {
        return 5;
    }
}
impl Grade for AntiSphereOnOrigin {
    fn grade() -> usize {
        return 1;
    }
}
impl Grade for Circle {
    fn grade() -> usize {
        return 3;
    }
}
impl Grade for CircleAligningOrigin {
    fn grade() -> usize {
        return 3;
    }
}
impl Grade for CircleAtInfinity {
    fn grade() -> usize {
        return 3;
    }
}
impl Grade for CircleAtOrigin {
    fn grade() -> usize {
        return 3;
    }
}
impl Grade for CircleOnOrigin {
    fn grade() -> usize {
        return 3;
    }
}
impl Grade for CircleOrthogonalOrigin {
    fn grade() -> usize {
        return 3;
    }
}
impl Grade for Dipole {
    fn grade() -> usize {
        return 2;
    }
}
impl Grade for DipoleAligningOrigin {
    fn grade() -> usize {
        return 2;
    }
}
impl Grade for DipoleAtInfinity {
    fn grade() -> usize {
        return 2;
    }
}
impl Grade for DipoleAtOrigin {
    fn grade() -> usize {
        return 2;
    }
}
impl Grade for DipoleOnOrigin {
    fn grade() -> usize {
        return 2;
    }
}
impl Grade for DipoleOrthogonalOrigin {
    fn grade() -> usize {
        return 2;
    }
}
impl Grade for FlatOrigin {
    fn grade() -> usize {
        return 2;
    }
}
impl Grade for FlatPoint {
    fn grade() -> usize {
        return 2;
    }
}
impl Grade for FlatPointAtInfinity {
    fn grade() -> usize {
        return 2;
    }
}
impl Grade for Horizon {
    fn grade() -> usize {
        return 4;
    }
}
impl Grade for Infinity {
    fn grade() -> usize {
        return 1;
    }
}
impl Grade for Line {
    fn grade() -> usize {
        return 3;
    }
}
impl Grade for LineAtInfinity {
    fn grade() -> usize {
        return 3;
    }
}
impl Grade for LineOnOrigin {
    fn grade() -> usize {
        return 3;
    }
}
impl Grade for NullCircleAtOrigin {
    fn grade() -> usize {
        return 3;
    }
}
impl Grade for NullDipoleAtOrigin {
    fn grade() -> usize {
        return 2;
    }
}
impl Grade for NullSphereAtOrigin {
    fn grade() -> usize {
        return 4;
    }
}
impl Grade for Origin {
    fn grade() -> usize {
        return 1;
    }
}
impl Grade for Plane {
    fn grade() -> usize {
        return 4;
    }
}
impl Grade for PlaneOnOrigin {
    fn grade() -> usize {
        return 4;
    }
}
impl Grade for RoundPoint {
    fn grade() -> usize {
        return 1;
    }
}
impl Grade for RoundPointAtOrigin {
    fn grade() -> usize {
        return 1;
    }
}
impl Grade for Scalar {
    fn grade() -> usize {
        return 0;
    }
}
impl Grade for Sphere {
    fn grade() -> usize {
        return 4;
    }
}
impl Grade for SphereAtOrigin {
    fn grade() -> usize {
        return 4;
    }
}
impl Grade for SphereOnOrigin {
    fn grade() -> usize {
        return 4;
    }
}
