use crate::data::*;
use crate::simd::*;

/// AntiVersorEvenOnOrigin.
/// This variant of VersorOddOrthogonalOrigin is the Dual to VersorEvenOnOrigin. It is common for
/// objects of this type to not intersect the null cone, which also prevents them from
/// projecting onto the horosphere in the usual manner. When this happens, this
/// object has behavioral and operative similarity to a VersorOddOrthogonalOrigin,
/// but an imaginary radius, and a spacial presence in the shape of a
/// VersorEvenOnOrigin with a real radius.
#[derive(Clone, Copy, nearly::NearlyEq, nearly::NearlyOrd, bytemuck::Pod, bytemuck::Zeroable, encase::ShaderType, serde::Serialize, serde::Deserialize)]
pub union AntiVersorEvenOnOrigin {
    groups: AntiVersorEvenOnOriginGroups,
    /// e41, e42, e43, scalar, e23, e31, e12, e1234
    elements: [f32; 8],
}
#[derive(Clone, Copy, nearly::NearlyEq, nearly::NearlyOrd, bytemuck::Pod, bytemuck::Zeroable, encase::ShaderType, serde::Serialize, serde::Deserialize)]
pub struct AntiVersorEvenOnOriginGroups {
    /// e41, e42, e43, scalar
    g0: Simd32x4,
    /// e23, e31, e12, e1234
    g1: Simd32x4,
}
impl AntiVersorEvenOnOrigin {
    #[allow(clippy::too_many_arguments)]
    pub const fn from_elements(e41: f32, e42: f32, e43: f32, scalar: f32, e23: f32, e31: f32, e12: f32, e1234: f32) -> Self {
        Self {
            elements: [e41, e42, e43, scalar, e23, e31, e12, e1234],
        }
    }
    pub const fn from_groups(g0: Simd32x4, g1: Simd32x4) -> Self {
        Self {
            groups: AntiVersorEvenOnOriginGroups { g0, g1 },
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
}
const ANTI_VERSOR_EVEN_ON_ORIGIN_INDEX_REMAP: [usize; 8] = [0, 1, 2, 3, 4, 5, 6, 7];
impl std::ops::Index<usize> for AntiVersorEvenOnOrigin {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[ANTI_VERSOR_EVEN_ON_ORIGIN_INDEX_REMAP[index]] }
    }
}
impl std::ops::IndexMut<usize> for AntiVersorEvenOnOrigin {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[ANTI_VERSOR_EVEN_ON_ORIGIN_INDEX_REMAP[index]] }
    }
}
impl From<AntiVersorEvenOnOrigin> for [f32; 8] {
    fn from(vector: AntiVersorEvenOnOrigin) -> Self {
        unsafe {
            [
                vector.elements[0], vector.elements[1], vector.elements[2], vector.elements[3], vector.elements[4], vector.elements[5], vector.elements[6], vector.elements[7],
            ]
        }
    }
}
impl From<[f32; 8]> for AntiVersorEvenOnOrigin {
    fn from(array: [f32; 8]) -> Self {
        Self {
            elements: [array[0], array[1], array[2], array[3], array[1], array[2], array[3], array[4]],
        }
    }
}
impl std::fmt::Debug for AntiVersorEvenOnOrigin {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .debug_struct("AntiVersorEvenOnOrigin")
            .field("e41", &self[0])
            .field("e42", &self[1])
            .field("e43", &self[2])
            .field("scalar", &self[3])
            .field("e23", &self[4])
            .field("e31", &self[5])
            .field("e12", &self[6])
            .field("e1234", &self[7])
            .finish()
    }
}

impl AntiVersorEvenOnOrigin {
    pub const LEN: usize = 8;
}

impl AntiVersorEvenOnOrigin {
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

impl PartialOrd for AntiVersorEvenOnOrigin {
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
impl Ord for AntiVersorEvenOnOrigin {
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
impl PartialEq for AntiVersorEvenOnOrigin {
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
impl Eq for AntiVersorEvenOnOrigin {}
impl std::hash::Hash for AntiVersorEvenOnOrigin {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for i in 0..Self::LEN {
            self[i].to_bits().hash(state);
        }
    }
}

impl std::ops::Index<crate::elements::e41> for AntiVersorEvenOnOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e41) -> &Self::Output {
        &self[0]
    }
}
impl std::ops::Index<crate::elements::e42> for AntiVersorEvenOnOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e42) -> &Self::Output {
        &self[1]
    }
}
impl std::ops::Index<crate::elements::e43> for AntiVersorEvenOnOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e43) -> &Self::Output {
        &self[2]
    }
}
impl std::ops::Index<crate::elements::scalar> for AntiVersorEvenOnOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::scalar) -> &Self::Output {
        &self[3]
    }
}
impl std::ops::Index<crate::elements::e23> for AntiVersorEvenOnOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e23) -> &Self::Output {
        &self[4]
    }
}
impl std::ops::Index<crate::elements::e31> for AntiVersorEvenOnOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e31) -> &Self::Output {
        &self[5]
    }
}
impl std::ops::Index<crate::elements::e12> for AntiVersorEvenOnOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e12) -> &Self::Output {
        &self[6]
    }
}
impl std::ops::Index<crate::elements::e1234> for AntiVersorEvenOnOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e1234) -> &Self::Output {
        &self[7]
    }
}
impl std::ops::IndexMut<crate::elements::e41> for AntiVersorEvenOnOrigin {
    fn index_mut(&self, _: crate::elements::e41) -> &mut Self::Output {
        &mut self[0]
    }
}
impl std::ops::IndexMut<crate::elements::e42> for AntiVersorEvenOnOrigin {
    fn index_mut(&self, _: crate::elements::e42) -> &mut Self::Output {
        &mut self[1]
    }
}
impl std::ops::IndexMut<crate::elements::e43> for AntiVersorEvenOnOrigin {
    fn index_mut(&self, _: crate::elements::e43) -> &mut Self::Output {
        &mut self[2]
    }
}
impl std::ops::IndexMut<crate::elements::scalar> for AntiVersorEvenOnOrigin {
    fn index_mut(&self, _: crate::elements::scalar) -> &mut Self::Output {
        &mut self[3]
    }
}
impl std::ops::IndexMut<crate::elements::e23> for AntiVersorEvenOnOrigin {
    fn index_mut(&self, _: crate::elements::e23) -> &mut Self::Output {
        &mut self[4]
    }
}
impl std::ops::IndexMut<crate::elements::e31> for AntiVersorEvenOnOrigin {
    fn index_mut(&self, _: crate::elements::e31) -> &mut Self::Output {
        &mut self[5]
    }
}
impl std::ops::IndexMut<crate::elements::e12> for AntiVersorEvenOnOrigin {
    fn index_mut(&self, _: crate::elements::e12) -> &mut Self::Output {
        &mut self[6]
    }
}
impl std::ops::IndexMut<crate::elements::e1234> for AntiVersorEvenOnOrigin {
    fn index_mut(&self, _: crate::elements::e1234) -> &mut Self::Output {
        &mut self[7]
    }
}
include!("./impls/anti_versor_even_on_origin.rs");
