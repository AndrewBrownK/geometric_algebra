use crate::data::*;
use crate::simd::*;

/// CircleOnOrigin.
/// This variant of Circle intersects the Origin.
#[derive(Clone, Copy, nearly::NearlyEq, nearly::NearlyOrd, bytemuck::Pod, bytemuck::Zeroable, encase::ShaderType, serde::Serialize, serde::Deserialize)]
pub union CircleOnOrigin {
    groups: CircleOnOriginGroups,
    /// e423, e431, e412, 0, e415, e425, e435, 0
    elements: [f32; 8],
}
#[derive(Clone, Copy, nearly::NearlyEq, nearly::NearlyOrd, bytemuck::Pod, bytemuck::Zeroable, encase::ShaderType, serde::Serialize, serde::Deserialize)]
pub struct CircleOnOriginGroups {
    /// e423, e431, e412
    g0: Simd32x3,
    /// e415, e425, e435
    g1: Simd32x3,
}
impl CircleOnOrigin {
    #[allow(clippy::too_many_arguments)]
    pub const fn from_elements(e423: f32, e431: f32, e412: f32, e415: f32, e425: f32, e435: f32) -> Self {
        Self {
            elements: [e423, e431, e412, 0.0, e415, e425, e435, 0.0],
        }
    }
    pub const fn from_groups(g0: Simd32x3, g1: Simd32x3) -> Self {
        Self {
            groups: CircleOnOriginGroups { g0, g1 },
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
const CIRCLE_ON_ORIGIN_INDEX_REMAP: [usize; 6] = [0, 1, 2, 4, 5, 6];
impl std::ops::Index<usize> for CircleOnOrigin {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[CIRCLE_ON_ORIGIN_INDEX_REMAP[index]] }
    }
}
impl std::ops::IndexMut<usize> for CircleOnOrigin {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[CIRCLE_ON_ORIGIN_INDEX_REMAP[index]] }
    }
}
impl From<CircleOnOrigin> for [f32; 6] {
    fn from(vector: CircleOnOrigin) -> Self {
        unsafe { [vector.elements[0], vector.elements[1], vector.elements[2], vector.elements[4], vector.elements[5], vector.elements[6]] }
    }
}
impl From<[f32; 6]> for CircleOnOrigin {
    fn from(array: [f32; 6]) -> Self {
        Self {
            elements: [array[0], array[1], array[2], 0.0, array[1], array[2], array[3], 0.0],
        }
    }
}
impl std::fmt::Debug for CircleOnOrigin {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .debug_struct("CircleOnOrigin")
            .field("e423", &self[0])
            .field("e431", &self[1])
            .field("e412", &self[2])
            .field("e415", &self[3])
            .field("e425", &self[4])
            .field("e435", &self[5])
            .finish()
    }
}

impl CircleOnOrigin {
    pub const LEN: usize = 6;
}

impl CircleOnOrigin {
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

impl PartialOrd for CircleOnOrigin {
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
impl Ord for CircleOnOrigin {
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
impl PartialEq for CircleOnOrigin {
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
impl Eq for CircleOnOrigin {}
impl std::hash::Hash for CircleOnOrigin {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for i in 0..Self::LEN {
            self[i].to_bits().hash(state);
        }
    }
}

impl std::ops::Index<crate::elements::e423> for CircleOnOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e423) -> &Self::Output {
        &self[0]
    }
}
impl std::ops::Index<crate::elements::e431> for CircleOnOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e431) -> &Self::Output {
        &self[1]
    }
}
impl std::ops::Index<crate::elements::e412> for CircleOnOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e412) -> &Self::Output {
        &self[2]
    }
}
impl std::ops::Index<crate::elements::e415> for CircleOnOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e415) -> &Self::Output {
        &self[3]
    }
}
impl std::ops::Index<crate::elements::e425> for CircleOnOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e425) -> &Self::Output {
        &self[4]
    }
}
impl std::ops::Index<crate::elements::e435> for CircleOnOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e435) -> &Self::Output {
        &self[5]
    }
}
impl std::ops::IndexMut<crate::elements::e423> for CircleOnOrigin {
    fn index_mut(&self, _: crate::elements::e423) -> &mut Self::Output {
        &mut self[0]
    }
}
impl std::ops::IndexMut<crate::elements::e431> for CircleOnOrigin {
    fn index_mut(&self, _: crate::elements::e431) -> &mut Self::Output {
        &mut self[1]
    }
}
impl std::ops::IndexMut<crate::elements::e412> for CircleOnOrigin {
    fn index_mut(&self, _: crate::elements::e412) -> &mut Self::Output {
        &mut self[2]
    }
}
impl std::ops::IndexMut<crate::elements::e415> for CircleOnOrigin {
    fn index_mut(&self, _: crate::elements::e415) -> &mut Self::Output {
        &mut self[3]
    }
}
impl std::ops::IndexMut<crate::elements::e425> for CircleOnOrigin {
    fn index_mut(&self, _: crate::elements::e425) -> &mut Self::Output {
        &mut self[4]
    }
}
impl std::ops::IndexMut<crate::elements::e435> for CircleOnOrigin {
    fn index_mut(&self, _: crate::elements::e435) -> &mut Self::Output {
        &mut self[5]
    }
}
include!("./impls/circle_on_origin.rs");
