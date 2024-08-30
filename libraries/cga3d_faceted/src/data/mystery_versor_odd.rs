use crate::data::*;
use crate::simd::*;

/// MysteryVersorOdd.
/// TODO this is currently a mystery I'm investigating
#[derive(Clone, Copy, nearly::NearlyEq, nearly::NearlyOrd, bytemuck::Pod, bytemuck::Zeroable, encase::ShaderType, serde::Serialize, serde::Deserialize)]
pub union MysteryVersorOdd {
    groups: MysteryVersorOddGroups,
    /// scalar, e4235, e4315, e4125, e23, e31, e12, e45
    elements: [f32; 8],
}
#[derive(Clone, Copy, nearly::NearlyEq, nearly::NearlyOrd, bytemuck::Pod, bytemuck::Zeroable, encase::ShaderType, serde::Serialize, serde::Deserialize)]
pub struct MysteryVersorOddGroups {
    /// scalar, e4235, e4315, e4125
    g0: Simd32x4,
    /// e23, e31, e12, e45
    g1: Simd32x4,
}
impl MysteryVersorOdd {
    #[allow(clippy::too_many_arguments)]
    pub const fn from_elements(scalar: f32, e4235: f32, e4315: f32, e4125: f32, e23: f32, e31: f32, e12: f32, e45: f32) -> Self {
        Self {
            elements: [scalar, e4235, e4315, e4125, e23, e31, e12, e45],
        }
    }
    pub const fn from_groups(g0: Simd32x4, g1: Simd32x4) -> Self {
        Self {
            groups: MysteryVersorOddGroups { g0, g1 },
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
    pub fn group1(&self) -> Simd32x4 {
        unsafe { self.groups.g1 }
    }
    #[inline(always)]
    pub fn group1_mut(&mut self) -> &mut Simd32x4 {
        unsafe { &mut self.groups.g1 }
    }
}
const MYSTERY_VERSOR_ODD_INDEX_REMAP: [usize; 8] = [0, 1, 2, 3, 4, 5, 6, 7];
impl std::ops::Index<usize> for MysteryVersorOdd {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[MYSTERY_VERSOR_ODD_INDEX_REMAP[index]] }
    }
}
impl std::ops::IndexMut<usize> for MysteryVersorOdd {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[MYSTERY_VERSOR_ODD_INDEX_REMAP[index]] }
    }
}
impl From<MysteryVersorOdd> for [f32; 8] {
    fn from(vector: MysteryVersorOdd) -> Self {
        unsafe {
            [
                vector.elements[0], vector.elements[1], vector.elements[2], vector.elements[3], vector.elements[4], vector.elements[5], vector.elements[6], vector.elements[7],
            ]
        }
    }
}
impl From<[f32; 8]> for MysteryVersorOdd {
    fn from(array: [f32; 8]) -> Self {
        Self {
            elements: [array[0], array[1], array[2], array[3], array[1], array[2], array[3], array[4]],
        }
    }
}
impl std::fmt::Debug for MysteryVersorOdd {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .debug_struct("MysteryVersorOdd")
            .field("scalar", &self[0])
            .field("e4235", &self[1])
            .field("e4315", &self[2])
            .field("e4125", &self[3])
            .field("e23", &self[4])
            .field("e31", &self[5])
            .field("e12", &self[6])
            .field("e45", &self[7])
            .finish()
    }
}

impl MysteryVersorOdd {
    pub const LEN: usize = 8;
}

impl MysteryVersorOdd {
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

impl PartialOrd for MysteryVersorOdd {
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
impl Ord for MysteryVersorOdd {
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
impl PartialEq for MysteryVersorOdd {
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
impl Eq for MysteryVersorOdd {}
impl std::hash::Hash for MysteryVersorOdd {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for i in 0..Self::LEN {
            self[i].to_bits().hash(state);
        }
    }
}

impl std::ops::Index<crate::elements::scalar> for MysteryVersorOdd {
    type Output = f32;
    fn index(&self, _: crate::elements::scalar) -> &Self::Output {
        &self[0]
    }
}
impl std::ops::Index<crate::elements::e4235> for MysteryVersorOdd {
    type Output = f32;
    fn index(&self, _: crate::elements::e4235) -> &Self::Output {
        &self[1]
    }
}
impl std::ops::Index<crate::elements::e4315> for MysteryVersorOdd {
    type Output = f32;
    fn index(&self, _: crate::elements::e4315) -> &Self::Output {
        &self[2]
    }
}
impl std::ops::Index<crate::elements::e4125> for MysteryVersorOdd {
    type Output = f32;
    fn index(&self, _: crate::elements::e4125) -> &Self::Output {
        &self[3]
    }
}
impl std::ops::Index<crate::elements::e23> for MysteryVersorOdd {
    type Output = f32;
    fn index(&self, _: crate::elements::e23) -> &Self::Output {
        &self[4]
    }
}
impl std::ops::Index<crate::elements::e31> for MysteryVersorOdd {
    type Output = f32;
    fn index(&self, _: crate::elements::e31) -> &Self::Output {
        &self[5]
    }
}
impl std::ops::Index<crate::elements::e12> for MysteryVersorOdd {
    type Output = f32;
    fn index(&self, _: crate::elements::e12) -> &Self::Output {
        &self[6]
    }
}
impl std::ops::Index<crate::elements::e45> for MysteryVersorOdd {
    type Output = f32;
    fn index(&self, _: crate::elements::e45) -> &Self::Output {
        &self[7]
    }
}
impl std::ops::IndexMut<crate::elements::scalar> for MysteryVersorOdd {
    fn index_mut(&self, _: crate::elements::scalar) -> &mut Self::Output {
        &mut self[0]
    }
}
impl std::ops::IndexMut<crate::elements::e4235> for MysteryVersorOdd {
    fn index_mut(&self, _: crate::elements::e4235) -> &mut Self::Output {
        &mut self[1]
    }
}
impl std::ops::IndexMut<crate::elements::e4315> for MysteryVersorOdd {
    fn index_mut(&self, _: crate::elements::e4315) -> &mut Self::Output {
        &mut self[2]
    }
}
impl std::ops::IndexMut<crate::elements::e4125> for MysteryVersorOdd {
    fn index_mut(&self, _: crate::elements::e4125) -> &mut Self::Output {
        &mut self[3]
    }
}
impl std::ops::IndexMut<crate::elements::e23> for MysteryVersorOdd {
    fn index_mut(&self, _: crate::elements::e23) -> &mut Self::Output {
        &mut self[4]
    }
}
impl std::ops::IndexMut<crate::elements::e31> for MysteryVersorOdd {
    fn index_mut(&self, _: crate::elements::e31) -> &mut Self::Output {
        &mut self[5]
    }
}
impl std::ops::IndexMut<crate::elements::e12> for MysteryVersorOdd {
    fn index_mut(&self, _: crate::elements::e12) -> &mut Self::Output {
        &mut self[6]
    }
}
impl std::ops::IndexMut<crate::elements::e45> for MysteryVersorOdd {
    fn index_mut(&self, _: crate::elements::e45) -> &mut Self::Output {
        &mut self[7]
    }
}
include!("./impls/mystery_versor_odd.rs");
