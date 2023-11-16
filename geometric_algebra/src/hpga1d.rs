#![allow(clippy::assign_op_pattern)]
use crate::{simd::*, *};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Clone, Copy)]
struct ScalarGroups {
    /// 1
    g0: f32,
}

#[derive(Clone, Copy)]
pub union Scalar {
    groups: ScalarGroups,
    /// 1
    elements: [f32; 1],
}

impl Scalar {
    #[allow(clippy::too_many_arguments)]
    pub const fn new(element0: f32) -> Self {
        Self { elements: [element0] }
    }
    pub const fn from_groups(g0: f32) -> Self {
        Self { groups: ScalarGroups { g0 } }
    }
    #[inline(always)]
    pub fn group0(&self) -> f32 {
        unsafe { self.groups.g0 }
    }
    #[inline(always)]
    pub fn group0_mut(&mut self) -> &mut f32 {
        unsafe { &mut self.groups.g0 }
    }
}

const SCALAR_INDEX_REMAP: [usize; 1] = [0];

impl std::ops::Index<usize> for Scalar {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[SCALAR_INDEX_REMAP[index]] }
    }
}

impl std::ops::IndexMut<usize> for Scalar {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[SCALAR_INDEX_REMAP[index]] }
    }
}

impl std::convert::From<Scalar> for [f32; 1] {
    fn from(vector: Scalar) -> Self {
        unsafe { [vector.elements[0]] }
    }
}

impl std::convert::From<[f32; 1]> for Scalar {
    fn from(array: [f32; 1]) -> Self {
        Self { elements: [array[0]] }
    }
}

impl std::fmt::Debug for Scalar {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .debug_struct("Scalar")
            .field("1", &self[0])
            .finish()
    }
}

#[derive(Clone, Copy)]
struct SplitComplexNumberGroups {
    /// 1, e01
    g0: Simd32x2,
}

#[derive(Clone, Copy)]
pub union SplitComplexNumber {
    groups: SplitComplexNumberGroups,
    /// 1, e01, 0, 0
    elements: [f32; 4],
}

impl SplitComplexNumber {
    #[allow(clippy::too_many_arguments)]
    pub const fn new(element0: f32, element1: f32) -> Self {
        Self { elements: [element0, element1, 0.0, 0.0] }
    }
    pub const fn from_groups(g0: Simd32x2) -> Self {
        Self { groups: SplitComplexNumberGroups { g0 } }
    }
    #[inline(always)]
    pub fn group0(&self) -> Simd32x2 {
        unsafe { self.groups.g0 }
    }
    #[inline(always)]
    pub fn group0_mut(&mut self) -> &mut Simd32x2 {
        unsafe { &mut self.groups.g0 }
    }
}

const SPLITCOMPLEXNUMBER_INDEX_REMAP: [usize; 2] = [0, 1];

impl std::ops::Index<usize> for SplitComplexNumber {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[SPLITCOMPLEXNUMBER_INDEX_REMAP[index]] }
    }
}

impl std::ops::IndexMut<usize> for SplitComplexNumber {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[SPLITCOMPLEXNUMBER_INDEX_REMAP[index]] }
    }
}

impl std::convert::From<SplitComplexNumber> for [f32; 2] {
    fn from(vector: SplitComplexNumber) -> Self {
        unsafe { [vector.elements[0], vector.elements[1]] }
    }
}

impl std::convert::From<[f32; 2]> for SplitComplexNumber {
    fn from(array: [f32; 2]) -> Self {
        Self { elements: [array[0], array[1], 0.0, 0.0] }
    }
}

impl std::fmt::Debug for SplitComplexNumber {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .debug_struct("SplitComplexNumber")
            .field("1", &self[0])
            .field("e01", &self[1])
            .finish()
    }
}

impl Zero for Scalar {
    fn zero() -> Self {
        Scalar { groups: ScalarGroups { g0: 0.0 } }
    }
}

impl One for Scalar {
    fn one() -> Self {
        Scalar { groups: ScalarGroups { g0: 1.0 } }
    }
}

impl Grade for Scalar {
    type Output = isize;

    fn grade(self) -> isize {
        0
    }
}

impl AntiGrade for Scalar {
    type Output = isize;

