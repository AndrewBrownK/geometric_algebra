use crate::data::*;
use crate::simd::*;

/// DualNum5
#[derive(Clone, Copy, nearly::NearlyEq, nearly::NearlyOrd, bytemuck::Pod, bytemuck::Zeroable, encase::ShaderType, serde::Serialize, serde::Deserialize)]
pub union DualNum5 {
    groups: DualNum5Groups,
    /// e5, e12345, 0, 0
    elements: [f32; 4],
}
#[derive(Clone, Copy, nearly::NearlyEq, nearly::NearlyOrd, bytemuck::Pod, bytemuck::Zeroable, encase::ShaderType, serde::Serialize, serde::Deserialize)]
pub struct DualNum5Groups {
    /// e5, e12345
    g0: Simd32x2,
}
impl DualNum5 {
    #[allow(clippy::too_many_arguments)]
    pub const fn from_elements(e5: f32, e12345: f32) -> Self {
        Self { elements: [e5, e12345, 0.0, 0.0] }
    }
    pub const fn from_groups(g0: Simd32x2) -> Self {
        Self { groups: DualNum5Groups { g0 } }
    }
    #[inline(always)]
    pub fn group0(&self) -> Simd32x2 {
        unsafe { self.groups.g0 }
    }
    #[inline(always)]
    pub fn group0_mut(&mut self) -> &mut Simd32x2 {
        unsafe { &mut self.groups.g0 }
    }
}
const DUAL_NUM5_INDEX_REMAP: [usize; 2] = [0, 1];
impl std::ops::Index<usize> for DualNum5 {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[DUAL_NUM5_INDEX_REMAP[index]] }
    }
}
impl std::ops::IndexMut<usize> for DualNum5 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[DUAL_NUM5_INDEX_REMAP[index]] }
    }
}
impl From<DualNum5> for [f32; 2] {
    fn from(vector: DualNum5) -> Self {
        unsafe { [vector.elements[0], vector.elements[1]] }
    }
}
impl From<[f32; 2]> for DualNum5 {
    fn from(array: [f32; 2]) -> Self {
        Self {
            elements: [array[0], array[1], 0.0, 0.0],
        }
    }
}
impl std::fmt::Debug for DualNum5 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.debug_struct("DualNum5").field("e5", &self[0]).field("e12345", &self[1]).finish()
    }
}

impl DualNum5 {
    pub const LEN: usize = 2;
}

impl DualNum5 {
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

impl PartialOrd for DualNum5 {
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
impl Ord for DualNum5 {
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
impl PartialEq for DualNum5 {
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
impl Eq for DualNum5 {}
impl std::hash::Hash for DualNum5 {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for i in 0..Self::LEN {
            self[i].to_bits().hash(state);
        }
    }
}

impl std::ops::Index<crate::elements::e5> for DualNum5 {
    type Output = f32;
    fn index(&self, _: crate::elements::e5) -> &Self::Output {
        &self[0]
    }
}
impl std::ops::Index<crate::elements::e12345> for DualNum5 {
    type Output = f32;
    fn index(&self, _: crate::elements::e12345) -> &Self::Output {
        &self[1]
    }
}
impl std::ops::IndexMut<crate::elements::e5> for DualNum5 {
    fn index_mut(&self, _: crate::elements::e5) -> &mut Self::Output {
        &mut self[0]
    }
}
impl std::ops::IndexMut<crate::elements::e12345> for DualNum5 {
    fn index_mut(&self, _: crate::elements::e12345) -> &mut Self::Output {
        &mut self[1]
    }
}
include!("./impls/dual_num5.rs");
