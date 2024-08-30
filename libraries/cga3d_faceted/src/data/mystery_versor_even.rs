use crate::data::*;
use crate::simd::*;

/// MysteryVersorEven.
/// TODO this is currently a mystery I'm investigating
#[derive(Clone, Copy, nearly::NearlyEq, nearly::NearlyOrd, bytemuck::Pod, bytemuck::Zeroable, encase::ShaderType, serde::Serialize, serde::Deserialize)]
pub union MysteryVersorEven {
    groups: MysteryVersorEvenGroups,
    /// e12345, e1, e2, e3, e415, e425, e435, e321
    elements: [f32; 8],
}
#[derive(Clone, Copy, nearly::NearlyEq, nearly::NearlyOrd, bytemuck::Pod, bytemuck::Zeroable, encase::ShaderType, serde::Serialize, serde::Deserialize)]
pub struct MysteryVersorEvenGroups {
    /// e12345, e1, e2, e3
    g0: Simd32x4,
    /// e415, e425, e435, e321
    g1: Simd32x4,
}
impl MysteryVersorEven {
    #[allow(clippy::too_many_arguments)]
    pub const fn from_elements(e12345: f32, e1: f32, e2: f32, e3: f32, e415: f32, e425: f32, e435: f32, e321: f32) -> Self {
        Self {
            elements: [e12345, e1, e2, e3, e415, e425, e435, e321],
        }
    }
    pub const fn from_groups(g0: Simd32x4, g1: Simd32x4) -> Self {
        Self {
            groups: MysteryVersorEvenGroups { g0, g1 },
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
const MYSTERY_VERSOR_EVEN_INDEX_REMAP: [usize; 8] = [0, 1, 2, 3, 4, 5, 6, 7];
impl std::ops::Index<usize> for MysteryVersorEven {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[MYSTERY_VERSOR_EVEN_INDEX_REMAP[index]] }
    }
}
impl std::ops::IndexMut<usize> for MysteryVersorEven {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[MYSTERY_VERSOR_EVEN_INDEX_REMAP[index]] }
    }
}
impl From<MysteryVersorEven> for [f32; 8] {
    fn from(vector: MysteryVersorEven) -> Self {
        unsafe {
            [
                vector.elements[0], vector.elements[1], vector.elements[2], vector.elements[3], vector.elements[4], vector.elements[5], vector.elements[6], vector.elements[7],
            ]
        }
    }
}
impl From<[f32; 8]> for MysteryVersorEven {
    fn from(array: [f32; 8]) -> Self {
        Self {
            elements: [array[0], array[1], array[2], array[3], array[1], array[2], array[3], array[4]],
        }
    }
}
impl std::fmt::Debug for MysteryVersorEven {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .debug_struct("MysteryVersorEven")
            .field("e12345", &self[0])
            .field("e1", &self[1])
            .field("e2", &self[2])
            .field("e3", &self[3])
            .field("e415", &self[4])
            .field("e425", &self[5])
            .field("e435", &self[6])
            .field("e321", &self[7])
            .finish()
    }
}

impl MysteryVersorEven {
    pub const LEN: usize = 8;
}

impl MysteryVersorEven {
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

impl PartialOrd for MysteryVersorEven {
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
impl Ord for MysteryVersorEven {
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
impl PartialEq for MysteryVersorEven {
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
impl Eq for MysteryVersorEven {}
impl std::hash::Hash for MysteryVersorEven {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for i in 0..Self::LEN {
            self[i].to_bits().hash(state);
        }
    }
}

impl std::ops::Index<crate::elements::e12345> for MysteryVersorEven {
    type Output = f32;
    fn index(&self, _: crate::elements::e12345) -> &Self::Output {
        &self[0]
    }
}
impl std::ops::Index<crate::elements::e1> for MysteryVersorEven {
    type Output = f32;
    fn index(&self, _: crate::elements::e1) -> &Self::Output {
        &self[1]
    }
}
impl std::ops::Index<crate::elements::e2> for MysteryVersorEven {
    type Output = f32;
    fn index(&self, _: crate::elements::e2) -> &Self::Output {
        &self[2]
    }
}
impl std::ops::Index<crate::elements::e3> for MysteryVersorEven {
    type Output = f32;
    fn index(&self, _: crate::elements::e3) -> &Self::Output {
        &self[3]
    }
}
impl std::ops::Index<crate::elements::e415> for MysteryVersorEven {
    type Output = f32;
    fn index(&self, _: crate::elements::e415) -> &Self::Output {
        &self[4]
    }
}
impl std::ops::Index<crate::elements::e425> for MysteryVersorEven {
    type Output = f32;
    fn index(&self, _: crate::elements::e425) -> &Self::Output {
        &self[5]
    }
}
impl std::ops::Index<crate::elements::e435> for MysteryVersorEven {
    type Output = f32;
    fn index(&self, _: crate::elements::e435) -> &Self::Output {
        &self[6]
    }
}
impl std::ops::Index<crate::elements::e321> for MysteryVersorEven {
    type Output = f32;
    fn index(&self, _: crate::elements::e321) -> &Self::Output {
        &self[7]
    }
}
impl std::ops::IndexMut<crate::elements::e12345> for MysteryVersorEven {
    fn index_mut(&self, _: crate::elements::e12345) -> &mut Self::Output {
        &mut self[0]
    }
}
impl std::ops::IndexMut<crate::elements::e1> for MysteryVersorEven {
    fn index_mut(&self, _: crate::elements::e1) -> &mut Self::Output {
        &mut self[1]
    }
}
impl std::ops::IndexMut<crate::elements::e2> for MysteryVersorEven {
    fn index_mut(&self, _: crate::elements::e2) -> &mut Self::Output {
        &mut self[2]
    }
}
impl std::ops::IndexMut<crate::elements::e3> for MysteryVersorEven {
    fn index_mut(&self, _: crate::elements::e3) -> &mut Self::Output {
        &mut self[3]
    }
}
impl std::ops::IndexMut<crate::elements::e415> for MysteryVersorEven {
    fn index_mut(&self, _: crate::elements::e415) -> &mut Self::Output {
        &mut self[4]
    }
}
impl std::ops::IndexMut<crate::elements::e425> for MysteryVersorEven {
    fn index_mut(&self, _: crate::elements::e425) -> &mut Self::Output {
        &mut self[5]
    }
}
impl std::ops::IndexMut<crate::elements::e435> for MysteryVersorEven {
    fn index_mut(&self, _: crate::elements::e435) -> &mut Self::Output {
        &mut self[6]
    }
}
impl std::ops::IndexMut<crate::elements::e321> for MysteryVersorEven {
    fn index_mut(&self, _: crate::elements::e321) -> &mut Self::Output {
        &mut self[7]
    }
}
include!("./impls/mystery_versor_even.rs");
