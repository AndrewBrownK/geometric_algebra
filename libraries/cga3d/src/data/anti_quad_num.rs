use crate::data::*;
use crate::simd::*;

/// AntiQuadNum
#[derive(Clone, Copy, nearly::NearlyEq, nearly::NearlyOrd, bytemuck::Pod, bytemuck::Zeroable, encase::ShaderType, serde::Serialize, serde::Deserialize)]
pub union AntiQuadNum {
    groups: AntiQuadNumGroups,
    /// e1234, e3215, e45, scalar
    elements: [f32; 4],
}
#[derive(Clone, Copy, nearly::NearlyEq, nearly::NearlyOrd, bytemuck::Pod, bytemuck::Zeroable, encase::ShaderType, serde::Serialize, serde::Deserialize)]
pub struct AntiQuadNumGroups {
    /// e1234, e3215, e45, scalar
    g0: Simd32x4,
}
impl AntiQuadNum {
    #[allow(clippy::too_many_arguments)]
    pub const fn from_elements(e1234: f32, e3215: f32, e45: f32, scalar: f32) -> Self {
        Self {
            elements: [e1234, e3215, e45, scalar],
        }
    }
    pub const fn from_groups(g0: Simd32x4) -> Self {
        Self { groups: AntiQuadNumGroups { g0 } }
    }
    #[inline(always)]
    pub fn group0(&self) -> Simd32x4 {
        unsafe { self.groups.g0 }
    }
    #[inline(always)]
    pub fn group0_mut(&mut self) -> &mut Simd32x4 {
        unsafe { &mut self.groups.g0 }
    }
}
const ANTI_QUAD_NUM_INDEX_REMAP: [usize; 4] = [0, 1, 2, 3];
impl std::ops::Index<usize> for AntiQuadNum {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[ANTI_QUAD_NUM_INDEX_REMAP[index]] }
    }
}
impl std::ops::IndexMut<usize> for AntiQuadNum {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[ANTI_QUAD_NUM_INDEX_REMAP[index]] }
    }
}
impl From<AntiQuadNum> for [f32; 4] {
    fn from(vector: AntiQuadNum) -> Self {
        unsafe { [vector.elements[0], vector.elements[1], vector.elements[2], vector.elements[3]] }
    }
}
impl From<[f32; 4]> for AntiQuadNum {
    fn from(array: [f32; 4]) -> Self {
        Self {
            elements: [array[0], array[1], array[2], array[3]],
        }
    }
}
impl std::fmt::Debug for AntiQuadNum {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .debug_struct("AntiQuadNum")
            .field("e1234", &self[0])
            .field("e3215", &self[1])
            .field("e45", &self[2])
            .field("scalar", &self[3])
            .finish()
    }
}

impl AntiQuadNum {
    pub const LEN: usize = 4;
}

impl AntiQuadNum {
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

impl PartialOrd for AntiQuadNum {
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
impl Ord for AntiQuadNum {
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
impl PartialEq for AntiQuadNum {
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
impl Eq for AntiQuadNum {}
impl std::hash::Hash for AntiQuadNum {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for i in 0..Self::LEN {
            self[i].to_bits().hash(state);
        }
    }
}

impl std::ops::Index<crate::elements::e1234> for AntiQuadNum {
    type Output = f32;
    fn index(&self, _: crate::elements::e1234) -> &Self::Output {
        &self[0]
    }
}
impl std::ops::Index<crate::elements::e3215> for AntiQuadNum {
    type Output = f32;
    fn index(&self, _: crate::elements::e3215) -> &Self::Output {
        &self[1]
    }
}
impl std::ops::Index<crate::elements::e45> for AntiQuadNum {
    type Output = f32;
    fn index(&self, _: crate::elements::e45) -> &Self::Output {
        &self[2]
    }
}
impl std::ops::Index<crate::elements::scalar> for AntiQuadNum {
    type Output = f32;
    fn index(&self, _: crate::elements::scalar) -> &Self::Output {
        &self[3]
    }
}
impl std::ops::IndexMut<crate::elements::e1234> for AntiQuadNum {
    fn index_mut(&self, _: crate::elements::e1234) -> &mut Self::Output {
        &mut self[0]
    }
}
impl std::ops::IndexMut<crate::elements::e3215> for AntiQuadNum {
    fn index_mut(&self, _: crate::elements::e3215) -> &mut Self::Output {
        &mut self[1]
    }
}
impl std::ops::IndexMut<crate::elements::e45> for AntiQuadNum {
    fn index_mut(&self, _: crate::elements::e45) -> &mut Self::Output {
        &mut self[2]
    }
}
impl std::ops::IndexMut<crate::elements::scalar> for AntiQuadNum {
    fn index_mut(&self, _: crate::elements::scalar) -> &mut Self::Output {
        &mut self[3]
    }
}
include!("./impls/anti_quad_num.rs");
