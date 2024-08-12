use crate::data::*;
use crate::simd::*;

/// AntiLineOnOrigin.
/// This variant of DipoleAtInfinity is the Dual to LineOnOrigin. It is common for
/// objects of this type to not intersect the null cone, which also prevents them from
/// projecting onto the horosphere in the usual manner. When this happens, this
/// object has behavioral and operative similarity to a DipoleAtInfinity,
/// but an imaginary radius, and a spacial presence in the shape of a
/// LineOnOrigin with a real radius.
#[derive(Clone, Copy, nearly::NearlyEq, nearly::NearlyOrd, bytemuck::Pod, bytemuck::Zeroable, encase::ShaderType, serde::Serialize, serde::Deserialize)]
pub union AntiLineOnOrigin {
    groups: AntiLineOnOriginGroups,
    /// e23, e31, e12, 0
    elements: [f32; 4],
}
#[derive(Clone, Copy, nearly::NearlyEq, nearly::NearlyOrd, bytemuck::Pod, bytemuck::Zeroable, encase::ShaderType, serde::Serialize, serde::Deserialize)]
pub struct AntiLineOnOriginGroups {
    /// e23, e31, e12
    g0: Simd32x3,
}
impl AntiLineOnOrigin {
    #[allow(clippy::too_many_arguments)]
    pub const fn from_elements(e23: f32, e31: f32, e12: f32) -> Self {
        Self { elements: [e23, e31, e12, 0.0] }
    }
    pub const fn from_groups(g0: Simd32x3) -> Self {
        Self {
            groups: AntiLineOnOriginGroups { g0 },
        }
    }
    #[inline(always)]
    pub fn group0(&self) -> Simd32x3 {
        unsafe { self.groups.g0 }
    }
    #[inline(always)]
    pub fn group0_mut(&mut self) -> &mut Simd32x3 {
        unsafe { &mut self.groups.g0 }
    }
}
const ANTI_LINE_ON_ORIGIN_INDEX_REMAP: [usize; 3] = [0, 1, 2];
impl std::ops::Index<usize> for AntiLineOnOrigin {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[ANTI_LINE_ON_ORIGIN_INDEX_REMAP[index]] }
    }
}
impl std::ops::IndexMut<usize> for AntiLineOnOrigin {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[ANTI_LINE_ON_ORIGIN_INDEX_REMAP[index]] }
    }
}
impl From<AntiLineOnOrigin> for [f32; 3] {
    fn from(vector: AntiLineOnOrigin) -> Self {
        unsafe { [vector.elements[0], vector.elements[1], vector.elements[2]] }
    }
}
impl From<[f32; 3]> for AntiLineOnOrigin {
    fn from(array: [f32; 3]) -> Self {
        Self {
            elements: [array[0], array[1], array[2], 0.0],
        }
    }
}
impl std::fmt::Debug for AntiLineOnOrigin {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.debug_struct("AntiLineOnOrigin").field("e23", &self[0]).field("e31", &self[1]).field("e12", &self[2]).finish()
    }
}

impl AntiLineOnOrigin {
    pub const LEN: usize = 3;
}

impl AntiLineOnOrigin {
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

impl PartialOrd for AntiLineOnOrigin {
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
impl Ord for AntiLineOnOrigin {
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
impl PartialEq for AntiLineOnOrigin {
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
impl Eq for AntiLineOnOrigin {}
impl std::hash::Hash for AntiLineOnOrigin {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for i in 0..Self::LEN {
            self[i].to_bits().hash(state);
        }
    }
}

impl std::ops::Index<crate::elements::e23> for AntiLineOnOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e23) -> &Self::Output {
        &self[0]
    }
}
impl std::ops::Index<crate::elements::e31> for AntiLineOnOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e31) -> &Self::Output {
        &self[1]
    }
}
impl std::ops::Index<crate::elements::e12> for AntiLineOnOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e12) -> &Self::Output {
        &self[2]
    }
}
impl std::ops::IndexMut<crate::elements::e23> for AntiLineOnOrigin {
    fn index_mut(&self, _: crate::elements::e23) -> &mut Self::Output {
        &mut self[0]
    }
}
impl std::ops::IndexMut<crate::elements::e31> for AntiLineOnOrigin {
    fn index_mut(&self, _: crate::elements::e31) -> &mut Self::Output {
        &mut self[1]
    }
}
impl std::ops::IndexMut<crate::elements::e12> for AntiLineOnOrigin {
    fn index_mut(&self, _: crate::elements::e12) -> &mut Self::Output {
        &mut self[2]
    }
}
include!("./impls/anti_line_on_origin.rs");
