use crate::data::*;
use crate::simd::*;

/// AntiVersorOddOnOrigin.
/// This variant of VersorEvenOrthogonalOrigin is the Dual to VersorOddOnOrigin. It is common for
/// objects of this type to not intersect the null cone, which also prevents them from
/// projecting onto the horosphere in the usual manner. When this happens, this
/// object has behavioral and operative similarity to a VersorEvenOrthogonalOrigin,
/// but an imaginary radius, and a spacial presence in the shape of a
/// VersorOddOnOrigin with a real radius.
#[derive(Clone, Copy, nearly::NearlyEq, nearly::NearlyOrd, bytemuck::Pod, bytemuck::Zeroable, encase::ShaderType, serde::Serialize, serde::Deserialize)]
pub union AntiVersorOddOnOrigin {
    groups: AntiVersorOddOnOriginGroups,
    /// e423, e431, e412, e321, e4, e1, e2, e3
    elements: [f32; 8],
}
#[derive(Clone, Copy, nearly::NearlyEq, nearly::NearlyOrd, bytemuck::Pod, bytemuck::Zeroable, encase::ShaderType, serde::Serialize, serde::Deserialize)]
pub struct AntiVersorOddOnOriginGroups {
    /// e423, e431, e412, e321
    g0: Simd32x4,
    /// e4, e1, e2, e3
    g1: Simd32x4,
}
impl AntiVersorOddOnOrigin {
    #[allow(clippy::too_many_arguments)]
    pub const fn from_elements(e423: f32, e431: f32, e412: f32, e321: f32, e4: f32, e1: f32, e2: f32, e3: f32) -> Self {
        Self {
            elements: [e423, e431, e412, e321, e4, e1, e2, e3],
        }
    }
    pub const fn from_groups(g0: Simd32x4, g1: Simd32x4) -> Self {
        Self {
            groups: AntiVersorOddOnOriginGroups { g0, g1 },
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
const ANTI_VERSOR_ODD_ON_ORIGIN_INDEX_REMAP: [usize; 8] = [0, 1, 2, 3, 4, 5, 6, 7];
impl std::ops::Index<usize> for AntiVersorOddOnOrigin {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[ANTI_VERSOR_ODD_ON_ORIGIN_INDEX_REMAP[index]] }
    }
}
impl std::ops::IndexMut<usize> for AntiVersorOddOnOrigin {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[ANTI_VERSOR_ODD_ON_ORIGIN_INDEX_REMAP[index]] }
    }
}
impl From<AntiVersorOddOnOrigin> for [f32; 8] {
    fn from(vector: AntiVersorOddOnOrigin) -> Self {
        unsafe {
            [
                vector.elements[0], vector.elements[1], vector.elements[2], vector.elements[3], vector.elements[4], vector.elements[5], vector.elements[6], vector.elements[7],
            ]
        }
    }
}
impl From<[f32; 8]> for AntiVersorOddOnOrigin {
    fn from(array: [f32; 8]) -> Self {
        Self {
            elements: [array[0], array[1], array[2], array[3], array[1], array[2], array[3], array[4]],
        }
    }
}
impl std::fmt::Debug for AntiVersorOddOnOrigin {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .debug_struct("AntiVersorOddOnOrigin")
            .field("e423", &self[0])
            .field("e431", &self[1])
            .field("e412", &self[2])
            .field("e321", &self[3])
            .field("e4", &self[4])
            .field("e1", &self[5])
            .field("e2", &self[6])
            .field("e3", &self[7])
            .finish()
    }
}

impl AntiVersorOddOnOrigin {
    pub const LEN: usize = 8;
}

impl AntiVersorOddOnOrigin {
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

impl PartialOrd for AntiVersorOddOnOrigin {
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
impl Ord for AntiVersorOddOnOrigin {
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
impl PartialEq for AntiVersorOddOnOrigin {
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
impl Eq for AntiVersorOddOnOrigin {}
impl std::hash::Hash for AntiVersorOddOnOrigin {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for i in 0..Self::LEN {
            self[i].to_bits().hash(state);
        }
    }
}

impl std::ops::Index<crate::elements::e423> for AntiVersorOddOnOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e423) -> &Self::Output {
        &self[0]
    }
}
impl std::ops::Index<crate::elements::e431> for AntiVersorOddOnOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e431) -> &Self::Output {
        &self[1]
    }
}
impl std::ops::Index<crate::elements::e412> for AntiVersorOddOnOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e412) -> &Self::Output {
        &self[2]
    }
}
impl std::ops::Index<crate::elements::e321> for AntiVersorOddOnOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e321) -> &Self::Output {
        &self[3]
    }
}
impl std::ops::Index<crate::elements::e4> for AntiVersorOddOnOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e4) -> &Self::Output {
        &self[4]
    }
}
impl std::ops::Index<crate::elements::e1> for AntiVersorOddOnOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e1) -> &Self::Output {
        &self[5]
    }
}
impl std::ops::Index<crate::elements::e2> for AntiVersorOddOnOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e2) -> &Self::Output {
        &self[6]
    }
}
impl std::ops::Index<crate::elements::e3> for AntiVersorOddOnOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e3) -> &Self::Output {
        &self[7]
    }
}
impl std::ops::IndexMut<crate::elements::e423> for AntiVersorOddOnOrigin {
    fn index_mut(&self, _: crate::elements::e423) -> &mut Self::Output {
        &mut self[0]
    }
}
impl std::ops::IndexMut<crate::elements::e431> for AntiVersorOddOnOrigin {
    fn index_mut(&self, _: crate::elements::e431) -> &mut Self::Output {
        &mut self[1]
    }
}
impl std::ops::IndexMut<crate::elements::e412> for AntiVersorOddOnOrigin {
    fn index_mut(&self, _: crate::elements::e412) -> &mut Self::Output {
        &mut self[2]
    }
}
impl std::ops::IndexMut<crate::elements::e321> for AntiVersorOddOnOrigin {
    fn index_mut(&self, _: crate::elements::e321) -> &mut Self::Output {
        &mut self[3]
    }
}
impl std::ops::IndexMut<crate::elements::e4> for AntiVersorOddOnOrigin {
    fn index_mut(&self, _: crate::elements::e4) -> &mut Self::Output {
        &mut self[4]
    }
}
impl std::ops::IndexMut<crate::elements::e1> for AntiVersorOddOnOrigin {
    fn index_mut(&self, _: crate::elements::e1) -> &mut Self::Output {
        &mut self[5]
    }
}
impl std::ops::IndexMut<crate::elements::e2> for AntiVersorOddOnOrigin {
    fn index_mut(&self, _: crate::elements::e2) -> &mut Self::Output {
        &mut self[6]
    }
}
impl std::ops::IndexMut<crate::elements::e3> for AntiVersorOddOnOrigin {
    fn index_mut(&self, _: crate::elements::e3) -> &mut Self::Output {
        &mut self[7]
    }
}
include!("./impls/anti_versor_odd_on_origin.rs");
