use crate::data::*;
use crate::simd::*;

/// DipoleInversion
#[derive(Clone, Copy, nearly::NearlyEq, nearly::NearlyOrd, bytemuck::Pod, bytemuck::Zeroable, encase::ShaderType, serde::Serialize, serde::Deserialize)]
pub union DipoleInversion {
    groups: DipoleInversionGroups,
    /// e41, e42, e43, 0, e23, e31, e12, e45, e15, e25, e35, e1234, e4235, e4315, e4125, e3215
    elements: [f32; 16],
}
#[derive(Clone, Copy, nearly::NearlyEq, nearly::NearlyOrd, bytemuck::Pod, bytemuck::Zeroable, encase::ShaderType, serde::Serialize, serde::Deserialize)]
pub struct DipoleInversionGroups {
    /// e41, e42, e43
    g0: Simd32x3,
    /// e23, e31, e12, e45
    g1: Simd32x4,
    /// e15, e25, e35, e1234
    g2: Simd32x4,
    /// e4235, e4315, e4125, e3215
    g3: Simd32x4,
}
impl DipoleInversion {
    #[allow(clippy::too_many_arguments)]
    pub const fn from_elements(
        e41: f32,
        e42: f32,
        e43: f32,
        e23: f32,
        e31: f32,
        e12: f32,
        e45: f32,
        e15: f32,
        e25: f32,
        e35: f32,
        e1234: f32,
        e4235: f32,
        e4315: f32,
        e4125: f32,
        e3215: f32,
    ) -> Self {
        Self {
            elements: [e41, e42, e43, 0.0, e23, e31, e12, e45, e15, e25, e35, e1234, e4235, e4315, e4125, e3215],
        }
    }
    pub const fn from_groups(g0: Simd32x3, g1: Simd32x4, g2: Simd32x4, g3: Simd32x4) -> Self {
        Self {
            groups: DipoleInversionGroups { g0, g1, g2, g3 },
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
    #[inline(always)]
    pub fn group3(&self) -> Simd32x4 {
        unsafe { self.groups.g3 }
    }
    #[inline(always)]
    pub fn group3_mut(&mut self) -> &mut Simd32x4 {
        unsafe { &mut self.groups.g3 }
    }
}
const DIPOLE_INVERSION_INDEX_REMAP: [usize; 15] = [0, 1, 2, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
impl std::ops::Index<usize> for DipoleInversion {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[DIPOLE_INVERSION_INDEX_REMAP[index]] }
    }
}
impl std::ops::IndexMut<usize> for DipoleInversion {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[DIPOLE_INVERSION_INDEX_REMAP[index]] }
    }
}
impl From<DipoleInversion> for [f32; 15] {
    fn from(vector: DipoleInversion) -> Self {
        unsafe {
            [
                vector.elements[0], vector.elements[1], vector.elements[2], vector.elements[4], vector.elements[5], vector.elements[6], vector.elements[7], vector.elements[8],
                vector.elements[9], vector.elements[10], vector.elements[11], vector.elements[12], vector.elements[13], vector.elements[14], vector.elements[15],
            ]
        }
    }
}
impl From<[f32; 15]> for DipoleInversion {
    fn from(array: [f32; 15]) -> Self {
        Self {
            elements: [
                array[0], array[1], array[2], 0.0, array[1], array[2], array[3], array[4], array[2], array[3], array[4], array[5], array[3], array[4], array[5], array[6],
            ],
        }
    }
}
impl std::fmt::Debug for DipoleInversion {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .debug_struct("DipoleInversion")
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
            .field("e1234", &self[10])
            .field("e4235", &self[11])
            .field("e4315", &self[12])
            .field("e4125", &self[13])
            .field("e3215", &self[14])
            .finish()
    }
}

impl DipoleInversion {
    pub const LEN: usize = 15;
}

