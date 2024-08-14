use crate::data::*;
use crate::simd::*;

/// VersorOddOrthogonalOrigin.
/// This variant of VersorOdd has a CoCarrier that intersects the Origin.
#[derive(Clone, Copy, nearly::NearlyEq, nearly::NearlyOrd, bytemuck::Pod, bytemuck::Zeroable, encase::ShaderType, serde::Serialize, serde::Deserialize)]
pub union VersorOddOrthogonalOrigin {
    groups: VersorOddOrthogonalOriginGroups,
    /// e41, e42, e43, scalar, e23, e31, e12, e3215, e15, e25, e35, e1234
    elements: [f32; 12],
}
#[derive(Clone, Copy, nearly::NearlyEq, nearly::NearlyOrd, bytemuck::Pod, bytemuck::Zeroable, encase::ShaderType, serde::Serialize, serde::Deserialize)]
pub struct VersorOddOrthogonalOriginGroups {
    /// e41, e42, e43, scalar
    g0: Simd32x4,
    /// e23, e31, e12, e3215
    g1: Simd32x4,
    /// e15, e25, e35, e1234
    g2: Simd32x4,
}
impl VersorOddOrthogonalOrigin {
    #[allow(clippy::too_many_arguments)]
    pub const fn from_elements(e41: f32, e42: f32, e43: f32, scalar: f32, e23: f32, e31: f32, e12: f32, e3215: f32, e15: f32, e25: f32, e35: f32, e1234: f32) -> Self {
        Self {
            elements: [e41, e42, e43, scalar, e23, e31, e12, e3215, e15, e25, e35, e1234],
        }
    }
    pub const fn from_groups(g0: Simd32x4, g1: Simd32x4, g2: Simd32x4) -> Self {
        Self {
            groups: VersorOddOrthogonalOriginGroups { g0, g1, g2 },
        }
    }
    #[inline(always)]
    pub fn group0(&self) -> Simd32x4 {
        unsafe { self.groups.g0 }
    }
    #[inline(always)]
    pub fn group0_mut(&mut self) -> &mut Simd32x4 {
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
    pub fn group2(&self) -> Simd32x4 {
        unsafe { self.groups.g2 }
    }
    #[inline(always)]
    pub fn group2_mut(&mut self) -> &mut Simd32x4 {
        unsafe { &mut self.groups.g2 }
    }
}
const VERSOR_ODD_ORTHOGONAL_ORIGIN_INDEX_REMAP: [usize; 12] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
impl std::ops::Index<usize> for VersorOddOrthogonalOrigin {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[VERSOR_ODD_ORTHOGONAL_ORIGIN_INDEX_REMAP[index]] }
    }
}
impl std::ops::IndexMut<usize> for VersorOddOrthogonalOrigin {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[VERSOR_ODD_ORTHOGONAL_ORIGIN_INDEX_REMAP[index]] }
    }
}
impl From<VersorOddOrthogonalOrigin> for [f32; 12] {
    fn from(vector: VersorOddOrthogonalOrigin) -> Self {
        unsafe {
            [
                vector.elements[0], vector.elements[1], vector.elements[2], vector.elements[3], vector.elements[4], vector.elements[5], vector.elements[6], vector.elements[7],
                vector.elements[8], vector.elements[9], vector.elements[10], vector.elements[11],
            ]
        }
    }
}
impl From<[f32; 12]> for VersorOddOrthogonalOrigin {
    fn from(array: [f32; 12]) -> Self {
        Self {
            elements: [array[0], array[1], array[2], array[3], array[1], array[2], array[3], array[4], array[2], array[3], array[4], array[5]],
        }
    }
}
impl std::fmt::Debug for VersorOddOrthogonalOrigin {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .debug_struct("VersorOddOrthogonalOrigin")
            .field("e41", &self[0])
            .field("e42", &self[1])
            .field("e43", &self[2])
            .field("scalar", &self[3])
            .field("e23", &self[4])
            .field("e31", &self[5])
            .field("e12", &self[6])
            .field("e3215", &self[7])
            .field("e15", &self[8])
            .field("e25", &self[9])
            .field("e35", &self[10])
            .field("e1234", &self[11])
            .finish()
    }
}

impl VersorOddOrthogonalOrigin {
    pub const LEN: usize = 12;
}

