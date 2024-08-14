use crate::data::*;
use crate::simd::*;

/// VersorEvenOrthogonalOrigin.
/// This variant of VersorEven has a CoCarrier that intersects the Origin.
#[derive(Clone, Copy, nearly::NearlyEq, nearly::NearlyOrd, bytemuck::Pod, bytemuck::Zeroable, encase::ShaderType, serde::Serialize, serde::Deserialize)]
pub union VersorEvenOrthogonalOrigin {
    groups: VersorEvenOrthogonalOriginGroups,
    /// e423, e431, e412, e321, e235, e315, e125, e5, e1, e2, e3, e4
    elements: [f32; 12],
}
#[derive(Clone, Copy, nearly::NearlyEq, nearly::NearlyOrd, bytemuck::Pod, bytemuck::Zeroable, encase::ShaderType, serde::Serialize, serde::Deserialize)]
pub struct VersorEvenOrthogonalOriginGroups {
    /// e423, e431, e412, e321
    g0: Simd32x4,
    /// e235, e315, e125, e5
    g1: Simd32x4,
    /// e1, e2, e3, e4
    g2: Simd32x4,
}
impl VersorEvenOrthogonalOrigin {
    #[allow(clippy::too_many_arguments)]
    pub const fn from_elements(e423: f32, e431: f32, e412: f32, e321: f32, e235: f32, e315: f32, e125: f32, e5: f32, e1: f32, e2: f32, e3: f32, e4: f32) -> Self {
        Self {
            elements: [e423, e431, e412, e321, e235, e315, e125, e5, e1, e2, e3, e4],
        }
    }
    pub const fn from_groups(g0: Simd32x4, g1: Simd32x4, g2: Simd32x4) -> Self {
        Self {
            groups: VersorEvenOrthogonalOriginGroups { g0, g1, g2 },
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
const VERSOR_EVEN_ORTHOGONAL_ORIGIN_INDEX_REMAP: [usize; 12] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
impl std::ops::Index<usize> for VersorEvenOrthogonalOrigin {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[VERSOR_EVEN_ORTHOGONAL_ORIGIN_INDEX_REMAP[index]] }
    }
}
impl std::ops::IndexMut<usize> for VersorEvenOrthogonalOrigin {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[VERSOR_EVEN_ORTHOGONAL_ORIGIN_INDEX_REMAP[index]] }
    }
}
impl From<VersorEvenOrthogonalOrigin> for [f32; 12] {
    fn from(vector: VersorEvenOrthogonalOrigin) -> Self {
        unsafe {
            [
                vector.elements[0], vector.elements[1], vector.elements[2], vector.elements[3], vector.elements[4], vector.elements[5], vector.elements[6], vector.elements[7],
                vector.elements[8], vector.elements[9], vector.elements[10], vector.elements[11],
            ]
        }
    }
}
impl From<[f32; 12]> for VersorEvenOrthogonalOrigin {
    fn from(array: [f32; 12]) -> Self {
        Self {
            elements: [array[0], array[1], array[2], array[3], array[1], array[2], array[3], array[4], array[2], array[3], array[4], array[5]],
        }
    }
}
impl std::fmt::Debug for VersorEvenOrthogonalOrigin {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .debug_struct("VersorEvenOrthogonalOrigin")
            .field("e423", &self[0])
            .field("e431", &self[1])
            .field("e412", &self[2])
            .field("e321", &self[3])
            .field("e235", &self[4])
            .field("e315", &self[5])
            .field("e125", &self[6])
            .field("e5", &self[7])
            .field("e1", &self[8])
            .field("e2", &self[9])
            .field("e3", &self[10])
            .field("e4", &self[11])
            .finish()
    }
}

impl VersorEvenOrthogonalOrigin {
    pub const LEN: usize = 12;
}