    fn anti_grade(self) -> isize {
        2
    }
}

impl Neg for Scalar {
    type Output = Scalar;

    fn neg(self) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0() * -1.0 } }
    }
}

impl Automorphism for Scalar {
    type Output = Scalar;

    fn automorphism(self) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0() } }
    }
}

impl Reversal for Scalar {
    type Output = Scalar;

    fn reversal(self) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0() } }
    }
}

impl Conjugation for Scalar {
    type Output = Scalar;

    fn conjugation(self) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0() } }
    }
}

impl AntiReversal for Scalar {
    type Output = Scalar;

    fn anti_reversal(self) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0() } }
    }
}

impl Add<Scalar> for Scalar {
    type Output = Scalar;

    fn add(self, other: Scalar) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0() + other.group0() } }
    }
}

impl AddAssign<Scalar> for Scalar {
    fn add_assign(&mut self, other: Scalar) {
        *self = (*self).add(other);
    }
}

impl Sub<Scalar> for Scalar {
    type Output = Scalar;

    fn sub(self, other: Scalar) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0() - other.group0() } }
    }
}

impl SubAssign<Scalar> for Scalar {
    fn sub_assign(&mut self, other: Scalar) {
        *self = (*self).sub(other);
    }
}

impl Mul<Scalar> for Scalar {
    type Output = Scalar;

    fn mul(self, other: Scalar) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0() * other.group0() } }
    }
}

impl MulAssign<Scalar> for Scalar {
    fn mul_assign(&mut self, other: Scalar) {
        *self = (*self).mul(other);
    }
}

impl Div<Scalar> for Scalar {
    type Output = Scalar;

    fn div(self, other: Scalar) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0() * 1.0 / other.group0() * 1.0 } }
    }
}

impl DivAssign<Scalar> for Scalar {
    fn div_assign(&mut self, other: Scalar) {
        *self = (*self).div(other);
    }
}

impl GeometricProduct<Scalar> for Scalar {
    type Output = Scalar;

    fn geometric_product(self, other: Scalar) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0() * other.group0() } }
    }
}

impl OuterProduct<Scalar> for Scalar {
    type Output = Scalar;

    fn outer_product(self, other: Scalar) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0() * other.group0() } }
    }
}

impl InnerProduct<Scalar> for Scalar {
    type Output = Scalar;

    fn inner_product(self, other: Scalar) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0() * other.group0() } }
    }
}

impl LeftContraction<Scalar> for Scalar {
    type Output = Scalar;

    fn left_contraction(self, other: Scalar) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0() * other.group0() } }
    }
}

impl RightContraction<Scalar> for Scalar {
    type Output = Scalar;

    fn right_contraction(self, other: Scalar) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0() * other.group0() } }
    }
}

impl ScalarProduct<Scalar> for Scalar {
    type Output = Scalar;

    fn scalar_product(self, other: Scalar) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0() * other.group0() } }
    }
}

impl Add<SplitComplexNumber> for Scalar {
    type Output = SplitComplexNumber;

    fn add(self, other: SplitComplexNumber) -> SplitComplexNumber {
        SplitComplexNumber { groups: SplitComplexNumberGroups { g0: Simd32x2::from(self.group0()) * Simd32x2::from([1.0, 0.0]) + other.group0() } }
    }
}

impl Sub<SplitComplexNumber> for Scalar {
    type Output = SplitComplexNumber;

    fn sub(self, other: SplitComplexNumber) -> SplitComplexNumber {
        SplitComplexNumber { groups: SplitComplexNumberGroups { g0: Simd32x2::from(self.group0()) * Simd32x2::from([1.0, 0.0]) - other.group0() } }
    }
}

impl GeometricProduct<SplitComplexNumber> for Scalar {
    type Output = SplitComplexNumber;

    fn geometric_product(self, other: SplitComplexNumber) -> SplitComplexNumber {
        SplitComplexNumber { groups: SplitComplexNumberGroups { g0: Simd32x2::from(self.group0()) * other.group0() } }
    }
}

impl RegressiveProduct<SplitComplexNumber> for Scalar {
    type Output = Scalar;

    fn regressive_product(self, other: SplitComplexNumber) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0() * other.group0()[1] } }
    }
}

