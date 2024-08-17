use crate::data::*;
use crate::simd::*;

/// AntiCircleRotorAligningOriginAtInfinity.
/// This variant of VersorOddOrthogonalOrigin is the Dual to CircleRotorAligningOriginAtInfinity. It is common for
/// objects of this type to not intersect the null cone, which also prevents them from
/// projecting onto the horosphere in the usual manner. When this happens, this
/// object has behavioral and operative similarity to a VersorOddOrthogonalOrigin,
/// but an imaginary radius, and a spacial presence in the shape of a
/// CircleRotorAligningOriginAtInfinity with a real radius.
#[derive(Clone, Copy, nearly::NearlyEq, nearly::NearlyOrd, bytemuck::Pod, bytemuck::Zeroable, encase::ShaderType, serde::Serialize, serde::Deserialize)]
pub union AntiCircleRotorAligningOriginAtInfinity {
    groups: AntiCircleRotorAligningOriginAtInfinityGroups,
    /// e23, e31, e12, 0, e15, e25, e35, scalar
    elements: [f32; 8],
}
#[derive(Clone, Copy, nearly::NearlyEq, nearly::NearlyOrd, bytemuck::Pod, bytemuck::Zeroable, encase::ShaderType, serde::Serialize, serde::Deserialize)]
pub struct AntiCircleRotorAligningOriginAtInfinityGroups {
    /// e23, e31, e12
    g0: Simd32x3,
    /// e15, e25, e35, scalar
    g1: Simd32x4,
}
impl AntiCircleRotorAligningOriginAtInfinity {
    #[allow(clippy::too_many_arguments)]
    pub const fn from_elements(e23: f32, e31: f32, e12: f32, e15: f32, e25: f32, e35: f32, scalar: f32) -> Self {
        Self {
            elements: [e23, e31, e12, 0.0, e15, e25, e35, scalar],
        }
    }
    pub const fn from_groups(g0: Simd32x3, g1: Simd32x4) -> Self {
        Self {
            groups: AntiCircleRotorAligningOriginAtInfinityGroups { g0, g1 },
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
    pub fn group1(&self) -> Simd32x4 {
        unsafe { self.groups.g1 }
    }
    #[inline(always)]
    pub fn group1_mut(&mut self) -> &mut Simd32x4 {
        unsafe { &mut self.groups.g1 }
    }
}
const ANTI_CIRCLE_ROTOR_ALIGNING_ORIGIN_AT_INFINITY_INDEX_REMAP: [usize; 7] = [0, 1, 2, 4, 5, 6, 7];
impl std::ops::Index<usize> for AntiCircleRotorAligningOriginAtInfinity {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[ANTI_CIRCLE_ROTOR_ALIGNING_ORIGIN_AT_INFINITY_INDEX_REMAP[index]] }
    }
}
impl std::ops::IndexMut<usize> for AntiCircleRotorAligningOriginAtInfinity {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[ANTI_CIRCLE_ROTOR_ALIGNING_ORIGIN_AT_INFINITY_INDEX_REMAP[index]] }
    }
}
impl From<AntiCircleRotorAligningOriginAtInfinity> for [f32; 7] {
    fn from(vector: AntiCircleRotorAligningOriginAtInfinity) -> Self {
        unsafe {
            [
                vector.elements[0], vector.elements[1], vector.elements[2], vector.elements[4], vector.elements[5], vector.elements[6], vector.elements[7],
            ]
        }
    }
}
impl From<[f32; 7]> for AntiCircleRotorAligningOriginAtInfinity {
    fn from(array: [f32; 7]) -> Self {
        Self {
            elements: [array[0], array[1], array[2], 0.0, array[1], array[2], array[3], array[4]],
        }
    }
}
impl std::fmt::Debug for AntiCircleRotorAligningOriginAtInfinity {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .debug_struct("AntiCircleRotorAligningOriginAtInfinity")
            .field("e23", &self[0])
            .field("e31", &self[1])
            .field("e12", &self[2])
            .field("e15", &self[3])
            .field("e25", &self[4])
            .field("e35", &self[5])
            .field("scalar", &self[6])
            .finish()
    }
}

