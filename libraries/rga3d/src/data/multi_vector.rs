use crate::data::*;
use crate::simd::*;

/// MultiVector
#[derive(Clone, Copy, nearly::NearlyEq, nearly::NearlyOrd, bytemuck::Pod, bytemuck::Zeroable, encase::ShaderType, serde::Serialize, serde::Deserialize)]
pub union MultiVector {
    groups: MultiVectorGroups,
    /// scalar, e1234, 0, 0, e1, e2, e3, e4, e41, e42, e43, 0, e23, e31, e12, 0, e423, e431, e412, e321
    elements: [f32; 20],
}
#[derive(Clone, Copy, nearly::NearlyEq, nearly::NearlyOrd, bytemuck::Pod, bytemuck::Zeroable, encase::ShaderType, serde::Serialize, serde::Deserialize)]
pub struct MultiVectorGroups {
    /// scalar, e1234
    g0: Simd32x2,
    /// e1, e2, e3, e4
    g1: Simd32x4,
    /// e41, e42, e43
    g2: Simd32x3,
    /// e23, e31, e12
    g3: Simd32x3,
    /// e423, e431, e412, e321
    g4: Simd32x4,
}
impl MultiVector {
    #[allow(clippy::too_many_arguments)]
    pub const fn from_elements(
        scalar: f32,
        e1234: f32,
        e1: f32,
        e2: f32,
        e3: f32,
        e4: f32,
        e41: f32,
        e42: f32,
        e43: f32,
        e23: f32,
        e31: f32,
        e12: f32,
        e423: f32,
        e431: f32,
        e412: f32,
        e321: f32,
    ) -> Self {
        Self {
            elements: [scalar, e1234, 0.0, 0.0, e1, e2, e3, e4, e41, e42, e43, 0.0, e23, e31, e12, 0.0, e423, e431, e412, e321],
        }
    }
    pub const fn from_groups(g0: Simd32x2, g1: Simd32x4, g2: Simd32x3, g3: Simd32x3, g4: Simd32x4) -> Self {
        Self {
            groups: MultiVectorGroups { g0, g1, g2, g3, g4 },
        }
    }
    #[inline(always)]
    pub fn group0(&self) -> Simd32x2 {
        unsafe { self.groups.g0 }
    }
    #[inline(always)]
    pub fn group0_mut(&mut self) -> &mut Simd32x2 {
        unsafe { &mut self.groups.g0 }
    }
    #[inline(always)]
    pub fn group1(&self) -> Simd32x4 {
        unsafe { self.groups.g1 }
    }
    #[inline(always)]
    pub fn group1_mut(&mut self) -> &mut Simd32x4 {
        unsafe { &mut self.groups.g1 }
    }
    #[inline(always)]
    pub fn group2(&self) -> Simd32x3 {
        unsafe { self.groups.g2 }
    }
    #[inline(always)]
    pub fn group2_mut(&mut self) -> &mut Simd32x3 {
        unsafe { &mut self.groups.g2 }
    }
    #[inline(always)]
    pub fn group3(&self) -> Simd32x3 {
        unsafe { self.groups.g3 }
    }
    #[inline(always)]
    pub fn group3_mut(&mut self) -> &mut Simd32x3 {
        unsafe { &mut self.groups.g3 }
    }
    #[inline(always)]
    pub fn group4(&self) -> Simd32x4 {
        unsafe { self.groups.g4 }
    }
    #[inline(always)]
    pub fn group4_mut(&mut self) -> &mut Simd32x4 {
        unsafe { &mut self.groups.g4 }
    }
}
const MULTI_VECTOR_INDEX_REMAP: [usize; 16] = [0, 1, 4, 5, 6, 7, 8, 9, 10, 12, 13, 14, 16, 17, 18, 19];
impl std::ops::Index<usize> for MultiVector {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[MULTI_VECTOR_INDEX_REMAP[index]] }
    }
}
impl std::ops::IndexMut<usize> for MultiVector {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[MULTI_VECTOR_INDEX_REMAP[index]] }
    }
}
impl From<MultiVector> for [f32; 16] {
    fn from(vector: MultiVector) -> Self {
        unsafe {
            [
                vector.elements[0], vector.elements[1], vector.elements[4], vector.elements[5], vector.elements[6], vector.elements[7], vector.elements[8], vector.elements[9],
                vector.elements[10], vector.elements[12], vector.elements[13], vector.elements[14], vector.elements[16], vector.elements[17], vector.elements[18],
                vector.elements[19],
            ]
        }
    }
}
impl From<[f32; 16]> for MultiVector {
    fn from(array: [f32; 16]) -> Self {
        Self {
            elements: [
                array[0], array[1], 0.0, 0.0, array[1], array[2], array[3], array[4], array[2], array[3], array[4], 0.0, array[3], array[4], array[5], 0.0, array[4], array[5],
                array[6], array[7],
            ],
        }
    }
}
impl std::fmt::Debug for MultiVector {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .debug_struct("MultiVector")
            .field("scalar", &self[0])
            .field("e1234", &self[1])
            .field("e1", &self[2])
            .field("e2", &self[3])
            .field("e3", &self[4])
            .field("e4", &self[5])
            .field("e41", &self[6])
            .field("e42", &self[7])
            .field("e43", &self[8])
            .field("e23", &self[9])
            .field("e31", &self[10])
            .field("e12", &self[11])
            .field("e423", &self[12])
            .field("e431", &self[13])
            .field("e412", &self[14])
            .field("e321", &self[15])
            .finish()
    }
}