impl OuterProduct<SplitComplexNumber> for Scalar {
    type Output = SplitComplexNumber;

    fn outer_product(self, other: SplitComplexNumber) -> SplitComplexNumber {
        SplitComplexNumber { groups: SplitComplexNumberGroups { g0: Simd32x2::from(self.group0()) * other.group0() } }
    }
}

impl InnerProduct<SplitComplexNumber> for Scalar {
    type Output = SplitComplexNumber;

    fn inner_product(self, other: SplitComplexNumber) -> SplitComplexNumber {
        SplitComplexNumber { groups: SplitComplexNumberGroups { g0: Simd32x2::from(self.group0()) * other.group0() } }
    }
}

impl GeometricAntiProduct<SplitComplexNumber> for Scalar {
    type Output = SplitComplexNumber;

    fn geometric_anti_product(self, other: SplitComplexNumber) -> SplitComplexNumber {
        SplitComplexNumber { groups: SplitComplexNumberGroups { g0: Simd32x2::from(self.group0()) * swizzle!(other.group0(), 1, 0) } }
    }
}

impl InnerAntiProduct<SplitComplexNumber> for Scalar {
    type Output = SplitComplexNumber;

    fn inner_anti_product(self, other: SplitComplexNumber) -> SplitComplexNumber {
        SplitComplexNumber { groups: SplitComplexNumberGroups { g0: Simd32x2::from(self.group0()) * swizzle!(other.group0(), 1, 0) } }
    }
}

impl LeftContraction<SplitComplexNumber> for Scalar {
    type Output = SplitComplexNumber;

    fn left_contraction(self, other: SplitComplexNumber) -> SplitComplexNumber {
        SplitComplexNumber { groups: SplitComplexNumberGroups { g0: Simd32x2::from(self.group0()) * other.group0() } }
    }
}

impl RightContraction<SplitComplexNumber> for Scalar {
    type Output = Scalar;

    fn right_contraction(self, other: SplitComplexNumber) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0() * other.group0()[0] } }
    }
}

impl RightAntiContraction<SplitComplexNumber> for Scalar {
    type Output = SplitComplexNumber;

    fn right_anti_contraction(self, other: SplitComplexNumber) -> SplitComplexNumber {
        SplitComplexNumber { groups: SplitComplexNumberGroups { g0: Simd32x2::from(self.group0()) * swizzle!(other.group0(), 1, 0) } }
    }
}

impl ScalarProduct<SplitComplexNumber> for Scalar {
    type Output = Scalar;

    fn scalar_product(self, other: SplitComplexNumber) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0() * other.group0()[0] } }
    }
}

impl SquaredMagnitude for Scalar {
    type Output = Scalar;

    fn squared_magnitude(self) -> Scalar {
        self.scalar_product(self.reversal())
    }
}

impl Magnitude for Scalar {
    type Output = Scalar;

    fn magnitude(self) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.squared_magnitude().group0().sqrt() } }
    }
}

impl Scale for Scalar {
    type Output = Scalar;

    fn scale(self, other: f32) -> Scalar {
        self.geometric_product(Scalar { groups: ScalarGroups { g0: other } })
    }
}

impl Signum for Scalar {
    type Output = Scalar;

    fn signum(self) -> Scalar {
        self.geometric_product(Scalar { groups: ScalarGroups { g0: 1.0 / self.magnitude().group0() } })
    }
}

impl Inverse for Scalar {
    type Output = Scalar;

    fn inverse(self) -> Scalar {
        self.reversal().geometric_product(Scalar { groups: ScalarGroups { g0: 1.0 / self.squared_magnitude().group0() } })
    }
}

impl Zero for SplitComplexNumber {
    fn zero() -> Self {
        SplitComplexNumber { groups: SplitComplexNumberGroups { g0: Simd32x2::from(0.0) } }
    }
}

impl One for SplitComplexNumber {
    fn one() -> Self {
        SplitComplexNumber { groups: SplitComplexNumberGroups { g0: Simd32x2::from([1.0, 0.0]) } }
    }
}

impl Neg for SplitComplexNumber {
    type Output = SplitComplexNumber;