impl DipoleInversion {
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

impl PartialOrd for DipoleInversion {
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
impl Ord for DipoleInversion {
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
impl PartialEq for DipoleInversion {
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
impl Eq for DipoleInversion {}
impl std::hash::Hash for DipoleInversion {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for i in 0..Self::LEN {
            self[i].to_bits().hash(state);
        }
    }
}

impl std::ops::Index<crate::elements::e41> for DipoleInversion {
    type Output = f32;
    fn index(&self, _: crate::elements::e41) -> &Self::Output {
        &self[0]
    }
}
impl std::ops::Index<crate::elements::e42> for DipoleInversion {
    type Output = f32;
    fn index(&self, _: crate::elements::e42) -> &Self::Output {
        &self[1]
    }
}
impl std::ops::Index<crate::elements::e43> for DipoleInversion {
    type Output = f32;
    fn index(&self, _: crate::elements::e43) -> &Self::Output {
        &self[2]
    }
}
impl std::ops::Index<crate::elements::e23> for DipoleInversion {
    type Output = f32;
    fn index(&self, _: crate::elements::e23) -> &Self::Output {
        &self[3]
    }
}
impl std::ops::Index<crate::elements::e31> for DipoleInversion {
    type Output = f32;
    fn index(&self, _: crate::elements::e31) -> &Self::Output {
        &self[4]
    }
}
impl std::ops::Index<crate::elements::e12> for DipoleInversion {
    type Output = f32;
    fn index(&self, _: crate::elements::e12) -> &Self::Output {
        &self[5]
    }
}
impl std::ops::Index<crate::elements::e45> for DipoleInversion {
    type Output = f32;
    fn index(&self, _: crate::elements::e45) -> &Self::Output {
        &self[6]
    }
}
impl std::ops::Index<crate::elements::e15> for DipoleInversion {
    type Output = f32;
    fn index(&self, _: crate::elements::e15) -> &Self::Output {
        &self[7]
    }
}
impl std::ops::Index<crate::elements::e25> for DipoleInversion {
    type Output = f32;
    fn index(&self, _: crate::elements::e25) -> &Self::Output {
        &self[8]
    }
}
impl std::ops::Index<crate::elements::e35> for DipoleInversion {
    type Output = f32;
    fn index(&self, _: crate::elements::e35) -> &Self::Output {
        &self[9]
    }
}
impl std::ops::Index<crate::elements::e1234> for DipoleInversion {
    type Output = f32;
    fn index(&self, _: crate::elements::e1234) -> &Self::Output {
        &self[10]
    }
}
impl std::ops::Index<crate::elements::e4235> for DipoleInversion {
    type Output = f32;
    fn index(&self, _: crate::elements::e4235) -> &Self::Output {
        &self[11]
    }
}
impl std::ops::Index<crate::elements::e4315> for DipoleInversion {
    type Output = f32;
    fn index(&self, _: crate::elements::e4315) -> &Self::Output {
        &self[12]
    }
}
impl std::ops::Index<crate::elements::e4125> for DipoleInversion {
    type Output = f32;
    fn index(&self, _: crate::elements::e4125) -> &Self::Output {
        &self[13]
    }
}
impl std::ops::Index<crate::elements::e3215> for DipoleInversion {
    type Output = f32;
    fn index(&self, _: crate::elements::e3215) -> &Self::Output {
        &self[14]
    }
}
impl std::ops::IndexMut<crate::elements::e41> for DipoleInversion {
    fn index_mut(&self, _: crate::elements::e41) -> &mut Self::Output {
        &mut self[0]
    }
}
impl std::ops::IndexMut<crate::elements::e42> for DipoleInversion {
    fn index_mut(&self, _: crate::elements::e42) -> &mut Self::Output {
        &mut self[1]
    }
}
impl std::ops::IndexMut<crate::elements::e43> for DipoleInversion {
    fn index_mut(&self, _: crate::elements::e43) -> &mut Self::Output {
        &mut self[2]
    }
}
impl std::ops::IndexMut<crate::elements::e23> for DipoleInversion {
    fn index_mut(&self, _: crate::elements::e23) -> &mut Self::Output {
        &mut self[3]
    }
}
impl std::ops::IndexMut<crate::elements::e31> for DipoleInversion {
    fn index_mut(&self, _: crate::elements::e31) -> &mut Self::Output {
        &mut self[4]
    }
}
impl std::ops::IndexMut<crate::elements::e12> for DipoleInversion {
    fn index_mut(&self, _: crate::elements::e12) -> &mut Self::Output {
        &mut self[5]
    }
}
impl std::ops::IndexMut<crate::elements::e45> for DipoleInversion {
    fn index_mut(&self, _: crate::elements::e45) -> &mut Self::Output {
        &mut self[6]
    }
}
impl std::ops::IndexMut<crate::elements::e15> for DipoleInversion {
    fn index_mut(&self, _: crate::elements::e15) -> &mut Self::Output {
        &mut self[7]
    }
}
impl std::ops::IndexMut<crate::elements::e25> for DipoleInversion {
    fn index_mut(&self, _: crate::elements::e25) -> &mut Self::Output {
        &mut self[8]
    }
}
impl std::ops::IndexMut<crate::elements::e35> for DipoleInversion {
    fn index_mut(&self, _: crate::elements::e35) -> &mut Self::Output {
        &mut self[9]
    }
}
impl std::ops::IndexMut<crate::elements::e1234> for DipoleInversion {
    fn index_mut(&self, _: crate::elements::e1234) -> &mut Self::Output {
        &mut self[10]
    }
}
impl std::ops::IndexMut<crate::elements::e4235> for DipoleInversion {
    fn index_mut(&self, _: crate::elements::e4235) -> &mut Self::Output {
        &mut self[11]
    }
}
impl std::ops::IndexMut<crate::elements::e4315> for DipoleInversion {
    fn index_mut(&self, _: crate::elements::e4315) -> &mut Self::Output {
        &mut self[12]
    }
}
impl std::ops::IndexMut<crate::elements::e4125> for DipoleInversion {
    fn index_mut(&self, _: crate::elements::e4125) -> &mut Self::Output {
        &mut self[13]
    }
}
impl std::ops::IndexMut<crate::elements::e3215> for DipoleInversion {
    fn index_mut(&self, _: crate::elements::e3215) -> &mut Self::Output {
        &mut self[14]
    }
}
include!("./impls/dipole_inversion.rs");
