use crate::data::*;
use crate::simd::*;

/// AntiLine.
/// This variant of DipoleAtInfinity is the Dual to Line. It is common for
/// objects of this type to not intersect the null cone, which also prevents them from
/// projecting onto the horosphere in the usual manner. When this happens, this
/// object has behavioral and operative similarity to a DipoleAtInfinity,
/// but an imaginary radius, and a spacial presence in the shape of a
/// Line with a real radius.
#[derive(Clone, Copy, nearly::NearlyEq, nearly::NearlyOrd, bytemuck::Pod, bytemuck::Zeroable, encase::ShaderType, serde::Serialize, serde::Deserialize)]
pub union AntiLine {
    groups: AntiLineGroups,
    /// e23, e31, e12, 0, e15, e25, e35, 0
    elements: [f32; 8],
}
#[derive(Clone, Copy, nearly::NearlyEq, nearly::NearlyOrd, bytemuck::Pod, bytemuck::Zeroable, encase::ShaderType, serde::Serialize, serde::Deserialize)]
pub struct AntiLineGroups {
    /// e23, e31, e12
    g0: Simd32x3,
    /// e15, e25, e35
    g1: Simd32x3,
}
impl AntiLine {
    #[allow(clippy::too_many_arguments)]
    pub const fn from_elements(e23: f32, e31: f32, e12: f32, e15: f32, e25: f32, e35: f32) -> Self {
        Self {
            elements: [e23, e31, e12, 0.0, e15, e25, e35, 0.0],
        }
    }
    pub const fn from_groups(g0: Simd32x3, g1: Simd32x3) -> Self {
        Self {
            groups: AntiLineGroups { g0, g1 },
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
const ANTI_LINE_INDEX_REMAP: [usize; 6] = [0, 1, 2, 4, 5, 6];
impl std::ops::Index<usize> for AntiLine {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[ANTI_LINE_INDEX_REMAP[index]] }
    }
}
impl std::ops::IndexMut<usize> for AntiLine {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[ANTI_LINE_INDEX_REMAP[index]] }
    }
}
impl From<AntiLine> for [f32; 6] {
    fn from(vector: AntiLine) -> Self {
        unsafe { [vector.elements[0], vector.elements[1], vector.elements[2], vector.elements[4], vector.elements[5], vector.elements[6]] }
    }
}
impl From<[f32; 6]> for AntiLine {
    fn from(array: [f32; 6]) -> Self {
        Self {
            elements: [array[0], array[1], array[2], 0.0, array[1], array[2], array[3], 0.0],
        }
    }
}
impl std::fmt::Debug for AntiLine {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .debug_struct("AntiLine")
            .field("e23", &self[0])
            .field("e31", &self[1])
            .field("e12", &self[2])
            .field("e15", &self[3])
            .field("e25", &self[4])
            .field("e35", &self[5])
            .finish()
    }
}

impl AntiLine {
    pub const LEN: usize = 6;
}

impl AntiLine {
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

impl PartialOrd for AntiLine {
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
impl Ord for AntiLine {
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
impl PartialEq for AntiLine {
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
impl Eq for AntiLine {}
impl std::hash::Hash for AntiLine {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for i in 0..Self::LEN {
            self[i].to_bits().hash(state);
        }
    }
}

impl std::ops::Index<crate::elements::e23> for AntiLine {
    type Output = f32;
    fn index(&self, _: crate::elements::e23) -> &Self::Output {
        &self[0]
    }
}
impl std::ops::Index<crate::elements::e31> for AntiLine {
    type Output = f32;
    fn index(&self, _: crate::elements::e31) -> &Self::Output {
        &self[1]
    }
}
impl std::ops::Index<crate::elements::e12> for AntiLine {
    type Output = f32;
    fn index(&self, _: crate::elements::e12) -> &Self::Output {
        &self[2]
    }
}
impl std::ops::Index<crate::elements::e15> for AntiLine {
    type Output = f32;
    fn index(&self, _: crate::elements::e15) -> &Self::Output {
        &self[3]
    }
}
impl std::ops::Index<crate::elements::e25> for AntiLine {
    type Output = f32;
    fn index(&self, _: crate::elements::e25) -> &Self::Output {
        &self[4]
    }
}
impl std::ops::Index<crate::elements::e35> for AntiLine {
    type Output = f32;
    fn index(&self, _: crate::elements::e35) -> &Self::Output {
        &self[5]
    }
}
impl std::ops::IndexMut<crate::elements::e23> for AntiLine {
    fn index_mut(&self, _: crate::elements::e23) -> &mut Self::Output {
        &mut self[0]
    }
}
impl std::ops::IndexMut<crate::elements::e31> for AntiLine {
    fn index_mut(&self, _: crate::elements::e31) -> &mut Self::Output {
        &mut self[1]
    }
}
impl std::ops::IndexMut<crate::elements::e12> for AntiLine {
    fn index_mut(&self, _: crate::elements::e12) -> &mut Self::Output {
        &mut self[2]
    }
}
impl std::ops::IndexMut<crate::elements::e15> for AntiLine {
    fn index_mut(&self, _: crate::elements::e15) -> &mut Self::Output {
        &mut self[3]
    }
}
impl std::ops::IndexMut<crate::elements::e25> for AntiLine {
    fn index_mut(&self, _: crate::elements::e25) -> &mut Self::Output {
        &mut self[4]
    }
}
impl std::ops::IndexMut<crate::elements::e35> for AntiLine {
    fn index_mut(&self, _: crate::elements::e35) -> &mut Self::Output {
        &mut self[5]
    }
}
include!("./impls/anti_line.rs");
