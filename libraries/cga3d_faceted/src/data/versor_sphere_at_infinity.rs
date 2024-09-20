use crate::data::*;
use crate::simd::*;

/// VersorSphereAtInfinity.
/// This variant of VersorSphere exists at the Horizon.
#[derive(Clone, Copy, nearly::NearlyEq, nearly::NearlyOrd, bytemuck::Pod, bytemuck::Zeroable, encase::ShaderType, serde::Serialize, serde::Deserialize)]
pub union VersorSphereAtInfinity {
    groups: VersorSphereAtInfinityGroups,
    /// e4235, e4315, e4125, e3215, scalar, 0, 0, 0
    elements: [f32; 8],
}
#[derive(Clone, Copy, nearly::NearlyEq, nearly::NearlyOrd, bytemuck::Pod, bytemuck::Zeroable, encase::ShaderType, serde::Serialize, serde::Deserialize)]
pub struct VersorSphereAtInfinityGroups {
    /// e4235, e4315, e4125, e3215
    g0: Simd32x4,
    /// scalar
    g1: f32,
}
impl VersorSphereAtInfinity {
    #[allow(clippy::too_many_arguments)]
    pub const fn from_elements(e4235: f32, e4315: f32, e4125: f32, e3215: f32, scalar: f32) -> Self {
        Self {
            elements: [e4235, e4315, e4125, e3215, scalar, 0.0, 0.0, 0.0],
        }
    }
    pub const fn from_groups(g0: Simd32x4, g1: f32) -> Self {
        Self {
            groups: VersorSphereAtInfinityGroups { g0, g1 },
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
    #[inline(always)]
    pub fn group1(&self) -> f32 {
        unsafe { self.groups.g1 }
    }
    #[inline(always)]
    pub fn group1_mut(&mut self) -> &mut f32 {
        unsafe { &mut self.groups.g1 }
    }
}
const VERSOR_SPHERE_AT_INFINITY_INDEX_REMAP: [usize; 5] = [0, 1, 2, 3, 4];
impl std::ops::Index<usize> for VersorSphereAtInfinity {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[VERSOR_SPHERE_AT_INFINITY_INDEX_REMAP[index]] }
    }
}
impl std::ops::IndexMut<usize> for VersorSphereAtInfinity {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[VERSOR_SPHERE_AT_INFINITY_INDEX_REMAP[index]] }
    }
}
impl From<VersorSphereAtInfinity> for [f32; 5] {
    fn from(vector: VersorSphereAtInfinity) -> Self {
        unsafe { [vector.elements[0], vector.elements[1], vector.elements[2], vector.elements[3], vector.elements[4]] }
    }
}
impl From<[f32; 5]> for VersorSphereAtInfinity {
    fn from(array: [f32; 5]) -> Self {
        Self {
            elements: [array[0], array[1], array[2], array[3], array[1], 0.0, 0.0, 0.0],
        }
    }
}
impl std::fmt::Debug for VersorSphereAtInfinity {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .debug_struct("VersorSphereAtInfinity")
            .field("e4235", &self[0])
            .field("e4315", &self[1])
            .field("e4125", &self[2])
            .field("e3215", &self[3])
            .field("scalar", &self[4])
            .finish()
    }
}

impl VersorSphereAtInfinity {
    pub const LEN: usize = 5;
}

impl VersorSphereAtInfinity {
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

impl PartialOrd for VersorSphereAtInfinity {
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
impl Ord for VersorSphereAtInfinity {
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
impl PartialEq for VersorSphereAtInfinity {
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
impl Eq for VersorSphereAtInfinity {}
impl std::hash::Hash for VersorSphereAtInfinity {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for i in 0..Self::LEN {
            self[i].to_bits().hash(state);
        }
    }
}

impl std::ops::Index<crate::elements::e4235> for VersorSphereAtInfinity {
    type Output = f32;
    fn index(&self, _: crate::elements::e4235) -> &Self::Output {
        &self[0]
    }
}
impl std::ops::Index<crate::elements::e4315> for VersorSphereAtInfinity {
    type Output = f32;
    fn index(&self, _: crate::elements::e4315) -> &Self::Output {
        &self[1]
    }
}
impl std::ops::Index<crate::elements::e4125> for VersorSphereAtInfinity {
    type Output = f32;
    fn index(&self, _: crate::elements::e4125) -> &Self::Output {
        &self[2]
    }
}
impl std::ops::Index<crate::elements::e3215> for VersorSphereAtInfinity {
    type Output = f32;
    fn index(&self, _: crate::elements::e3215) -> &Self::Output {
        &self[3]
    }
}
impl std::ops::Index<crate::elements::scalar> for VersorSphereAtInfinity {
    type Output = f32;
    fn index(&self, _: crate::elements::scalar) -> &Self::Output {
        &self[4]
    }
}
impl std::ops::IndexMut<crate::elements::e4235> for VersorSphereAtInfinity {
    fn index_mut(&self, _: crate::elements::e4235) -> &mut Self::Output {
        &mut self[0]
    }
}
impl std::ops::IndexMut<crate::elements::e4315> for VersorSphereAtInfinity {
    fn index_mut(&self, _: crate::elements::e4315) -> &mut Self::Output {
        &mut self[1]
    }
}
impl std::ops::IndexMut<crate::elements::e4125> for VersorSphereAtInfinity {
    fn index_mut(&self, _: crate::elements::e4125) -> &mut Self::Output {
        &mut self[2]
    }
}
impl std::ops::IndexMut<crate::elements::e3215> for VersorSphereAtInfinity {
    fn index_mut(&self, _: crate::elements::e3215) -> &mut Self::Output {
        &mut self[3]
    }
}
impl std::ops::IndexMut<crate::elements::scalar> for VersorSphereAtInfinity {
    fn index_mut(&self, _: crate::elements::scalar) -> &mut Self::Output {
        &mut self[4]
    }
}
include!("./impls/versor_sphere_at_infinity.rs");
