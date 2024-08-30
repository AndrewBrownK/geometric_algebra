use crate::data::*;
use crate::simd::*;

/// AntiMysteryCircleRotor.
/// This variant of MysteryVersorOdd is the Dual to MysteryCircleRotor. It is common for
/// objects of this type to not intersect the null cone, which also prevents them from
/// projecting onto the horosphere in the usual manner. When this happens, this
/// object has behavioral and operative similarity to a MysteryVersorOdd,
/// but an imaginary radius, and a spacial presence in the shape of a
/// MysteryCircleRotor with a real radius.
#[derive(Clone, Copy, nearly::NearlyEq, nearly::NearlyOrd, bytemuck::Pod, bytemuck::Zeroable, encase::ShaderType, serde::Serialize, serde::Deserialize)]
pub union AntiMysteryCircleRotor {
    groups: AntiMysteryCircleRotorGroups,
    /// e23, e31, e12, e45, scalar, 0, 0, 0
    elements: [f32; 8],
}
#[derive(Clone, Copy, nearly::NearlyEq, nearly::NearlyOrd, bytemuck::Pod, bytemuck::Zeroable, encase::ShaderType, serde::Serialize, serde::Deserialize)]
pub struct AntiMysteryCircleRotorGroups {
    /// e23, e31, e12, e45
    g0: Simd32x4,
    /// scalar
    g1: f32,
}
impl AntiMysteryCircleRotor {
    #[allow(clippy::too_many_arguments)]
    pub const fn from_elements(e23: f32, e31: f32, e12: f32, e45: f32, scalar: f32) -> Self {
        Self {
            elements: [e23, e31, e12, e45, scalar, 0.0, 0.0, 0.0],
        }
    }
    pub const fn from_groups(g0: Simd32x4, g1: f32) -> Self {
        Self {
            groups: AntiMysteryCircleRotorGroups { g0, g1 },
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
const ANTI_MYSTERY_CIRCLE_ROTOR_INDEX_REMAP: [usize; 5] = [0, 1, 2, 3, 4];
impl std::ops::Index<usize> for AntiMysteryCircleRotor {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[ANTI_MYSTERY_CIRCLE_ROTOR_INDEX_REMAP[index]] }
    }
}
impl std::ops::IndexMut<usize> for AntiMysteryCircleRotor {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[ANTI_MYSTERY_CIRCLE_ROTOR_INDEX_REMAP[index]] }
    }
}
impl From<AntiMysteryCircleRotor> for [f32; 5] {
    fn from(vector: AntiMysteryCircleRotor) -> Self {
        unsafe { [vector.elements[0], vector.elements[1], vector.elements[2], vector.elements[3], vector.elements[4]] }
    }
}
impl From<[f32; 5]> for AntiMysteryCircleRotor {
    fn from(array: [f32; 5]) -> Self {
        Self {
            elements: [array[0], array[1], array[2], array[3], array[1], 0.0, 0.0, 0.0],
        }
    }
}
impl std::fmt::Debug for AntiMysteryCircleRotor {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .debug_struct("AntiMysteryCircleRotor")
            .field("e23", &self[0])
            .field("e31", &self[1])
            .field("e12", &self[2])
            .field("e45", &self[3])
            .field("scalar", &self[4])
            .finish()
    }
}

impl AntiMysteryCircleRotor {
    pub const LEN: usize = 5;
}

impl AntiMysteryCircleRotor {
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

impl PartialOrd for AntiMysteryCircleRotor {
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
impl Ord for AntiMysteryCircleRotor {
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
impl PartialEq for AntiMysteryCircleRotor {
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
impl Eq for AntiMysteryCircleRotor {}
impl std::hash::Hash for AntiMysteryCircleRotor {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for i in 0..Self::LEN {
            self[i].to_bits().hash(state);
        }
    }
}

impl std::ops::Index<crate::elements::e23> for AntiMysteryCircleRotor {
    type Output = f32;
    fn index(&self, _: crate::elements::e23) -> &Self::Output {
        &self[0]
    }
}
impl std::ops::Index<crate::elements::e31> for AntiMysteryCircleRotor {
    type Output = f32;
    fn index(&self, _: crate::elements::e31) -> &Self::Output {
        &self[1]
    }
}
impl std::ops::Index<crate::elements::e12> for AntiMysteryCircleRotor {
    type Output = f32;
    fn index(&self, _: crate::elements::e12) -> &Self::Output {
        &self[2]
    }
}
impl std::ops::Index<crate::elements::e45> for AntiMysteryCircleRotor {
    type Output = f32;
    fn index(&self, _: crate::elements::e45) -> &Self::Output {
        &self[3]
    }
}
impl std::ops::Index<crate::elements::scalar> for AntiMysteryCircleRotor {
    type Output = f32;
    fn index(&self, _: crate::elements::scalar) -> &Self::Output {
        &self[4]
    }
}
impl std::ops::IndexMut<crate::elements::e23> for AntiMysteryCircleRotor {
    fn index_mut(&self, _: crate::elements::e23) -> &mut Self::Output {
        &mut self[0]
    }
}
impl std::ops::IndexMut<crate::elements::e31> for AntiMysteryCircleRotor {
    fn index_mut(&self, _: crate::elements::e31) -> &mut Self::Output {
        &mut self[1]
    }
}
impl std::ops::IndexMut<crate::elements::e12> for AntiMysteryCircleRotor {
    fn index_mut(&self, _: crate::elements::e12) -> &mut Self::Output {
        &mut self[2]
    }
}
impl std::ops::IndexMut<crate::elements::e45> for AntiMysteryCircleRotor {
    fn index_mut(&self, _: crate::elements::e45) -> &mut Self::Output {
        &mut self[3]
    }
}
impl std::ops::IndexMut<crate::elements::scalar> for AntiMysteryCircleRotor {
    fn index_mut(&self, _: crate::elements::scalar) -> &mut Self::Output {
        &mut self[4]
    }
}
include!("./impls/anti_mystery_circle_rotor.rs");
