use crate::data::*;
use crate::simd::*;

/// MysteryVersorSphere.
/// TODO this is currently a mystery I'm investigating
#[derive(Clone, Copy, nearly::NearlyEq, nearly::NearlyOrd, bytemuck::Pod, bytemuck::Zeroable, encase::ShaderType, serde::Serialize, serde::Deserialize)]
pub union MysteryVersorSphere {
    groups: MysteryVersorSphereGroups,
    /// e4235, e4315, e4125, scalar
    elements: [f32; 4],
}
#[derive(Clone, Copy, nearly::NearlyEq, nearly::NearlyOrd, bytemuck::Pod, bytemuck::Zeroable, encase::ShaderType, serde::Serialize, serde::Deserialize)]
pub struct MysteryVersorSphereGroups {
    /// e4235, e4315, e4125, scalar
    g0: Simd32x4,
}
impl MysteryVersorSphere {
    #[allow(clippy::too_many_arguments)]
    pub const fn from_elements(e4235: f32, e4315: f32, e4125: f32, scalar: f32) -> Self {
        Self {
            elements: [e4235, e4315, e4125, scalar],
        }
    }
    pub const fn from_groups(g0: Simd32x4) -> Self {
        Self {
            groups: MysteryVersorSphereGroups { g0 },
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
const MYSTERY_VERSOR_SPHERE_INDEX_REMAP: [usize; 4] = [0, 1, 2, 3];
impl std::ops::Index<usize> for MysteryVersorSphere {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[MYSTERY_VERSOR_SPHERE_INDEX_REMAP[index]] }
    }
}
impl std::ops::IndexMut<usize> for MysteryVersorSphere {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[MYSTERY_VERSOR_SPHERE_INDEX_REMAP[index]] }
    }
}
impl From<MysteryVersorSphere> for [f32; 4] {
    fn from(vector: MysteryVersorSphere) -> Self {
        unsafe { [vector.elements[0], vector.elements[1], vector.elements[2], vector.elements[3]] }
    }
}
impl From<[f32; 4]> for MysteryVersorSphere {
    fn from(array: [f32; 4]) -> Self {
        Self {
            elements: [array[0], array[1], array[2], array[3]],
        }
    }
}
impl std::fmt::Debug for MysteryVersorSphere {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .debug_struct("MysteryVersorSphere")
            .field("e4235", &self[0])
            .field("e4315", &self[1])
            .field("e4125", &self[2])
            .field("scalar", &self[3])
            .finish()
    }
}

impl MysteryVersorSphere {
    pub const LEN: usize = 4;
}

impl MysteryVersorSphere {
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

impl PartialOrd for MysteryVersorSphere {
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
impl Ord for MysteryVersorSphere {
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
impl PartialEq for MysteryVersorSphere {
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
impl Eq for MysteryVersorSphere {}
impl std::hash::Hash for MysteryVersorSphere {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for i in 0..Self::LEN {
            self[i].to_bits().hash(state);
        }
    }
}

impl std::ops::Index<crate::elements::e4235> for MysteryVersorSphere {
    type Output = f32;
    fn index(&self, _: crate::elements::e4235) -> &Self::Output {
        &self[0]
    }
}
impl std::ops::Index<crate::elements::e4315> for MysteryVersorSphere {
    type Output = f32;
    fn index(&self, _: crate::elements::e4315) -> &Self::Output {
        &self[1]
    }
}
impl std::ops::Index<crate::elements::e4125> for MysteryVersorSphere {
    type Output = f32;
    fn index(&self, _: crate::elements::e4125) -> &Self::Output {
        &self[2]
    }
}
impl std::ops::Index<crate::elements::scalar> for MysteryVersorSphere {
    type Output = f32;
    fn index(&self, _: crate::elements::scalar) -> &Self::Output {
        &self[3]
    }
}
impl std::ops::IndexMut<crate::elements::e4235> for MysteryVersorSphere {
    fn index_mut(&self, _: crate::elements::e4235) -> &mut Self::Output {
        &mut self[0]
    }
}
impl std::ops::IndexMut<crate::elements::e4315> for MysteryVersorSphere {
    fn index_mut(&self, _: crate::elements::e4315) -> &mut Self::Output {
        &mut self[1]
    }
}
impl std::ops::IndexMut<crate::elements::e4125> for MysteryVersorSphere {
    fn index_mut(&self, _: crate::elements::e4125) -> &mut Self::Output {
        &mut self[2]
    }
}
impl std::ops::IndexMut<crate::elements::scalar> for MysteryVersorSphere {
    fn index_mut(&self, _: crate::elements::scalar) -> &mut Self::Output {
        &mut self[3]
    }
}
include!("./impls/mystery_versor_sphere.rs");