impl MultiVector {
    pub const LEN: usize = 16;
}

impl MultiVector {
    pub fn clamp_zeros(mut self, tolerance: nearly::Tolerance<f32>) -> Self {
        for i in 0..Self::LEN {
            let f = self[i];
            if nearly::nearly!(0.0 == f, tol = tolerance) {
                self[i] = 0.0;
            }
        }
        self
    }
}

impl PartialOrd for MultiVector {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        for i in 0..Self::LEN {
            let a = float_ord::FloatOrd(self[i]);
            let b = float_ord::FloatOrd(other[i]);
            match a.cmp(&b) {
                std::cmp::Ordering::Equal => continue,
                result => return Some(result),
            }
        }
        Some(std::cmp::Ordering::Equal)
    }
}
impl Ord for MultiVector {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        for i in 0..Self::LEN {
            let a = float_ord::FloatOrd(self[i]);
            let b = float_ord::FloatOrd(other[i]);
            match a.cmp(&b) {
                std::cmp::Ordering::Equal => continue,
                result => return result,
            }
        }
        std::cmp::Ordering::Equal
    }
}
impl PartialEq for MultiVector {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..Self::LEN {
            let a = float_ord::FloatOrd(self[i]);
            let b = float_ord::FloatOrd(other[i]);
            if a != b {
                return false;
            }
        }
        true
    }
}
impl Eq for MultiVector {}
impl std::hash::Hash for MultiVector {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for i in 0..Self::LEN {
            self[i].to_bits().hash(state);
        }
    }
}

