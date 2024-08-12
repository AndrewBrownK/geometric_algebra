use crate::data::*;
use crate::simd::*;

/// RoundPoint
#[derive(Clone, Copy, nearly::NearlyEq, nearly::NearlyOrd, bytemuck::Pod, bytemuck::Zeroable, encase::ShaderType, serde::Serialize, serde::Deserialize)]
pub union RoundPoint {
    groups: RoundPointGroups,
    /// e1, e2, e3, 0, e4, e5, 0, 0
    elements: [f32; 8],
}
#[derive(Clone, Copy, nearly::NearlyEq, nearly::NearlyOrd, bytemuck::Pod, bytemuck::Zeroable, encase::ShaderType, serde::Serialize, serde::Deserialize)]
pub struct RoundPointGroups {
    /// e1, e2, e3
    g0: Simd32x3,
    /// e4, e5
    g1: Simd32x2,
}
impl RoundPoint {
    #[allow(clippy::too_many_arguments)]
    pub const fn from_elements(e1: f32, e2: f32, e3: f32, e4: f32, e5: f32) -> Self {
        Self {
            elements: [e1, e2, e3, 0.0, e4, e5, 0.0, 0.0],
        }
    }
    pub const fn from_groups(g0: Simd32x3, g1: Simd32x2) -> Self {
        Self {
            groups: RoundPointGroups { g0, g1 },
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
    pub fn group1(&self) -> Simd32x2 {
        unsafe { self.groups.g1 }
    }
    #[inline(always)]
    pub fn group1_mut(&mut self) -> &mut Simd32x2 {
        unsafe { &mut self.groups.g1 }
    }
}
const ROUND_POINT_INDEX_REMAP: [usize; 5] = [0, 1, 2, 4, 5];
impl std::ops::Index<usize> for RoundPoint {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[ROUND_POINT_INDEX_REMAP[index]] }
    }
}
impl std::ops::IndexMut<usize> for RoundPoint {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[ROUND_POINT_INDEX_REMAP[index]] }
    }
}
impl From<RoundPoint> for [f32; 5] {
    fn from(vector: RoundPoint) -> Self {
        unsafe { [vector.elements[0], vector.elements[1], vector.elements[2], vector.elements[4], vector.elements[5]] }
    }
}
impl From<[f32; 5]> for RoundPoint {
    fn from(array: [f32; 5]) -> Self {
        Self {
            elements: [array[0], array[1], array[2], 0.0, array[1], array[2], 0.0, 0.0],
        }
    }
}
impl std::fmt::Debug for RoundPoint {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .debug_struct("RoundPoint")
            .field("e1", &self[0])
            .field("e2", &self[1])
            .field("e3", &self[2])
            .field("e4", &self[3])
            .field("e5", &self[4])
            .finish()
    }
}

impl RoundPoint {
    pub const LEN: usize = 5;
}

impl RoundPoint {
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

impl PartialOrd for RoundPoint {
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
impl Ord for RoundPoint {
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
impl PartialEq for RoundPoint {
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
impl Eq for RoundPoint {}
impl std::hash::Hash for RoundPoint {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for i in 0..Self::LEN {
            self[i].to_bits().hash(state);
        }
    }
}

impl std::ops::Index<crate::elements::e1> for RoundPoint {
    type Output = f32;
    fn index(&self, _: crate::elements::e1) -> &Self::Output {
        &self[0]
    }
}
impl std::ops::Index<crate::elements::e2> for RoundPoint {
    type Output = f32;
    fn index(&self, _: crate::elements::e2) -> &Self::Output {
        &self[1]
    }
}
impl std::ops::Index<crate::elements::e3> for RoundPoint {
    type Output = f32;
    fn index(&self, _: crate::elements::e3) -> &Self::Output {
        &self[2]
    }
}
impl std::ops::Index<crate::elements::e4> for RoundPoint {
    type Output = f32;
    fn index(&self, _: crate::elements::e4) -> &Self::Output {
        &self[3]
    }
}
impl std::ops::Index<crate::elements::e5> for RoundPoint {
    type Output = f32;
    fn index(&self, _: crate::elements::e5) -> &Self::Output {
        &self[4]
    }
}
impl std::ops::IndexMut<crate::elements::e1> for RoundPoint {
    fn index_mut(&self, _: crate::elements::e1) -> &mut Self::Output {
        &mut self[0]
    }
}
impl std::ops::IndexMut<crate::elements::e2> for RoundPoint {
    fn index_mut(&self, _: crate::elements::e2) -> &mut Self::Output {
        &mut self[1]
    }
}
impl std::ops::IndexMut<crate::elements::e3> for RoundPoint {
    fn index_mut(&self, _: crate::elements::e3) -> &mut Self::Output {
        &mut self[2]
    }
}
impl std::ops::IndexMut<crate::elements::e4> for RoundPoint {
    fn index_mut(&self, _: crate::elements::e4) -> &mut Self::Output {
        &mut self[3]
    }
}
impl std::ops::IndexMut<crate::elements::e5> for RoundPoint {
    fn index_mut(&self, _: crate::elements::e5) -> &mut Self::Output {
        &mut self[4]
    }
}
include!("./impls/round_point.rs");
