use crate::data::*;
use crate::simd::*;

/// TripleNum
#[derive(Clone, Copy, nearly::NearlyEq, nearly::NearlyOrd, bytemuck::Pod, bytemuck::Zeroable, encase::ShaderType, serde::Serialize, serde::Deserialize)]
pub union TripleNum {
    groups: TripleNumGroups,
    /// e4, e5, e12345, 0
    elements: [f32; 4],
}
#[derive(Clone, Copy, nearly::NearlyEq, nearly::NearlyOrd, bytemuck::Pod, bytemuck::Zeroable, encase::ShaderType, serde::Serialize, serde::Deserialize)]
pub struct TripleNumGroups {
    /// e4, e5, e12345
    g0: Simd32x3,
}
impl TripleNum {
    #[allow(clippy::too_many_arguments)]
    pub const fn from_elements(e4: f32, e5: f32, e12345: f32) -> Self {
        Self { elements: [e4, e5, e12345, 0.0] }
    }
    pub const fn from_groups(g0: Simd32x3) -> Self {
        Self { groups: TripleNumGroups { g0 } }
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
const TRIPLE_NUM_INDEX_REMAP: [usize; 3] = [0, 1, 2];
impl std::ops::Index<usize> for TripleNum {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[TRIPLE_NUM_INDEX_REMAP[index]] }
    }
}
impl std::ops::IndexMut<usize> for TripleNum {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[TRIPLE_NUM_INDEX_REMAP[index]] }
    }
}
impl From<TripleNum> for [f32; 3] {
    fn from(vector: TripleNum) -> Self {
        unsafe { [vector.elements[0], vector.elements[1], vector.elements[2]] }
    }
}
impl From<[f32; 3]> for TripleNum {
    fn from(array: [f32; 3]) -> Self {
        Self {
            elements: [array[0], array[1], array[2], 0.0],
        }
    }
}
impl std::fmt::Debug for TripleNum {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.debug_struct("TripleNum").field("e4", &self[0]).field("e5", &self[1]).field("e12345", &self[2]).finish()
    }
}

impl TripleNum {
    pub const LEN: usize = 3;
}

impl TripleNum {
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

impl PartialOrd for TripleNum {
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
impl Ord for TripleNum {
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
impl PartialEq for TripleNum {
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
impl Eq for TripleNum {}
impl std::hash::Hash for TripleNum {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for i in 0..Self::LEN {
            self[i].to_bits().hash(state);
        }
    }
}

impl std::ops::Index<crate::elements::e4> for TripleNum {
    type Output = f32;
    fn index(&self, _: crate::elements::e4) -> &Self::Output {
        &self[0]
    }
}
impl std::ops::Index<crate::elements::e5> for TripleNum {
    type Output = f32;
    fn index(&self, _: crate::elements::e5) -> &Self::Output {
        &self[1]
    }
}
impl std::ops::Index<crate::elements::e12345> for TripleNum {
    type Output = f32;
    fn index(&self, _: crate::elements::e12345) -> &Self::Output {
        &self[2]
    }
}
impl std::ops::IndexMut<crate::elements::e4> for TripleNum {
    fn index_mut(&self, _: crate::elements::e4) -> &mut Self::Output {
        &mut self[0]
    }
}
impl std::ops::IndexMut<crate::elements::e5> for TripleNum {
    fn index_mut(&self, _: crate::elements::e5) -> &mut Self::Output {
        &mut self[1]
    }
}
impl std::ops::IndexMut<crate::elements::e12345> for TripleNum {
    fn index_mut(&self, _: crate::elements::e12345) -> &mut Self::Output {
        &mut self[2]
    }
}
include!("./impls/triple_num.rs");