    fn neg(self) -> SplitComplexNumber {
        SplitComplexNumber { groups: SplitComplexNumberGroups { g0: self.group0() * Simd32x2::from(-1.0) } }
    }
}

impl Automorphism for SplitComplexNumber {
    type Output = SplitComplexNumber;

    fn automorphism(self) -> SplitComplexNumber {
        SplitComplexNumber { groups: SplitComplexNumberGroups { g0: self.group0() } }
    }
}

impl Reversal for SplitComplexNumber {
    type Output = SplitComplexNumber;

    fn reversal(self) -> SplitComplexNumber {
        SplitComplexNumber { groups: SplitComplexNumberGroups { g0: self.group0() * Simd32x2::from([1.0, -1.0]) } }
    }
}

impl Conjugation for SplitComplexNumber {
    type Output = SplitComplexNumber;

    fn conjugation(self) -> SplitComplexNumber {
        SplitComplexNumber { groups: SplitComplexNumberGroups { g0: self.group0() * Simd32x2::from([1.0, -1.0]) } }
    }
}

impl Dual for SplitComplexNumber {
    type Output = SplitComplexNumber;

    fn dual(self) -> SplitComplexNumber {
        SplitComplexNumber { groups: SplitComplexNumberGroups { g0: swizzle!(self.group0(), 1, 0) } }
    }
}

impl AntiReversal for SplitComplexNumber {
    type Output = SplitComplexNumber;

    fn anti_reversal(self) -> SplitComplexNumber {
        SplitComplexNumber { groups: SplitComplexNumberGroups { g0: self.group0() * Simd32x2::from([1.0, -1.0]) } }
    }
}

impl Into<Scalar> for SplitComplexNumber {
    fn into(self) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[0] } }
    }
}

impl Add<Scalar> for SplitComplexNumber {
    type Output = SplitComplexNumber;

    fn add(self, other: Scalar) -> SplitComplexNumber {
        SplitComplexNumber { groups: SplitComplexNumberGroups { g0: self.group0() + Simd32x2::from(other.group0()) * Simd32x2::from([1.0, 0.0]) } }
    }
}

impl AddAssign<Scalar> for SplitComplexNumber {
    fn add_assign(&mut self, other: Scalar) {
        *self = (*self).add(other);
    }
}

impl Sub<Scalar> for SplitComplexNumber {
    type Output = SplitComplexNumber;

    fn sub(self, other: Scalar) -> SplitComplexNumber {
        SplitComplexNumber { groups: SplitComplexNumberGroups { g0: self.group0() - Simd32x2::from(other.group0()) * Simd32x2::from([1.0, 0.0]) } }
    }
}

impl SubAssign<Scalar> for SplitComplexNumber {
    fn sub_assign(&mut self, other: Scalar) {
        *self = (*self).sub(other);
    }
}

impl GeometricProduct<Scalar> for SplitComplexNumber {
    type Output = SplitComplexNumber;

    fn geometric_product(self, other: Scalar) -> SplitComplexNumber {
        SplitComplexNumber { groups: SplitComplexNumberGroups { g0: self.group0() * Simd32x2::from(other.group0()) } }
    }
}

impl RegressiveProduct<Scalar> for SplitComplexNumber {
    type Output = Scalar;

    fn regressive_product(self, other: Scalar) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[1] * other.group0() } }
    }
}

impl OuterProduct<Scalar> for SplitComplexNumber {
    type Output = SplitComplexNumber;

    fn outer_product(self, other: Scalar) -> SplitComplexNumber {
        SplitComplexNumber { groups: SplitComplexNumberGroups { g0: self.group0() * Simd32x2::from(other.group0()) } }
    }
}

impl InnerProduct<Scalar> for SplitComplexNumber {
    type Output = SplitComplexNumber;

    fn inner_product(self, other: Scalar) -> SplitComplexNumber {
        SplitComplexNumber { groups: SplitComplexNumberGroups { g0: self.group0() * Simd32x2::from(other.group0()) } }
    }
}

impl GeometricAntiProduct<Scalar> for SplitComplexNumber {
    type Output = SplitComplexNumber;

    fn geometric_anti_product(self, other: Scalar) -> SplitComplexNumber {
        SplitComplexNumber { groups: SplitComplexNumberGroups { g0: swizzle!(self.group0(), 1, 0) * Simd32x2::from(other.group0()) } }
    }
}

