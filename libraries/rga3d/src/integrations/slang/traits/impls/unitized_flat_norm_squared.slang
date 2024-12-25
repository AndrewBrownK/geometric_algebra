using traits::FlatBulkNormSquared;
using traits::FlatWeightNormSquared;
impl std::ops::Div<unitized_flat_norm_squared> for Flector {
    type Output = float;
    fn div(self, _rhs: unitized_flat_norm_squared) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for Flector {
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        6        8        1
//    simd4        0        2        0
// Totals...
// yes simd        6       10        1
//  no simd        6       16        1
    fn unitized_flat_norm_squared(self) -> float {
        use crate::elements::*;
        return self.flat_bulk_norm_squared()[scalar] / (self.flat_weight_norm_squared()[e1234]);
    }
}
impl std::ops::Div<unitized_flat_norm_squared> for Line {
    type Output = float;
    fn div(self, _rhs: unitized_flat_norm_squared) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for Line {
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        2        4        1
//    simd4        0        2        0
// Totals...
// yes simd        2        6        1
//  no simd        2       12        1
    fn unitized_flat_norm_squared(self) -> float {
        use crate::elements::*;
        return self.flat_bulk_norm_squared()[scalar] / (self.flat_weight_norm_squared()[e1234]);
    }
}
impl std::ops::Div<unitized_flat_norm_squared> for Motor {
    type Output = float;
    fn div(self, _rhs: unitized_flat_norm_squared) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for Motor {
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        6        9        1
//    simd4        0        2        0
// Totals...
// yes simd        6       11        1
//  no simd        6       17        1
    fn unitized_flat_norm_squared(self) -> float {
        use crate::elements::*;
        return self.flat_bulk_norm_squared()[scalar] / (self.flat_weight_norm_squared()[e1234]);
    }
}
impl std::ops::Div<unitized_flat_norm_squared> for MultiVector {
    type Output = float;
    fn div(self, _rhs: unitized_flat_norm_squared) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for MultiVector {
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32       14       18        1
//    simd2        0        1        0
//    simd3        0        2        0
//    simd4        0        2        0
// Totals...
// yes simd       14       23        1
//  no simd       14       34        1
    fn unitized_flat_norm_squared(self) -> float {
        use crate::elements::*;
        return self.flat_bulk_norm_squared()[scalar] / (self.flat_weight_norm_squared()[e1234]);
    }
}
impl std::ops::Div<unitized_flat_norm_squared> for Point {
    type Output = float;
    fn div(self, _rhs: unitized_flat_norm_squared) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for Point {
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        2        4        1
//    simd3        0        2        0
// Totals...
// yes simd        2        6        1
//  no simd        2       10        1
    fn unitized_flat_norm_squared(self) -> float {
        use crate::elements::*;
        return self.flat_bulk_norm_squared()[scalar] / (self.flat_weight_norm_squared()[e1234]);
    }
}
