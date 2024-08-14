use crate::data::*;
use crate::simd::*;

/// NullVersorOddAtOrigin.
/// This variant of VersorOdd has a radius of zero and is centered on the Origin.
#[derive(Clone, Copy, nearly::NearlyEq, nearly::NearlyOrd, bytemuck::Pod, bytemuck::Zeroable, encase::ShaderType, serde::Serialize, serde::Deserialize)]
pub union NullVersorOddAtOrigin {
    groups: NullVersorOddAtOriginGroups,
    /// e41, e42, e43, e1234
    elements: [f32; 4],
}
#[derive(Clone, Copy, nearly::NearlyEq, nearly::NearlyOrd, bytemuck::Pod, bytemuck::Zeroable, encase::ShaderType, serde::Serialize, serde::Deserialize)]
pub struct NullVersorOddAtOriginGroups {
    /// e41, e42, e43, e1234
    g0: Simd32x4,
}
impl NullVersorOddAtOrigin {
    #[allow(clippy::too_many_arguments)]
    pub const fn from_elements(e41: f32, e42: f32, e43: f32, e1234: f32) -> Self {
        Self { elements: [e41, e42, e43, e1234] }
    }
    pub const fn from_groups(g0: Simd32x4) -> Self {
        Self {
            groups: NullVersorOddAtOriginGroups { g0 },
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
}
const NULL_VERSOR_ODD_AT_ORIGIN_INDEX_REMAP: [usize; 4] = [0, 1, 2, 3];
impl std::ops::Index<usize> for NullVersorOddAtOrigin {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[NULL_VERSOR_ODD_AT_ORIGIN_INDEX_REMAP[index]] }
    }
}
impl std::ops::IndexMut<usize> for NullVersorOddAtOrigin {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[NULL_VERSOR_ODD_AT_ORIGIN_INDEX_REMAP[index]] }
    }
}
impl From<NullVersorOddAtOrigin> for [f32; 4] {
    fn from(vector: NullVersorOddAtOrigin) -> Self {
        unsafe { [vector.elements[0], vector.elements[1], vector.elements[2], vector.elements[3]] }
    }
}
impl From<[f32; 4]> for NullVersorOddAtOrigin {
    fn from(array: [f32; 4]) -> Self {
        Self {
            elements: [array[0], array[1], array[2], array[3]],
        }
    }
}
impl std::fmt::Debug for NullVersorOddAtOrigin {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .debug_struct("NullVersorOddAtOrigin")
            .field("e41", &self[0])
            .field("e42", &self[1])
            .field("e43", &self[2])
            .field("e1234", &self[3])
            .finish()
    }
}

impl NullVersorOddAtOrigin {
    pub const LEN: usize = 4;
}

impl NullVersorOddAtOrigin {
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

impl PartialOrd for NullVersorOddAtOrigin {
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
impl Ord for NullVersorOddAtOrigin {
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
impl PartialEq for NullVersorOddAtOrigin {
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
impl Eq for NullVersorOddAtOrigin {}
impl std::hash::Hash for NullVersorOddAtOrigin {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for i in 0..Self::LEN {
            self[i].to_bits().hash(state);
        }
    }
}

impl std::ops::Index<crate::elements::e41> for NullVersorOddAtOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e41) -> &Self::Output {
        &self[0]
    }
}
impl std::ops::Index<crate::elements::e42> for NullVersorOddAtOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e42) -> &Self::Output {
        &self[1]
    }
}
impl std::ops::Index<crate::elements::e43> for NullVersorOddAtOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e43) -> &Self::Output {
        &self[2]
    }
}
impl std::ops::Index<crate::elements::e1234> for NullVersorOddAtOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e1234) -> &Self::Output {
        &self[3]
    }
}
impl std::ops::IndexMut<crate::elements::e41> for NullVersorOddAtOrigin {
    fn index_mut(&self, _: crate::elements::e41) -> &mut Self::Output {
        &mut self[0]
    }
}
impl std::ops::IndexMut<crate::elements::e42> for NullVersorOddAtOrigin {
    fn index_mut(&self, _: crate::elements::e42) -> &mut Self::Output {
        &mut self[1]
    }
}
impl std::ops::IndexMut<crate::elements::e43> for NullVersorOddAtOrigin {
    fn index_mut(&self, _: crate::elements::e43) -> &mut Self::Output {
        &mut self[2]
    }
}
impl std::ops::IndexMut<crate::elements::e1234> for NullVersorOddAtOrigin {
    fn index_mut(&self, _: crate::elements::e1234) -> &mut Self::Output {
        &mut self[3]
    }
}
include!("./impls/null_versor_odd_at_origin.rs");