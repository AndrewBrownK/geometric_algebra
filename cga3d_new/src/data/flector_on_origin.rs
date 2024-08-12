use crate::data::*;
use crate::simd::*;

/// FlectorOnOrigin.
/// This variant of Flector intersects the Origin.
#[derive(Clone, Copy, nearly::NearlyEq, nearly::NearlyOrd, bytemuck::Pod, bytemuck::Zeroable, encase::ShaderType, serde::Serialize, serde::Deserialize)]
pub union FlectorOnOrigin {
    groups: FlectorOnOriginGroups,
    /// e45, 0, 0, 0, e4235, e4315, e4125, 0
    elements: [f32; 8],
}
#[derive(Clone, Copy, nearly::NearlyEq, nearly::NearlyOrd, bytemuck::Pod, bytemuck::Zeroable, encase::ShaderType, serde::Serialize, serde::Deserialize)]
pub struct FlectorOnOriginGroups {
    /// e45
    g0: f32,
    /// e4235, e4315, e4125
    g1: Simd32x3,
}
impl FlectorOnOrigin {
    #[allow(clippy::too_many_arguments)]
    pub const fn from_elements(e45: f32, e4235: f32, e4315: f32, e4125: f32) -> Self {
        Self {
            elements: [e45, 0.0, 0.0, 0.0, e4235, e4315, e4125, 0.0],
        }
    }
    pub const fn from_groups(g0: f32, g1: Simd32x3) -> Self {
        Self {
            groups: FlectorOnOriginGroups { g0, g1 },
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
const FLECTOR_ON_ORIGIN_INDEX_REMAP: [usize; 4] = [0, 4, 5, 6];
impl std::ops::Index<usize> for FlectorOnOrigin {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[FLECTOR_ON_ORIGIN_INDEX_REMAP[index]] }
    }
}
impl std::ops::IndexMut<usize> for FlectorOnOrigin {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[FLECTOR_ON_ORIGIN_INDEX_REMAP[index]] }
    }
}
impl From<FlectorOnOrigin> for [f32; 4] {
    fn from(vector: FlectorOnOrigin) -> Self {
        unsafe { [vector.elements[0], vector.elements[4], vector.elements[5], vector.elements[6]] }
    }
}
impl From<[f32; 4]> for FlectorOnOrigin {
    fn from(array: [f32; 4]) -> Self {
        Self {
            elements: [array[0], 0.0, 0.0, 0.0, array[1], array[2], array[3], 0.0],
        }
    }
}
impl std::fmt::Debug for FlectorOnOrigin {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .debug_struct("FlectorOnOrigin")
            .field("e45", &self[0])
            .field("e4235", &self[1])
            .field("e4315", &self[2])
            .field("e4125", &self[3])
            .finish()
    }
}

impl FlectorOnOrigin {
    pub const LEN: usize = 4;
}

impl FlectorOnOrigin {
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

impl PartialOrd for FlectorOnOrigin {
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
impl Ord for FlectorOnOrigin {
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
impl PartialEq for FlectorOnOrigin {
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
impl Eq for FlectorOnOrigin {}
impl std::hash::Hash for FlectorOnOrigin {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for i in 0..Self::LEN {
            self[i].to_bits().hash(state);
        }
    }
}

impl std::ops::Index<crate::elements::e45> for FlectorOnOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e45) -> &Self::Output {
        &self[0]
    }
}
impl std::ops::Index<crate::elements::e4235> for FlectorOnOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e4235) -> &Self::Output {
        &self[1]
    }
}
impl std::ops::Index<crate::elements::e4315> for FlectorOnOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e4315) -> &Self::Output {
        &self[2]
    }
}
impl std::ops::Index<crate::elements::e4125> for FlectorOnOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e4125) -> &Self::Output {
        &self[3]
    }
}
impl std::ops::IndexMut<crate::elements::e45> for FlectorOnOrigin {
    fn index_mut(&self, _: crate::elements::e45) -> &mut Self::Output {
        &mut self[0]
    }
}
impl std::ops::IndexMut<crate::elements::e4235> for FlectorOnOrigin {
    fn index_mut(&self, _: crate::elements::e4235) -> &mut Self::Output {
        &mut self[1]
    }
}
impl std::ops::IndexMut<crate::elements::e4315> for FlectorOnOrigin {
    fn index_mut(&self, _: crate::elements::e4315) -> &mut Self::Output {
        &mut self[2]
    }
}
impl std::ops::IndexMut<crate::elements::e4125> for FlectorOnOrigin {
    fn index_mut(&self, _: crate::elements::e4125) -> &mut Self::Output {
        &mut self[3]
    }
}
include!("./impls/flector_on_origin.rs");
