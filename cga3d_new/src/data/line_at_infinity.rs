use crate::data::*;
use crate::simd::*;

/// LineAtInfinity.
/// This variant of Line exists in the Horizon.
#[derive(Clone, Copy, nearly::NearlyEq, nearly::NearlyOrd, bytemuck::Pod, bytemuck::Zeroable, encase::ShaderType, serde::Serialize, serde::Deserialize)]
pub union LineAtInfinity {
    groups: LineAtInfinityGroups,
    /// e235, e315, e125, 0
    elements: [f32; 4],
}
#[derive(Clone, Copy, nearly::NearlyEq, nearly::NearlyOrd, bytemuck::Pod, bytemuck::Zeroable, encase::ShaderType, serde::Serialize, serde::Deserialize)]
pub struct LineAtInfinityGroups {
    /// e235, e315, e125
    g0: Simd32x3,
}
impl LineAtInfinity {
    #[allow(clippy::too_many_arguments)]
    pub const fn from_elements(e235: f32, e315: f32, e125: f32) -> Self {
        Self {
            elements: [e235, e315, e125, 0.0],
        }
    }
    pub const fn from_groups(g0: Simd32x3) -> Self {
        Self {
            groups: LineAtInfinityGroups { g0 },
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
const LINE_AT_INFINITY_INDEX_REMAP: [usize; 3] = [0, 1, 2];
impl std::ops::Index<usize> for LineAtInfinity {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[LINE_AT_INFINITY_INDEX_REMAP[index]] }
    }
}
impl std::ops::IndexMut<usize> for LineAtInfinity {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[LINE_AT_INFINITY_INDEX_REMAP[index]] }
    }
}
impl From<LineAtInfinity> for [f32; 3] {
    fn from(vector: LineAtInfinity) -> Self {
        unsafe { [vector.elements[0], vector.elements[1], vector.elements[2]] }
    }
}
impl From<[f32; 3]> for LineAtInfinity {
    fn from(array: [f32; 3]) -> Self {
        Self {
            elements: [array[0], array[1], array[2], 0.0],
        }
    }
}
impl std::fmt::Debug for LineAtInfinity {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .debug_struct("LineAtInfinity")
            .field("e235", &self[0])
            .field("e315", &self[1])
            .field("e125", &self[2])
            .finish()
    }
}

impl LineAtInfinity {
    pub const LEN: usize = 3;
}

impl LineAtInfinity {
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

impl PartialOrd for LineAtInfinity {
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
impl Ord for LineAtInfinity {
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
impl PartialEq for LineAtInfinity {
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
impl Eq for LineAtInfinity {}
impl std::hash::Hash for LineAtInfinity {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for i in 0..Self::LEN {
            self[i].to_bits().hash(state);
        }
    }
}

impl std::ops::Index<crate::elements::e235> for LineAtInfinity {
    type Output = f32;
    fn index(&self, _: crate::elements::e235) -> &Self::Output {
        &self[0]
    }
}
impl std::ops::Index<crate::elements::e315> for LineAtInfinity {
    type Output = f32;
    fn index(&self, _: crate::elements::e315) -> &Self::Output {
        &self[1]
    }
}
impl std::ops::Index<crate::elements::e125> for LineAtInfinity {
    type Output = f32;
    fn index(&self, _: crate::elements::e125) -> &Self::Output {
        &self[2]
    }
}
impl std::ops::IndexMut<crate::elements::e235> for LineAtInfinity {
    fn index_mut(&self, _: crate::elements::e235) -> &mut Self::Output {
        &mut self[0]
    }
}
impl std::ops::IndexMut<crate::elements::e315> for LineAtInfinity {
    fn index_mut(&self, _: crate::elements::e315) -> &mut Self::Output {
        &mut self[1]
    }
}
impl std::ops::IndexMut<crate::elements::e125> for LineAtInfinity {
    fn index_mut(&self, _: crate::elements::e125) -> &mut Self::Output {
        &mut self[2]
    }
}
include!("./impls/line_at_infinity.rs");
