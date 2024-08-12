use crate::data::*;
use crate::simd::*;

/// CircleAtInfinity.
/// This variant of Circle exists at the Horizon.
#[derive(Clone, Copy, nearly::NearlyEq, nearly::NearlyOrd, bytemuck::Pod, bytemuck::Zeroable, encase::ShaderType, serde::Serialize, serde::Deserialize)]
pub union CircleAtInfinity {
    groups: CircleAtInfinityGroups,
    /// e321, e415, e425, e435, e235, e315, e125, 0
    elements: [f32; 8],
}
#[derive(Clone, Copy, nearly::NearlyEq, nearly::NearlyOrd, bytemuck::Pod, bytemuck::Zeroable, encase::ShaderType, serde::Serialize, serde::Deserialize)]
pub struct CircleAtInfinityGroups {
    /// e321, e415, e425, e435
    g0: Simd32x4,
    /// e235, e315, e125
    g1: Simd32x3,
}
impl CircleAtInfinity {
    #[allow(clippy::too_many_arguments)]
    pub const fn from_elements(e321: f32, e415: f32, e425: f32, e435: f32, e235: f32, e315: f32, e125: f32) -> Self {
        Self {
            elements: [e321, e415, e425, e435, e235, e315, e125, 0.0],
        }
    }
    pub const fn from_groups(g0: Simd32x4, g1: Simd32x3) -> Self {
        Self {
            groups: CircleAtInfinityGroups { g0, g1 },
        }
    }
    #[inline(always)]
    pub fn group0(&self) -> Simd32x4 {
        unsafe { self.groups.g0 }
    }
    #[inline(always)]
    pub fn group0_mut(&mut self) -> &mut Simd32x4 {
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
const CIRCLE_AT_INFINITY_INDEX_REMAP: [usize; 7] = [0, 1, 2, 3, 4, 5, 6];
impl std::ops::Index<usize> for CircleAtInfinity {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[CIRCLE_AT_INFINITY_INDEX_REMAP[index]] }
    }
}
impl std::ops::IndexMut<usize> for CircleAtInfinity {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[CIRCLE_AT_INFINITY_INDEX_REMAP[index]] }
    }
}
impl From<CircleAtInfinity> for [f32; 7] {
    fn from(vector: CircleAtInfinity) -> Self {
        unsafe {
            [
                vector.elements[0], vector.elements[1], vector.elements[2], vector.elements[3], vector.elements[4], vector.elements[5], vector.elements[6],
            ]
        }
    }
}
impl From<[f32; 7]> for CircleAtInfinity {
    fn from(array: [f32; 7]) -> Self {
        Self {
            elements: [array[0], array[1], array[2], array[3], array[1], array[2], array[3], 0.0],
        }
    }
}
impl std::fmt::Debug for CircleAtInfinity {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .debug_struct("CircleAtInfinity")
            .field("e321", &self[0])
            .field("e415", &self[1])
            .field("e425", &self[2])
            .field("e435", &self[3])
            .field("e235", &self[4])
            .field("e315", &self[5])
            .field("e125", &self[6])
            .finish()
    }
}

impl CircleAtInfinity {
    pub const LEN: usize = 7;
}

impl CircleAtInfinity {
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

impl PartialOrd for CircleAtInfinity {
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
impl Ord for CircleAtInfinity {
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
impl PartialEq for CircleAtInfinity {
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
impl Eq for CircleAtInfinity {}
impl std::hash::Hash for CircleAtInfinity {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for i in 0..Self::LEN {
            self[i].to_bits().hash(state);
        }
    }
}

impl std::ops::Index<crate::elements::e321> for CircleAtInfinity {
    type Output = f32;
    fn index(&self, _: crate::elements::e321) -> &Self::Output {
        &self[0]
    }
}
impl std::ops::Index<crate::elements::e415> for CircleAtInfinity {
    type Output = f32;
    fn index(&self, _: crate::elements::e415) -> &Self::Output {
        &self[1]
    }
}
impl std::ops::Index<crate::elements::e425> for CircleAtInfinity {
    type Output = f32;
    fn index(&self, _: crate::elements::e425) -> &Self::Output {
        &self[2]
    }
}
impl std::ops::Index<crate::elements::e435> for CircleAtInfinity {
    type Output = f32;
    fn index(&self, _: crate::elements::e435) -> &Self::Output {
        &self[3]
    }
}
impl std::ops::Index<crate::elements::e235> for CircleAtInfinity {
    type Output = f32;
    fn index(&self, _: crate::elements::e235) -> &Self::Output {
        &self[4]
    }
}
impl std::ops::Index<crate::elements::e315> for CircleAtInfinity {
    type Output = f32;
    fn index(&self, _: crate::elements::e315) -> &Self::Output {
        &self[5]
    }
}
impl std::ops::Index<crate::elements::e125> for CircleAtInfinity {
    type Output = f32;
    fn index(&self, _: crate::elements::e125) -> &Self::Output {
        &self[6]
    }
}
impl std::ops::IndexMut<crate::elements::e321> for CircleAtInfinity {
    fn index_mut(&self, _: crate::elements::e321) -> &mut Self::Output {
        &mut self[0]
    }
}
impl std::ops::IndexMut<crate::elements::e415> for CircleAtInfinity {
    fn index_mut(&self, _: crate::elements::e415) -> &mut Self::Output {
        &mut self[1]
    }
}
impl std::ops::IndexMut<crate::elements::e425> for CircleAtInfinity {
    fn index_mut(&self, _: crate::elements::e425) -> &mut Self::Output {
        &mut self[2]
    }
}
impl std::ops::IndexMut<crate::elements::e435> for CircleAtInfinity {
    fn index_mut(&self, _: crate::elements::e435) -> &mut Self::Output {
        &mut self[3]
    }
}
impl std::ops::IndexMut<crate::elements::e235> for CircleAtInfinity {
    fn index_mut(&self, _: crate::elements::e235) -> &mut Self::Output {
        &mut self[4]
    }
}
impl std::ops::IndexMut<crate::elements::e315> for CircleAtInfinity {
    fn index_mut(&self, _: crate::elements::e315) -> &mut Self::Output {
        &mut self[5]
    }
}
impl std::ops::IndexMut<crate::elements::e125> for CircleAtInfinity {
    fn index_mut(&self, _: crate::elements::e125) -> &mut Self::Output {
        &mut self[6]
    }
}
include!("./impls/circle_at_infinity.rs");