impl InnerAntiProduct<Scalar> for SplitComplexNumber {
    type Output = SplitComplexNumber;

    fn inner_anti_product(self, other: Scalar) -> SplitComplexNumber {
        SplitComplexNumber { groups: SplitComplexNumberGroups { g0: swizzle!(self.group0(), 1, 0) * Simd32x2::from(other.group0()) } }
    }
}

impl LeftContraction<Scalar> for SplitComplexNumber {
    type Output = Scalar;

    fn left_contraction(self, other: Scalar) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[0] * other.group0() } }
    }
}

impl RightContraction<Scalar> for SplitComplexNumber {
    type Output = SplitComplexNumber;

    fn right_contraction(self, other: Scalar) -> SplitComplexNumber {
        SplitComplexNumber { groups: SplitComplexNumberGroups { g0: self.group0() * Simd32x2::from(other.group0()) } }
    }
}

impl LeftAntiContraction<Scalar> for SplitComplexNumber {
    type Output = SplitComplexNumber;

    fn left_anti_contraction(self, other: Scalar) -> SplitComplexNumber {
        SplitComplexNumber { groups: SplitComplexNumberGroups { g0: swizzle!(self.group0(), 1, 0) * Simd32x2::from(other.group0()) } }
    }
}

impl ScalarProduct<Scalar> for SplitComplexNumber {
    type Output = Scalar;

    fn scalar_product(self, other: Scalar) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[0] * other.group0() } }
    }
}

impl Add<SplitComplexNumber> for SplitComplexNumber {
    type Output = SplitComplexNumber;

    fn add(self, other: SplitComplexNumber) -> SplitComplexNumber {
        SplitComplexNumber { groups: SplitComplexNumberGroups { g0: self.group0() + other.group0() } }
    }
}

impl AddAssign<SplitComplexNumber> for SplitComplexNumber {
    fn add_assign(&mut self, other: SplitComplexNumber) {
        *self = (*self).add(other);
    }
}

impl Sub<SplitComplexNumber> for SplitComplexNumber {
    type Output = SplitComplexNumber;

    fn sub(self, other: SplitComplexNumber) -> SplitComplexNumber {
        SplitComplexNumber { groups: SplitComplexNumberGroups { g0: self.group0() - other.group0() } }
    }
}

impl SubAssign<SplitComplexNumber> for SplitComplexNumber {
    fn sub_assign(&mut self, other: SplitComplexNumber) {
        *self = (*self).sub(other);
    }
}

impl Mul<SplitComplexNumber> for SplitComplexNumber {
    type Output = SplitComplexNumber;

    fn mul(self, other: SplitComplexNumber) -> SplitComplexNumber {
        SplitComplexNumber { groups: SplitComplexNumberGroups { g0: self.group0() * other.group0() } }
    }
}

impl MulAssign<SplitComplexNumber> for SplitComplexNumber {
    fn mul_assign(&mut self, other: SplitComplexNumber) {
        *self = (*self).mul(other);
    }
}

impl Div<SplitComplexNumber> for SplitComplexNumber {
    type Output = SplitComplexNumber;

    fn div(self, other: SplitComplexNumber) -> SplitComplexNumber {
        SplitComplexNumber { groups: SplitComplexNumberGroups { g0: Simd32x2::from([self.group0()[0], self.group0()[1]]) * Simd32x2::from([1.0, 1.0]) / Simd32x2::from([other.group0()[0], other.group0()[1]]) * Simd32x2::from([1.0, 1.0]) } }
    }
}

impl DivAssign<SplitComplexNumber> for SplitComplexNumber {
    fn div_assign(&mut self, other: SplitComplexNumber) {
        *self = (*self).div(other);
    }
}

impl GeometricProduct<SplitComplexNumber> for SplitComplexNumber {
    type Output = SplitComplexNumber;

    fn geometric_product(self, other: SplitComplexNumber) -> SplitComplexNumber {
        SplitComplexNumber { groups: SplitComplexNumberGroups { g0: Simd32x2::from(self.group0()[0]) * other.group0() + Simd32x2::from(self.group0()[1]) * swizzle!(other.group0(), 1, 0) } }
    }
}

