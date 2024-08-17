use crate::data::*;
use crate::simd::*;

/// CircleRotorAligningOrigin.
/// This variant of CircleRotor has a Carrier that intersects the Origin.
#[derive(Clone, Copy, nearly::NearlyEq, nearly::NearlyOrd, bytemuck::Pod, bytemuck::Zeroable, encase::ShaderType, serde::Serialize, serde::Deserialize)]
pub union CircleRotorAligningOrigin {
    groups: CircleRotorAligningOriginGroups,
    /// e423, e431, e412, 0, e415, e425, e435, 0, e235, e315, e125, e12345
    elements: [f32; 12],
}
#[derive(Clone, Copy, nearly::NearlyEq, nearly::NearlyOrd, bytemuck::Pod, bytemuck::Zeroable, encase::ShaderType, serde::Serialize, serde::Deserialize)]
pub struct CircleRotorAligningOriginGroups {
    /// e423, e431, e412
    g0: Simd32x3,
    /// e415, e425, e435
    g1: Simd32x3,
    /// e235, e315, e125, e12345
    g2: Simd32x4,
}
impl CircleRotorAligningOrigin {
    #[allow(clippy::too_many_arguments)]
    pub const fn from_elements(e423: f32, e431: f32, e412: f32, e415: f32, e425: f32, e435: f32, e235: f32, e315: f32, e125: f32, e12345: f32) -> Self {
        Self {
            elements: [e423, e431, e412, 0.0, e415, e425, e435, 0.0, e235, e315, e125, e12345],
        }
    }
    pub const fn from_groups(g0: Simd32x3, g1: Simd32x3, g2: Simd32x4) -> Self {
        Self {
            groups: CircleRotorAligningOriginGroups { g0, g1, g2 },
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
    #[inline(always)]
    pub fn group2(&self) -> Simd32x4 {
        unsafe { self.groups.g2 }
    }
    #[inline(always)]
    pub fn group2_mut(&mut self) -> &mut Simd32x4 {
        unsafe { &mut self.groups.g2 }
    }
}
const CIRCLE_ROTOR_ALIGNING_ORIGIN_INDEX_REMAP: [usize; 10] = [0, 1, 2, 4, 5, 6, 8, 9, 10, 11];
impl std::ops::Index<usize> for CircleRotorAligningOrigin {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[CIRCLE_ROTOR_ALIGNING_ORIGIN_INDEX_REMAP[index]] }
    }
}
impl std::ops::IndexMut<usize> for CircleRotorAligningOrigin {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[CIRCLE_ROTOR_ALIGNING_ORIGIN_INDEX_REMAP[index]] }
    }
}
impl From<CircleRotorAligningOrigin> for [f32; 10] {
    fn from(vector: CircleRotorAligningOrigin) -> Self {
        unsafe {
            [
                vector.elements[0], vector.elements[1], vector.elements[2], vector.elements[4], vector.elements[5], vector.elements[6], vector.elements[8], vector.elements[9],
                vector.elements[10], vector.elements[11],
            ]
        }
    }
}
impl From<[f32; 10]> for CircleRotorAligningOrigin {
    fn from(array: [f32; 10]) -> Self {
        Self {
            elements: [array[0], array[1], array[2], 0.0, array[1], array[2], array[3], 0.0, array[2], array[3], array[4], array[5]],
        }
    }
}
impl std::fmt::Debug for CircleRotorAligningOrigin {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .debug_struct("CircleRotorAligningOrigin")
            .field("e423", &self[0])
            .field("e431", &self[1])
            .field("e412", &self[2])
            .field("e415", &self[3])
            .field("e425", &self[4])
            .field("e435", &self[5])
            .field("e235", &self[6])
            .field("e315", &self[7])
            .field("e125", &self[8])
            .field("e12345", &self[9])
            .finish()
    }
}

impl CircleRotorAligningOrigin {
    pub const LEN: usize = 10;
}

