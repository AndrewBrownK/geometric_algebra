use crate::data::*;
use crate::simd::*;

/// LineOnOrigin.
/// This variant of Line intersects the Origin.
#[derive(Clone, Copy, nearly::NearlyEq, nearly::NearlyOrd, bytemuck::Pod, bytemuck::Zeroable, encase::ShaderType, serde::Serialize, serde::Deserialize)]
pub union LineOnOrigin {
    groups: LineOnOriginGroups,
    /// e415, e425, e435, 0
    elements: [f32; 4],
}
#[derive(Clone, Copy, nearly::NearlyEq, nearly::NearlyOrd, bytemuck::Pod, bytemuck::Zeroable, encase::ShaderType, serde::Serialize, serde::Deserialize)]
pub struct LineOnOriginGroups {
    /// e415, e425, e435
    g0: Simd32x3,
}
impl LineOnOrigin {
    #[allow(clippy::too_many_arguments)]
    pub const fn from_elements(e415: f32, e425: f32, e435: f32) -> Self {
        Self {
            elements: [e415, e425, e435, 0.0],
        }
    }
    pub const fn from_groups(g0: Simd32x3) -> Self {
        Self {
            groups: LineOnOriginGroups { g0 },
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
const LINE_ON_ORIGIN_INDEX_REMAP: [usize; 3] = [0, 1, 2];
impl std::ops::Index<usize> for LineOnOrigin {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[LINE_ON_ORIGIN_INDEX_REMAP[index]] }
    }
}
impl std::ops::IndexMut<usize> for LineOnOrigin {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[LINE_ON_ORIGIN_INDEX_REMAP[index]] }
    }
}
impl From<LineOnOrigin> for [f32; 3] {
    fn from(vector: LineOnOrigin) -> Self {
        unsafe { [vector.elements[0], vector.elements[1], vector.elements[2]] }
    }
}
impl From<[f32; 3]> for LineOnOrigin {
    fn from(array: [f32; 3]) -> Self {
        Self {
            elements: [array[0], array[1], array[2], 0.0],
        }
    }
}
impl std::fmt::Debug for LineOnOrigin {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.debug_struct("LineOnOrigin").field("e415", &self[0]).field("e425", &self[1]).field("e435", &self[2]).finish()
    }
}

impl LineOnOrigin {
    pub const LEN: usize = 3;
}

impl LineOnOrigin {
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

impl PartialOrd for LineOnOrigin {
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
impl Ord for LineOnOrigin {
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
impl PartialEq for LineOnOrigin {
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
impl Eq for LineOnOrigin {}
impl std::hash::Hash for LineOnOrigin {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for i in 0..Self::LEN {
            self[i].to_bits().hash(state);
        }
    }
}

impl std::ops::Index<crate::elements::e415> for LineOnOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e415) -> &Self::Output {
        &self[0]
    }
}
impl std::ops::Index<crate::elements::e425> for LineOnOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e425) -> &Self::Output {
        &self[1]
    }
}
impl std::ops::Index<crate::elements::e435> for LineOnOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e435) -> &Self::Output {
        &self[2]
    }
}
impl std::ops::IndexMut<crate::elements::e415> for LineOnOrigin {
    fn index_mut(&self, _: crate::elements::e415) -> &mut Self::Output {
        &mut self[0]
    }
}
impl std::ops::IndexMut<crate::elements::e425> for LineOnOrigin {
    fn index_mut(&self, _: crate::elements::e425) -> &mut Self::Output {
        &mut self[1]
    }
}
impl std::ops::IndexMut<crate::elements::e435> for LineOnOrigin {
    fn index_mut(&self, _: crate::elements::e435) -> &mut Self::Output {
        &mut self[2]
    }
}
include!("./impls/line_on_origin.rs");
