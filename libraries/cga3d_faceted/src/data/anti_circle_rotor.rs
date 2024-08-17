use crate::data::*;
use crate::simd::*;

/// AntiCircleRotor.
/// This variant of VersorOdd is the Dual to CircleRotor. It is common for
/// objects of this type to not intersect the null cone, which also prevents them from
/// projecting onto the horosphere in the usual manner. When this happens, this
/// object has behavioral and operative similarity to a VersorOdd,
/// but an imaginary radius, and a spacial presence in the shape of a
/// CircleRotor with a real radius.
#[derive(Clone, Copy, nearly::NearlyEq, nearly::NearlyOrd, bytemuck::Pod, bytemuck::Zeroable, encase::ShaderType, serde::Serialize, serde::Deserialize)]
pub union AntiCircleRotor {
    groups: AntiCircleRotorGroups,
    /// e41, e42, e43, 0, e23, e31, e12, e45, e15, e25, e35, scalar
    elements: [f32; 12],
}
#[derive(Clone, Copy, nearly::NearlyEq, nearly::NearlyOrd, bytemuck::Pod, bytemuck::Zeroable, encase::ShaderType, serde::Serialize, serde::Deserialize)]
pub struct AntiCircleRotorGroups {
    /// e41, e42, e43
    g0: Simd32x3,
    /// e23, e31, e12, e45
    g1: Simd32x4,
    /// e15, e25, e35, scalar
    g2: Simd32x4,
}
impl AntiCircleRotor {
    #[allow(clippy::too_many_arguments)]
    pub const fn from_elements(e41: f32, e42: f32, e43: f32, e23: f32, e31: f32, e12: f32, e45: f32, e15: f32, e25: f32, e35: f32, scalar: f32) -> Self {
        Self {
            elements: [e41, e42, e43, 0.0, e23, e31, e12, e45, e15, e25, e35, scalar],
        }
    }
    pub const fn from_groups(g0: Simd32x3, g1: Simd32x4, g2: Simd32x4) -> Self {
        Self {
            groups: AntiCircleRotorGroups { g0, g1, g2 },
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
    #[inline(always)]
    pub fn group2(&self) -> Simd32x4 {
        unsafe { self.groups.g2 }
    }
    #[inline(always)]
    pub fn group2_mut(&mut self) -> &mut Simd32x4 {
        unsafe { &mut self.groups.g2 }
    }
}
const ANTI_CIRCLE_ROTOR_INDEX_REMAP: [usize; 11] = [0, 1, 2, 4, 5, 6, 7, 8, 9, 10, 11];
impl std::ops::Index<usize> for AntiCircleRotor {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[ANTI_CIRCLE_ROTOR_INDEX_REMAP[index]] }
    }
}
impl std::ops::IndexMut<usize> for AntiCircleRotor {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[ANTI_CIRCLE_ROTOR_INDEX_REMAP[index]] }
    }
}
impl From<AntiCircleRotor> for [f32; 11] {
    fn from(vector: AntiCircleRotor) -> Self {
        unsafe {
            [
                vector.elements[0], vector.elements[1], vector.elements[2], vector.elements[4], vector.elements[5], vector.elements[6], vector.elements[7], vector.elements[8],
                vector.elements[9], vector.elements[10], vector.elements[11],
            ]
        }
    }
}
impl From<[f32; 11]> for AntiCircleRotor {
    fn from(array: [f32; 11]) -> Self {
        Self {
            elements: [array[0], array[1], array[2], 0.0, array[1], array[2], array[3], array[4], array[2], array[3], array[4], array[5]],
        }
    }
}
impl std::fmt::Debug for AntiCircleRotor {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .debug_struct("AntiCircleRotor")
            .field("e41", &self[0])
            .field("e42", &self[1])
            .field("e43", &self[2])
            .field("e23", &self[3])
            .field("e31", &self[4])
            .field("e12", &self[5])
            .field("e45", &self[6])
            .field("e15", &self[7])
            .field("e25", &self[8])
            .field("e35", &self[9])
            .field("scalar", &self[10])
            .finish()
    }
}

impl AntiCircleRotor {
    pub const LEN: usize = 11;
}

