use crate::data::*;
use crate::simd::*;

/// Point
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Zeroable)]
pub union Point {
    groups: PointGroups,
    /// e1, e2, e3, e4
    elements: [f32; 4],
}
#[repr(C)]
#[derive(Clone, Copy, nearly::NearlyEq, nearly::NearlyOrd, bytemuck::Pod, bytemuck::Zeroable, encase::ShaderType, serde::Serialize, serde::Deserialize)]
pub struct PointGroups {
    /// e1, e2, e3, e4
    g0: Simd32x4,
}
impl Point {
    #[allow(clippy::too_many_arguments)]
    pub const fn from_elements(e1: f32, e2: f32, e3: f32, e4: f32) -> Self {
        Self { elements: [e1, e2, e3, e4] }
    }
    pub const fn from_groups(g0: Simd32x4) -> Self {
        Self { groups: PointGroups { g0 } }
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
const POINT_INDEX_REMAP: [usize; 4] = [0, 1, 2, 3];
impl std::ops::Index<usize> for Point {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[POINT_INDEX_REMAP[index]] }
    }
}
impl std::ops::IndexMut<usize> for Point {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[POINT_INDEX_REMAP[index]] }
    }
}
impl From<Point> for [f32; 4] {
    fn from(vector: Point) -> Self {
        unsafe { [vector.elements[0], vector.elements[1], vector.elements[2], vector.elements[3]] }
    }
}
impl From<[f32; 4]> for Point {
    fn from(array: [f32; 4]) -> Self {
        Self {
            elements: [array[0], array[1], array[2], array[3]],
        }
    }
}
impl std::fmt::Debug for Point {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .debug_struct("Point")
            .field("e1", &self[0])
            .field("e2", &self[1])
            .field("e3", &self[2])
            .field("e4", &self[3])
            .finish()
    }
}

impl Point {
    pub const LEN: usize = 4;
}

impl nearly::EpsTolerance<Point> for Point {
    type T = f32;
    const DEFAULT: Self::T = <f32 as nearly::EpsTolerance>::DEFAULT;
}
impl nearly::UlpsTolerance<Point> for Point {
    type T = i32;
    const DEFAULT: Self::T = <f32 as nearly::UlpsTolerance>::DEFAULT;
}
impl nearly::NearlyEqEps<Point, Point, Point> for Point {
    fn nearly_eq_eps(&self, other: &Point, eps: &nearly::EpsToleranceType<Point, Point>) -> bool {
        let g = unsafe { &self.groups };
        let other = unsafe { &other.groups };
        return g.nearly_eq_eps(other, eps);
    }
}
impl nearly::NearlyEqUlps<Point, Point, Point> for Point {
    fn nearly_eq_ulps(&self, other: &Point, ulps: &nearly::UlpsToleranceType<Point, Point>) -> bool {
        let g = unsafe { &self.groups };
        let other = unsafe { &other.groups };
        return g.nearly_eq_ulps(other, ulps);
    }
}
impl nearly::NearlyEqTol for Point {}
impl nearly::NearlyEq for Point {}
impl nearly::NearlyOrdUlps<Point, Point, Point> for Point {
    fn nearly_lt_ulps(&self, other: &Point, ulps: &nearly::UlpsToleranceType<Point, Point>) -> bool {
        let g = unsafe { &self.groups };
        let other = unsafe { &other.groups };
        return g.nearly_lt_ulps(other, ulps);
    }

    fn nearly_gt_ulps(&self, other: &Point, ulps: &nearly::UlpsToleranceType<Point, Point>) -> bool {
        let g = unsafe { &self.groups };
        let other = unsafe { &other.groups };
        return g.nearly_gt_ulps(other, ulps);
    }
}
impl nearly::NearlyOrdEps<Point, Point, Point> for Point {
    fn nearly_lt_eps(&self, other: &Point, eps: &nearly::EpsToleranceType<Point, Point>) -> bool {
        let g = unsafe { &self.groups };
        let other = unsafe { &other.groups };
        return g.nearly_lt_eps(other, eps);
    }

    fn nearly_gt_eps(&self, other: &Point, eps: &nearly::EpsToleranceType<Point, Point>) -> bool {
        let g = unsafe { &self.groups };
        let other = unsafe { &other.groups };
        return g.nearly_gt_eps(other, eps);
    }
}
impl nearly::NearlyOrdTol<Point, Point, Point> for Point {}
impl nearly::NearlyOrd for Point {}

impl Point {
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

impl PartialOrd for Point {
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
impl Ord for Point {
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
impl PartialEq for Point {
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
impl Eq for Point {}
impl std::hash::Hash for Point {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for i in 0..Self::LEN {
            self[i].to_bits().hash(state);
        }
    }
}

unsafe impl bytemuck::Pod for Point {}
impl encase::ShaderType for Point {
    type ExtraMetadata = <PointGroups as encase::ShaderType>::ExtraMetadata;
    const METADATA: encase::private::Metadata<Self::ExtraMetadata> = <PointGroups as encase::ShaderType>::METADATA;
    fn min_size() -> std::num::NonZeroU64 {
        return <PointGroups as encase::ShaderType>::min_size();
    }
    fn size(&self) -> std::num::NonZeroU64 {
        return encase::ShaderType::size(unsafe { &self.groups });
    }
    const UNIFORM_COMPAT_ASSERT: fn() = <PointGroups as encase::ShaderType>::UNIFORM_COMPAT_ASSERT;
    fn assert_uniform_compat() {
        return <PointGroups as encase::ShaderType>::assert_uniform_compat();
    }
}

impl serde::Serialize for Point {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let g = unsafe { &self.groups };
        return g.serialize(serializer);
    }
}
impl<'de> serde::Deserialize<'de> for Point {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let groups = PointGroups::deserialize(deserializer)?;
        return Ok(Point { groups });
    }
}
impl std::ops::Index<crate::elements::e1> for Point {
    type Output = f32;
    fn index(&self, _: crate::elements::e1) -> &Self::Output {
        &self[0]
    }
}
impl std::ops::Index<crate::elements::e2> for Point {
    type Output = f32;
    fn index(&self, _: crate::elements::e2) -> &Self::Output {
        &self[1]
    }
}
impl std::ops::Index<crate::elements::e3> for Point {
    type Output = f32;
    fn index(&self, _: crate::elements::e3) -> &Self::Output {
        &self[2]
    }
}
impl std::ops::Index<crate::elements::e4> for Point {
    type Output = f32;
    fn index(&self, _: crate::elements::e4) -> &Self::Output {
        &self[3]
    }
}
impl std::ops::IndexMut<crate::elements::e1> for Point {
    fn index_mut(&mut self, _: crate::elements::e1) -> &mut Self::Output {
        &mut self[0]
    }
}
impl std::ops::IndexMut<crate::elements::e2> for Point {
    fn index_mut(&mut self, _: crate::elements::e2) -> &mut Self::Output {
        &mut self[1]
    }
}
impl std::ops::IndexMut<crate::elements::e3> for Point {
    fn index_mut(&mut self, _: crate::elements::e3) -> &mut Self::Output {
        &mut self[2]
    }
}
impl std::ops::IndexMut<crate::elements::e4> for Point {
    fn index_mut(&mut self, _: crate::elements::e4) -> &mut Self::Output {
        &mut self[3]
    }
}
include!("./impls/point.rs");