impl std::ops::Index<crate::elements::scalar> for MultiVector {
    type Output = f32;
    fn index(&self, _: crate::elements::scalar) -> &Self::Output {
        &self[0]
    }
}
impl std::ops::Index<crate::elements::e1234> for MultiVector {
    type Output = f32;
    fn index(&self, _: crate::elements::e1234) -> &Self::Output {
        &self[1]
    }
}
impl std::ops::Index<crate::elements::e1> for MultiVector {
    type Output = f32;
    fn index(&self, _: crate::elements::e1) -> &Self::Output {
        &self[2]
    }
}
impl std::ops::Index<crate::elements::e2> for MultiVector {
    type Output = f32;
    fn index(&self, _: crate::elements::e2) -> &Self::Output {
        &self[3]
    }
}
impl std::ops::Index<crate::elements::e3> for MultiVector {
    type Output = f32;
    fn index(&self, _: crate::elements::e3) -> &Self::Output {
        &self[4]
    }
}
impl std::ops::Index<crate::elements::e4> for MultiVector {
    type Output = f32;
    fn index(&self, _: crate::elements::e4) -> &Self::Output {
        &self[5]
    }
}
impl std::ops::Index<crate::elements::e41> for MultiVector {
    type Output = f32;
    fn index(&self, _: crate::elements::e41) -> &Self::Output {
        &self[6]
    }
}
impl std::ops::Index<crate::elements::e42> for MultiVector {
    type Output = f32;
    fn index(&self, _: crate::elements::e42) -> &Self::Output {
        &self[7]
    }
}
impl std::ops::Index<crate::elements::e43> for MultiVector {
    type Output = f32;
    fn index(&self, _: crate::elements::e43) -> &Self::Output {
        &self[8]
    }
}
impl std::ops::Index<crate::elements::e23> for MultiVector {
    type Output = f32;
    fn index(&self, _: crate::elements::e23) -> &Self::Output {
        &self[9]
    }
}
impl std::ops::Index<crate::elements::e31> for MultiVector {
    type Output = f32;
    fn index(&self, _: crate::elements::e31) -> &Self::Output {
        &self[10]
    }
}
impl std::ops::Index<crate::elements::e12> for MultiVector {
    type Output = f32;
    fn index(&self, _: crate::elements::e12) -> &Self::Output {
        &self[11]
    }
}
impl std::ops::Index<crate::elements::e423> for MultiVector {
    type Output = f32;
    fn index(&self, _: crate::elements::e423) -> &Self::Output {
        &self[12]
    }
}
impl std::ops::Index<crate::elements::e431> for MultiVector {
    type Output = f32;
    fn index(&self, _: crate::elements::e431) -> &Self::Output {
        &self[13]
    }
}
impl std::ops::Index<crate::elements::e412> for MultiVector {
    type Output = f32;
    fn index(&self, _: crate::elements::e412) -> &Self::Output {
        &self[14]
    }
}
impl std::ops::Index<crate::elements::e321> for MultiVector {
    type Output = f32;
    fn index(&self, _: crate::elements::e321) -> &Self::Output {
        &self[15]
    }
}
impl std::ops::IndexMut<crate::elements::scalar> for MultiVector {
    fn index_mut(&self, _: crate::elements::scalar) -> &mut Self::Output {
        &mut self[0]
    }
}
impl std::ops::IndexMut<crate::elements::e1234> for MultiVector {
    fn index_mut(&self, _: crate::elements::e1234) -> &mut Self::Output {
        &mut self[1]
    }
}
impl std::ops::IndexMut<crate::elements::e1> for MultiVector {
    fn index_mut(&self, _: crate::elements::e1) -> &mut Self::Output {
        &mut self[2]
    }
}
impl std::ops::IndexMut<crate::elements::e2> for MultiVector {
    fn index_mut(&self, _: crate::elements::e2) -> &mut Self::Output {
        &mut self[3]
    }
}
impl std::ops::IndexMut<crate::elements::e3> for MultiVector {
    fn index_mut(&self, _: crate::elements::e3) -> &mut Self::Output {
        &mut self[4]
    }
}
impl std::ops::IndexMut<crate::elements::e4> for MultiVector {
    fn index_mut(&self, _: crate::elements::e4) -> &mut Self::Output {
        &mut self[5]
    }
}
impl std::ops::IndexMut<crate::elements::e41> for MultiVector {
    fn index_mut(&self, _: crate::elements::e41) -> &mut Self::Output {
        &mut self[6]
    }
}
impl std::ops::IndexMut<crate::elements::e42> for MultiVector {
    fn index_mut(&self, _: crate::elements::e42) -> &mut Self::Output {
        &mut self[7]
    }
}
impl std::ops::IndexMut<crate::elements::e43> for MultiVector {
    fn index_mut(&self, _: crate::elements::e43) -> &mut Self::Output {
        &mut self[8]
    }
}
impl std::ops::IndexMut<crate::elements::e23> for MultiVector {
    fn index_mut(&self, _: crate::elements::e23) -> &mut Self::Output {
        &mut self[9]
    }
}
impl std::ops::IndexMut<crate::elements::e31> for MultiVector {
    fn index_mut(&self, _: crate::elements::e31) -> &mut Self::Output {
        &mut self[10]
    }
}
impl std::ops::IndexMut<crate::elements::e12> for MultiVector {
    fn index_mut(&self, _: crate::elements::e12) -> &mut Self::Output {
        &mut self[11]
    }
}
impl std::ops::IndexMut<crate::elements::e423> for MultiVector {
    fn index_mut(&self, _: crate::elements::e423) -> &mut Self::Output {
        &mut self[12]
    }
}
impl std::ops::IndexMut<crate::elements::e431> for MultiVector {
    fn index_mut(&self, _: crate::elements::e431) -> &mut Self::Output {
        &mut self[13]
    }
}
impl std::ops::IndexMut<crate::elements::e412> for MultiVector {
    fn index_mut(&self, _: crate::elements::e412) -> &mut Self::Output {
        &mut self[14]
    }
}
impl std::ops::IndexMut<crate::elements::e321> for MultiVector {
    fn index_mut(&self, _: crate::elements::e321) -> &mut Self::Output {
        &mut self[15]
    }
}
include!("./impls/multi_vector.rs");
