use crate::data::*;
use crate::simd::*;

/// Plane
#[derive(Clone, Copy, nearly::NearlyEq, nearly::NearlyOrd, bytemuck::Pod, bytemuck::Zeroable, encase::ShaderType, serde::Serialize, serde::Deserialize)]
pub union Plane {
    groups: PlaneGroups,
    /// e4235, e4315, e4125, e3215
    elements: [f32; 4],
}
#[derive(Clone, Copy, nearly::NearlyEq, nearly::NearlyOrd, bytemuck::Pod, bytemuck::Zeroable, encase::ShaderType, serde::Serialize, serde::Deserialize)]
pub struct PlaneGroups {
    /// e4235, e4315, e4125, e3215
    g0: Simd32x4,
}
impl Plane {
    #[allow(clippy::too_many_arguments)]
    pub const fn from_elements(e4235: f32, e4315: f32, e4125: f32, e3215: f32) -> Self {
        Self {
            elements: [e4235, e4315, e4125, e3215],
        }
    }
    pub const fn from_groups(g0: Simd32x4) -> Self {
        Self { groups: PlaneGroups { g0 } }
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
const PLANE_INDEX_REMAP: [usize; 4] = [0, 1, 2, 3];
impl std::ops::Index<usize> for Plane {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[PLANE_INDEX_REMAP[index]] }
    }
}
impl std::ops::IndexMut<usize> for Plane {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[PLANE_INDEX_REMAP[index]] }
    }
}
impl From<Plane> for [f32; 4] {
    fn from(vector: Plane) -> Self {
        unsafe { [vector.elements[0], vector.elements[1], vector.elements[2], vector.elements[3]] }
    }
}
impl From<[f32; 4]> for Plane {
    fn from(array: [f32; 4]) -> Self {
        Self {
            elements: [array[0], array[1], array[2], array[3]],
        }
    }
}
impl std::fmt::Debug for Plane {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .debug_struct("Plane")
            .field("e4235", &self[0])
            .field("e4315", &self[1])
            .field("e4125", &self[2])
            .field("e3215", &self[3])
            .finish()
    }
}

impl Plane {
    pub const LEN: usize = 4;
}

impl Plane {
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

impl PartialOrd for Plane {
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
impl Ord for Plane {
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
impl PartialEq for Plane {
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
impl Eq for Plane {}
impl std::hash::Hash for Plane {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for i in 0..Self::LEN {
            self[i].to_bits().hash(state);
        }
    }
}

impl std::ops::Index<crate::elements::e4235> for Plane {
    type Output = f32;
    fn index(&self, _: crate::elements::e4235) -> &Self::Output {
        &self[0]
    }
}
impl std::ops::Index<crate::elements::e4315> for Plane {
    type Output = f32;
    fn index(&self, _: crate::elements::e4315) -> &Self::Output {
        &self[1]
    }
}
impl std::ops::Index<crate::elements::e4125> for Plane {
    type Output = f32;
    fn index(&self, _: crate::elements::e4125) -> &Self::Output {
        &self[2]
    }
}
impl std::ops::Index<crate::elements::e3215> for Plane {
    type Output = f32;
    fn index(&self, _: crate::elements::e3215) -> &Self::Output {
        &self[3]
    }
}
impl std::ops::IndexMut<crate::elements::e4235> for Plane {
    fn index_mut(&self, _: crate::elements::e4235) -> &mut Self::Output {
        &mut self[0]
    }
}
impl std::ops::IndexMut<crate::elements::e4315> for Plane {
    fn index_mut(&self, _: crate::elements::e4315) -> &mut Self::Output {
        &mut self[1]
    }
}
impl std::ops::IndexMut<crate::elements::e4125> for Plane {
    fn index_mut(&self, _: crate::elements::e4125) -> &mut Self::Output {
        &mut self[2]
    }
}
impl std::ops::IndexMut<crate::elements::e3215> for Plane {
    fn index_mut(&self, _: crate::elements::e3215) -> &mut Self::Output {
        &mut self[3]
    }
}
include!("./impls/plane.rs");
