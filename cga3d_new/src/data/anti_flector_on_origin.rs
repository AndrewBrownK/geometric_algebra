use crate::data::*;
use crate::simd::*;

/// AntiFlectorOnOrigin.
/// This variant of MultiVector is the Dual to FlectorOnOrigin. It is common for
/// objects of this type to not intersect the null cone, which also prevents them from
/// projecting onto the horosphere in the usual manner. When this happens, this
/// object has behavioral and operative similarity to a MultiVector,
/// but an imaginary radius, and a spacial presence in the shape of a
/// FlectorOnOrigin with a real radius.
#[derive(Clone, Copy, nearly::NearlyEq, nearly::NearlyOrd, bytemuck::Pod, bytemuck::Zeroable, encase::ShaderType, serde::Serialize, serde::Deserialize)]
pub union AntiFlectorOnOrigin {
    groups: AntiFlectorOnOriginGroups,
    /// e321, 0, 0, 0, e1, e2, e3, 0
    elements: [f32; 8],
}
#[derive(Clone, Copy, nearly::NearlyEq, nearly::NearlyOrd, bytemuck::Pod, bytemuck::Zeroable, encase::ShaderType, serde::Serialize, serde::Deserialize)]
pub struct AntiFlectorOnOriginGroups {
    /// e321
    g0: f32,
    /// e1, e2, e3
    g1: Simd32x3,
}
impl AntiFlectorOnOrigin {
    #[allow(clippy::too_many_arguments)]
    pub const fn from_elements(e321: f32, e1: f32, e2: f32, e3: f32) -> Self {
        Self {
            elements: [e321, 0.0, 0.0, 0.0, e1, e2, e3, 0.0],
        }
    }
    pub const fn from_groups(g0: f32, g1: Simd32x3) -> Self {
        Self {
            groups: AntiFlectorOnOriginGroups { g0, g1 },
        }
    }
    #[inline(always)]
    pub fn group0(&self) -> f32 {
        unsafe { self.groups.g0 }
    }
    #[inline(always)]
    pub fn group0_mut(&mut self) -> &mut f32 {
        unsafe { &mut self.groups.g0 }
    }
    #[inline(always)]
    pub fn group1(&self) -> Simd32x3 {
        unsafe { self.groups.g1 }
    }
    #[inline(always)]
    pub fn group1_mut(&mut self) -> &mut Simd32x3 {
        unsafe { &mut self.groups.g1 }
    }
}
const ANTI_FLECTOR_ON_ORIGIN_INDEX_REMAP: [usize; 4] = [0, 4, 5, 6];
impl std::ops::Index<usize> for AntiFlectorOnOrigin {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[ANTI_FLECTOR_ON_ORIGIN_INDEX_REMAP[index]] }
    }
}
impl std::ops::IndexMut<usize> for AntiFlectorOnOrigin {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[ANTI_FLECTOR_ON_ORIGIN_INDEX_REMAP[index]] }
    }
}
impl From<AntiFlectorOnOrigin> for [f32; 4] {
    fn from(vector: AntiFlectorOnOrigin) -> Self {
        unsafe { [vector.elements[0], vector.elements[4], vector.elements[5], vector.elements[6]] }
    }
}
impl From<[f32; 4]> for AntiFlectorOnOrigin {
    fn from(array: [f32; 4]) -> Self {
        Self {
            elements: [array[0], 0.0, 0.0, 0.0, array[1], array[2], array[3], 0.0],
        }
    }
}
impl std::fmt::Debug for AntiFlectorOnOrigin {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .debug_struct("AntiFlectorOnOrigin")
            .field("e321", &self[0])
            .field("e1", &self[1])
            .field("e2", &self[2])
            .field("e3", &self[3])
            .finish()
    }
}

impl AntiFlectorOnOrigin {
    pub const LEN: usize = 4;
}

impl AntiFlectorOnOrigin {
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

impl PartialOrd for AntiFlectorOnOrigin {
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
impl Ord for AntiFlectorOnOrigin {
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
impl PartialEq for AntiFlectorOnOrigin {
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
impl Eq for AntiFlectorOnOrigin {}
impl std::hash::Hash for AntiFlectorOnOrigin {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for i in 0..Self::LEN {
            self[i].to_bits().hash(state);
        }
    }
}

impl std::ops::Index<crate::elements::e321> for AntiFlectorOnOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e321) -> &Self::Output {
        &self[0]
    }
}
impl std::ops::Index<crate::elements::e1> for AntiFlectorOnOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e1) -> &Self::Output {
        &self[1]
    }
}
impl std::ops::Index<crate::elements::e2> for AntiFlectorOnOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e2) -> &Self::Output {
        &self[2]
    }
}
impl std::ops::Index<crate::elements::e3> for AntiFlectorOnOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e3) -> &Self::Output {
        &self[3]
    }
}
impl std::ops::IndexMut<crate::elements::e321> for AntiFlectorOnOrigin {
    fn index_mut(&self, _: crate::elements::e321) -> &mut Self::Output {
        &mut self[0]
    }
}
impl std::ops::IndexMut<crate::elements::e1> for AntiFlectorOnOrigin {
    fn index_mut(&self, _: crate::elements::e1) -> &mut Self::Output {
        &mut self[1]
    }
}
impl std::ops::IndexMut<crate::elements::e2> for AntiFlectorOnOrigin {
    fn index_mut(&self, _: crate::elements::e2) -> &mut Self::Output {
        &mut self[2]
    }
}
impl std::ops::IndexMut<crate::elements::e3> for AntiFlectorOnOrigin {
    fn index_mut(&self, _: crate::elements::e3) -> &mut Self::Output {
        &mut self[3]
    }
}
include!("./impls/anti_flector_on_origin.rs");
