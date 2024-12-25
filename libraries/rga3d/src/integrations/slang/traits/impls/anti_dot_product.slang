impl std::ops::Div<anti_dot_product> for AntiScalar {
    type Output = anti_dot_product_partial<AntiScalar>;
    fn div(self, _rhs: anti_dot_product) -> Self::Output {
        anti_dot_product_partial(self)
    }
}
impl AntiDotProduct<AntiScalar> for AntiScalar {
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        0        1        0
    fn anti_dot_product(self, other: AntiScalar) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */other[e1234] * self[e1234]);
    }
}
impl AntiDotProduct<DualNum> for AntiScalar {
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        0        1        0
    fn anti_dot_product(self, other: DualNum) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */self[e1234] * other[e1234]);
    }
}
impl AntiDotProduct<Motor> for AntiScalar {
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        0        1        0
    fn anti_dot_product(self, other: Motor) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */self[e1234] * other[e1234]);
    }
}
impl AntiDotProduct<MultiVector> for AntiScalar {
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        0        1        0
    fn anti_dot_product(self, other: MultiVector) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */self[e1234] * other[e1234]);
    }
}
impl std::ops::Div<anti_dot_product> for DualNum {
    type Output = anti_dot_product_partial<DualNum>;
    fn div(self, _rhs: anti_dot_product) -> Self::Output {
        anti_dot_product_partial(self)
    }
}
impl AntiDotProduct<AntiScalar> for DualNum {
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        0        1        0
    fn anti_dot_product(self, other: AntiScalar) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */other[e1234] * self[e1234]);
    }
}
impl AntiDotProduct<DualNum> for DualNum {
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        0        1        0
    fn anti_dot_product(self, other: DualNum) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */other[e1234] * self[e1234]);
    }
}
impl AntiDotProduct<Motor> for DualNum {
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        0        1        0
    fn anti_dot_product(self, other: Motor) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */self[e1234] * other[e1234]);
    }
}
impl AntiDotProduct<MultiVector> for DualNum {
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        0        1        0
    fn anti_dot_product(self, other: MultiVector) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */self[e1234] * other[e1234]);
    }
}
impl std::ops::Div<anti_dot_product> for Flector {
    type Output = anti_dot_product_partial<Flector>;
    fn div(self, _rhs: anti_dot_product) -> Self::Output {
        anti_dot_product_partial(self)
    }
}
impl AntiDotProduct<Flector> for Flector {
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        3        4        0
    fn anti_dot_product(self, other: Flector) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */(other[e4] * self[e4]) + (other[e423] * self[e423]) + (other[e431] * self[e431]) + (other[e412] * self[e412]));
    }
}
impl AntiDotProduct<MultiVector> for Flector {
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        3        4        0
    fn anti_dot_product(self, other: MultiVector) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */(self[e4] * other[e4]) + (self[e423] * other[e423]) + (self[e431] * other[e431]) + (self[e412] * other[e412]));
    }
}
impl AntiDotProduct<Origin> for Flector {
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        0        1        0
    fn anti_dot_product(self, other: Origin) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */self[e4] * other[e4]);
    }
}
impl AntiDotProduct<Plane> for Flector {
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        2        3        0
    fn anti_dot_product(self, other: Plane) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */(self[e423] * other[e423]) + (self[e431] * other[e431]) + (self[e412] * other[e412]));
    }
}
impl AntiDotProduct<Point> for Flector {
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        0        1        0
    fn anti_dot_product(self, other: Point) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */self[e4] * other[e4]);
    }
}
impl std::ops::Div<anti_dot_product> for Line {
    type Output = anti_dot_product_partial<Line>;
    fn div(self, _rhs: anti_dot_product) -> Self::Output {
        anti_dot_product_partial(self)
    }
}
impl AntiDotProduct<Line> for Line {
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        2        3        0
    fn anti_dot_product(self, other: Line) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */(other[e41] * self[e41]) + (other[e42] * self[e42]) + (other[e43] * self[e43]));
    }
}
impl AntiDotProduct<Motor> for Line {
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        2        3        0
    fn anti_dot_product(self, other: Motor) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */(self[e41] * other[e41]) + (self[e42] * other[e42]) + (self[e43] * other[e43]));
    }
}
impl AntiDotProduct<MultiVector> for Line {
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        2        3        0
    fn anti_dot_product(self, other: MultiVector) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */(self[e41] * other[e41]) + (self[e42] * other[e42]) + (self[e43] * other[e43]));
    }
}
impl std::ops::Div<anti_dot_product> for Motor {
    type Output = anti_dot_product_partial<Motor>;
    fn div(self, _rhs: anti_dot_product) -> Self::Output {
        anti_dot_product_partial(self)
    }
}
impl AntiDotProduct<AntiScalar> for Motor {
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        0        1        0
    fn anti_dot_product(self, other: AntiScalar) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */other[e1234] * self[e1234]);
    }
}
impl AntiDotProduct<DualNum> for Motor {
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        0        1        0
    fn anti_dot_product(self, other: DualNum) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */other[e1234] * self[e1234]);
    }
}
impl AntiDotProduct<Line> for Motor {
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        2        3        0
    fn anti_dot_product(self, other: Line) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */(other[e41] * self[e41]) + (other[e42] * self[e42]) + (other[e43] * self[e43]));
    }
}
impl AntiDotProduct<Motor> for Motor {
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        3        4        0
    fn anti_dot_product(self, other: Motor) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */(other[e41] * self[e41]) + (other[e42] * self[e42]) + (other[e43] * self[e43]) + (other[e1234] * self[e1234]));
    }
}
impl AntiDotProduct<MultiVector> for Motor {
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        3        4        0
    fn anti_dot_product(self, other: MultiVector) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */(self[e41] * other[e41]) + (self[e42] * other[e42]) + (self[e43] * other[e43]) + (self[e1234] * other[e1234]));
    }
}
impl std::ops::Div<anti_dot_product> for MultiVector {
    type Output = anti_dot_product_partial<MultiVector>;
    fn div(self, _rhs: anti_dot_product) -> Self::Output {
        anti_dot_product_partial(self)
    }
}
impl AntiDotProduct<AntiScalar> for MultiVector {
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        0        1        0
    fn anti_dot_product(self, other: AntiScalar) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */other[e1234] * self[e1234]);
    }
}
impl AntiDotProduct<DualNum> for MultiVector {
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        0        1        0
    fn anti_dot_product(self, other: DualNum) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */other[e1234] * self[e1234]);
    }
}
impl AntiDotProduct<Flector> for MultiVector {
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        3        4        0
    fn anti_dot_product(self, other: Flector) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */(other[e4] * self[e4]) + (other[e423] * self[e423]) + (other[e431] * self[e431]) + (other[e412] * self[e412]));
    }
}
impl AntiDotProduct<Line> for MultiVector {
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        2        3        0
    fn anti_dot_product(self, other: Line) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */(other[e41] * self[e41]) + (other[e42] * self[e42]) + (other[e43] * self[e43]));
    }
}
impl AntiDotProduct<Motor> for MultiVector {
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        3        4        0
    fn anti_dot_product(self, other: Motor) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */(other[e41] * self[e41]) + (other[e42] * self[e42]) + (other[e43] * self[e43]) + (other[e1234] * self[e1234]));
    }
}
impl AntiDotProduct<MultiVector> for MultiVector {
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        7        8        0
    fn anti_dot_product(self, other: MultiVector) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */(other[e1234] * self[e1234]) + (other[e4] * self[e4]) + (other[e41] * self[e41]) + (other[e42] * self[e42]) + (other[e43] * self[e43]) + (other[e423] * self[e423]) + (other[e431] * self[e431]) + (other[e412] * self[e412]));
    }
}
impl AntiDotProduct<Origin> for MultiVector {
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        0        1        0
    fn anti_dot_product(self, other: Origin) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */self[e4] * other[e4]);
    }
}
impl AntiDotProduct<Plane> for MultiVector {
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        2        3        0
    fn anti_dot_product(self, other: Plane) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */(self[e423] * other[e423]) + (self[e431] * other[e431]) + (self[e412] * other[e412]));
    }
}
impl AntiDotProduct<Point> for MultiVector {
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        0        1        0
    fn anti_dot_product(self, other: Point) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */self[e4] * other[e4]);
    }
}
impl std::ops::Div<anti_dot_product> for Origin {
    type Output = anti_dot_product_partial<Origin>;
    fn div(self, _rhs: anti_dot_product) -> Self::Output {
        anti_dot_product_partial(self)
    }
}
impl AntiDotProduct<Flector> for Origin {
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        0        1        0
    fn anti_dot_product(self, other: Flector) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */other[e4] * self[e4]);
    }
}
impl AntiDotProduct<MultiVector> for Origin {
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        0        1        0
    fn anti_dot_product(self, other: MultiVector) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */other[e4] * self[e4]);
    }
}
impl AntiDotProduct<Origin> for Origin {
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        0        1        0
    fn anti_dot_product(self, other: Origin) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */other[e4] * self[e4]);
    }
}
impl AntiDotProduct<Point> for Origin {
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        0        1        0
    fn anti_dot_product(self, other: Point) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */self[e4] * other[e4]);
    }
}
impl std::ops::Div<anti_dot_product> for Plane {
    type Output = anti_dot_product_partial<Plane>;
    fn div(self, _rhs: anti_dot_product) -> Self::Output {
        anti_dot_product_partial(self)
    }
}
impl AntiDotProduct<Flector> for Plane {
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        2        3        0
    fn anti_dot_product(self, other: Flector) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */(other[e423] * self[e423]) + (other[e431] * self[e431]) + (other[e412] * self[e412]));
    }
}
impl AntiDotProduct<MultiVector> for Plane {
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        2        3        0
    fn anti_dot_product(self, other: MultiVector) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */(other[e423] * self[e423]) + (other[e431] * self[e431]) + (other[e412] * self[e412]));
    }
}
impl AntiDotProduct<Plane> for Plane {
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        2        3        0
    fn anti_dot_product(self, other: Plane) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */(other[e423] * self[e423]) + (other[e431] * self[e431]) + (other[e412] * self[e412]));
    }
}
impl std::ops::Div<anti_dot_product> for Point {
    type Output = anti_dot_product_partial<Point>;
    fn div(self, _rhs: anti_dot_product) -> Self::Output {
        anti_dot_product_partial(self)
    }
}
impl AntiDotProduct<Flector> for Point {
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        0        1        0
    fn anti_dot_product(self, other: Flector) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */other[e4] * self[e4]);
    }
}
impl AntiDotProduct<MultiVector> for Point {
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        0        1        0
    fn anti_dot_product(self, other: MultiVector) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */other[e4] * self[e4]);
    }
}
impl AntiDotProduct<Origin> for Point {
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        0        1        0
    fn anti_dot_product(self, other: Origin) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */other[e4] * self[e4]);
    }
}
impl AntiDotProduct<Point> for Point {
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        0        1        0
    fn anti_dot_product(self, other: Point) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */other[e4] * self[e4]);
    }
}
