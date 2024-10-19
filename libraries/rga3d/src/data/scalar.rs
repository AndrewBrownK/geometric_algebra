use crate::data::*;
use crate::simd::*;

/// Scalar
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Zeroable)]
pub union Scalar {
    groups: ScalarGroups,
    /// scalar, 0, 0, 0
    elements: [f32; 4],
}
#[repr(C)]
#[derive(Clone, Copy, nearly::NearlyEq, nearly::NearlyOrd, bytemuck::Pod, bytemuck::Zeroable, encase::ShaderType, serde::Serialize, serde::Deserialize)]
pub struct ScalarGroups {
    /// scalar
    g0: f32,
}
impl Scalar {
    #[allow(clippy::too_many_arguments)]
    pub const fn from_elements(scalar: f32) -> Self {
        Self {
            elements: [scalar, 0.0, 0.0, 0.0],
        }
    }
    pub const fn from_groups(g0: f32) -> Self {
        Self { groups: ScalarGroups { g0 } }
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
const SCALAR_INDEX_REMAP: [usize; 1] = [0];
impl std::ops::Index<usize> for Scalar {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[SCALAR_INDEX_REMAP[index]] }
    }
}
impl std::ops::IndexMut<usize> for Scalar {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[SCALAR_INDEX_REMAP[index]] }
    }
}
impl From<Scalar> for [f32; 1] {
    fn from(vector: Scalar) -> Self {
        unsafe { [vector.elements[0]] }
    }
}
impl From<[f32; 1]> for Scalar {
    fn from(array: [f32; 1]) -> Self {
        Self {
            elements: [array[0], 0.0, 0.0, 0.0],
        }
    }
}
impl std::fmt::Debug for Scalar {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.debug_struct("Scalar").field("scalar", &self[0]).finish()
    }
}

impl Scalar {
    pub const LEN: usize = 1;
}

impl nearly::EpsTolerance<Scalar> for Scalar {
    type T = f32;
    const DEFAULT: Self::T = <f32 as nearly::EpsTolerance>::DEFAULT;
}
impl nearly::UlpsTolerance<Scalar> for Scalar {
    type T = i32;
    const DEFAULT: Self::T = <f32 as nearly::UlpsTolerance>::DEFAULT;
}
impl nearly::NearlyEqEps<Scalar, Scalar, Scalar> for Scalar {
    fn nearly_eq_eps(&self, other: &Scalar, eps: &nearly::EpsToleranceType<Scalar, Scalar>) -> bool {
        let g = unsafe { &self.groups };
        let other = unsafe { &other.groups };
        return g.nearly_eq_eps(other, eps);
    }
}
impl nearly::NearlyEqUlps<Scalar, Scalar, Scalar> for Scalar {
    fn nearly_eq_ulps(&self, other: &Scalar, ulps: &nearly::UlpsToleranceType<Scalar, Scalar>) -> bool {
        let g = unsafe { &self.groups };
        let other = unsafe { &other.groups };
        return g.nearly_eq_ulps(other, ulps);
    }
}
impl nearly::NearlyEqTol for Scalar {}
impl nearly::NearlyEq for Scalar {}
impl nearly::NearlyOrdUlps<Scalar, Scalar, Scalar> for Scalar {
    fn nearly_lt_ulps(&self, other: &Scalar, ulps: &nearly::UlpsToleranceType<Scalar, Scalar>) -> bool {
        let g = unsafe { &self.groups };
        let other = unsafe { &other.groups };
        return g.nearly_lt_ulps(other, ulps);
    }

    fn nearly_gt_ulps(&self, other: &Scalar, ulps: &nearly::UlpsToleranceType<Scalar, Scalar>) -> bool {
        let g = unsafe { &self.groups };
        let other = unsafe { &other.groups };
        return g.nearly_gt_ulps(other, ulps);
    }
}
impl nearly::NearlyOrdEps<Scalar, Scalar, Scalar> for Scalar {
    fn nearly_lt_eps(&self, other: &Scalar, eps: &nearly::EpsToleranceType<Scalar, Scalar>) -> bool {
        let g = unsafe { &self.groups };
        let other = unsafe { &other.groups };
        return g.nearly_lt_eps(other, eps);
    }

    fn nearly_gt_eps(&self, other: &Scalar, eps: &nearly::EpsToleranceType<Scalar, Scalar>) -> bool {
        let g = unsafe { &self.groups };
        let other = unsafe { &other.groups };
        return g.nearly_gt_eps(other, eps);
    }
}
impl nearly::NearlyOrdTol<Scalar, Scalar, Scalar> for Scalar {}
impl nearly::NearlyOrd for Scalar {}

impl Scalar {
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

impl PartialOrd for Scalar {
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
impl Ord for Scalar {
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
impl PartialEq for Scalar {
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
impl Eq for Scalar {}
impl std::hash::Hash for Scalar {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for i in 0..Self::LEN {
            self[i].to_bits().hash(state);
        }
    }
}

unsafe impl bytemuck::Pod for Scalar {}
impl encase::ShaderType for Scalar {
    type ExtraMetadata = <ScalarGroups as encase::ShaderType>::ExtraMetadata;
    const METADATA: encase::private::Metadata<Self::ExtraMetadata> = <ScalarGroups as encase::ShaderType>::METADATA;
    fn min_size() -> std::num::NonZeroU64 {
        return <ScalarGroups as encase::ShaderType>::min_size();
    }
    fn size(&self) -> std::num::NonZeroU64 {
        return encase::ShaderType::size(unsafe { &self.groups });
    }
    const UNIFORM_COMPAT_ASSERT: fn() = <ScalarGroups as encase::ShaderType>::UNIFORM_COMPAT_ASSERT;
    fn assert_uniform_compat() {
        return <ScalarGroups as encase::ShaderType>::assert_uniform_compat();
    }
}

impl serde::Serialize for Scalar {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let g = unsafe { &self.groups };
        return g.serialize(serializer);
    }
}
impl<'de> serde::Deserialize<'de> for Scalar {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let groups = ScalarGroups::deserialize(deserializer)?;
        return Ok(Scalar { groups });
    }
}
impl std::ops::Index<crate::elements::scalar> for Scalar {
    type Output = f32;
    fn index(&self, _: crate::elements::scalar) -> &Self::Output {
        &self[0]
    }
}
impl std::ops::IndexMut<crate::elements::scalar> for Scalar {
    fn index_mut(&mut self, _: crate::elements::scalar) -> &mut Self::Output {
        &mut self[0]
    }
}
include!("./impls/scalar.rs");
