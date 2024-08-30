use crate::data::*;
use crate::simd::*;

/// AntiFlatPoint
#[derive(Clone, Copy, nearly::NearlyEq, nearly::NearlyOrd, bytemuck::Pod, bytemuck::Zeroable, encase::ShaderType, serde::Serialize, serde::Deserialize)]
pub union AntiFlatPoint {
    groups: AntiFlatPointGroups,
    /// e235, e315, e125, e321
    elements: [f32; 4],
}
#[derive(Clone, Copy, nearly::NearlyEq, nearly::NearlyOrd, bytemuck::Pod, bytemuck::Zeroable, encase::ShaderType, serde::Serialize, serde::Deserialize)]
pub struct AntiFlatPointGroups {
    /// e235, e315, e125, e321
    g0: Simd32x4,
}
impl AntiFlatPoint {
    #[allow(clippy::too_many_arguments)]
    pub const fn from_elements(e235: f32, e315: f32, e125: f32, e321: f32) -> Self {
        Self {
            elements: [e235, e315, e125, e321],
        }
    }
    pub const fn from_groups(g0: Simd32x4) -> Self {
        Self {
            groups: AntiFlatPointGroups { g0 },
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
const ANTI_FLAT_POINT_INDEX_REMAP: [usize; 4] = [0, 1, 2, 3];
impl std::ops::Index<usize> for AntiFlatPoint {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[ANTI_FLAT_POINT_INDEX_REMAP[index]] }
    }
}
impl std::ops::IndexMut<usize> for AntiFlatPoint {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[ANTI_FLAT_POINT_INDEX_REMAP[index]] }
    }
}
impl From<AntiFlatPoint> for [f32; 4] {
    fn from(vector: AntiFlatPoint) -> Self {
        unsafe { [vector.elements[0], vector.elements[1], vector.elements[2], vector.elements[3]] }
    }
}
impl From<[f32; 4]> for AntiFlatPoint {
    fn from(array: [f32; 4]) -> Self {
        Self {
            elements: [array[0], array[1], array[2], array[3]],
        }
    }
}
impl std::fmt::Debug for AntiFlatPoint {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .debug_struct("AntiFlatPoint")
            .field("e235", &self[0])
            .field("e315", &self[1])
            .field("e125", &self[2])
            .field("e321", &self[3])
            .finish()
    }
}

impl AntiFlatPoint {
    pub const LEN: usize = 4;
}

impl AntiFlatPoint {
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

impl PartialOrd for AntiFlatPoint {
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
impl Ord for AntiFlatPoint {
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
impl PartialEq for AntiFlatPoint {
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
impl Eq for AntiFlatPoint {}
impl std::hash::Hash for AntiFlatPoint {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for i in 0..Self::LEN {
            self[i].to_bits().hash(state);
        }
    }
}

impl std::ops::Index<crate::elements::e235> for AntiFlatPoint {
    type Output = f32;
    fn index(&self, _: crate::elements::e235) -> &Self::Output {
        &self[0]
    }
}
impl std::ops::Index<crate::elements::e315> for AntiFlatPoint {
    type Output = f32;
    fn index(&self, _: crate::elements::e315) -> &Self::Output {
        &self[1]
    }
}
impl std::ops::Index<crate::elements::e125> for AntiFlatPoint {
    type Output = f32;
    fn index(&self, _: crate::elements::e125) -> &Self::Output {
        &self[2]
    }
}
impl std::ops::Index<crate::elements::e321> for AntiFlatPoint {
    type Output = f32;
    fn index(&self, _: crate::elements::e321) -> &Self::Output {
        &self[3]
    }
}
impl std::ops::IndexMut<crate::elements::e235> for AntiFlatPoint {
    fn index_mut(&self, _: crate::elements::e235) -> &mut Self::Output {
        &mut self[0]
    }
}
impl std::ops::IndexMut<crate::elements::e315> for AntiFlatPoint {
    fn index_mut(&self, _: crate::elements::e315) -> &mut Self::Output {
        &mut self[1]
    }
}
impl std::ops::IndexMut<crate::elements::e125> for AntiFlatPoint {
    fn index_mut(&self, _: crate::elements::e125) -> &mut Self::Output {
        &mut self[2]
    }
}
impl std::ops::IndexMut<crate::elements::e321> for AntiFlatPoint {
    fn index_mut(&self, _: crate::elements::e321) -> &mut Self::Output {
        &mut self[3]
    }
}
include!("./impls/anti_flat_point.rs");