impl CircleRotorAligningOrigin {
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

impl PartialOrd for CircleRotorAligningOrigin {
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
impl Ord for CircleRotorAligningOrigin {
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
impl PartialEq for CircleRotorAligningOrigin {
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
impl Eq for CircleRotorAligningOrigin {}
impl std::hash::Hash for CircleRotorAligningOrigin {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for i in 0..Self::LEN {
            self[i].to_bits().hash(state);
        }
    }
}

impl std::ops::Index<crate::elements::e423> for CircleRotorAligningOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e423) -> &Self::Output {
        &self[0]
    }
}
impl std::ops::Index<crate::elements::e431> for CircleRotorAligningOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e431) -> &Self::Output {
        &self[1]
    }
}
impl std::ops::Index<crate::elements::e412> for CircleRotorAligningOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e412) -> &Self::Output {
        &self[2]
    }
}
impl std::ops::Index<crate::elements::e415> for CircleRotorAligningOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e415) -> &Self::Output {
        &self[3]
    }
}
impl std::ops::Index<crate::elements::e425> for CircleRotorAligningOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e425) -> &Self::Output {
        &self[4]
    }
}
impl std::ops::Index<crate::elements::e435> for CircleRotorAligningOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e435) -> &Self::Output {
        &self[5]
    }
}
impl std::ops::Index<crate::elements::e235> for CircleRotorAligningOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e235) -> &Self::Output {
        &self[6]
    }
}
impl std::ops::Index<crate::elements::e315> for CircleRotorAligningOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e315) -> &Self::Output {
        &self[7]
    }
}
impl std::ops::Index<crate::elements::e125> for CircleRotorAligningOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e125) -> &Self::Output {
        &self[8]
    }
}
impl std::ops::Index<crate::elements::e12345> for CircleRotorAligningOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e12345) -> &Self::Output {
        &self[9]
    }
}
impl std::ops::IndexMut<crate::elements::e423> for CircleRotorAligningOrigin {
    fn index_mut(&self, _: crate::elements::e423) -> &mut Self::Output {
        &mut self[0]
    }
}
impl std::ops::IndexMut<crate::elements::e431> for CircleRotorAligningOrigin {
    fn index_mut(&self, _: crate::elements::e431) -> &mut Self::Output {
        &mut self[1]
    }
}
impl std::ops::IndexMut<crate::elements::e412> for CircleRotorAligningOrigin {
    fn index_mut(&self, _: crate::elements::e412) -> &mut Self::Output {
        &mut self[2]
    }
}
impl std::ops::IndexMut<crate::elements::e415> for CircleRotorAligningOrigin {
    fn index_mut(&self, _: crate::elements::e415) -> &mut Self::Output {
        &mut self[3]
    }
}
impl std::ops::IndexMut<crate::elements::e425> for CircleRotorAligningOrigin {
    fn index_mut(&self, _: crate::elements::e425) -> &mut Self::Output {
        &mut self[4]
    }
}
impl std::ops::IndexMut<crate::elements::e435> for CircleRotorAligningOrigin {
    fn index_mut(&self, _: crate::elements::e435) -> &mut Self::Output {
        &mut self[5]
    }
}
impl std::ops::IndexMut<crate::elements::e235> for CircleRotorAligningOrigin {
    fn index_mut(&self, _: crate::elements::e235) -> &mut Self::Output {
        &mut self[6]
    }
}
impl std::ops::IndexMut<crate::elements::e315> for CircleRotorAligningOrigin {
    fn index_mut(&self, _: crate::elements::e315) -> &mut Self::Output {
        &mut self[7]
    }
}
impl std::ops::IndexMut<crate::elements::e125> for CircleRotorAligningOrigin {
    fn index_mut(&self, _: crate::elements::e125) -> &mut Self::Output {
        &mut self[8]
    }
}
impl std::ops::IndexMut<crate::elements::e12345> for CircleRotorAligningOrigin {
    fn index_mut(&self, _: crate::elements::e12345) -> &mut Self::Output {
        &mut self[9]
    }
}
include!("./impls/circle_rotor_aligning_origin.rs");