impl RegressiveProduct<SplitComplexNumber> for SplitComplexNumber {
    type Output = SplitComplexNumber;

    fn regressive_product(self, other: SplitComplexNumber) -> SplitComplexNumber {
        SplitComplexNumber { groups: SplitComplexNumberGroups { g0: Simd32x2::from(self.group0()[1]) * other.group0() + Simd32x2::from(self.group0()[0]) * swizzle!(other.group0(), 1, 0) * Simd32x2::from([1.0, 0.0]) } }
    }
}

impl OuterProduct<SplitComplexNumber> for SplitComplexNumber {
    type Output = SplitComplexNumber;

    fn outer_product(self, other: SplitComplexNumber) -> SplitComplexNumber {
        SplitComplexNumber { groups: SplitComplexNumberGroups { g0: Simd32x2::from(self.group0()[0]) * other.group0() + self.group0() * Simd32x2::from(other.group0()[0]) * Simd32x2::from([0.0, 1.0]) } }
    }
}

impl InnerProduct<SplitComplexNumber> for SplitComplexNumber {
    type Output = SplitComplexNumber;

    fn inner_product(self, other: SplitComplexNumber) -> SplitComplexNumber {
        SplitComplexNumber { groups: SplitComplexNumberGroups { g0: Simd32x2::from(self.group0()[0]) * other.group0() + Simd32x2::from(self.group0()[1]) * swizzle!(other.group0(), 1, 0) } }
    }
}

impl GeometricAntiProduct<SplitComplexNumber> for SplitComplexNumber {
    type Output = SplitComplexNumber;

    fn geometric_anti_product(self, other: SplitComplexNumber) -> SplitComplexNumber {
        SplitComplexNumber { groups: SplitComplexNumberGroups { g0: Simd32x2::from(self.group0()[0]) * swizzle!(other.group0(), 1, 0) + Simd32x2::from(self.group0()[1]) * other.group0() } }
    }
}

impl InnerAntiProduct<SplitComplexNumber> for SplitComplexNumber {
    type Output = SplitComplexNumber;

    fn inner_anti_product(self, other: SplitComplexNumber) -> SplitComplexNumber {
        SplitComplexNumber { groups: SplitComplexNumberGroups { g0: Simd32x2::from(self.group0()[0]) * swizzle!(other.group0(), 1, 0) + Simd32x2::from(self.group0()[1]) * other.group0() } }
    }
}

impl LeftContraction<SplitComplexNumber> for SplitComplexNumber {
    type Output = SplitComplexNumber;

    fn left_contraction(self, other: SplitComplexNumber) -> SplitComplexNumber {
        SplitComplexNumber { groups: SplitComplexNumberGroups { g0: Simd32x2::from(self.group0()[0]) * other.group0() + swizzle!(self.group0(), 1, 0) * swizzle!(other.group0(), 1, 0) * Simd32x2::from([1.0, 0.0]) } }
    }
}

impl RightContraction<SplitComplexNumber> for SplitComplexNumber {
    type Output = SplitComplexNumber;

    fn right_contraction(self, other: SplitComplexNumber) -> SplitComplexNumber {
        SplitComplexNumber { groups: SplitComplexNumberGroups { g0: Simd32x2::from(self.group0()[1]) * swizzle!(other.group0(), 1, 0) + Simd32x2::from(self.group0()[0]) * Simd32x2::from(other.group0()[0]) * Simd32x2::from([1.0, 0.0]) } }
    }
}

impl LeftAntiContraction<SplitComplexNumber> for SplitComplexNumber {
    type Output = SplitComplexNumber;

    fn left_anti_contraction(self, other: SplitComplexNumber) -> SplitComplexNumber {
        SplitComplexNumber { groups: SplitComplexNumberGroups { g0: Simd32x2::from(self.group0()[1]) * other.group0() + Simd32x2::from(self.group0()[0]) * Simd32x2::from(other.group0()[0]) * Simd32x2::from([0.0, 1.0]) } }
    }
}

impl RightAntiContraction<SplitComplexNumber> for SplitComplexNumber {
    type Output = SplitComplexNumber;

