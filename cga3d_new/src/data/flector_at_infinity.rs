use crate::data::*;
use crate::simd::*;

/// FlectorAtInfinity.
/// This variant of Flector exists in the Horizon.
#[derive(Clone, Copy, nearly::NearlyEq, nearly::NearlyOrd, bytemuck::Pod, bytemuck::Zeroable, encase::ShaderType, serde::Serialize, serde::Deserialize)]
pub union FlectorAtInfinity {
    groups: FlectorAtInfinityGroups,
    /// e15, e25, e35, 0, e3215, 0, 0, 0
    elements: [f32; 8],
}
#[derive(Clone, Copy, nearly::NearlyEq, nearly::NearlyOrd, bytemuck::Pod, bytemuck::Zeroable, encase::ShaderType, serde::Serialize, serde::Deserialize)]
pub struct FlectorAtInfinityGroups {
    /// e15, e25, e35
    g0: Simd32x3,
    /// e3215
    g1: f32,
}
impl FlectorAtInfinity {
    #[allow(clippy::too_many_arguments)]
    pub const fn from_elements(e15: f32, e25: f32, e35: f32, e3215: f32) -> Self {
        Self {
            elements: [e15, e25, e35, 0.0, e3215, 0.0, 0.0, 0.0],
        }
    }
    pub const fn from_groups(g0: Simd32x3, g1: f32) -> Self {
        Self {
            groups: FlectorAtInfinityGroups { g0, g1 },
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
    pub fn group1(&self) -> f32 {
        unsafe { self.groups.g1 }
    }
    #[inline(always)]
    pub fn group1_mut(&mut self) -> &mut f32 {
        unsafe { &mut self.groups.g1 }
    }
}
const FLECTOR_AT_INFINITY_INDEX_REMAP: [usize; 4] = [0, 1, 2, 4];
impl std::ops::Index<usize> for FlectorAtInfinity {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[FLECTOR_AT_INFINITY_INDEX_REMAP[index]] }
    }
}
impl std::ops::IndexMut<usize> for FlectorAtInfinity {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[FLECTOR_AT_INFINITY_INDEX_REMAP[index]] }
    }
}
impl From<FlectorAtInfinity> for [f32; 4] {
    fn from(vector: FlectorAtInfinity) -> Self {
        unsafe { [vector.elements[0], vector.elements[1], vector.elements[2], vector.elements[4]] }
    }
}
impl From<[f32; 4]> for FlectorAtInfinity {
    fn from(array: [f32; 4]) -> Self {
        Self {
            elements: [array[0], array[1], array[2], 0.0, array[1], 0.0, 0.0, 0.0],
        }
    }
}
impl std::fmt::Debug for FlectorAtInfinity {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .debug_struct("FlectorAtInfinity")
            .field("e15", &self[0])
            .field("e25", &self[1])
            .field("e35", &self[2])
            .field("e3215", &self[3])
            .finish()
    }
}

impl FlectorAtInfinity {
    pub const LEN: usize = 4;
}

impl FlectorAtInfinity {
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

impl PartialOrd for FlectorAtInfinity {
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
impl Ord for FlectorAtInfinity {
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
impl PartialEq for FlectorAtInfinity {
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
impl Eq for FlectorAtInfinity {}
impl std::hash::Hash for FlectorAtInfinity {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for i in 0..Self::LEN {
            self[i].to_bits().hash(state);
        }
    }
}

impl std::ops::Index<crate::elements::e15> for FlectorAtInfinity {
    type Output = f32;
    fn index(&self, _: crate::elements::e15) -> &Self::Output {
        &self[0]
    }
}
impl std::ops::Index<crate::elements::e25> for FlectorAtInfinity {
    type Output = f32;
    fn index(&self, _: crate::elements::e25) -> &Self::Output {
        &self[1]
    }
}
impl std::ops::Index<crate::elements::e35> for FlectorAtInfinity {
    type Output = f32;
    fn index(&self, _: crate::elements::e35) -> &Self::Output {
        &self[2]
    }
}
impl std::ops::Index<crate::elements::e3215> for FlectorAtInfinity {
    type Output = f32;
    fn index(&self, _: crate::elements::e3215) -> &Self::Output {
        &self[3]
    }
}
impl std::ops::IndexMut<crate::elements::e15> for FlectorAtInfinity {
    fn index_mut(&self, _: crate::elements::e15) -> &mut Self::Output {
        &mut self[0]
    }
}
impl std::ops::IndexMut<crate::elements::e25> for FlectorAtInfinity {
    fn index_mut(&self, _: crate::elements::e25) -> &mut Self::Output {
        &mut self[1]
    }
}
impl std::ops::IndexMut<crate::elements::e35> for FlectorAtInfinity {
    fn index_mut(&self, _: crate::elements::e35) -> &mut Self::Output {
        &mut self[2]
    }
}
impl std::ops::IndexMut<crate::elements::e3215> for FlectorAtInfinity {
    fn index_mut(&self, _: crate::elements::e3215) -> &mut Self::Output {
        &mut self[3]
    }
}
include!("./impls/flector_at_infinity.rs");
