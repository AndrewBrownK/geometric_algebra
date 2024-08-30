use crate::data::*;
use crate::simd::*;

/// AntiPlane
#[derive(Clone, Copy, nearly::NearlyEq, nearly::NearlyOrd, bytemuck::Pod, bytemuck::Zeroable, encase::ShaderType, serde::Serialize, serde::Deserialize)]
pub union AntiPlane {
    groups: AntiPlaneGroups,
    /// e1, e2, e3, e5
    elements: [f32; 4],
}
#[derive(Clone, Copy, nearly::NearlyEq, nearly::NearlyOrd, bytemuck::Pod, bytemuck::Zeroable, encase::ShaderType, serde::Serialize, serde::Deserialize)]
pub struct AntiPlaneGroups {
    /// e1, e2, e3, e5
    g0: Simd32x4,
}
impl AntiPlane {
    #[allow(clippy::too_many_arguments)]
    pub const fn from_elements(e1: f32, e2: f32, e3: f32, e5: f32) -> Self {
        Self { elements: [e1, e2, e3, e5] }
    }
    pub const fn from_groups(g0: Simd32x4) -> Self {
        Self { groups: AntiPlaneGroups { g0 } }
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
const ANTI_PLANE_INDEX_REMAP: [usize; 4] = [0, 1, 2, 3];
impl std::ops::Index<usize> for AntiPlane {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[ANTI_PLANE_INDEX_REMAP[index]] }
    }
}
impl std::ops::IndexMut<usize> for AntiPlane {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[ANTI_PLANE_INDEX_REMAP[index]] }
    }
}
impl From<AntiPlane> for [f32; 4] {
    fn from(vector: AntiPlane) -> Self {
        unsafe { [vector.elements[0], vector.elements[1], vector.elements[2], vector.elements[3]] }
    }
}
impl From<[f32; 4]> for AntiPlane {
    fn from(array: [f32; 4]) -> Self {
        Self {
            elements: [array[0], array[1], array[2], array[3]],
        }
    }
}
impl std::fmt::Debug for AntiPlane {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .debug_struct("AntiPlane")
            .field("e1", &self[0])
            .field("e2", &self[1])
            .field("e3", &self[2])
            .field("e5", &self[3])
            .finish()
    }
}

impl AntiPlane {
    pub const LEN: usize = 4;
}

impl AntiPlane {
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

impl PartialOrd for AntiPlane {
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
impl Ord for AntiPlane {
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
impl PartialEq for AntiPlane {
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
impl Eq for AntiPlane {}
impl std::hash::Hash for AntiPlane {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for i in 0..Self::LEN {
            self[i].to_bits().hash(state);
        }
    }
}

impl std::ops::Index<crate::elements::e1> for AntiPlane {
    type Output = f32;
    fn index(&self, _: crate::elements::e1) -> &Self::Output {
        &self[0]
    }
}
impl std::ops::Index<crate::elements::e2> for AntiPlane {
    type Output = f32;
    fn index(&self, _: crate::elements::e2) -> &Self::Output {
        &self[1]
    }
}
impl std::ops::Index<crate::elements::e3> for AntiPlane {
    type Output = f32;
    fn index(&self, _: crate::elements::e3) -> &Self::Output {
        &self[2]
    }
}
impl std::ops::Index<crate::elements::e5> for AntiPlane {
    type Output = f32;
    fn index(&self, _: crate::elements::e5) -> &Self::Output {
        &self[3]
    }
}
impl std::ops::IndexMut<crate::elements::e1> for AntiPlane {
    fn index_mut(&self, _: crate::elements::e1) -> &mut Self::Output {
        &mut self[0]
    }
}
impl std::ops::IndexMut<crate::elements::e2> for AntiPlane {
    fn index_mut(&self, _: crate::elements::e2) -> &mut Self::Output {
        &mut self[1]
    }
}
impl std::ops::IndexMut<crate::elements::e3> for AntiPlane {
    fn index_mut(&self, _: crate::elements::e3) -> &mut Self::Output {
        &mut self[2]
    }
}
impl std::ops::IndexMut<crate::elements::e5> for AntiPlane {
    fn index_mut(&self, _: crate::elements::e5) -> &mut Self::Output {
        &mut self[3]
    }
}
include!("./impls/anti_plane.rs");