impl VersorOddOrthogonalOrigin {
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

impl PartialOrd for VersorOddOrthogonalOrigin {
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
impl Ord for VersorOddOrthogonalOrigin {
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
impl PartialEq for VersorOddOrthogonalOrigin {
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
impl Eq for VersorOddOrthogonalOrigin {}
impl std::hash::Hash for VersorOddOrthogonalOrigin {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for i in 0..Self::LEN {
            self[i].to_bits().hash(state);
        }
    }
}

impl std::ops::Index<crate::elements::e41> for VersorOddOrthogonalOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e41) -> &Self::Output {
        &self[0]
    }
}
impl std::ops::Index<crate::elements::e42> for VersorOddOrthogonalOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e42) -> &Self::Output {
        &self[1]
    }
}
impl std::ops::Index<crate::elements::e43> for VersorOddOrthogonalOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e43) -> &Self::Output {
        &self[2]
    }
}
impl std::ops::Index<crate::elements::scalar> for VersorOddOrthogonalOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::scalar) -> &Self::Output {
        &self[3]
    }
}
impl std::ops::Index<crate::elements::e23> for VersorOddOrthogonalOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e23) -> &Self::Output {
        &self[4]
    }
}
impl std::ops::Index<crate::elements::e31> for VersorOddOrthogonalOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e31) -> &Self::Output {
        &self[5]
    }
}
impl std::ops::Index<crate::elements::e12> for VersorOddOrthogonalOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e12) -> &Self::Output {
        &self[6]
    }
}
impl std::ops::Index<crate::elements::e3215> for VersorOddOrthogonalOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e3215) -> &Self::Output {
        &self[7]
    }
}
impl std::ops::Index<crate::elements::e15> for VersorOddOrthogonalOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e15) -> &Self::Output {
        &self[8]
    }
}
impl std::ops::Index<crate::elements::e25> for VersorOddOrthogonalOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e25) -> &Self::Output {
        &self[9]
    }
}
impl std::ops::Index<crate::elements::e35> for VersorOddOrthogonalOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e35) -> &Self::Output {
        &self[10]
    }
}
impl std::ops::Index<crate::elements::e1234> for VersorOddOrthogonalOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e1234) -> &Self::Output {
        &self[11]
    }
}
impl std::ops::IndexMut<crate::elements::e41> for VersorOddOrthogonalOrigin {
    fn index_mut(&self, _: crate::elements::e41) -> &mut Self::Output {
        &mut self[0]
    }
}
impl std::ops::IndexMut<crate::elements::e42> for VersorOddOrthogonalOrigin {
    fn index_mut(&self, _: crate::elements::e42) -> &mut Self::Output {
        &mut self[1]
    }
}
impl std::ops::IndexMut<crate::elements::e43> for VersorOddOrthogonalOrigin {
    fn index_mut(&self, _: crate::elements::e43) -> &mut Self::Output {
        &mut self[2]
    }
}
impl std::ops::IndexMut<crate::elements::scalar> for VersorOddOrthogonalOrigin {
    fn index_mut(&self, _: crate::elements::scalar) -> &mut Self::Output {
        &mut self[3]
    }
}
impl std::ops::IndexMut<crate::elements::e23> for VersorOddOrthogonalOrigin {
    fn index_mut(&self, _: crate::elements::e23) -> &mut Self::Output {
        &mut self[4]
    }
}
impl std::ops::IndexMut<crate::elements::e31> for VersorOddOrthogonalOrigin {
    fn index_mut(&self, _: crate::elements::e31) -> &mut Self::Output {
        &mut self[5]
    }
}
impl std::ops::IndexMut<crate::elements::e12> for VersorOddOrthogonalOrigin {
    fn index_mut(&self, _: crate::elements::e12) -> &mut Self::Output {
        &mut self[6]
    }
}
impl std::ops::IndexMut<crate::elements::e3215> for VersorOddOrthogonalOrigin {
    fn index_mut(&self, _: crate::elements::e3215) -> &mut Self::Output {
        &mut self[7]
    }
}
impl std::ops::IndexMut<crate::elements::e15> for VersorOddOrthogonalOrigin {
    fn index_mut(&self, _: crate::elements::e15) -> &mut Self::Output {
        &mut self[8]
    }
}
impl std::ops::IndexMut<crate::elements::e25> for VersorOddOrthogonalOrigin {
    fn index_mut(&self, _: crate::elements::e25) -> &mut Self::Output {
        &mut self[9]
    }
}
impl std::ops::IndexMut<crate::elements::e35> for VersorOddOrthogonalOrigin {
    fn index_mut(&self, _: crate::elements::e35) -> &mut Self::Output {
        &mut self[10]
    }
}
impl std::ops::IndexMut<crate::elements::e1234> for VersorOddOrthogonalOrigin {
    fn index_mut(&self, _: crate::elements::e1234) -> &mut Self::Output {
        &mut self[11]
    }
}
include!("./impls/versor_odd_orthogonal_origin.rs");
