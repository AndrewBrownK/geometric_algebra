use crate::data::*;
use crate::simd::*;

/// MultiVector
#[derive(Clone, Copy, nearly::NearlyEq, nearly::NearlyOrd, bytemuck::Pod, bytemuck::Zeroable, encase::ShaderType, serde::Serialize, serde::Deserialize)]
pub union MultiVector {
    groups: MultiVectorGroups,
    /// scalar, e12345, 0, 0, e1, e2, e3, e4, e5, 0, 0, 0, e15, e25, e35, e45, e41, e42, e43, 0, e415, e425, e435, e321, e423, e431, e412, 0, e235, e315, e125, 0, e4235, e4315, e4125, e3215, e1234, 0, 0, 0, e12, e31, e23, 0
    elements: [f32; 44],
}
#[derive(Clone, Copy, nearly::NearlyEq, nearly::NearlyOrd, bytemuck::Pod, bytemuck::Zeroable, encase::ShaderType, serde::Serialize, serde::Deserialize)]
pub struct MultiVectorGroups {
    /// scalar, e12345
    g0: Simd32x2,
    /// e1, e2, e3, e4
    g1: Simd32x4,
    /// e5
    g2: f32,
    /// e15, e25, e35, e45
    g3: Simd32x4,
    /// e41, e42, e43
    g4: Simd32x3,
    /// e415, e425, e435, e321
    g5: Simd32x4,
    /// e423, e431, e412
    g6: Simd32x3,
    /// e235, e315, e125
    g7: Simd32x3,
    /// e4235, e4315, e4125, e3215
    g8: Simd32x4,
    /// e1234
    g9: f32,
    /// e12, e31, e23
    g10: Simd32x3,
}
impl MultiVector {
    #[allow(clippy::too_many_arguments)]
    pub const fn from_elements(
        scalar: f32,
        e12345: f32,
        e1: f32,
        e2: f32,
        e3: f32,
        e4: f32,
        e5: f32,
        e15: f32,
        e25: f32,
        e35: f32,
        e45: f32,
        e41: f32,
        e42: f32,
        e43: f32,
        e415: f32,
        e425: f32,
        e435: f32,
        e321: f32,
        e423: f32,
        e431: f32,
        e412: f32,
        e235: f32,
        e315: f32,
        e125: f32,
        e4235: f32,
        e4315: f32,
        e4125: f32,
        e3215: f32,
        e1234: f32,
        e12: f32,
        e31: f32,
        e23: f32,
    ) -> Self {
        Self {
            elements: [
                scalar, e12345, 0.0, 0.0, e1, e2, e3, e4, e5, 0.0, 0.0, 0.0, e15, e25, e35, e45, e41, e42, e43, 0.0, e415, e425, e435, e321, e423, e431, e412, 0.0, e235, e315,
                e125, 0.0, e4235, e4315, e4125, e3215, e1234, 0.0, 0.0, 0.0, e12, e31, e23, 0.0,
            ],
        }
    }
    pub const fn from_groups(
        g0: Simd32x2,
        g1: Simd32x4,
        g2: f32,
        g3: Simd32x4,
        g4: Simd32x3,
        g5: Simd32x4,
        g6: Simd32x3,
        g7: Simd32x3,
        g8: Simd32x4,
        g9: f32,
        g10: Simd32x3,
    ) -> Self {
        Self {
            groups: MultiVectorGroups {
                g0,
                g1,
                g2,
                g3,
                g4,
                g5,
                g6,
                g7,
                g8,
                g9,
                g10,
            },
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
    pub fn group2(&self) -> f32 {
        unsafe { self.groups.g2 }
    }
    #[inline(always)]
    pub fn group2_mut(&mut self) -> &mut f32 {
        unsafe { &mut self.groups.g2 }
    }
    #[inline(always)]
    pub fn group3(&self) -> Simd32x4 {
        unsafe { self.groups.g3 }
    }
    #[inline(always)]
    pub fn group3_mut(&mut self) -> &mut Simd32x4 {
        unsafe { &mut self.groups.g3 }
    }
    #[inline(always)]
    pub fn group4(&self) -> Simd32x3 {
        unsafe { self.groups.g4 }
    }
    #[inline(always)]
    pub fn group4_mut(&mut self) -> &mut Simd32x3 {
        unsafe { &mut self.groups.g4 }
    }
    #[inline(always)]
    pub fn group5(&self) -> Simd32x4 {
        unsafe { self.groups.g5 }
    }
    #[inline(always)]
    pub fn group5_mut(&mut self) -> &mut Simd32x4 {
        unsafe { &mut self.groups.g5 }
    }
    #[inline(always)]
    pub fn group6(&self) -> Simd32x3 {
        unsafe { self.groups.g6 }
    }
    #[inline(always)]
    pub fn group6_mut(&mut self) -> &mut Simd32x3 {
        unsafe { &mut self.groups.g6 }
    }
    #[inline(always)]
    pub fn group7(&self) -> Simd32x3 {
        unsafe { self.groups.g7 }
    }
    #[inline(always)]
    pub fn group7_mut(&mut self) -> &mut Simd32x3 {
        unsafe { &mut self.groups.g7 }
    }
    #[inline(always)]
    pub fn group8(&self) -> Simd32x4 {
        unsafe { self.groups.g8 }
    }
    #[inline(always)]
    pub fn group8_mut(&mut self) -> &mut Simd32x4 {
        unsafe { &mut self.groups.g8 }
    }
    #[inline(always)]
    pub fn group9(&self) -> f32 {
        unsafe { self.groups.g9 }
    }
    #[inline(always)]
    pub fn group9_mut(&mut self) -> &mut f32 {
        unsafe { &mut self.groups.g9 }
    }
    #[inline(always)]
    pub fn group10(&self) -> Simd32x3 {
        unsafe { self.groups.g10 }
    }
    #[inline(always)]
    pub fn group10_mut(&mut self) -> &mut Simd32x3 {
        unsafe { &mut self.groups.g10 }
    }
}
const MULTI_VECTOR_INDEX_REMAP: [usize; 32] = [0, 1, 4, 5, 6, 7, 8, 12, 13, 14, 15, 16, 17, 18, 20, 21, 22, 23, 24, 25, 26, 28, 29, 30, 32, 33, 34, 35, 36, 40, 41, 42];
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
impl From<MultiVector> for [f32; 32] {
    fn from(vector: MultiVector) -> Self {
        unsafe {
            [
                vector.elements[0], vector.elements[1], vector.elements[4], vector.elements[5], vector.elements[6], vector.elements[7], vector.elements[8], vector.elements[12],
                vector.elements[13], vector.elements[14], vector.elements[15], vector.elements[16], vector.elements[17], vector.elements[18], vector.elements[20],
                vector.elements[21], vector.elements[22], vector.elements[23], vector.elements[24], vector.elements[25], vector.elements[26], vector.elements[28],
                vector.elements[29], vector.elements[30], vector.elements[32], vector.elements[33], vector.elements[34], vector.elements[35], vector.elements[36],
                vector.elements[40], vector.elements[41], vector.elements[42],
            ]
        }
    }
}
impl From<[f32; 32]> for MultiVector {
    fn from(array: [f32; 32]) -> Self {
        Self {
            elements: [
                array[0], array[1], 0.0, 0.0, array[1], array[2], array[3], array[4], array[2], 0.0, 0.0, 0.0, array[3], array[4], array[5], array[6], array[4], array[5],
                array[6], 0.0, array[5], array[6], array[7], array[8], array[6], array[7], array[8], 0.0, array[7], array[8], array[9], 0.0, array[8], array[9], array[10],
                array[11], array[9], 0.0, 0.0, 0.0, array[10], array[11], array[12], 0.0,
            ],
        }
    }
}
impl std::fmt::Debug for MultiVector {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .debug_struct("MultiVector")
            .field("scalar", &self[0])
            .field("e12345", &self[1])
            .field("e1", &self[2])
            .field("e2", &self[3])
            .field("e3", &self[4])
            .field("e4", &self[5])
            .field("e5", &self[6])
            .field("e15", &self[7])
            .field("e25", &self[8])
            .field("e35", &self[9])
            .field("e45", &self[10])
            .field("e41", &self[11])
            .field("e42", &self[12])
            .field("e43", &self[13])
            .field("e415", &self[14])
            .field("e425", &self[15])
            .field("e435", &self[16])
            .field("e321", &self[17])
            .field("e423", &self[18])
            .field("e431", &self[19])
            .field("e412", &self[20])
            .field("e235", &self[21])
            .field("e315", &self[22])
            .field("e125", &self[23])
            .field("e4235", &self[24])
            .field("e4315", &self[25])
            .field("e4125", &self[26])
            .field("e3215", &self[27])
            .field("e1234", &self[28])
            .field("e12", &self[29])
            .field("e31", &self[30])
            .field("e23", &self[31])
            .finish()
    }
}

impl MultiVector {
    pub const LEN: usize = 32;
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
impl std::ops::Index<crate::elements::e12345> for MultiVector {
    type Output = f32;
    fn index(&self, _: crate::elements::e12345) -> &Self::Output {
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
impl std::ops::Index<crate::elements::e5> for MultiVector {
    type Output = f32;
    fn index(&self, _: crate::elements::e5) -> &Self::Output {
        &self[6]
    }
}
impl std::ops::Index<crate::elements::e15> for MultiVector {
    type Output = f32;
    fn index(&self, _: crate::elements::e15) -> &Self::Output {
        &self[7]
    }
}
impl std::ops::Index<crate::elements::e25> for MultiVector {
    type Output = f32;
    fn index(&self, _: crate::elements::e25) -> &Self::Output {
        &self[8]
    }
}
impl std::ops::Index<crate::elements::e35> for MultiVector {
    type Output = f32;
    fn index(&self, _: crate::elements::e35) -> &Self::Output {
        &self[9]
    }
}
impl std::ops::Index<crate::elements::e45> for MultiVector {
    type Output = f32;
    fn index(&self, _: crate::elements::e45) -> &Self::Output {
        &self[10]
    }
}
impl std::ops::Index<crate::elements::e41> for MultiVector {
    type Output = f32;
    fn index(&self, _: crate::elements::e41) -> &Self::Output {
        &self[11]
    }
}
impl std::ops::Index<crate::elements::e42> for MultiVector {
    type Output = f32;
    fn index(&self, _: crate::elements::e42) -> &Self::Output {
        &self[12]
    }
}
impl std::ops::Index<crate::elements::e43> for MultiVector {
    type Output = f32;
    fn index(&self, _: crate::elements::e43) -> &Self::Output {
        &self[13]
    }
}
impl std::ops::Index<crate::elements::e415> for MultiVector {
    type Output = f32;
    fn index(&self, _: crate::elements::e415) -> &Self::Output {
        &self[14]
    }
}
impl std::ops::Index<crate::elements::e425> for MultiVector {
    type Output = f32;
    fn index(&self, _: crate::elements::e425) -> &Self::Output {
        &self[15]
    }
}
impl std::ops::Index<crate::elements::e435> for MultiVector {
    type Output = f32;
    fn index(&self, _: crate::elements::e435) -> &Self::Output {
        &self[16]
    }
}
impl std::ops::Index<crate::elements::e321> for MultiVector {
    type Output = f32;
    fn index(&self, _: crate::elements::e321) -> &Self::Output {
        &self[17]
    }
}
impl std::ops::Index<crate::elements::e423> for MultiVector {
    type Output = f32;
    fn index(&self, _: crate::elements::e423) -> &Self::Output {
        &self[18]
    }
}
impl std::ops::Index<crate::elements::e431> for MultiVector {
    type Output = f32;
    fn index(&self, _: crate::elements::e431) -> &Self::Output {
        &self[19]
    }
}
impl std::ops::Index<crate::elements::e412> for MultiVector {
    type Output = f32;
    fn index(&self, _: crate::elements::e412) -> &Self::Output {
        &self[20]
    }
}
impl std::ops::Index<crate::elements::e235> for MultiVector {
    type Output = f32;
    fn index(&self, _: crate::elements::e235) -> &Self::Output {
        &self[21]
    }
}
impl std::ops::Index<crate::elements::e315> for MultiVector {
    type Output = f32;
    fn index(&self, _: crate::elements::e315) -> &Self::Output {
        &self[22]
    }
}
impl std::ops::Index<crate::elements::e125> for MultiVector {
    type Output = f32;
    fn index(&self, _: crate::elements::e125) -> &Self::Output {
        &self[23]
    }
}
impl std::ops::Index<crate::elements::e4235> for MultiVector {
    type Output = f32;
    fn index(&self, _: crate::elements::e4235) -> &Self::Output {
        &self[24]
    }
}
impl std::ops::Index<crate::elements::e4315> for MultiVector {
    type Output = f32;
    fn index(&self, _: crate::elements::e4315) -> &Self::Output {
        &self[25]
    }
}
impl std::ops::Index<crate::elements::e4125> for MultiVector {
    type Output = f32;
    fn index(&self, _: crate::elements::e4125) -> &Self::Output {
        &self[26]
    }
}
impl std::ops::Index<crate::elements::e3215> for MultiVector {
    type Output = f32;
    fn index(&self, _: crate::elements::e3215) -> &Self::Output {
        &self[27]
    }
}
impl std::ops::Index<crate::elements::e1234> for MultiVector {
    type Output = f32;
    fn index(&self, _: crate::elements::e1234) -> &Self::Output {
        &self[28]
    }
}
impl std::ops::Index<crate::elements::e12> for MultiVector {
    type Output = f32;
    fn index(&self, _: crate::elements::e12) -> &Self::Output {
        &self[29]
    }
}
impl std::ops::Index<crate::elements::e31> for MultiVector {
    type Output = f32;
    fn index(&self, _: crate::elements::e31) -> &Self::Output {
        &self[30]
    }
}
impl std::ops::Index<crate::elements::e23> for MultiVector {
    type Output = f32;
    fn index(&self, _: crate::elements::e23) -> &Self::Output {
        &self[31]
    }
}
impl std::ops::IndexMut<crate::elements::scalar> for MultiVector {
    fn index_mut(&self, _: crate::elements::scalar) -> &mut Self::Output {
        &mut self[0]
    }
}
impl std::ops::IndexMut<crate::elements::e12345> for MultiVector {
    fn index_mut(&self, _: crate::elements::e12345) -> &mut Self::Output {
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
impl std::ops::IndexMut<crate::elements::e5> for MultiVector {
    fn index_mut(&self, _: crate::elements::e5) -> &mut Self::Output {
        &mut self[6]
    }
}
impl std::ops::IndexMut<crate::elements::e15> for MultiVector {
    fn index_mut(&self, _: crate::elements::e15) -> &mut Self::Output {
        &mut self[7]
    }
}
impl std::ops::IndexMut<crate::elements::e25> for MultiVector {
    fn index_mut(&self, _: crate::elements::e25) -> &mut Self::Output {
        &mut self[8]
    }
}
impl std::ops::IndexMut<crate::elements::e35> for MultiVector {
    fn index_mut(&self, _: crate::elements::e35) -> &mut Self::Output {
        &mut self[9]
    }
}
impl std::ops::IndexMut<crate::elements::e45> for MultiVector {
    fn index_mut(&self, _: crate::elements::e45) -> &mut Self::Output {
        &mut self[10]
    }
}
impl std::ops::IndexMut<crate::elements::e41> for MultiVector {
    fn index_mut(&self, _: crate::elements::e41) -> &mut Self::Output {
        &mut self[11]
    }
}
impl std::ops::IndexMut<crate::elements::e42> for MultiVector {
    fn index_mut(&self, _: crate::elements::e42) -> &mut Self::Output {
        &mut self[12]
    }
}
impl std::ops::IndexMut<crate::elements::e43> for MultiVector {
    fn index_mut(&self, _: crate::elements::e43) -> &mut Self::Output {
        &mut self[13]
    }
}
impl std::ops::IndexMut<crate::elements::e415> for MultiVector {
    fn index_mut(&self, _: crate::elements::e415) -> &mut Self::Output {
        &mut self[14]
    }
}
impl std::ops::IndexMut<crate::elements::e425> for MultiVector {
    fn index_mut(&self, _: crate::elements::e425) -> &mut Self::Output {
        &mut self[15]
    }
}
impl std::ops::IndexMut<crate::elements::e435> for MultiVector {
    fn index_mut(&self, _: crate::elements::e435) -> &mut Self::Output {
        &mut self[16]
    }
}
impl std::ops::IndexMut<crate::elements::e321> for MultiVector {
    fn index_mut(&self, _: crate::elements::e321) -> &mut Self::Output {
        &mut self[17]
    }
}
impl std::ops::IndexMut<crate::elements::e423> for MultiVector {
    fn index_mut(&self, _: crate::elements::e423) -> &mut Self::Output {
        &mut self[18]
    }
}
impl std::ops::IndexMut<crate::elements::e431> for MultiVector {
    fn index_mut(&self, _: crate::elements::e431) -> &mut Self::Output {
        &mut self[19]
    }
}
impl std::ops::IndexMut<crate::elements::e412> for MultiVector {
    fn index_mut(&self, _: crate::elements::e412) -> &mut Self::Output {
        &mut self[20]
    }
}
impl std::ops::IndexMut<crate::elements::e235> for MultiVector {
    fn index_mut(&self, _: crate::elements::e235) -> &mut Self::Output {
        &mut self[21]
    }
}
impl std::ops::IndexMut<crate::elements::e315> for MultiVector {
    fn index_mut(&self, _: crate::elements::e315) -> &mut Self::Output {
        &mut self[22]
    }
}
impl std::ops::IndexMut<crate::elements::e125> for MultiVector {
    fn index_mut(&self, _: crate::elements::e125) -> &mut Self::Output {
        &mut self[23]
    }
}
impl std::ops::IndexMut<crate::elements::e4235> for MultiVector {
    fn index_mut(&self, _: crate::elements::e4235) -> &mut Self::Output {
        &mut self[24]
    }
}
impl std::ops::IndexMut<crate::elements::e4315> for MultiVector {
    fn index_mut(&self, _: crate::elements::e4315) -> &mut Self::Output {
        &mut self[25]
    }
}
impl std::ops::IndexMut<crate::elements::e4125> for MultiVector {
    fn index_mut(&self, _: crate::elements::e4125) -> &mut Self::Output {
        &mut self[26]
    }
}
impl std::ops::IndexMut<crate::elements::e3215> for MultiVector {
    fn index_mut(&self, _: crate::elements::e3215) -> &mut Self::Output {
        &mut self[27]
    }
}
impl std::ops::IndexMut<crate::elements::e1234> for MultiVector {
    fn index_mut(&self, _: crate::elements::e1234) -> &mut Self::Output {
        &mut self[28]
    }
}
impl std::ops::IndexMut<crate::elements::e12> for MultiVector {
    fn index_mut(&self, _: crate::elements::e12) -> &mut Self::Output {
        &mut self[29]
    }
}
impl std::ops::IndexMut<crate::elements::e31> for MultiVector {
    fn index_mut(&self, _: crate::elements::e31) -> &mut Self::Output {
        &mut self[30]
    }
}
impl std::ops::IndexMut<crate::elements::e23> for MultiVector {
    fn index_mut(&self, _: crate::elements::e23) -> &mut Self::Output {
        &mut self[31]
    }
}
include!("./impls/multi_vector.rs");
