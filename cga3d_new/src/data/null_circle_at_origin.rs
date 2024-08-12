use crate::data::*;
use crate::simd::*;

/// NullCircleAtOrigin.
/// This variant of Circle has a radius of zero and is centered on the Origin.
#[derive(Clone, Copy, nearly::NearlyEq, nearly::NearlyOrd, bytemuck::Pod, bytemuck::Zeroable, encase::ShaderType, serde::Serialize, serde::Deserialize)]
pub union NullCircleAtOrigin {
    groups: NullCircleAtOriginGroups,
    /// e423, e431, e412, 0
    elements: [f32; 4],
}
#[derive(Clone, Copy, nearly::NearlyEq, nearly::NearlyOrd, bytemuck::Pod, bytemuck::Zeroable, encase::ShaderType, serde::Serialize, serde::Deserialize)]
pub struct NullCircleAtOriginGroups {
    /// e423, e431, e412
    g0: Simd32x3,
}
impl NullCircleAtOrigin {
    #[allow(clippy::too_many_arguments)]
    pub const fn from_elements(e423: f32, e431: f32, e412: f32) -> Self {
        Self {
            elements: [e423, e431, e412, 0.0],
        }
    }
    pub const fn from_groups(g0: Simd32x3) -> Self {
        Self {
            groups: NullCircleAtOriginGroups { g0 },
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
const NULL_CIRCLE_AT_ORIGIN_INDEX_REMAP: [usize; 3] = [0, 1, 2];
impl std::ops::Index<usize> for NullCircleAtOrigin {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[NULL_CIRCLE_AT_ORIGIN_INDEX_REMAP[index]] }
    }
}
impl std::ops::IndexMut<usize> for NullCircleAtOrigin {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[NULL_CIRCLE_AT_ORIGIN_INDEX_REMAP[index]] }
    }
}
impl From<NullCircleAtOrigin> for [f32; 3] {
    fn from(vector: NullCircleAtOrigin) -> Self {
        unsafe { [vector.elements[0], vector.elements[1], vector.elements[2]] }
    }
}
impl From<[f32; 3]> for NullCircleAtOrigin {
    fn from(array: [f32; 3]) -> Self {
        Self {
            elements: [array[0], array[1], array[2], 0.0],
        }
    }
}
impl std::fmt::Debug for NullCircleAtOrigin {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .debug_struct("NullCircleAtOrigin")
            .field("e423", &self[0])
            .field("e431", &self[1])
            .field("e412", &self[2])
            .finish()
    }
}

impl NullCircleAtOrigin {
    pub const LEN: usize = 3;
}

impl NullCircleAtOrigin {
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

impl PartialOrd for NullCircleAtOrigin {
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
impl Ord for NullCircleAtOrigin {
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
impl PartialEq for NullCircleAtOrigin {
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
impl Eq for NullCircleAtOrigin {}
impl std::hash::Hash for NullCircleAtOrigin {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for i in 0..Self::LEN {
            self[i].to_bits().hash(state);
        }
    }
}

impl std::ops::Index<crate::elements::e423> for NullCircleAtOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e423) -> &Self::Output {
        &self[0]
    }
}
impl std::ops::Index<crate::elements::e431> for NullCircleAtOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e431) -> &Self::Output {
        &self[1]
    }
}
impl std::ops::Index<crate::elements::e412> for NullCircleAtOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e412) -> &Self::Output {
        &self[2]
    }
}
impl std::ops::IndexMut<crate::elements::e423> for NullCircleAtOrigin {
    fn index_mut(&self, _: crate::elements::e423) -> &mut Self::Output {
        &mut self[0]
    }
}
impl std::ops::IndexMut<crate::elements::e431> for NullCircleAtOrigin {
    fn index_mut(&self, _: crate::elements::e431) -> &mut Self::Output {
        &mut self[1]
    }
}
impl std::ops::IndexMut<crate::elements::e412> for NullCircleAtOrigin {
    fn index_mut(&self, _: crate::elements::e412) -> &mut Self::Output {
        &mut self[2]
    }
}
include!("./impls/null_circle_at_origin.rs");
