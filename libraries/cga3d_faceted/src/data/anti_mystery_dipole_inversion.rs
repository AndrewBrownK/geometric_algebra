use crate::data::*;
use crate::simd::*;

/// AntiMysteryDipoleInversion.
/// This variant of MysteryVersorEven is the Dual to MysteryDipoleInversion. It is common for
/// objects of this type to not intersect the null cone, which also prevents them from
/// projecting onto the horosphere in the usual manner. When this happens, this
/// object has behavioral and operative similarity to a MysteryVersorEven,
/// but an imaginary radius, and a spacial presence in the shape of a
/// MysteryDipoleInversion with a real radius.
#[derive(Clone, Copy, nearly::NearlyEq, nearly::NearlyOrd, bytemuck::Pod, bytemuck::Zeroable, encase::ShaderType, serde::Serialize, serde::Deserialize)]
pub union AntiMysteryDipoleInversion {
    groups: AntiMysteryDipoleInversionGroups,
    /// e415, e425, e435, e321, e1, e2, e3, 0
    elements: [f32; 8],
}
#[derive(Clone, Copy, nearly::NearlyEq, nearly::NearlyOrd, bytemuck::Pod, bytemuck::Zeroable, encase::ShaderType, serde::Serialize, serde::Deserialize)]
pub struct AntiMysteryDipoleInversionGroups {
    /// e415, e425, e435, e321
    g0: Simd32x4,
    /// e1, e2, e3
    g1: Simd32x3,
}
impl AntiMysteryDipoleInversion {
    #[allow(clippy::too_many_arguments)]
    pub const fn from_elements(e415: f32, e425: f32, e435: f32, e321: f32, e1: f32, e2: f32, e3: f32) -> Self {
        Self {
            elements: [e415, e425, e435, e321, e1, e2, e3, 0.0],
        }
    }
    pub const fn from_groups(g0: Simd32x4, g1: Simd32x3) -> Self {
        Self {
            groups: AntiMysteryDipoleInversionGroups { g0, g1 },
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
const ANTI_MYSTERY_DIPOLE_INVERSION_INDEX_REMAP: [usize; 7] = [0, 1, 2, 3, 4, 5, 6];
impl std::ops::Index<usize> for AntiMysteryDipoleInversion {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[ANTI_MYSTERY_DIPOLE_INVERSION_INDEX_REMAP[index]] }
    }
}
impl std::ops::IndexMut<usize> for AntiMysteryDipoleInversion {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[ANTI_MYSTERY_DIPOLE_INVERSION_INDEX_REMAP[index]] }
    }
}
impl From<AntiMysteryDipoleInversion> for [f32; 7] {
    fn from(vector: AntiMysteryDipoleInversion) -> Self {
        unsafe {
            [
                vector.elements[0], vector.elements[1], vector.elements[2], vector.elements[3], vector.elements[4], vector.elements[5], vector.elements[6],
            ]
        }
    }
}
impl From<[f32; 7]> for AntiMysteryDipoleInversion {
    fn from(array: [f32; 7]) -> Self {
        Self {
            elements: [array[0], array[1], array[2], array[3], array[1], array[2], array[3], 0.0],
        }
    }
}
impl std::fmt::Debug for AntiMysteryDipoleInversion {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .debug_struct("AntiMysteryDipoleInversion")
            .field("e415", &self[0])
            .field("e425", &self[1])
            .field("e435", &self[2])
            .field("e321", &self[3])
            .field("e1", &self[4])
            .field("e2", &self[5])
            .field("e3", &self[6])
            .finish()
    }
}

impl AntiMysteryDipoleInversion {
    pub const LEN: usize = 7;
}

impl AntiMysteryDipoleInversion {
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

impl PartialOrd for AntiMysteryDipoleInversion {
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
impl Ord for AntiMysteryDipoleInversion {
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
impl PartialEq for AntiMysteryDipoleInversion {
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
impl Eq for AntiMysteryDipoleInversion {}
impl std::hash::Hash for AntiMysteryDipoleInversion {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for i in 0..Self::LEN {
            self[i].to_bits().hash(state);
        }
    }
}

impl std::ops::Index<crate::elements::e415> for AntiMysteryDipoleInversion {
    type Output = f32;
    fn index(&self, _: crate::elements::e415) -> &Self::Output {
        &self[0]
    }
}
impl std::ops::Index<crate::elements::e425> for AntiMysteryDipoleInversion {
    type Output = f32;
    fn index(&self, _: crate::elements::e425) -> &Self::Output {
        &self[1]
    }
}
impl std::ops::Index<crate::elements::e435> for AntiMysteryDipoleInversion {
    type Output = f32;
    fn index(&self, _: crate::elements::e435) -> &Self::Output {
        &self[2]
    }
}
impl std::ops::Index<crate::elements::e321> for AntiMysteryDipoleInversion {
    type Output = f32;
    fn index(&self, _: crate::elements::e321) -> &Self::Output {
        &self[3]
    }
}
impl std::ops::Index<crate::elements::e1> for AntiMysteryDipoleInversion {
    type Output = f32;
    fn index(&self, _: crate::elements::e1) -> &Self::Output {
        &self[4]
    }
}
impl std::ops::Index<crate::elements::e2> for AntiMysteryDipoleInversion {
    type Output = f32;
    fn index(&self, _: crate::elements::e2) -> &Self::Output {
        &self[5]
    }
}
impl std::ops::Index<crate::elements::e3> for AntiMysteryDipoleInversion {
    type Output = f32;
    fn index(&self, _: crate::elements::e3) -> &Self::Output {
        &self[6]
    }
}
impl std::ops::IndexMut<crate::elements::e415> for AntiMysteryDipoleInversion {
    fn index_mut(&self, _: crate::elements::e415) -> &mut Self::Output {
        &mut self[0]
    }
}
impl std::ops::IndexMut<crate::elements::e425> for AntiMysteryDipoleInversion {
    fn index_mut(&self, _: crate::elements::e425) -> &mut Self::Output {
        &mut self[1]
    }
}
impl std::ops::IndexMut<crate::elements::e435> for AntiMysteryDipoleInversion {
    fn index_mut(&self, _: crate::elements::e435) -> &mut Self::Output {
        &mut self[2]
    }
}
impl std::ops::IndexMut<crate::elements::e321> for AntiMysteryDipoleInversion {
    fn index_mut(&self, _: crate::elements::e321) -> &mut Self::Output {
        &mut self[3]
    }
}
impl std::ops::IndexMut<crate::elements::e1> for AntiMysteryDipoleInversion {
    fn index_mut(&self, _: crate::elements::e1) -> &mut Self::Output {
        &mut self[4]
    }
}
impl std::ops::IndexMut<crate::elements::e2> for AntiMysteryDipoleInversion {
    fn index_mut(&self, _: crate::elements::e2) -> &mut Self::Output {
        &mut self[5]
    }
}
impl std::ops::IndexMut<crate::elements::e3> for AntiMysteryDipoleInversion {
    fn index_mut(&self, _: crate::elements::e3) -> &mut Self::Output {
        &mut self[6]
    }
}
include!("./impls/anti_mystery_dipole_inversion.rs");
