use crate::data::*;
use crate::simd::*;

/// AntiDipoleOnOrigin.
/// This variant of CircleOrthogonalOrigin is the Dual to DipoleOnOrigin. It is common for
/// objects of this type to not intersect the null cone, which also prevents them from
/// projecting onto the horosphere in the usual manner. When this happens, this
/// object has behavioral and operative similarity to a CircleOrthogonalOrigin,
/// but an imaginary radius, and a spacial presence in the shape of a
/// DipoleOnOrigin with a real radius.
#[derive(Clone, Copy, nearly::NearlyEq, nearly::NearlyOrd, bytemuck::Pod, bytemuck::Zeroable, encase::ShaderType, serde::Serialize, serde::Deserialize)]
pub union AntiDipoleOnOrigin {
    groups: AntiDipoleOnOriginGroups,
    /// e423, e431, e412, 0, e321, 0, 0, 0
    elements: [f32; 8],
}
#[derive(Clone, Copy, nearly::NearlyEq, nearly::NearlyOrd, bytemuck::Pod, bytemuck::Zeroable, encase::ShaderType, serde::Serialize, serde::Deserialize)]
pub struct AntiDipoleOnOriginGroups {
    /// e423, e431, e412
    g0: Simd32x3,
    /// e321
    g1: f32,
}
impl AntiDipoleOnOrigin {
    #[allow(clippy::too_many_arguments)]
    pub const fn from_elements(e423: f32, e431: f32, e412: f32, e321: f32) -> Self {
        Self {
            elements: [e423, e431, e412, 0.0, e321, 0.0, 0.0, 0.0],
        }
    }
    pub const fn from_groups(g0: Simd32x3, g1: f32) -> Self {
        Self {
            groups: AntiDipoleOnOriginGroups { g0, g1 },
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
    pub fn group1(&self) -> f32 {
        unsafe { self.groups.g1 }
    }
    #[inline(always)]
    pub fn group1_mut(&mut self) -> &mut f32 {
        unsafe { &mut self.groups.g1 }
    }
}
const ANTI_DIPOLE_ON_ORIGIN_INDEX_REMAP: [usize; 4] = [0, 1, 2, 4];
impl std::ops::Index<usize> for AntiDipoleOnOrigin {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[ANTI_DIPOLE_ON_ORIGIN_INDEX_REMAP[index]] }
    }
}
impl std::ops::IndexMut<usize> for AntiDipoleOnOrigin {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[ANTI_DIPOLE_ON_ORIGIN_INDEX_REMAP[index]] }
    }
}
impl From<AntiDipoleOnOrigin> for [f32; 4] {
    fn from(vector: AntiDipoleOnOrigin) -> Self {
        unsafe { [vector.elements[0], vector.elements[1], vector.elements[2], vector.elements[4]] }
    }
}
impl From<[f32; 4]> for AntiDipoleOnOrigin {
    fn from(array: [f32; 4]) -> Self {
        Self {
            elements: [array[0], array[1], array[2], 0.0, array[1], 0.0, 0.0, 0.0],
        }
    }
}
impl std::fmt::Debug for AntiDipoleOnOrigin {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .debug_struct("AntiDipoleOnOrigin")
            .field("e423", &self[0])
            .field("e431", &self[1])
            .field("e412", &self[2])
            .field("e321", &self[3])
            .finish()
    }
}

impl AntiDipoleOnOrigin {
    pub const LEN: usize = 4;
}

impl AntiDipoleOnOrigin {
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

impl PartialOrd for AntiDipoleOnOrigin {
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
impl Ord for AntiDipoleOnOrigin {
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
impl PartialEq for AntiDipoleOnOrigin {
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
impl Eq for AntiDipoleOnOrigin {}
impl std::hash::Hash for AntiDipoleOnOrigin {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for i in 0..Self::LEN {
            self[i].to_bits().hash(state);
        }
    }
}

impl std::ops::Index<crate::elements::e423> for AntiDipoleOnOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e423) -> &Self::Output {
        &self[0]
    }
}
impl std::ops::Index<crate::elements::e431> for AntiDipoleOnOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e431) -> &Self::Output {
        &self[1]
    }
}
impl std::ops::Index<crate::elements::e412> for AntiDipoleOnOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e412) -> &Self::Output {
        &self[2]
    }
}
impl std::ops::Index<crate::elements::e321> for AntiDipoleOnOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e321) -> &Self::Output {
        &self[3]
    }
}
impl std::ops::IndexMut<crate::elements::e423> for AntiDipoleOnOrigin {
    fn index_mut(&self, _: crate::elements::e423) -> &mut Self::Output {
        &mut self[0]
    }
}
impl std::ops::IndexMut<crate::elements::e431> for AntiDipoleOnOrigin {
    fn index_mut(&self, _: crate::elements::e431) -> &mut Self::Output {
        &mut self[1]
    }
}
impl std::ops::IndexMut<crate::elements::e412> for AntiDipoleOnOrigin {
    fn index_mut(&self, _: crate::elements::e412) -> &mut Self::Output {
        &mut self[2]
    }
}
impl std::ops::IndexMut<crate::elements::e321> for AntiDipoleOnOrigin {
    fn index_mut(&self, _: crate::elements::e321) -> &mut Self::Output {
        &mut self[3]
    }
}
include!("./impls/anti_dipole_on_origin.rs");
