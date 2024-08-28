// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
impl SquareRoot for Scalar {
    fn square_root(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ f32::powf(self[scalar], 0.5));
    }
}
