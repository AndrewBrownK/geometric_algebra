use crate::data::*;
use crate::simd::*;

/// MysteryCircleRotor.
/// TODO this is currently a mystery I'm investigating
#[derive(Clone, Copy, nearly::NearlyEq, nearly::NearlyOrd, bytemuck::Pod, bytemuck::Zeroable, encase::ShaderType, serde::Serialize, serde::Deserialize)]
pub union MysteryCircleRotor {
    groups: MysteryCircleRotorGroups,
    /// e415, e425, e435, e321, e12345, 0, 0, 0
    elements: [f32; 8],
}
#[derive(Clone, Copy, nearly::NearlyEq, nearly::NearlyOrd, bytemuck::Pod, bytemuck::Zeroable, encase::ShaderType, serde::Serialize, serde::Deserialize)]
pub struct MysteryCircleRotorGroups {
    /// e415, e425, e435, e321
    g0: Simd32x4,
    /// e12345
    g1: f32,
}
impl MysteryCircleRotor {
    #[allow(clippy::too_many_arguments)]
    pub const fn from_elements(e415: f32, e425: f32, e435: f32, e321: f32, e12345: f32) -> Self {
        Self {
            elements: [e415, e425, e435, e321, e12345, 0.0, 0.0, 0.0],
        }
    }
    pub const fn from_groups(g0: Simd32x4, g1: f32) -> Self {
        Self {
            groups: MysteryCircleRotorGroups { g0, g1 },
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
    pub fn group1(&self) -> f32 {
        unsafe { self.groups.g1 }
    }
    #[inline(always)]
    pub fn group1_mut(&mut self) -> &mut f32 {
        unsafe { &mut self.groups.g1 }
    }
}
const MYSTERY_CIRCLE_ROTOR_INDEX_REMAP: [usize; 5] = [0, 1, 2, 3, 4];
impl std::ops::Index<usize> for MysteryCircleRotor {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[MYSTERY_CIRCLE_ROTOR_INDEX_REMAP[index]] }
    }
}
impl std::ops::IndexMut<usize> for MysteryCircleRotor {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[MYSTERY_CIRCLE_ROTOR_INDEX_REMAP[index]] }
    }
}
impl From<MysteryCircleRotor> for [f32; 5] {
    fn from(vector: MysteryCircleRotor) -> Self {
        unsafe { [vector.elements[0], vector.elements[1], vector.elements[2], vector.elements[3], vector.elements[4]] }
    }
}
impl From<[f32; 5]> for MysteryCircleRotor {
    fn from(array: [f32; 5]) -> Self {
        Self {
            elements: [array[0], array[1], array[2], array[3], array[1], 0.0, 0.0, 0.0],
        }
    }
}
impl std::fmt::Debug for MysteryCircleRotor {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .debug_struct("MysteryCircleRotor")
            .field("e415", &self[0])
            .field("e425", &self[1])
            .field("e435", &self[2])
            .field("e321", &self[3])
            .field("e12345", &self[4])
            .finish()
    }
}

impl MysteryCircleRotor {
    pub const LEN: usize = 5;
}

impl MysteryCircleRotor {
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

impl PartialOrd for MysteryCircleRotor {
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
impl Ord for MysteryCircleRotor {
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
impl PartialEq for MysteryCircleRotor {
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
impl Eq for MysteryCircleRotor {}
impl std::hash::Hash for MysteryCircleRotor {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for i in 0..Self::LEN {
            self[i].to_bits().hash(state);
        }
    }
}

impl std::ops::Index<crate::elements::e415> for MysteryCircleRotor {
    type Output = f32;
    fn index(&self, _: crate::elements::e415) -> &Self::Output {
        &self[0]
    }
}
impl std::ops::Index<crate::elements::e425> for MysteryCircleRotor {
    type Output = f32;
    fn index(&self, _: crate::elements::e425) -> &Self::Output {
        &self[1]
    }
}
impl std::ops::Index<crate::elements::e435> for MysteryCircleRotor {
    type Output = f32;
    fn index(&self, _: crate::elements::e435) -> &Self::Output {
        &self[2]
    }
}
impl std::ops::Index<crate::elements::e321> for MysteryCircleRotor {
    type Output = f32;
    fn index(&self, _: crate::elements::e321) -> &Self::Output {
        &self[3]
    }
}
impl std::ops::Index<crate::elements::e12345> for MysteryCircleRotor {
    type Output = f32;
    fn index(&self, _: crate::elements::e12345) -> &Self::Output {
        &self[4]
    }
}
impl std::ops::IndexMut<crate::elements::e415> for MysteryCircleRotor {
    fn index_mut(&self, _: crate::elements::e415) -> &mut Self::Output {
        &mut self[0]
    }
}
impl std::ops::IndexMut<crate::elements::e425> for MysteryCircleRotor {
    fn index_mut(&self, _: crate::elements::e425) -> &mut Self::Output {
        &mut self[1]
    }
}
impl std::ops::IndexMut<crate::elements::e435> for MysteryCircleRotor {
    fn index_mut(&self, _: crate::elements::e435) -> &mut Self::Output {
        &mut self[2]
    }
}
impl std::ops::IndexMut<crate::elements::e321> for MysteryCircleRotor {
    fn index_mut(&self, _: crate::elements::e321) -> &mut Self::Output {
        &mut self[3]
    }
}
impl std::ops::IndexMut<crate::elements::e12345> for MysteryCircleRotor {
    fn index_mut(&self, _: crate::elements::e12345) -> &mut Self::Output {
        &mut self[4]
    }
}
include!("./impls/mystery_circle_rotor.rs");