    fn right_anti_contraction(self, other: SplitComplexNumber) -> SplitComplexNumber {
        SplitComplexNumber { groups: SplitComplexNumberGroups { g0: Simd32x2::from(self.group0()[0]) * swizzle!(other.group0(), 1, 0) + self.group0() * other.group0() * Simd32x2::from([0.0, 1.0]) } }
    }
}

impl ScalarProduct<SplitComplexNumber> for SplitComplexNumber {
    type Output = Scalar;

    fn scalar_product(self, other: SplitComplexNumber) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] } }
    }
}

impl SquaredMagnitude for SplitComplexNumber {
    type Output = Scalar;

    fn squared_magnitude(self) -> Scalar {
        self.scalar_product(self.reversal())
    }
}

impl Magnitude for SplitComplexNumber {
    type Output = Scalar;

    fn magnitude(self) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.squared_magnitude().group0().sqrt() } }
    }
}

impl Scale for SplitComplexNumber {
    type Output = SplitComplexNumber;

    fn scale(self, other: f32) -> SplitComplexNumber {
        self.geometric_product(Scalar { groups: ScalarGroups { g0: other } })
    }
}

impl Signum for SplitComplexNumber {
    type Output = SplitComplexNumber;

    fn signum(self) -> SplitComplexNumber {
        self.geometric_product(Scalar { groups: ScalarGroups { g0: 1.0 / self.magnitude().group0() } })
    }
}

impl Inverse for SplitComplexNumber {
    type Output = SplitComplexNumber;

    fn inverse(self) -> SplitComplexNumber {
        self.reversal().geometric_product(Scalar { groups: ScalarGroups { g0: 1.0 / self.squared_magnitude().group0() } })
    }
}

impl Powi for Scalar {
    type Output = Scalar;

    fn powi(self, exponent: isize) -> Scalar {
        if exponent == 0 {
            return Scalar::one();
        }
        let mut x: Scalar = if exponent < 0 { self.inverse() } else { self };
        let mut y: Scalar = Scalar::one();
        let mut n: isize = exponent.abs();
        while 1 < n {
            if n & 1 == 1 {
                y = x.geometric_product(y);
            }
            x = x.geometric_product(x);
            n = n >> 1;
        }
        x.geometric_product(y)
    }
}

impl GeometricQuotient<Scalar> for Scalar {
    type Output = Scalar;

    fn geometric_quotient(self, other: Scalar) -> Scalar {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Scalar> for Scalar {
    type Output = Scalar;

    fn transformation(self, other: Scalar) -> Scalar {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl GeometricQuotient<SplitComplexNumber> for Scalar {
    type Output = SplitComplexNumber;

    fn geometric_quotient(self, other: SplitComplexNumber) -> SplitComplexNumber {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<SplitComplexNumber> for Scalar {
    type Output = SplitComplexNumber;

    fn transformation(self, other: SplitComplexNumber) -> SplitComplexNumber {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl GeometricQuotient<Scalar> for SplitComplexNumber {
    type Output = SplitComplexNumber;

    fn geometric_quotient(self, other: Scalar) -> SplitComplexNumber {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Scalar> for SplitComplexNumber {
    type Output = Scalar;

    fn transformation(self, other: Scalar) -> Scalar {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl Powi for SplitComplexNumber {
    type Output = SplitComplexNumber;

    fn powi(self, exponent: isize) -> SplitComplexNumber {
        if exponent == 0 {
            return SplitComplexNumber::one();
        }
        let mut x: SplitComplexNumber = if exponent < 0 { self.inverse() } else { self };
        let mut y: SplitComplexNumber = SplitComplexNumber::one();
        let mut n: isize = exponent.abs();
        while 1 < n {
            if n & 1 == 1 {
                y = x.geometric_product(y);
            }
            x = x.geometric_product(x);
            n = n >> 1;
        }
        x.geometric_product(y)
    }
}

impl GeometricQuotient<SplitComplexNumber> for SplitComplexNumber {
    type Output = SplitComplexNumber;

    fn geometric_quotient(self, other: SplitComplexNumber) -> SplitComplexNumber {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<SplitComplexNumber> for SplitComplexNumber {
    type Output = SplitComplexNumber;

    fn transformation(self, other: SplitComplexNumber) -> SplitComplexNumber {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

