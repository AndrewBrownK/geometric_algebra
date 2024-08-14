use crate::data::*;
use crate::simd::*;

/// VersorOddAtInfinity.
/// This variant of VersorOdd exists at the Horizon.
#[derive(Clone, Copy, nearly::NearlyEq, nearly::NearlyOrd, bytemuck::Pod, bytemuck::Zeroable, encase::ShaderType, serde::Serialize, serde::Deserialize)]
pub union VersorOddAtInfinity {
    groups: VersorOddAtInfinityGroups,
    /// scalar, e15, e25, e35, e23, e31, e12, e45, e4235, e4315, e4125, e3215
    elements: [f32; 12],
}
#[derive(Clone, Copy, nearly::NearlyEq, nearly::NearlyOrd, bytemuck::Pod, bytemuck::Zeroable, encase::ShaderType, serde::Serialize, serde::Deserialize)]
pub struct VersorOddAtInfinityGroups {
    /// scalar, e15, e25, e35
    g0: Simd32x4,
    /// e23, e31, e12, e45
    g1: Simd32x4,
    /// e4235, e4315, e4125, e3215
    g2: Simd32x4,
}
impl VersorOddAtInfinity {
    #[allow(clippy::too_many_arguments)]
    pub const fn from_elements(scalar: f32, e15: f32, e25: f32, e35: f32, e23: f32, e31: f32, e12: f32, e45: f32, e4235: f32, e4315: f32, e4125: f32, e3215: f32) -> Self {
        Self {
            elements: [scalar, e15, e25, e35, e23, e31, e12, e45, e4235, e4315, e4125, e3215],
        }
    }
    pub const fn from_groups(g0: Simd32x4, g1: Simd32x4, g2: Simd32x4) -> Self {
        Self {
            groups: VersorOddAtInfinityGroups { g0, g1, g2 },
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
    #[inline(always)]
    pub fn group2(&self) -> Simd32x4 {
        unsafe { self.groups.g2 }
    }
    #[inline(always)]
    pub fn group2_mut(&mut self) -> &mut Simd32x4 {
        unsafe { &mut self.groups.g2 }
    }
}
const VERSOR_ODD_AT_INFINITY_INDEX_REMAP: [usize; 12] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
impl std::ops::Index<usize> for VersorOddAtInfinity {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[VERSOR_ODD_AT_INFINITY_INDEX_REMAP[index]] }
    }
}
impl std::ops::IndexMut<usize> for VersorOddAtInfinity {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[VERSOR_ODD_AT_INFINITY_INDEX_REMAP[index]] }
    }
}
impl From<VersorOddAtInfinity> for [f32; 12] {
    fn from(vector: VersorOddAtInfinity) -> Self {
        unsafe {
            [
                vector.elements[0], vector.elements[1], vector.elements[2], vector.elements[3], vector.elements[4], vector.elements[5], vector.elements[6], vector.elements[7],
                vector.elements[8], vector.elements[9], vector.elements[10], vector.elements[11],
            ]
        }
    }
}
impl From<[f32; 12]> for VersorOddAtInfinity {
    fn from(array: [f32; 12]) -> Self {
        Self {
            elements: [array[0], array[1], array[2], array[3], array[1], array[2], array[3], array[4], array[2], array[3], array[4], array[5]],
        }
    }
}
impl std::fmt::Debug for VersorOddAtInfinity {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .debug_struct("VersorOddAtInfinity")
            .field("scalar", &self[0])
            .field("e15", &self[1])
            .field("e25", &self[2])
            .field("e35", &self[3])
            .field("e23", &self[4])
            .field("e31", &self[5])
            .field("e12", &self[6])
            .field("e45", &self[7])
            .field("e4235", &self[8])
            .field("e4315", &self[9])
            .field("e4125", &self[10])
            .field("e3215", &self[11])
            .finish()
    }
}

impl VersorOddAtInfinity {
    pub const LEN: usize = 12;
}

