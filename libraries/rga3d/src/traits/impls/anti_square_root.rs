// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
impl AntiSquareRoot for AntiScalar {
    fn anti_square_root(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */ f32::powf(self[e1234], 0.5));
    }
}