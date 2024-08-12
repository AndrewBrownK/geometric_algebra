use crate::data::*;
use crate::simd::*;

/// Horizon
#[derive(Clone, Copy, nearly::NearlyEq, nearly::NearlyOrd, bytemuck::Pod, bytemuck::Zeroable, encase::ShaderType, serde::Serialize, serde::Deserialize)]
pub union Horizon {
    groups: HorizonGroups,
    /// e3215, 0, 0, 0
    elements: [f32; 4],
}
#[derive(Clone, Copy, nearly::NearlyEq, nearly::NearlyOrd, bytemuck::Pod, bytemuck::Zeroable, encase::ShaderType, serde::Serialize, serde::Deserialize)]
pub struct HorizonGroups {
    /// e3215
    g0: f32,
}
impl Horizon {
    #[allow(clippy::too_many_arguments)]
    pub const fn from_elements(e3215: f32) -> Self {
        Self { elements: [e3215, 0.0, 0.0, 0.0] }
    }
    pub const fn from_groups(g0: f32) -> Self {
        Self { groups: HorizonGroups { g0 } }
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
const HORIZON_INDEX_REMAP: [usize; 1] = [0];
impl std::ops::Index<usize> for Horizon {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[HORIZON_INDEX_REMAP[index]] }
    }
}
impl std::ops::IndexMut<usize> for Horizon {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[HORIZON_INDEX_REMAP[index]] }
    }
}
impl From<Horizon> for [f32; 1] {
    fn from(vector: Horizon) -> Self {
        unsafe { [vector.elements[0]] }
    }
}
impl From<[f32; 1]> for Horizon {
    fn from(array: [f32; 1]) -> Self {
        Self {
            elements: [array[0], 0.0, 0.0, 0.0],
        }
    }
}
impl std::fmt::Debug for Horizon {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.debug_struct("Horizon").field("e3215", &self[0]).finish()
    }
}

impl Horizon {
    pub const LEN: usize = 1;
}

impl Horizon {
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

impl PartialOrd for Horizon {
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
impl Ord for Horizon {
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
impl PartialEq for Horizon {
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
impl Eq for Horizon {}
impl std::hash::Hash for Horizon {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for i in 0..Self::LEN {
            self[i].to_bits().hash(state);
        }
    }
}

impl std::ops::Index<crate::elements::e3215> for Horizon {
    type Output = f32;
    fn index(&self, _: crate::elements::e3215) -> &Self::Output {
        &self[0]
    }
}
impl std::ops::IndexMut<crate::elements::e3215> for Horizon {
    fn index_mut(&self, _: crate::elements::e3215) -> &mut Self::Output {
        &mut self[0]
    }
}
include!("./impls/horizon.rs");