impl AntiCircleRotorAligningOriginAtInfinity {
    pub const LEN: usize = 7;
}

impl AntiCircleRotorAligningOriginAtInfinity {
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

impl PartialOrd for AntiCircleRotorAligningOriginAtInfinity {
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
impl Ord for AntiCircleRotorAligningOriginAtInfinity {
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
impl PartialEq for AntiCircleRotorAligningOriginAtInfinity {
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
impl Eq for AntiCircleRotorAligningOriginAtInfinity {}
impl std::hash::Hash for AntiCircleRotorAligningOriginAtInfinity {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for i in 0..Self::LEN {
            self[i].to_bits().hash(state);
        }
    }
}

impl std::ops::Index<crate::elements::e23> for AntiCircleRotorAligningOriginAtInfinity {
    type Output = f32;
    fn index(&self, _: crate::elements::e23) -> &Self::Output {
        &self[0]
    }
}
impl std::ops::Index<crate::elements::e31> for AntiCircleRotorAligningOriginAtInfinity {
    type Output = f32;
    fn index(&self, _: crate::elements::e31) -> &Self::Output {
        &self[1]
    }
}
impl std::ops::Index<crate::elements::e12> for AntiCircleRotorAligningOriginAtInfinity {
    type Output = f32;
    fn index(&self, _: crate::elements::e12) -> &Self::Output {
        &self[2]
    }
}
impl std::ops::Index<crate::elements::e15> for AntiCircleRotorAligningOriginAtInfinity {
    type Output = f32;
    fn index(&self, _: crate::elements::e15) -> &Self::Output {
        &self[3]
    }
}
impl std::ops::Index<crate::elements::e25> for AntiCircleRotorAligningOriginAtInfinity {
    type Output = f32;
    fn index(&self, _: crate::elements::e25) -> &Self::Output {
        &self[4]
    }
}
impl std::ops::Index<crate::elements::e35> for AntiCircleRotorAligningOriginAtInfinity {
    type Output = f32;
    fn index(&self, _: crate::elements::e35) -> &Self::Output {
        &self[5]
    }
}
impl std::ops::Index<crate::elements::scalar> for AntiCircleRotorAligningOriginAtInfinity {
    type Output = f32;
    fn index(&self, _: crate::elements::scalar) -> &Self::Output {
        &self[6]
    }
}
impl std::ops::IndexMut<crate::elements::e23> for AntiCircleRotorAligningOriginAtInfinity {
    fn index_mut(&self, _: crate::elements::e23) -> &mut Self::Output {
        &mut self[0]
    }
}
impl std::ops::IndexMut<crate::elements::e31> for AntiCircleRotorAligningOriginAtInfinity {
    fn index_mut(&self, _: crate::elements::e31) -> &mut Self::Output {
        &mut self[1]
    }
}
impl std::ops::IndexMut<crate::elements::e12> for AntiCircleRotorAligningOriginAtInfinity {
    fn index_mut(&self, _: crate::elements::e12) -> &mut Self::Output {
        &mut self[2]
    }
}
impl std::ops::IndexMut<crate::elements::e15> for AntiCircleRotorAligningOriginAtInfinity {
    fn index_mut(&self, _: crate::elements::e15) -> &mut Self::Output {
        &mut self[3]
    }
}
impl std::ops::IndexMut<crate::elements::e25> for AntiCircleRotorAligningOriginAtInfinity {
    fn index_mut(&self, _: crate::elements::e25) -> &mut Self::Output {
        &mut self[4]
    }
}
impl std::ops::IndexMut<crate::elements::e35> for AntiCircleRotorAligningOriginAtInfinity {
    fn index_mut(&self, _: crate::elements::e35) -> &mut Self::Output {
        &mut self[5]
    }
}
impl std::ops::IndexMut<crate::elements::scalar> for AntiCircleRotorAligningOriginAtInfinity {
    fn index_mut(&self, _: crate::elements::scalar) -> &mut Self::Output {
        &mut self[6]
    }
}
include!("./impls/anti_circle_rotor_aligning_origin_at_infinity.rs");
