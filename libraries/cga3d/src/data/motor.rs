use crate::data::*;
use crate::simd::*;

/// Motor
#[derive(Clone, Copy, nearly::NearlyEq, nearly::NearlyOrd, bytemuck::Pod, bytemuck::Zeroable, encase::ShaderType, serde::Serialize, serde::Deserialize)]
pub union Motor {
    groups: MotorGroups,
    /// e415, e425, e435, e12345, e235, e315, e125, e5
    elements: [f32; 8],
}
#[derive(Clone, Copy, nearly::NearlyEq, nearly::NearlyOrd, bytemuck::Pod, bytemuck::Zeroable, encase::ShaderType, serde::Serialize, serde::Deserialize)]
pub struct MotorGroups {
    /// e415, e425, e435, e12345
    g0: Simd32x4,
    /// e235, e315, e125, e5
    g1: Simd32x4,
}
impl Motor {
    #[allow(clippy::too_many_arguments)]
    pub const fn from_elements(e415: f32, e425: f32, e435: f32, e12345: f32, e235: f32, e315: f32, e125: f32, e5: f32) -> Self {
        Self {
            elements: [e415, e425, e435, e12345, e235, e315, e125, e5],
        }
    }
    pub const fn from_groups(g0: Simd32x4, g1: Simd32x4) -> Self {
        Self { groups: MotorGroups { g0, g1 } }
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
    pub fn group1(&self) -> Simd32x4 {
        unsafe { self.groups.g1 }
    }
    #[inline(always)]
    pub fn group1_mut(&mut self) -> &mut Simd32x4 {
        unsafe { &mut self.groups.g1 }
    }
}
const MOTOR_INDEX_REMAP: [usize; 8] = [0, 1, 2, 3, 4, 5, 6, 7];
impl std::ops::Index<usize> for Motor {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[MOTOR_INDEX_REMAP[index]] }
    }
}
impl std::ops::IndexMut<usize> for Motor {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[MOTOR_INDEX_REMAP[index]] }
    }
}
impl From<Motor> for [f32; 8] {
    fn from(vector: Motor) -> Self {
        unsafe {
            [
                vector.elements[0], vector.elements[1], vector.elements[2], vector.elements[3], vector.elements[4], vector.elements[5], vector.elements[6], vector.elements[7],
            ]
        }
    }
}
impl From<[f32; 8]> for Motor {
    fn from(array: [f32; 8]) -> Self {
        Self {
            elements: [array[0], array[1], array[2], array[3], array[1], array[2], array[3], array[4]],
        }
    }
}
impl std::fmt::Debug for Motor {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .debug_struct("Motor")
            .field("e415", &self[0])
            .field("e425", &self[1])
            .field("e435", &self[2])
            .field("e12345", &self[3])
            .field("e235", &self[4])
            .field("e315", &self[5])
            .field("e125", &self[6])
            .field("e5", &self[7])
            .finish()
    }
}

impl Motor {
    pub const LEN: usize = 8;
}

impl Motor {
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

impl PartialOrd for Motor {
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
impl Ord for Motor {
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
impl PartialEq for Motor {
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
impl Eq for Motor {}
impl std::hash::Hash for Motor {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for i in 0..Self::LEN {
            self[i].to_bits().hash(state);
        }
    }
}

impl std::ops::Index<crate::elements::e415> for Motor {
    type Output = f32;
    fn index(&self, _: crate::elements::e415) -> &Self::Output {
        &self[0]
    }
}
impl std::ops::Index<crate::elements::e425> for Motor {
    type Output = f32;
    fn index(&self, _: crate::elements::e425) -> &Self::Output {
        &self[1]
    }
}
impl std::ops::Index<crate::elements::e435> for Motor {
    type Output = f32;
    fn index(&self, _: crate::elements::e435) -> &Self::Output {
        &self[2]
    }
}
impl std::ops::Index<crate::elements::e12345> for Motor {
    type Output = f32;
    fn index(&self, _: crate::elements::e12345) -> &Self::Output {
        &self[3]
    }
}
impl std::ops::Index<crate::elements::e235> for Motor {
    type Output = f32;
    fn index(&self, _: crate::elements::e235) -> &Self::Output {
        &self[4]
    }
}
impl std::ops::Index<crate::elements::e315> for Motor {
    type Output = f32;
    fn index(&self, _: crate::elements::e315) -> &Self::Output {
        &self[5]
    }
}
impl std::ops::Index<crate::elements::e125> for Motor {
    type Output = f32;
    fn index(&self, _: crate::elements::e125) -> &Self::Output {
        &self[6]
    }
}
impl std::ops::Index<crate::elements::e5> for Motor {
    type Output = f32;
    fn index(&self, _: crate::elements::e5) -> &Self::Output {
        &self[7]
    }
}
impl std::ops::IndexMut<crate::elements::e415> for Motor {
    fn index_mut(&self, _: crate::elements::e415) -> &mut Self::Output {
        &mut self[0]
    }
}
impl std::ops::IndexMut<crate::elements::e425> for Motor {
    fn index_mut(&self, _: crate::elements::e425) -> &mut Self::Output {
        &mut self[1]
    }
}
impl std::ops::IndexMut<crate::elements::e435> for Motor {
    fn index_mut(&self, _: crate::elements::e435) -> &mut Self::Output {
        &mut self[2]
    }
}
impl std::ops::IndexMut<crate::elements::e12345> for Motor {
    fn index_mut(&self, _: crate::elements::e12345) -> &mut Self::Output {
        &mut self[3]
    }
}
impl std::ops::IndexMut<crate::elements::e235> for Motor {
    fn index_mut(&self, _: crate::elements::e235) -> &mut Self::Output {
        &mut self[4]
    }
}
impl std::ops::IndexMut<crate::elements::e315> for Motor {
    fn index_mut(&self, _: crate::elements::e315) -> &mut Self::Output {
        &mut self[5]
    }
}
impl std::ops::IndexMut<crate::elements::e125> for Motor {
    fn index_mut(&self, _: crate::elements::e125) -> &mut Self::Output {
        &mut self[6]
    }
}
impl std::ops::IndexMut<crate::elements::e5> for Motor {
    fn index_mut(&self, _: crate::elements::e5) -> &mut Self::Output {
        &mut self[7]
    }
}
include!("./impls/motor.rs");
