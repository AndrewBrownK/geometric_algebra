impl std::ops::Div<dot_product> for DualNum {
    type Output = dot_product_partial<DualNum>;
    fn div(self, _rhs: dot_product) -> Self::Output {
        dot_product_partial(self)
    }
}
impl DotProduct<DualNum> for DualNum {
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        0        1        0
    fn dot_product(self, other: DualNum) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */other[scalar] * self[scalar]);
    }
}
impl DotProduct<Motor> for DualNum {
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        0        1        0
    fn dot_product(self, other: Motor) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */self[scalar] * other[scalar]);
    }
}
impl DotProduct<MultiVector> for DualNum {
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        0        1        0
    fn dot_product(self, other: MultiVector) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */self[scalar] * other[scalar]);
    }
}
impl DotProduct<Scalar> for DualNum {
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        0        1        0
    fn dot_product(self, other: Scalar) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */self[scalar] * other[scalar]);
    }
}
impl std::ops::Div<dot_product> for Flector {
    type Output = dot_product_partial<Flector>;
    fn div(self, _rhs: dot_product) -> Self::Output {
        dot_product_partial(self)
    }
}
impl DotProduct<Flector> for Flector {
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        3        4        0
    fn dot_product(self, other: Flector) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */(other[e1] * self[e1]) + (other[e2] * self[e2]) + (other[e3] * self[e3]) + (other[e321] * self[e321]));
    }
}
impl DotProduct<Horizon> for Flector {
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        0        1        0
    fn dot_product(self, other: Horizon) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */self[e321] * other[e321]);
    }
}
impl DotProduct<MultiVector> for Flector {
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        3        4        0
    fn dot_product(self, other: MultiVector) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */(self[e1] * other[e1]) + (self[e2] * other[e2]) + (self[e3] * other[e3]) + (self[e321] * other[e321]));
    }
}
impl DotProduct<Plane> for Flector {
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        0        1        0
    fn dot_product(self, other: Plane) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */self[e321] * other[e321]);
    }
}
impl DotProduct<Point> for Flector {
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        2        3        0
    fn dot_product(self, other: Point) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */(self[e1] * other[e1]) + (self[e2] * other[e2]) + (self[e3] * other[e3]));
    }
}
impl std::ops::Div<dot_product> for Horizon {
    type Output = dot_product_partial<Horizon>;
    fn div(self, _rhs: dot_product) -> Self::Output {
        dot_product_partial(self)
    }
}
impl DotProduct<Flector> for Horizon {
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        0        1        0
    fn dot_product(self, other: Flector) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */other[e321] * self[e321]);
    }
}
impl DotProduct<Horizon> for Horizon {
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        0        1        0
    fn dot_product(self, other: Horizon) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */other[e321] * self[e321]);
    }
}
impl DotProduct<MultiVector> for Horizon {
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        0        1        0
    fn dot_product(self, other: MultiVector) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */self[e321] * other[e321]);
    }
}
impl DotProduct<Plane> for Horizon {
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        0        1        0
    fn dot_product(self, other: Plane) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */self[e321] * other[e321]);
    }
}
impl std::ops::Div<dot_product> for Line {
    type Output = dot_product_partial<Line>;
    fn div(self, _rhs: dot_product) -> Self::Output {
        dot_product_partial(self)
    }
}
impl DotProduct<Line> for Line {
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        2        3        0
    fn dot_product(self, other: Line) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */(other[e23] * self[e23]) + (other[e31] * self[e31]) + (other[e12] * self[e12]));
    }
}
impl DotProduct<Motor> for Line {
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        2        3        0
    fn dot_product(self, other: Motor) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */(self[e23] * other[e23]) + (self[e31] * other[e31]) + (self[e12] * other[e12]));
    }
}
impl DotProduct<MultiVector> for Line {
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        2        3        0
    fn dot_product(self, other: MultiVector) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */(self[e23] * other[e23]) + (self[e31] * other[e31]) + (self[e12] * other[e12]));
    }
}
impl std::ops::Div<dot_product> for Motor {
    type Output = dot_product_partial<Motor>;
    fn div(self, _rhs: dot_product) -> Self::Output {
        dot_product_partial(self)
    }
}
impl DotProduct<DualNum> for Motor {
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        0        1        0
    fn dot_product(self, other: DualNum) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */other[scalar] * self[scalar]);
    }
}
impl DotProduct<Line> for Motor {
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        2        3        0
    fn dot_product(self, other: Line) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */(other[e23] * self[e23]) + (other[e31] * self[e31]) + (other[e12] * self[e12]));
    }
}
impl DotProduct<Motor> for Motor {
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        3        4        0
    fn dot_product(self, other: Motor) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */(other[e23] * self[e23]) + (other[e31] * self[e31]) + (other[e12] * self[e12]) + (other[scalar] * self[scalar]));
    }
}
impl DotProduct<MultiVector> for Motor {
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        3        4        0
    fn dot_product(self, other: MultiVector) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */(self[e23] * other[e23]) + (self[e31] * other[e31]) + (self[e12] * other[e12]) + (self[scalar] * other[scalar]));
    }
}
impl DotProduct<Scalar> for Motor {
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        0        1        0
    fn dot_product(self, other: Scalar) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */self[scalar] * other[scalar]);
    }
}
impl std::ops::Div<dot_product> for MultiVector {
    type Output = dot_product_partial<MultiVector>;
    fn div(self, _rhs: dot_product) -> Self::Output {
        dot_product_partial(self)
    }
}
impl DotProduct<DualNum> for MultiVector {
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        0        1        0
    fn dot_product(self, other: DualNum) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */other[scalar] * self[scalar]);
    }
}
impl DotProduct<Flector> for MultiVector {
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        3        4        0
    fn dot_product(self, other: Flector) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */(other[e1] * self[e1]) + (other[e2] * self[e2]) + (other[e3] * self[e3]) + (other[e321] * self[e321]));
    }
}
impl DotProduct<Horizon> for MultiVector {
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        0        1        0
    fn dot_product(self, other: Horizon) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */other[e321] * self[e321]);
    }
}
impl DotProduct<Line> for MultiVector {
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        2        3        0
    fn dot_product(self, other: Line) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */(other[e23] * self[e23]) + (other[e31] * self[e31]) + (other[e12] * self[e12]));
    }
}
impl DotProduct<Motor> for MultiVector {
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        3        4        0
    fn dot_product(self, other: Motor) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */(other[e23] * self[e23]) + (other[e31] * self[e31]) + (other[e12] * self[e12]) + (other[scalar] * self[scalar]));
    }
}
impl DotProduct<MultiVector> for MultiVector {
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        7        8        0
    fn dot_product(self, other: MultiVector) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */(other[scalar] * self[scalar]) + (other[e1] * self[e1]) + (other[e2] * self[e2]) + (other[e3] * self[e3]) + (other[e23] * self[e23]) + (other[e31] * self[e31]) + (other[e12] * self[e12]) + (other[e321] * self[e321]));
    }
}
impl DotProduct<Plane> for MultiVector {
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        0        1        0
    fn dot_product(self, other: Plane) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */self[e321] * other[e321]);
    }
}
impl DotProduct<Point> for MultiVector {
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        2        3        0
    fn dot_product(self, other: Point) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */(self[e1] * other[e1]) + (self[e2] * other[e2]) + (self[e3] * other[e3]));
    }
}
impl DotProduct<Scalar> for MultiVector {
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        0        1        0
    fn dot_product(self, other: Scalar) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */self[scalar] * other[scalar]);
    }
}
impl std::ops::Div<dot_product> for Plane {
    type Output = dot_product_partial<Plane>;
    fn div(self, _rhs: dot_product) -> Self::Output {
        dot_product_partial(self)
    }
}
impl DotProduct<Flector> for Plane {
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        0        1        0
    fn dot_product(self, other: Flector) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */other[e321] * self[e321]);
    }
}
impl DotProduct<Horizon> for Plane {
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        0        1        0
    fn dot_product(self, other: Horizon) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */other[e321] * self[e321]);
    }
}
impl DotProduct<MultiVector> for Plane {
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        0        1        0
    fn dot_product(self, other: MultiVector) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */other[e321] * self[e321]);
    }
}
impl DotProduct<Plane> for Plane {
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        0        1        0
    fn dot_product(self, other: Plane) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */other[e321] * self[e321]);
    }
}
impl std::ops::Div<dot_product> for Point {
    type Output = dot_product_partial<Point>;
    fn div(self, _rhs: dot_product) -> Self::Output {
        dot_product_partial(self)
    }
}
impl DotProduct<Flector> for Point {
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        2        3        0
    fn dot_product(self, other: Flector) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */(other[e1] * self[e1]) + (other[e2] * self[e2]) + (other[e3] * self[e3]));
    }
}
impl DotProduct<MultiVector> for Point {
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        2        3        0
    fn dot_product(self, other: MultiVector) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */(other[e1] * self[e1]) + (other[e2] * self[e2]) + (other[e3] * self[e3]));
    }
}
impl DotProduct<Point> for Point {
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        2        3        0
    fn dot_product(self, other: Point) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */(other[e1] * self[e1]) + (other[e2] * self[e2]) + (other[e3] * self[e3]));
    }
}
impl std::ops::Div<dot_product> for Scalar {
    type Output = dot_product_partial<Scalar>;
    fn div(self, _rhs: dot_product) -> Self::Output {
        dot_product_partial(self)
    }
}
impl DotProduct<DualNum> for Scalar {
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        0        1        0
    fn dot_product(self, other: DualNum) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */other[scalar] * self[scalar]);
    }
}
impl DotProduct<Motor> for Scalar {
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        0        1        0
    fn dot_product(self, other: Motor) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */other[scalar] * self[scalar]);
    }
}
impl DotProduct<MultiVector> for Scalar {
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        0        1        0
    fn dot_product(self, other: MultiVector) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */other[scalar] * self[scalar]);
    }
}
impl DotProduct<Scalar> for Scalar {
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        0        1        0
    fn dot_product(self, other: Scalar) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */other[scalar] * self[scalar]);
    }
}
