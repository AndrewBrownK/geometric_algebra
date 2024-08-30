use crate::data::*;
use crate::simd::*;

/// AntiQuadNumAligningOrigin.
/// This variant of VersorOddOrthogonalOrigin is the Dual to QuadNumAligningOrigin. It is common for
/// objects of this type to not intersect the null cone, which also prevents them from
/// projecting onto the horosphere in the usual manner. When this happens, this
/// object has behavioral and operative similarity to a VersorOddOrthogonalOrigin,
/// but an imaginary radius, and a spacial presence in the shape of a
/// QuadNumAligningOrigin with a real radius.
#[derive(Clone, Copy, nearly::NearlyEq, nearly::NearlyOrd, bytemuck::Pod, bytemuck::Zeroable, encase::ShaderType, serde::Serialize, serde::Deserialize)]
pub union AntiQuadNumAligningOrigin {
    groups: AntiQuadNumAligningOriginGroups,
    /// e1234, e3215, scalar, 0
    elements: [f32; 4],
}
#[derive(Clone, Copy, nearly::NearlyEq, nearly::NearlyOrd, bytemuck::Pod, bytemuck::Zeroable, encase::ShaderType, serde::Serialize, serde::Deserialize)]
pub struct AntiQuadNumAligningOriginGroups {
    /// e1234, e3215, scalar
    g0: Simd32x3,
}
impl AntiQuadNumAligningOrigin {
    #[allow(clippy::too_many_arguments)]
    pub const fn from_elements(e1234: f32, e3215: f32, scalar: f32) -> Self {
        Self {
            elements: [e1234, e3215, scalar, 0.0],
        }
    }
    pub const fn from_groups(g0: Simd32x3) -> Self {
        Self {
            groups: AntiQuadNumAligningOriginGroups { g0 },
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
}
const ANTI_QUAD_NUM_ALIGNING_ORIGIN_INDEX_REMAP: [usize; 3] = [0, 1, 2];
impl std::ops::Index<usize> for AntiQuadNumAligningOrigin {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[ANTI_QUAD_NUM_ALIGNING_ORIGIN_INDEX_REMAP[index]] }
    }
}
impl std::ops::IndexMut<usize> for AntiQuadNumAligningOrigin {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[ANTI_QUAD_NUM_ALIGNING_ORIGIN_INDEX_REMAP[index]] }
    }
}
impl From<AntiQuadNumAligningOrigin> for [f32; 3] {
    fn from(vector: AntiQuadNumAligningOrigin) -> Self {
        unsafe { [vector.elements[0], vector.elements[1], vector.elements[2]] }
    }
}
impl From<[f32; 3]> for AntiQuadNumAligningOrigin {
    fn from(array: [f32; 3]) -> Self {
        Self {
            elements: [array[0], array[1], array[2], 0.0],
        }
    }
}
impl std::fmt::Debug for AntiQuadNumAligningOrigin {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .debug_struct("AntiQuadNumAligningOrigin")
            .field("e1234", &self[0])
            .field("e3215", &self[1])
            .field("scalar", &self[2])
            .finish()
    }
}

impl AntiQuadNumAligningOrigin {
    pub const LEN: usize = 3;
}

impl AntiQuadNumAligningOrigin {
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

impl PartialOrd for AntiQuadNumAligningOrigin {
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
impl Ord for AntiQuadNumAligningOrigin {
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
impl PartialEq for AntiQuadNumAligningOrigin {
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
impl Eq for AntiQuadNumAligningOrigin {}
impl std::hash::Hash for AntiQuadNumAligningOrigin {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for i in 0..Self::LEN {
            self[i].to_bits().hash(state);
        }
    }
}

impl std::ops::Index<crate::elements::e1234> for AntiQuadNumAligningOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e1234) -> &Self::Output {
        &self[0]
    }
}
impl std::ops::Index<crate::elements::e3215> for AntiQuadNumAligningOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e3215) -> &Self::Output {
        &self[1]
    }
}
impl std::ops::Index<crate::elements::scalar> for AntiQuadNumAligningOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::scalar) -> &Self::Output {
        &self[2]
    }
}
impl std::ops::IndexMut<crate::elements::e1234> for AntiQuadNumAligningOrigin {
    fn index_mut(&self, _: crate::elements::e1234) -> &mut Self::Output {
        &mut self[0]
    }
}
impl std::ops::IndexMut<crate::elements::e3215> for AntiQuadNumAligningOrigin {
    fn index_mut(&self, _: crate::elements::e3215) -> &mut Self::Output {
        &mut self[1]
    }
}
impl std::ops::IndexMut<crate::elements::scalar> for AntiQuadNumAligningOrigin {
    fn index_mut(&self, _: crate::elements::scalar) -> &mut Self::Output {
        &mut self[2]
    }
}
include!("./impls/anti_quad_num_aligning_origin.rs");