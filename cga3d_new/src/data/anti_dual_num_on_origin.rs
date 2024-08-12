use crate::data::*;
use crate::simd::*;

/// AntiDualNumOnOrigin.
/// This variant of MultiVector is the Dual to DualNumOnOrigin. It is common for
/// objects of this type to not intersect the null cone, which also prevents them from
/// projecting onto the horosphere in the usual manner. When this happens, this
/// object has behavioral and operative similarity to a MultiVector,
/// but an imaginary radius, and a spacial presence in the shape of a
/// DualNumOnOrigin with a real radius.
#[derive(Clone, Copy, nearly::NearlyEq, nearly::NearlyOrd, bytemuck::Pod, bytemuck::Zeroable, encase::ShaderType, serde::Serialize, serde::Deserialize)]
pub union AntiDualNumOnOrigin {
    groups: AntiDualNumOnOriginGroups,
    /// scalar, 0, 0, 0
    elements: [f32; 4],
}
#[derive(Clone, Copy, nearly::NearlyEq, nearly::NearlyOrd, bytemuck::Pod, bytemuck::Zeroable, encase::ShaderType, serde::Serialize, serde::Deserialize)]
pub struct AntiDualNumOnOriginGroups {
    /// scalar
    g0: f32,
}
impl AntiDualNumOnOrigin {
    #[allow(clippy::too_many_arguments)]
    pub const fn from_elements(scalar: f32) -> Self {
        Self {
            elements: [scalar, 0.0, 0.0, 0.0],
        }
    }
    pub const fn from_groups(g0: f32) -> Self {
        Self {
            groups: AntiDualNumOnOriginGroups { g0 },
        }
    }
    #[inline(always)]
    pub fn group0(&self) -> f32 {
        unsafe { self.groups.g0 }
    }
    #[inline(always)]
    pub fn group0_mut(&mut self) -> &mut f32 {
        unsafe { &mut self.groups.g0 }
    }
}
const ANTI_DUAL_NUM_ON_ORIGIN_INDEX_REMAP: [usize; 1] = [0];
impl std::ops::Index<usize> for AntiDualNumOnOrigin {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[ANTI_DUAL_NUM_ON_ORIGIN_INDEX_REMAP[index]] }
    }
}
impl std::ops::IndexMut<usize> for AntiDualNumOnOrigin {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[ANTI_DUAL_NUM_ON_ORIGIN_INDEX_REMAP[index]] }
    }
}
impl From<AntiDualNumOnOrigin> for [f32; 1] {
    fn from(vector: AntiDualNumOnOrigin) -> Self {
        unsafe { [vector.elements[0]] }
    }
}
impl From<[f32; 1]> for AntiDualNumOnOrigin {
    fn from(array: [f32; 1]) -> Self {
        Self {
            elements: [array[0], 0.0, 0.0, 0.0],
        }
    }
}
impl std::fmt::Debug for AntiDualNumOnOrigin {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.debug_struct("AntiDualNumOnOrigin").field("scalar", &self[0]).finish()
    }
}

impl AntiDualNumOnOrigin {
    pub const LEN: usize = 1;
}

impl AntiDualNumOnOrigin {
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

impl PartialOrd for AntiDualNumOnOrigin {
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
impl Ord for AntiDualNumOnOrigin {
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
impl PartialEq for AntiDualNumOnOrigin {
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
impl Eq for AntiDualNumOnOrigin {}
impl std::hash::Hash for AntiDualNumOnOrigin {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for i in 0..Self::LEN {
            self[i].to_bits().hash(state);
        }
    }
}

impl std::ops::Index<crate::elements::scalar> for AntiDualNumOnOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::scalar) -> &Self::Output {
        &self[0]
    }
}
impl std::ops::IndexMut<crate::elements::scalar> for AntiDualNumOnOrigin {
    fn index_mut(&self, _: crate::elements::scalar) -> &mut Self::Output {
        &mut self[0]
    }
}
include!("./impls/anti_dual_num_on_origin.rs");
