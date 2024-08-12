use crate::data::*;
use crate::simd::*;

/// CircleAtOrigin.
/// This variant of Circle is centered on the Origin.
#[derive(Clone, Copy, nearly::NearlyEq, nearly::NearlyOrd, bytemuck::Pod, bytemuck::Zeroable, encase::ShaderType, serde::Serialize, serde::Deserialize)]
pub union CircleAtOrigin {
    groups: CircleAtOriginGroups,
    /// e423, e431, e412, 0, e235, e315, e125, 0
    elements: [f32; 8],
}
#[derive(Clone, Copy, nearly::NearlyEq, nearly::NearlyOrd, bytemuck::Pod, bytemuck::Zeroable, encase::ShaderType, serde::Serialize, serde::Deserialize)]
pub struct CircleAtOriginGroups {
    /// e423, e431, e412
    g0: Simd32x3,
    /// e235, e315, e125
    g1: Simd32x3,
}
impl CircleAtOrigin {
    #[allow(clippy::too_many_arguments)]
    pub const fn from_elements(e423: f32, e431: f32, e412: f32, e235: f32, e315: f32, e125: f32) -> Self {
        Self {
            elements: [e423, e431, e412, 0.0, e235, e315, e125, 0.0],
        }
    }
    pub const fn from_groups(g0: Simd32x3, g1: Simd32x3) -> Self {
        Self {
            groups: CircleAtOriginGroups { g0, g1 },
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
    #[inline(always)]
    pub fn group1(&self) -> Simd32x3 {
        unsafe { self.groups.g1 }
    }
    #[inline(always)]
    pub fn group1_mut(&mut self) -> &mut Simd32x3 {
        unsafe { &mut self.groups.g1 }
    }
}
const CIRCLE_AT_ORIGIN_INDEX_REMAP: [usize; 6] = [0, 1, 2, 4, 5, 6];
impl std::ops::Index<usize> for CircleAtOrigin {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[CIRCLE_AT_ORIGIN_INDEX_REMAP[index]] }
    }
}
impl std::ops::IndexMut<usize> for CircleAtOrigin {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[CIRCLE_AT_ORIGIN_INDEX_REMAP[index]] }
    }
}
impl From<CircleAtOrigin> for [f32; 6] {
    fn from(vector: CircleAtOrigin) -> Self {
        unsafe { [vector.elements[0], vector.elements[1], vector.elements[2], vector.elements[4], vector.elements[5], vector.elements[6]] }
    }
}
impl From<[f32; 6]> for CircleAtOrigin {
    fn from(array: [f32; 6]) -> Self {
        Self {
            elements: [array[0], array[1], array[2], 0.0, array[1], array[2], array[3], 0.0],
        }
    }
}
impl std::fmt::Debug for CircleAtOrigin {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .debug_struct("CircleAtOrigin")
            .field("e423", &self[0])
            .field("e431", &self[1])
            .field("e412", &self[2])
            .field("e235", &self[3])
            .field("e315", &self[4])
            .field("e125", &self[5])
            .finish()
    }
}

impl CircleAtOrigin {
    pub const LEN: usize = 6;
}

impl CircleAtOrigin {
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

impl PartialOrd for CircleAtOrigin {
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
impl Ord for CircleAtOrigin {
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
impl PartialEq for CircleAtOrigin {
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
impl Eq for CircleAtOrigin {}
impl std::hash::Hash for CircleAtOrigin {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for i in 0..Self::LEN {
            self[i].to_bits().hash(state);
        }
    }
}

impl std::ops::Index<crate::elements::e423> for CircleAtOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e423) -> &Self::Output {
        &self[0]
    }
}
impl std::ops::Index<crate::elements::e431> for CircleAtOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e431) -> &Self::Output {
        &self[1]
    }
}
impl std::ops::Index<crate::elements::e412> for CircleAtOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e412) -> &Self::Output {
        &self[2]
    }
}
impl std::ops::Index<crate::elements::e235> for CircleAtOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e235) -> &Self::Output {
        &self[3]
    }
}
impl std::ops::Index<crate::elements::e315> for CircleAtOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e315) -> &Self::Output {
        &self[4]
    }
}
impl std::ops::Index<crate::elements::e125> for CircleAtOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e125) -> &Self::Output {
        &self[5]
    }
}
impl std::ops::IndexMut<crate::elements::e423> for CircleAtOrigin {
    fn index_mut(&self, _: crate::elements::e423) -> &mut Self::Output {
        &mut self[0]
    }
}
impl std::ops::IndexMut<crate::elements::e431> for CircleAtOrigin {
    fn index_mut(&self, _: crate::elements::e431) -> &mut Self::Output {
        &mut self[1]
    }
}
impl std::ops::IndexMut<crate::elements::e412> for CircleAtOrigin {
    fn index_mut(&self, _: crate::elements::e412) -> &mut Self::Output {
        &mut self[2]
    }
}
impl std::ops::IndexMut<crate::elements::e235> for CircleAtOrigin {
    fn index_mut(&self, _: crate::elements::e235) -> &mut Self::Output {
        &mut self[3]
    }
}
impl std::ops::IndexMut<crate::elements::e315> for CircleAtOrigin {
    fn index_mut(&self, _: crate::elements::e315) -> &mut Self::Output {
        &mut self[4]
    }
}
impl std::ops::IndexMut<crate::elements::e125> for CircleAtOrigin {
    fn index_mut(&self, _: crate::elements::e125) -> &mut Self::Output {
        &mut self[5]
    }
}
include!("./impls/circle_at_origin.rs");
