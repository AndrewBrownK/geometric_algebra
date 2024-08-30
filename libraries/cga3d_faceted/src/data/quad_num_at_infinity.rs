use crate::data::*;
use crate::simd::*;

/// QuadNumAtInfinity.
/// This variant of QuadNum exists at the Horizon.
#[derive(Clone, Copy, nearly::NearlyEq, nearly::NearlyOrd, bytemuck::Pod, bytemuck::Zeroable, encase::ShaderType, serde::Serialize, serde::Deserialize)]
pub union QuadNumAtInfinity {
    groups: QuadNumAtInfinityGroups,
    /// e5, e321, e12345, 0
    elements: [f32; 4],
}
#[derive(Clone, Copy, nearly::NearlyEq, nearly::NearlyOrd, bytemuck::Pod, bytemuck::Zeroable, encase::ShaderType, serde::Serialize, serde::Deserialize)]
pub struct QuadNumAtInfinityGroups {
    /// e5, e321, e12345
    g0: Simd32x3,
}
impl QuadNumAtInfinity {
    #[allow(clippy::too_many_arguments)]
    pub const fn from_elements(e5: f32, e321: f32, e12345: f32) -> Self {
        Self {
            elements: [e5, e321, e12345, 0.0],
        }
    }
    pub const fn from_groups(g0: Simd32x3) -> Self {
        Self {
            groups: QuadNumAtInfinityGroups { g0 },
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
const QUAD_NUM_AT_INFINITY_INDEX_REMAP: [usize; 3] = [0, 1, 2];
impl std::ops::Index<usize> for QuadNumAtInfinity {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[QUAD_NUM_AT_INFINITY_INDEX_REMAP[index]] }
    }
}
impl std::ops::IndexMut<usize> for QuadNumAtInfinity {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[QUAD_NUM_AT_INFINITY_INDEX_REMAP[index]] }
    }
}
impl From<QuadNumAtInfinity> for [f32; 3] {
    fn from(vector: QuadNumAtInfinity) -> Self {
        unsafe { [vector.elements[0], vector.elements[1], vector.elements[2]] }
    }
}
impl From<[f32; 3]> for QuadNumAtInfinity {
    fn from(array: [f32; 3]) -> Self {
        Self {
            elements: [array[0], array[1], array[2], 0.0],
        }
    }
}
impl std::fmt::Debug for QuadNumAtInfinity {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .debug_struct("QuadNumAtInfinity")
            .field("e5", &self[0])
            .field("e321", &self[1])
            .field("e12345", &self[2])
            .finish()
    }
}

impl QuadNumAtInfinity {
    pub const LEN: usize = 3;
}

impl QuadNumAtInfinity {
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

impl PartialOrd for QuadNumAtInfinity {
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
impl Ord for QuadNumAtInfinity {
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
impl PartialEq for QuadNumAtInfinity {
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
impl Eq for QuadNumAtInfinity {}
impl std::hash::Hash for QuadNumAtInfinity {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for i in 0..Self::LEN {
            self[i].to_bits().hash(state);
        }
    }
}

impl std::ops::Index<crate::elements::e5> for QuadNumAtInfinity {
    type Output = f32;
    fn index(&self, _: crate::elements::e5) -> &Self::Output {
        &self[0]
    }
}
impl std::ops::Index<crate::elements::e321> for QuadNumAtInfinity {
    type Output = f32;
    fn index(&self, _: crate::elements::e321) -> &Self::Output {
        &self[1]
    }
}
impl std::ops::Index<crate::elements::e12345> for QuadNumAtInfinity {
    type Output = f32;
    fn index(&self, _: crate::elements::e12345) -> &Self::Output {
        &self[2]
    }
}
impl std::ops::IndexMut<crate::elements::e5> for QuadNumAtInfinity {
    fn index_mut(&self, _: crate::elements::e5) -> &mut Self::Output {
        &mut self[0]
    }
}
impl std::ops::IndexMut<crate::elements::e321> for QuadNumAtInfinity {
    fn index_mut(&self, _: crate::elements::e321) -> &mut Self::Output {
        &mut self[1]
    }
}
impl std::ops::IndexMut<crate::elements::e12345> for QuadNumAtInfinity {
    fn index_mut(&self, _: crate::elements::e12345) -> &mut Self::Output {
        &mut self[2]
    }
}
include!("./impls/quad_num_at_infinity.rs");
