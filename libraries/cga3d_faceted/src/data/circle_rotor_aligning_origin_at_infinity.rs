use crate::data::*;
use crate::simd::*;

/// CircleRotorAligningOriginAtInfinity.
/// This variant of CircleRotorAligningOrigin exists at the Horizon.
#[derive(Clone, Copy, nearly::NearlyEq, nearly::NearlyOrd, bytemuck::Pod, bytemuck::Zeroable, encase::ShaderType, serde::Serialize, serde::Deserialize)]
pub union CircleRotorAligningOriginAtInfinity {
    groups: CircleRotorAligningOriginAtInfinityGroups,
    /// e415, e425, e435, 0, e235, e315, e125, e12345
    elements: [f32; 8],
}
#[derive(Clone, Copy, nearly::NearlyEq, nearly::NearlyOrd, bytemuck::Pod, bytemuck::Zeroable, encase::ShaderType, serde::Serialize, serde::Deserialize)]
pub struct CircleRotorAligningOriginAtInfinityGroups {
    /// e415, e425, e435
    g0: Simd32x3,
    /// e235, e315, e125, e12345
    g1: Simd32x4,
}
impl CircleRotorAligningOriginAtInfinity {
    #[allow(clippy::too_many_arguments)]
    pub const fn from_elements(e415: f32, e425: f32, e435: f32, e235: f32, e315: f32, e125: f32, e12345: f32) -> Self {
        Self {
            elements: [e415, e425, e435, 0.0, e235, e315, e125, e12345],
        }
    }
    pub const fn from_groups(g0: Simd32x3, g1: Simd32x4) -> Self {
        Self {
            groups: CircleRotorAligningOriginAtInfinityGroups { g0, g1 },
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
    pub fn group1(&self) -> Simd32x4 {
        unsafe { self.groups.g1 }
    }
    #[inline(always)]
    pub fn group1_mut(&mut self) -> &mut Simd32x4 {
        unsafe { &mut self.groups.g1 }
    }
}
const CIRCLE_ROTOR_ALIGNING_ORIGIN_AT_INFINITY_INDEX_REMAP: [usize; 7] = [0, 1, 2, 4, 5, 6, 7];
impl std::ops::Index<usize> for CircleRotorAligningOriginAtInfinity {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[CIRCLE_ROTOR_ALIGNING_ORIGIN_AT_INFINITY_INDEX_REMAP[index]] }
    }
}
impl std::ops::IndexMut<usize> for CircleRotorAligningOriginAtInfinity {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[CIRCLE_ROTOR_ALIGNING_ORIGIN_AT_INFINITY_INDEX_REMAP[index]] }
    }
}
impl From<CircleRotorAligningOriginAtInfinity> for [f32; 7] {
    fn from(vector: CircleRotorAligningOriginAtInfinity) -> Self {
        unsafe {
            [
                vector.elements[0], vector.elements[1], vector.elements[2], vector.elements[4], vector.elements[5], vector.elements[6], vector.elements[7],
            ]
        }
    }
}
impl From<[f32; 7]> for CircleRotorAligningOriginAtInfinity {
    fn from(array: [f32; 7]) -> Self {
        Self {
            elements: [array[0], array[1], array[2], 0.0, array[1], array[2], array[3], array[4]],
        }
    }
}
impl std::fmt::Debug for CircleRotorAligningOriginAtInfinity {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .debug_struct("CircleRotorAligningOriginAtInfinity")
            .field("e415", &self[0])
            .field("e425", &self[1])
            .field("e435", &self[2])
            .field("e235", &self[3])
            .field("e315", &self[4])
            .field("e125", &self[5])
            .field("e12345", &self[6])
            .finish()
    }
}

impl CircleRotorAligningOriginAtInfinity {
    pub const LEN: usize = 7;
}

impl CircleRotorAligningOriginAtInfinity {
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

impl PartialOrd for CircleRotorAligningOriginAtInfinity {
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
impl Ord for CircleRotorAligningOriginAtInfinity {
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
impl PartialEq for CircleRotorAligningOriginAtInfinity {
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
impl Eq for CircleRotorAligningOriginAtInfinity {}
impl std::hash::Hash for CircleRotorAligningOriginAtInfinity {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for i in 0..Self::LEN {
            self[i].to_bits().hash(state);
        }
    }
}

impl std::ops::Index<crate::elements::e415> for CircleRotorAligningOriginAtInfinity {
    type Output = f32;
    fn index(&self, _: crate::elements::e415) -> &Self::Output {
        &self[0]
    }
}
impl std::ops::Index<crate::elements::e425> for CircleRotorAligningOriginAtInfinity {
    type Output = f32;
    fn index(&self, _: crate::elements::e425) -> &Self::Output {
        &self[1]
    }
}
impl std::ops::Index<crate::elements::e435> for CircleRotorAligningOriginAtInfinity {
    type Output = f32;
    fn index(&self, _: crate::elements::e435) -> &Self::Output {
        &self[2]
    }
}
impl std::ops::Index<crate::elements::e235> for CircleRotorAligningOriginAtInfinity {
    type Output = f32;
    fn index(&self, _: crate::elements::e235) -> &Self::Output {
        &self[3]
    }
}
impl std::ops::Index<crate::elements::e315> for CircleRotorAligningOriginAtInfinity {
    type Output = f32;
    fn index(&self, _: crate::elements::e315) -> &Self::Output {
        &self[4]
    }
}
impl std::ops::Index<crate::elements::e125> for CircleRotorAligningOriginAtInfinity {
    type Output = f32;
    fn index(&self, _: crate::elements::e125) -> &Self::Output {
        &self[5]
    }
}
impl std::ops::Index<crate::elements::e12345> for CircleRotorAligningOriginAtInfinity {
    type Output = f32;
    fn index(&self, _: crate::elements::e12345) -> &Self::Output {
        &self[6]
    }
}
impl std::ops::IndexMut<crate::elements::e415> for CircleRotorAligningOriginAtInfinity {
    fn index_mut(&self, _: crate::elements::e415) -> &mut Self::Output {
        &mut self[0]
    }
}
impl std::ops::IndexMut<crate::elements::e425> for CircleRotorAligningOriginAtInfinity {
    fn index_mut(&self, _: crate::elements::e425) -> &mut Self::Output {
        &mut self[1]
    }
}
impl std::ops::IndexMut<crate::elements::e435> for CircleRotorAligningOriginAtInfinity {
    fn index_mut(&self, _: crate::elements::e435) -> &mut Self::Output {
        &mut self[2]
    }
}
impl std::ops::IndexMut<crate::elements::e235> for CircleRotorAligningOriginAtInfinity {
    fn index_mut(&self, _: crate::elements::e235) -> &mut Self::Output {
        &mut self[3]
    }
}
impl std::ops::IndexMut<crate::elements::e315> for CircleRotorAligningOriginAtInfinity {
    fn index_mut(&self, _: crate::elements::e315) -> &mut Self::Output {
        &mut self[4]
    }
}
impl std::ops::IndexMut<crate::elements::e125> for CircleRotorAligningOriginAtInfinity {
    fn index_mut(&self, _: crate::elements::e125) -> &mut Self::Output {
        &mut self[5]
    }
}
impl std::ops::IndexMut<crate::elements::e12345> for CircleRotorAligningOriginAtInfinity {
    fn index_mut(&self, _: crate::elements::e12345) -> &mut Self::Output {
        &mut self[6]
    }
}
include!("./impls/circle_rotor_aligning_origin_at_infinity.rs");
