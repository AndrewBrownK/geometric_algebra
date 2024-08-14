use crate::data::*;
use crate::simd::*;

/// VersorOddOnOrigin.
/// This variant of VersorOdd intersects the Origin.
#[derive(Clone, Copy, nearly::NearlyEq, nearly::NearlyOrd, bytemuck::Pod, bytemuck::Zeroable, encase::ShaderType, serde::Serialize, serde::Deserialize)]
pub union VersorOddOnOrigin {
    groups: VersorOddOnOriginGroups,
    /// e41, e42, e43, e45, e1234, e4235, e4315, e4125
    elements: [f32; 8],
}
#[derive(Clone, Copy, nearly::NearlyEq, nearly::NearlyOrd, bytemuck::Pod, bytemuck::Zeroable, encase::ShaderType, serde::Serialize, serde::Deserialize)]
pub struct VersorOddOnOriginGroups {
    /// e41, e42, e43, e45
    g0: Simd32x4,
    /// e1234, e4235, e4315, e4125
    g1: Simd32x4,
}
impl VersorOddOnOrigin {
    #[allow(clippy::too_many_arguments)]
    pub const fn from_elements(e41: f32, e42: f32, e43: f32, e45: f32, e1234: f32, e4235: f32, e4315: f32, e4125: f32) -> Self {
        Self {
            elements: [e41, e42, e43, e45, e1234, e4235, e4315, e4125],
        }
    }
    pub const fn from_groups(g0: Simd32x4, g1: Simd32x4) -> Self {
        Self {
            groups: VersorOddOnOriginGroups { g0, g1 },
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
const VERSOR_ODD_ON_ORIGIN_INDEX_REMAP: [usize; 8] = [0, 1, 2, 3, 4, 5, 6, 7];
impl std::ops::Index<usize> for VersorOddOnOrigin {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[VERSOR_ODD_ON_ORIGIN_INDEX_REMAP[index]] }
    }
}
impl std::ops::IndexMut<usize> for VersorOddOnOrigin {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[VERSOR_ODD_ON_ORIGIN_INDEX_REMAP[index]] }
    }
}
impl From<VersorOddOnOrigin> for [f32; 8] {
    fn from(vector: VersorOddOnOrigin) -> Self {
        unsafe {
            [
                vector.elements[0], vector.elements[1], vector.elements[2], vector.elements[3], vector.elements[4], vector.elements[5], vector.elements[6], vector.elements[7],
            ]
        }
    }
}
impl From<[f32; 8]> for VersorOddOnOrigin {
    fn from(array: [f32; 8]) -> Self {
        Self {
            elements: [array[0], array[1], array[2], array[3], array[1], array[2], array[3], array[4]],
        }
    }
}
impl std::fmt::Debug for VersorOddOnOrigin {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .debug_struct("VersorOddOnOrigin")
            .field("e41", &self[0])
            .field("e42", &self[1])
            .field("e43", &self[2])
            .field("e45", &self[3])
            .field("e1234", &self[4])
            .field("e4235", &self[5])
            .field("e4315", &self[6])
            .field("e4125", &self[7])
            .finish()
    }
}

impl VersorOddOnOrigin {
    pub const LEN: usize = 8;
}

impl VersorOddOnOrigin {
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

impl PartialOrd for VersorOddOnOrigin {
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
impl Ord for VersorOddOnOrigin {
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
impl PartialEq for VersorOddOnOrigin {
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
impl Eq for VersorOddOnOrigin {}
impl std::hash::Hash for VersorOddOnOrigin {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for i in 0..Self::LEN {
            self[i].to_bits().hash(state);
        }
    }
}

impl std::ops::Index<crate::elements::e41> for VersorOddOnOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e41) -> &Self::Output {
        &self[0]
    }
}
impl std::ops::Index<crate::elements::e42> for VersorOddOnOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e42) -> &Self::Output {
        &self[1]
    }
}
impl std::ops::Index<crate::elements::e43> for VersorOddOnOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e43) -> &Self::Output {
        &self[2]
    }
}
impl std::ops::Index<crate::elements::e45> for VersorOddOnOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e45) -> &Self::Output {
        &self[3]
    }
}
impl std::ops::Index<crate::elements::e1234> for VersorOddOnOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e1234) -> &Self::Output {
        &self[4]
    }
}
impl std::ops::Index<crate::elements::e4235> for VersorOddOnOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e4235) -> &Self::Output {
        &self[5]
    }
}
impl std::ops::Index<crate::elements::e4315> for VersorOddOnOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e4315) -> &Self::Output {
        &self[6]
    }
}
impl std::ops::Index<crate::elements::e4125> for VersorOddOnOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e4125) -> &Self::Output {
        &self[7]
    }
}
impl std::ops::IndexMut<crate::elements::e41> for VersorOddOnOrigin {
    fn index_mut(&self, _: crate::elements::e41) -> &mut Self::Output {
        &mut self[0]
    }
}
impl std::ops::IndexMut<crate::elements::e42> for VersorOddOnOrigin {
    fn index_mut(&self, _: crate::elements::e42) -> &mut Self::Output {
        &mut self[1]
    }
}
impl std::ops::IndexMut<crate::elements::e43> for VersorOddOnOrigin {
    fn index_mut(&self, _: crate::elements::e43) -> &mut Self::Output {
        &mut self[2]
    }
}
impl std::ops::IndexMut<crate::elements::e45> for VersorOddOnOrigin {
    fn index_mut(&self, _: crate::elements::e45) -> &mut Self::Output {
        &mut self[3]
    }
}
impl std::ops::IndexMut<crate::elements::e1234> for VersorOddOnOrigin {
    fn index_mut(&self, _: crate::elements::e1234) -> &mut Self::Output {
        &mut self[4]
    }
}
impl std::ops::IndexMut<crate::elements::e4235> for VersorOddOnOrigin {
    fn index_mut(&self, _: crate::elements::e4235) -> &mut Self::Output {
        &mut self[5]
    }
}
impl std::ops::IndexMut<crate::elements::e4315> for VersorOddOnOrigin {
    fn index_mut(&self, _: crate::elements::e4315) -> &mut Self::Output {
        &mut self[6]
    }
}
impl std::ops::IndexMut<crate::elements::e4125> for VersorOddOnOrigin {
    fn index_mut(&self, _: crate::elements::e4125) -> &mut Self::Output {
        &mut self[7]
    }
}
include!("./impls/versor_odd_on_origin.rs");