impl VersorOddAtInfinity {
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

impl PartialOrd for VersorOddAtInfinity {
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
impl Ord for VersorOddAtInfinity {
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
impl PartialEq for VersorOddAtInfinity {
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
impl Eq for VersorOddAtInfinity {}
impl std::hash::Hash for VersorOddAtInfinity {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for i in 0..Self::LEN {
            self[i].to_bits().hash(state);
        }
    }
}

impl std::ops::Index<crate::elements::scalar> for VersorOddAtInfinity {
    type Output = f32;
    fn index(&self, _: crate::elements::scalar) -> &Self::Output {
        &self[0]
    }
}
impl std::ops::Index<crate::elements::e15> for VersorOddAtInfinity {
    type Output = f32;
    fn index(&self, _: crate::elements::e15) -> &Self::Output {
        &self[1]
    }
}
impl std::ops::Index<crate::elements::e25> for VersorOddAtInfinity {
    type Output = f32;
    fn index(&self, _: crate::elements::e25) -> &Self::Output {
        &self[2]
    }
}
impl std::ops::Index<crate::elements::e35> for VersorOddAtInfinity {
    type Output = f32;
    fn index(&self, _: crate::elements::e35) -> &Self::Output {
        &self[3]
    }
}
impl std::ops::Index<crate::elements::e23> for VersorOddAtInfinity {
    type Output = f32;
    fn index(&self, _: crate::elements::e23) -> &Self::Output {
        &self[4]
    }
}
impl std::ops::Index<crate::elements::e31> for VersorOddAtInfinity {
    type Output = f32;
    fn index(&self, _: crate::elements::e31) -> &Self::Output {
        &self[5]
    }
}
impl std::ops::Index<crate::elements::e12> for VersorOddAtInfinity {
    type Output = f32;
    fn index(&self, _: crate::elements::e12) -> &Self::Output {
        &self[6]
    }
}
impl std::ops::Index<crate::elements::e45> for VersorOddAtInfinity {
    type Output = f32;
    fn index(&self, _: crate::elements::e45) -> &Self::Output {
        &self[7]
    }
}
impl std::ops::Index<crate::elements::e4235> for VersorOddAtInfinity {
    type Output = f32;
    fn index(&self, _: crate::elements::e4235) -> &Self::Output {
        &self[8]
    }
}
impl std::ops::Index<crate::elements::e4315> for VersorOddAtInfinity {
    type Output = f32;
    fn index(&self, _: crate::elements::e4315) -> &Self::Output {
        &self[9]
    }
}
impl std::ops::Index<crate::elements::e4125> for VersorOddAtInfinity {
    type Output = f32;
    fn index(&self, _: crate::elements::e4125) -> &Self::Output {
        &self[10]
    }
}
impl std::ops::Index<crate::elements::e3215> for VersorOddAtInfinity {
    type Output = f32;
    fn index(&self, _: crate::elements::e3215) -> &Self::Output {
        &self[11]
    }
}
impl std::ops::IndexMut<crate::elements::scalar> for VersorOddAtInfinity {
    fn index_mut(&self, _: crate::elements::scalar) -> &mut Self::Output {
        &mut self[0]
    }
}
impl std::ops::IndexMut<crate::elements::e15> for VersorOddAtInfinity {
    fn index_mut(&self, _: crate::elements::e15) -> &mut Self::Output {
        &mut self[1]
    }
}
impl std::ops::IndexMut<crate::elements::e25> for VersorOddAtInfinity {
    fn index_mut(&self, _: crate::elements::e25) -> &mut Self::Output {
        &mut self[2]
    }
}
impl std::ops::IndexMut<crate::elements::e35> for VersorOddAtInfinity {
    fn index_mut(&self, _: crate::elements::e35) -> &mut Self::Output {
        &mut self[3]
    }
}
impl std::ops::IndexMut<crate::elements::e23> for VersorOddAtInfinity {
    fn index_mut(&self, _: crate::elements::e23) -> &mut Self::Output {
        &mut self[4]
    }
}
impl std::ops::IndexMut<crate::elements::e31> for VersorOddAtInfinity {
    fn index_mut(&self, _: crate::elements::e31) -> &mut Self::Output {
        &mut self[5]
    }
}
impl std::ops::IndexMut<crate::elements::e12> for VersorOddAtInfinity {
    fn index_mut(&self, _: crate::elements::e12) -> &mut Self::Output {
        &mut self[6]
    }
}
impl std::ops::IndexMut<crate::elements::e45> for VersorOddAtInfinity {
    fn index_mut(&self, _: crate::elements::e45) -> &mut Self::Output {
        &mut self[7]
    }
}
impl std::ops::IndexMut<crate::elements::e4235> for VersorOddAtInfinity {
    fn index_mut(&self, _: crate::elements::e4235) -> &mut Self::Output {
        &mut self[8]
    }
}
impl std::ops::IndexMut<crate::elements::e4315> for VersorOddAtInfinity {
    fn index_mut(&self, _: crate::elements::e4315) -> &mut Self::Output {
        &mut self[9]
    }
}
impl std::ops::IndexMut<crate::elements::e4125> for VersorOddAtInfinity {
    fn index_mut(&self, _: crate::elements::e4125) -> &mut Self::Output {
        &mut self[10]
    }
}
impl std::ops::IndexMut<crate::elements::e3215> for VersorOddAtInfinity {
    fn index_mut(&self, _: crate::elements::e3215) -> &mut Self::Output {
        &mut self[11]
    }
}
include!("./impls/versor_odd_at_infinity.rs");