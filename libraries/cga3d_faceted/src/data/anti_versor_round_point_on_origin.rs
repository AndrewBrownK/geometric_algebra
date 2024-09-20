use crate::data::*;
use crate::simd::*;

/// AntiVersorRoundPointOnOrigin.
/// This variant of VersorSphereOrthogonalOrigin is the Dual to VersorRoundPointOnOrigin. It is common for
/// objects of this type to not intersect the null cone, which also prevents them from
/// projecting onto the horosphere in the usual manner. When this happens, this
/// object has behavioral and operative similarity to a VersorSphereOrthogonalOrigin,
/// but an imaginary radius, and a spacial presence in the shape of a
/// VersorRoundPointOnOrigin with a real radius.
#[derive(Clone, Copy, nearly::NearlyEq, nearly::NearlyOrd, bytemuck::Pod, bytemuck::Zeroable, encase::ShaderType, serde::Serialize, serde::Deserialize)]
pub union AntiVersorRoundPointOnOrigin {
    groups: AntiVersorRoundPointOnOriginGroups,
    /// e1234, scalar, 0, 0
    elements: [f32; 4],
}
#[derive(Clone, Copy, nearly::NearlyEq, nearly::NearlyOrd, bytemuck::Pod, bytemuck::Zeroable, encase::ShaderType, serde::Serialize, serde::Deserialize)]
pub struct AntiVersorRoundPointOnOriginGroups {
    /// e1234, scalar
    g0: Simd32x2,
}
impl AntiVersorRoundPointOnOrigin {
    #[allow(clippy::too_many_arguments)]
    pub const fn from_elements(e1234: f32, scalar: f32) -> Self {
        Self {
            elements: [e1234, scalar, 0.0, 0.0],
        }
    }
    pub const fn from_groups(g0: Simd32x2) -> Self {
        Self {
            groups: AntiVersorRoundPointOnOriginGroups { g0 },
        }
    }
    #[inline(always)]
    pub fn group0(&self) -> Simd32x2 {
        unsafe { self.groups.g0 }
    }
    #[inline(always)]
    pub fn group0_mut(&mut self) -> &mut Simd32x2 {
        unsafe { &mut self.groups.g0 }
    }
}
const ANTI_VERSOR_ROUND_POINT_ON_ORIGIN_INDEX_REMAP: [usize; 2] = [0, 1];
impl std::ops::Index<usize> for AntiVersorRoundPointOnOrigin {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[ANTI_VERSOR_ROUND_POINT_ON_ORIGIN_INDEX_REMAP[index]] }
    }
}
impl std::ops::IndexMut<usize> for AntiVersorRoundPointOnOrigin {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[ANTI_VERSOR_ROUND_POINT_ON_ORIGIN_INDEX_REMAP[index]] }
    }
}
impl From<AntiVersorRoundPointOnOrigin> for [f32; 2] {
    fn from(vector: AntiVersorRoundPointOnOrigin) -> Self {
        unsafe { [vector.elements[0], vector.elements[1]] }
    }
}
impl From<[f32; 2]> for AntiVersorRoundPointOnOrigin {
    fn from(array: [f32; 2]) -> Self {
        Self {
            elements: [array[0], array[1], 0.0, 0.0],
        }
    }
}
impl std::fmt::Debug for AntiVersorRoundPointOnOrigin {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.debug_struct("AntiVersorRoundPointOnOrigin").field("e1234", &self[0]).field("scalar", &self[1]).finish()
    }
}

impl AntiVersorRoundPointOnOrigin {
    pub const LEN: usize = 2;
}

impl AntiVersorRoundPointOnOrigin {
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

impl PartialOrd for AntiVersorRoundPointOnOrigin {
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
impl Ord for AntiVersorRoundPointOnOrigin {
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
impl PartialEq for AntiVersorRoundPointOnOrigin {
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
impl Eq for AntiVersorRoundPointOnOrigin {}
impl std::hash::Hash for AntiVersorRoundPointOnOrigin {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for i in 0..Self::LEN {
            self[i].to_bits().hash(state);
        }
    }
}

impl std::ops::Index<crate::elements::e1234> for AntiVersorRoundPointOnOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e1234) -> &Self::Output {
        &self[0]
    }
}
impl std::ops::Index<crate::elements::scalar> for AntiVersorRoundPointOnOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::scalar) -> &Self::Output {
        &self[1]
    }
}
impl std::ops::IndexMut<crate::elements::e1234> for AntiVersorRoundPointOnOrigin {
    fn index_mut(&self, _: crate::elements::e1234) -> &mut Self::Output {
        &mut self[0]
    }
}
impl std::ops::IndexMut<crate::elements::scalar> for AntiVersorRoundPointOnOrigin {
    fn index_mut(&self, _: crate::elements::scalar) -> &mut Self::Output {
        &mut self[1]
    }
}
include!("./impls/anti_versor_round_point_on_origin.rs");