impl AntiCircleRotor {
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

impl PartialOrd for AntiCircleRotor {
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
impl Ord for AntiCircleRotor {
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
impl PartialEq for AntiCircleRotor {
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
impl Eq for AntiCircleRotor {}
impl std::hash::Hash for AntiCircleRotor {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for i in 0..Self::LEN {
            self[i].to_bits().hash(state);
        }
    }
}

impl std::ops::Index<crate::elements::e41> for AntiCircleRotor {
    type Output = f32;
    fn index(&self, _: crate::elements::e41) -> &Self::Output {
        &self[0]
    }
}
impl std::ops::Index<crate::elements::e42> for AntiCircleRotor {
    type Output = f32;
    fn index(&self, _: crate::elements::e42) -> &Self::Output {
        &self[1]
    }
}
impl std::ops::Index<crate::elements::e43> for AntiCircleRotor {
    type Output = f32;
    fn index(&self, _: crate::elements::e43) -> &Self::Output {
        &self[2]
    }
}
impl std::ops::Index<crate::elements::e23> for AntiCircleRotor {
    type Output = f32;
    fn index(&self, _: crate::elements::e23) -> &Self::Output {
        &self[3]
    }
}
impl std::ops::Index<crate::elements::e31> for AntiCircleRotor {
    type Output = f32;
    fn index(&self, _: crate::elements::e31) -> &Self::Output {
        &self[4]
    }
}
impl std::ops::Index<crate::elements::e12> for AntiCircleRotor {
    type Output = f32;
    fn index(&self, _: crate::elements::e12) -> &Self::Output {
        &self[5]
    }
}
impl std::ops::Index<crate::elements::e45> for AntiCircleRotor {
    type Output = f32;
    fn index(&self, _: crate::elements::e45) -> &Self::Output {
        &self[6]
    }
}
impl std::ops::Index<crate::elements::e15> for AntiCircleRotor {
    type Output = f32;
    fn index(&self, _: crate::elements::e15) -> &Self::Output {
        &self[7]
    }
}
impl std::ops::Index<crate::elements::e25> for AntiCircleRotor {
    type Output = f32;
    fn index(&self, _: crate::elements::e25) -> &Self::Output {
        &self[8]
    }
}
impl std::ops::Index<crate::elements::e35> for AntiCircleRotor {
    type Output = f32;
    fn index(&self, _: crate::elements::e35) -> &Self::Output {
        &self[9]
    }
}
impl std::ops::Index<crate::elements::scalar> for AntiCircleRotor {
    type Output = f32;
    fn index(&self, _: crate::elements::scalar) -> &Self::Output {
        &self[10]
    }
}
impl std::ops::IndexMut<crate::elements::e41> for AntiCircleRotor {
    fn index_mut(&self, _: crate::elements::e41) -> &mut Self::Output {
        &mut self[0]
    }
}
impl std::ops::IndexMut<crate::elements::e42> for AntiCircleRotor {
    fn index_mut(&self, _: crate::elements::e42) -> &mut Self::Output {
        &mut self[1]
    }
}
impl std::ops::IndexMut<crate::elements::e43> for AntiCircleRotor {
    fn index_mut(&self, _: crate::elements::e43) -> &mut Self::Output {
        &mut self[2]
    }
}
impl std::ops::IndexMut<crate::elements::e23> for AntiCircleRotor {
    fn index_mut(&self, _: crate::elements::e23) -> &mut Self::Output {
        &mut self[3]
    }
}
impl std::ops::IndexMut<crate::elements::e31> for AntiCircleRotor {
    fn index_mut(&self, _: crate::elements::e31) -> &mut Self::Output {
        &mut self[4]
    }
}
impl std::ops::IndexMut<crate::elements::e12> for AntiCircleRotor {
    fn index_mut(&self, _: crate::elements::e12) -> &mut Self::Output {
        &mut self[5]
    }
}
impl std::ops::IndexMut<crate::elements::e45> for AntiCircleRotor {
    fn index_mut(&self, _: crate::elements::e45) -> &mut Self::Output {
        &mut self[6]
    }
}
impl std::ops::IndexMut<crate::elements::e15> for AntiCircleRotor {
    fn index_mut(&self, _: crate::elements::e15) -> &mut Self::Output {
        &mut self[7]
    }
}
impl std::ops::IndexMut<crate::elements::e25> for AntiCircleRotor {
    fn index_mut(&self, _: crate::elements::e25) -> &mut Self::Output {
        &mut self[8]
    }
}
impl std::ops::IndexMut<crate::elements::e35> for AntiCircleRotor {
    fn index_mut(&self, _: crate::elements::e35) -> &mut Self::Output {
        &mut self[9]
    }
}
impl std::ops::IndexMut<crate::elements::scalar> for AntiCircleRotor {
    fn index_mut(&self, _: crate::elements::scalar) -> &mut Self::Output {
        &mut self[10]
    }
}
include!("./impls/anti_circle_rotor.rs");