impl VersorEvenOrthogonalOrigin {
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

impl PartialOrd for VersorEvenOrthogonalOrigin {
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
impl Ord for VersorEvenOrthogonalOrigin {
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
impl PartialEq for VersorEvenOrthogonalOrigin {
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
impl Eq for VersorEvenOrthogonalOrigin {}
impl std::hash::Hash for VersorEvenOrthogonalOrigin {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for i in 0..Self::LEN {
            self[i].to_bits().hash(state);
        }
    }
}

impl std::ops::Index<crate::elements::e423> for VersorEvenOrthogonalOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e423) -> &Self::Output {
        &self[0]
    }
}
impl std::ops::Index<crate::elements::e431> for VersorEvenOrthogonalOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e431) -> &Self::Output {
        &self[1]
    }
}
impl std::ops::Index<crate::elements::e412> for VersorEvenOrthogonalOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e412) -> &Self::Output {
        &self[2]
    }
}
impl std::ops::Index<crate::elements::e321> for VersorEvenOrthogonalOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e321) -> &Self::Output {
        &self[3]
    }
}
impl std::ops::Index<crate::elements::e235> for VersorEvenOrthogonalOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e235) -> &Self::Output {
        &self[4]
    }
}
impl std::ops::Index<crate::elements::e315> for VersorEvenOrthogonalOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e315) -> &Self::Output {
        &self[5]
    }
}
impl std::ops::Index<crate::elements::e125> for VersorEvenOrthogonalOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e125) -> &Self::Output {
        &self[6]
    }
}
impl std::ops::Index<crate::elements::e5> for VersorEvenOrthogonalOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e5) -> &Self::Output {
        &self[7]
    }
}
impl std::ops::Index<crate::elements::e1> for VersorEvenOrthogonalOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e1) -> &Self::Output {
        &self[8]
    }
}
impl std::ops::Index<crate::elements::e2> for VersorEvenOrthogonalOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e2) -> &Self::Output {
        &self[9]
    }
}
impl std::ops::Index<crate::elements::e3> for VersorEvenOrthogonalOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e3) -> &Self::Output {
        &self[10]
    }
}
impl std::ops::Index<crate::elements::e4> for VersorEvenOrthogonalOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e4) -> &Self::Output {
        &self[11]
    }
}
impl std::ops::IndexMut<crate::elements::e423> for VersorEvenOrthogonalOrigin {
    fn index_mut(&self, _: crate::elements::e423) -> &mut Self::Output {
        &mut self[0]
    }
}
impl std::ops::IndexMut<crate::elements::e431> for VersorEvenOrthogonalOrigin {
    fn index_mut(&self, _: crate::elements::e431) -> &mut Self::Output {
        &mut self[1]
    }
}
impl std::ops::IndexMut<crate::elements::e412> for VersorEvenOrthogonalOrigin {
    fn index_mut(&self, _: crate::elements::e412) -> &mut Self::Output {
        &mut self[2]
    }
}
impl std::ops::IndexMut<crate::elements::e321> for VersorEvenOrthogonalOrigin {
    fn index_mut(&self, _: crate::elements::e321) -> &mut Self::Output {
        &mut self[3]
    }
}
impl std::ops::IndexMut<crate::elements::e235> for VersorEvenOrthogonalOrigin {
    fn index_mut(&self, _: crate::elements::e235) -> &mut Self::Output {
        &mut self[4]
    }
}
impl std::ops::IndexMut<crate::elements::e315> for VersorEvenOrthogonalOrigin {
    fn index_mut(&self, _: crate::elements::e315) -> &mut Self::Output {
        &mut self[5]
    }
}
impl std::ops::IndexMut<crate::elements::e125> for VersorEvenOrthogonalOrigin {
    fn index_mut(&self, _: crate::elements::e125) -> &mut Self::Output {
        &mut self[6]
    }
}
impl std::ops::IndexMut<crate::elements::e5> for VersorEvenOrthogonalOrigin {
    fn index_mut(&self, _: crate::elements::e5) -> &mut Self::Output {
        &mut self[7]
    }
}
impl std::ops::IndexMut<crate::elements::e1> for VersorEvenOrthogonalOrigin {
    fn index_mut(&self, _: crate::elements::e1) -> &mut Self::Output {
        &mut self[8]
    }
}
impl std::ops::IndexMut<crate::elements::e2> for VersorEvenOrthogonalOrigin {
    fn index_mut(&self, _: crate::elements::e2) -> &mut Self::Output {
        &mut self[9]
    }
}
impl std::ops::IndexMut<crate::elements::e3> for VersorEvenOrthogonalOrigin {
    fn index_mut(&self, _: crate::elements::e3) -> &mut Self::Output {
        &mut self[10]
    }
}
impl std::ops::IndexMut<crate::elements::e4> for VersorEvenOrthogonalOrigin {
    fn index_mut(&self, _: crate::elements::e4) -> &mut Self::Output {
        &mut self[11]
    }
}
include!("./impls/versor_even_orthogonal_origin.rs");
