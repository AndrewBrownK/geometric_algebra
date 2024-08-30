use crate::data::*;
use crate::simd::*;

/// MysteryDipoleInversion.
/// TODO this is currently a mystery I'm investigating
#[derive(Clone, Copy, nearly::NearlyEq, nearly::NearlyOrd, bytemuck::Pod, bytemuck::Zeroable, encase::ShaderType, serde::Serialize, serde::Deserialize)]
pub union MysteryDipoleInversion {
    groups: MysteryDipoleInversionGroups,
    /// e23, e31, e12, e45, e4235, e4315, e4125, 0
    elements: [f32; 8],
}
#[derive(Clone, Copy, nearly::NearlyEq, nearly::NearlyOrd, bytemuck::Pod, bytemuck::Zeroable, encase::ShaderType, serde::Serialize, serde::Deserialize)]
pub struct MysteryDipoleInversionGroups {
    /// e23, e31, e12, e45
    g0: Simd32x4,
    /// e4235, e4315, e4125
    g1: Simd32x3,
}
impl MysteryDipoleInversion {
    #[allow(clippy::too_many_arguments)]
    pub const fn from_elements(e23: f32, e31: f32, e12: f32, e45: f32, e4235: f32, e4315: f32, e4125: f32) -> Self {
        Self {
            elements: [e23, e31, e12, e45, e4235, e4315, e4125, 0.0],
        }
    }
    pub const fn from_groups(g0: Simd32x4, g1: Simd32x3) -> Self {
        Self {
            groups: MysteryDipoleInversionGroups { g0, g1 },
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
const MYSTERY_DIPOLE_INVERSION_INDEX_REMAP: [usize; 7] = [0, 1, 2, 3, 4, 5, 6];
impl std::ops::Index<usize> for MysteryDipoleInversion {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[MYSTERY_DIPOLE_INVERSION_INDEX_REMAP[index]] }
    }
}
impl std::ops::IndexMut<usize> for MysteryDipoleInversion {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[MYSTERY_DIPOLE_INVERSION_INDEX_REMAP[index]] }
    }
}
impl From<MysteryDipoleInversion> for [f32; 7] {
    fn from(vector: MysteryDipoleInversion) -> Self {
        unsafe {
            [
                vector.elements[0], vector.elements[1], vector.elements[2], vector.elements[3], vector.elements[4], vector.elements[5], vector.elements[6],
            ]
        }
    }
}
impl From<[f32; 7]> for MysteryDipoleInversion {
    fn from(array: [f32; 7]) -> Self {
        Self {
            elements: [array[0], array[1], array[2], array[3], array[1], array[2], array[3], 0.0],
        }
    }
}
impl std::fmt::Debug for MysteryDipoleInversion {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .debug_struct("MysteryDipoleInversion")
            .field("e23", &self[0])
            .field("e31", &self[1])
            .field("e12", &self[2])
            .field("e45", &self[3])
            .field("e4235", &self[4])
            .field("e4315", &self[5])
            .field("e4125", &self[6])
            .finish()
    }
}

impl MysteryDipoleInversion {
    pub const LEN: usize = 7;
}

impl MysteryDipoleInversion {
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

impl PartialOrd for MysteryDipoleInversion {
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
impl Ord for MysteryDipoleInversion {
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
impl PartialEq for MysteryDipoleInversion {
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
impl Eq for MysteryDipoleInversion {}
impl std::hash::Hash for MysteryDipoleInversion {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for i in 0..Self::LEN {
            self[i].to_bits().hash(state);
        }
    }
}

impl std::ops::Index<crate::elements::e23> for MysteryDipoleInversion {
    type Output = f32;
    fn index(&self, _: crate::elements::e23) -> &Self::Output {
        &self[0]
    }
}
impl std::ops::Index<crate::elements::e31> for MysteryDipoleInversion {
    type Output = f32;
    fn index(&self, _: crate::elements::e31) -> &Self::Output {
        &self[1]
    }
}
impl std::ops::Index<crate::elements::e12> for MysteryDipoleInversion {
    type Output = f32;
    fn index(&self, _: crate::elements::e12) -> &Self::Output {
        &self[2]
    }
}
impl std::ops::Index<crate::elements::e45> for MysteryDipoleInversion {
    type Output = f32;
    fn index(&self, _: crate::elements::e45) -> &Self::Output {
        &self[3]
    }
}
impl std::ops::Index<crate::elements::e4235> for MysteryDipoleInversion {
    type Output = f32;
    fn index(&self, _: crate::elements::e4235) -> &Self::Output {
        &self[4]
    }
}
impl std::ops::Index<crate::elements::e4315> for MysteryDipoleInversion {
    type Output = f32;
    fn index(&self, _: crate::elements::e4315) -> &Self::Output {
        &self[5]
    }
}
impl std::ops::Index<crate::elements::e4125> for MysteryDipoleInversion {
    type Output = f32;
    fn index(&self, _: crate::elements::e4125) -> &Self::Output {
        &self[6]
    }
}
impl std::ops::IndexMut<crate::elements::e23> for MysteryDipoleInversion {
    fn index_mut(&self, _: crate::elements::e23) -> &mut Self::Output {
        &mut self[0]
    }
}
impl std::ops::IndexMut<crate::elements::e31> for MysteryDipoleInversion {
    fn index_mut(&self, _: crate::elements::e31) -> &mut Self::Output {
        &mut self[1]
    }
}
impl std::ops::IndexMut<crate::elements::e12> for MysteryDipoleInversion {
    fn index_mut(&self, _: crate::elements::e12) -> &mut Self::Output {
        &mut self[2]
    }
}
impl std::ops::IndexMut<crate::elements::e45> for MysteryDipoleInversion {
    fn index_mut(&self, _: crate::elements::e45) -> &mut Self::Output {
        &mut self[3]
    }
}
impl std::ops::IndexMut<crate::elements::e4235> for MysteryDipoleInversion {
    fn index_mut(&self, _: crate::elements::e4235) -> &mut Self::Output {
        &mut self[4]
    }
}
impl std::ops::IndexMut<crate::elements::e4315> for MysteryDipoleInversion {
    fn index_mut(&self, _: crate::elements::e4315) -> &mut Self::Output {
        &mut self[5]
    }
}
impl std::ops::IndexMut<crate::elements::e4125> for MysteryDipoleInversion {
    fn index_mut(&self, _: crate::elements::e4125) -> &mut Self::Output {
        &mut self[6]
    }
}
include!("./impls/mystery_dipole_inversion.rs");
