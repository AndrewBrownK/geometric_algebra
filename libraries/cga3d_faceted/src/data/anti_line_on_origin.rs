use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// AntiLineOnOrigin.
/// This variant of MysteryDipole is the Dual to LineOnOrigin. It is common for
/// objects of this type to not intersect the null cone, which also prevents them from
/// projecting onto the horosphere in the usual manner. When this happens, this
/// object has behavioral and operative similarity to a MysteryDipole,
/// but an imaginary radius, and a spacial presence in the shape of a
/// LineOnOrigin with a real radius.
#[repr(C)]
#[derive(Clone, Copy)]
pub union AntiLineOnOrigin {
    groups: AntiLineOnOriginGroups,
    /// e23, e31, e12, 0
    elements: [f32; 4],
}
#[repr(C)]
#[derive(Clone, Copy, encase::ShaderType)]
pub struct AntiLineOnOriginGroups {
    /// e23, e31, e12
    g0: Simd32x3,
}
impl AntiLineOnOrigin {
    #[allow(clippy::too_many_arguments)]
    pub const fn from_elements(e23: f32, e31: f32, e12: f32) -> Self {
        Self { elements: [e23, e31, e12, 0.0] }
    }
    pub const fn from_groups(g0: Simd32x3) -> Self {
        Self {
            groups: AntiLineOnOriginGroups { g0 },
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
}
const ANTI_LINE_ON_ORIGIN_INDEX_REMAP: [usize; 3] = [0, 1, 2];
impl std::ops::Index<usize> for AntiLineOnOrigin {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[ANTI_LINE_ON_ORIGIN_INDEX_REMAP[index]] }
    }
}
impl std::ops::IndexMut<usize> for AntiLineOnOrigin {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[ANTI_LINE_ON_ORIGIN_INDEX_REMAP[index]] }
    }
}
impl From<AntiLineOnOrigin> for [f32; 3] {
    fn from(vector: AntiLineOnOrigin) -> Self {
        unsafe { [vector.elements[0], vector.elements[1], vector.elements[2]] }
    }
}
impl From<[f32; 3]> for AntiLineOnOrigin {
    fn from(array: [f32; 3]) -> Self {
        Self {
            elements: [array[0], array[1], array[2], 0.0],
        }
    }
}
impl std::fmt::Debug for AntiLineOnOrigin {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.debug_struct("AntiLineOnOrigin").field("e23", &self[0]).field("e31", &self[1]).field("e12", &self[2]).finish()
    }
}

impl AntiLineOnOrigin {
    pub const LEN: usize = 3;
}

impl nearly::NearlyEqEps<AntiLineOnOrigin, f32, f32> for AntiLineOnOrigin {
    fn nearly_eq_eps(&self, other: &AntiLineOnOrigin, eps: &nearly::EpsToleranceType<f32, f32>) -> bool {
        let mut i = 0;
        while i < Self::LEN {
            let a = &self[i];
            let b = &other[i];
            if nearly::NearlyEqEps::nearly_ne_eps(a, b, eps) {
                return false;
            }
            i += 1;
        }
        return true;
    }
}
impl nearly::NearlyEqUlps<AntiLineOnOrigin, f32, f32> for AntiLineOnOrigin {
    fn nearly_eq_ulps(&self, other: &AntiLineOnOrigin, ulps: &nearly::UlpsToleranceType<f32, f32>) -> bool {
        let mut i = 0;
        while i < Self::LEN {
            let a = &self[i];
            let b = &other[i];
            if nearly::NearlyEqUlps::nearly_ne_ulps(a, b, ulps) {
                return false;
            }
            i += 1;
        }
        return true;
    }
}
impl nearly::NearlyEqTol<AntiLineOnOrigin, f32, f32> for AntiLineOnOrigin {}
impl nearly::NearlyEq<AntiLineOnOrigin, f32, f32> for AntiLineOnOrigin {}
impl nearly::NearlyOrdUlps<AntiLineOnOrigin, f32, f32> for AntiLineOnOrigin {
    fn nearly_lt_ulps(&self, other: &AntiLineOnOrigin, ulps: &nearly::UlpsToleranceType<f32, f32>) -> bool {
        let mut i = 0;
        while i < Self::LEN {
            let a = &self[i];
            let b = &other[i];
            if nearly::NearlyEqUlps::nearly_eq_ulps(a, b, ulps) {
                // Too close, compare next element
                i += 1;
                continue;
            }
            if a < b {
                // Nearly equal until less-than wins
                return true;
            } else {
                // else greater-than wins
                return false;
            }
        }
        // Nearly equal the whole way
        return false;
    }

    fn nearly_gt_ulps(&self, other: &AntiLineOnOrigin, ulps: &nearly::UlpsToleranceType<f32, f32>) -> bool {
        let mut i = 0;
        while i < Self::LEN {
            let a = &self[i];
            let b = &other[i];
            if nearly::NearlyEqUlps::nearly_eq_ulps(a, b, ulps) {
                // Too close, compare next element
                i += 1;
                continue;
            }
            if a > b {
                // Nearly equal until greater-than wins
                return true;
            } else {
                // else less-than wins
                return false;
            }
        }
        // Nearly equal the whole way
        return false;
    }
}
impl nearly::NearlyOrdEps<AntiLineOnOrigin, f32, f32> for AntiLineOnOrigin {
    fn nearly_lt_eps(&self, other: &AntiLineOnOrigin, eps: &nearly::EpsToleranceType<f32, f32>) -> bool {
        let mut i = 0;
        while i < Self::LEN {
            let a = &self[i];
            let b = &other[i];
            if nearly::NearlyEqEps::nearly_eq_eps(a, b, eps) {
                // Too close, compare next element
                i += 1;
                continue;
            }
            if a < b {
                // Nearly equal until less-than wins
                return true;
            } else {
                // else greater-than wins
                return false;
            }
        }
        // Nearly equal the whole way
        return false;
    }

    fn nearly_gt_eps(&self, other: &AntiLineOnOrigin, eps: &nearly::EpsToleranceType<f32, f32>) -> bool {
        let mut i = 0;
        while i < Self::LEN {
            let a = &self[i];
            let b = &other[i];
            if nearly::NearlyEqEps::nearly_eq_eps(a, b, eps) {
                // Too close, compare next element
                i += 1;
                continue;
            }
            if a > b {
                // Nearly equal until greater-than wins
                return true;
            } else {
                // else less-than wins
                return false;
            }
        }
        // Nearly equal the whole way
        return false;
    }
}
impl nearly::NearlyOrdTol<AntiLineOnOrigin, f32, f32> for AntiLineOnOrigin {}
impl nearly::NearlyOrd<AntiLineOnOrigin, f32, f32> for AntiLineOnOrigin {}

impl AntiLineOnOrigin {
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

impl PartialOrd for AntiLineOnOrigin {
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
impl Ord for AntiLineOnOrigin {
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
impl PartialEq for AntiLineOnOrigin {
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
impl Eq for AntiLineOnOrigin {}
impl std::hash::Hash for AntiLineOnOrigin {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for i in 0..Self::LEN {
            self[i].to_bits().hash(state);
        }
    }
}

unsafe impl bytemuck::Zeroable for AntiLineOnOrigin {}
unsafe impl bytemuck::Pod for AntiLineOnOrigin {}
impl encase::ShaderType for AntiLineOnOrigin {
    type ExtraMetadata = <AntiLineOnOriginGroups as encase::ShaderType>::ExtraMetadata;
    const METADATA: encase::private::Metadata<Self::ExtraMetadata> = <AntiLineOnOriginGroups as encase::ShaderType>::METADATA;
    fn min_size() -> std::num::NonZeroU64 {
        return <AntiLineOnOriginGroups as encase::ShaderType>::min_size();
    }
    fn size(&self) -> std::num::NonZeroU64 {
        return encase::ShaderType::size(unsafe { &self.groups });
    }
    const UNIFORM_COMPAT_ASSERT: fn() = <AntiLineOnOriginGroups as encase::ShaderType>::UNIFORM_COMPAT_ASSERT;
    fn assert_uniform_compat() {
        return <AntiLineOnOriginGroups as encase::ShaderType>::assert_uniform_compat();
    }
}

impl serde::Serialize for AntiLineOnOrigin {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("AntiLineOnOrigin", 3)?;
        state.serialize_field("e23", &self[crate::elements::e23])?;
        state.serialize_field("e31", &self[crate::elements::e31])?;
        state.serialize_field("e12", &self[crate::elements::e12])?;
        state.end()
    }
}
impl<'de> serde::Deserialize<'de> for AntiLineOnOrigin {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::{MapAccess, Visitor};
        use std::fmt;
        #[allow(non_camel_case_types)]
        #[derive(serde::Deserialize)]
        enum AntiLineOnOriginField {
            e23,
            e31,
            e12,
        }
        struct AntiLineOnOriginVisitor;
        impl<'de> Visitor<'de> for AntiLineOnOriginVisitor {
            type Value = AntiLineOnOrigin;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct AntiLineOnOrigin")
            }
            fn visit_map<V>(self, mut map: V) -> Result<AntiLineOnOrigin, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut e23 = None;
                let mut e31 = None;
                let mut e12 = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        AntiLineOnOriginField::e23 => {
                            if e23.is_some() {
                                return Err(serde::de::Error::duplicate_field("e23"));
                            }
                            e23 = Some(map.next_value()?);
                        }

                        AntiLineOnOriginField::e31 => {
                            if e31.is_some() {
                                return Err(serde::de::Error::duplicate_field("e31"));
                            }
                            e31 = Some(map.next_value()?);
                        }

                        AntiLineOnOriginField::e12 => {
                            if e12.is_some() {
                                return Err(serde::de::Error::duplicate_field("e12"));
                            }
                            e12 = Some(map.next_value()?);
                        }
                    }
                }
                let mut result = AntiLineOnOrigin::from([0.0; 3]);
                result[crate::elements::e23] = e23.ok_or_else(|| serde::de::Error::missing_field("e23"))?;
                result[crate::elements::e31] = e31.ok_or_else(|| serde::de::Error::missing_field("e31"))?;
                result[crate::elements::e12] = e12.ok_or_else(|| serde::de::Error::missing_field("e12"))?;
                Ok(result)
            }
        }

        const FIELDS: &'static [&'static str] = &["e23", "e31", "e12"];
        deserializer.deserialize_struct("AntiLineOnOrigin", FIELDS, AntiLineOnOriginVisitor)
    }
}
impl std::ops::Index<crate::elements::e23> for AntiLineOnOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e23) -> &Self::Output {
        &self[0]
    }
}
impl std::ops::Index<crate::elements::e31> for AntiLineOnOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e31) -> &Self::Output {
        &self[1]
    }
}
impl std::ops::Index<crate::elements::e12> for AntiLineOnOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e12) -> &Self::Output {
        &self[2]
    }
}
impl std::ops::IndexMut<crate::elements::e23> for AntiLineOnOrigin {
    fn index_mut(&mut self, _: crate::elements::e23) -> &mut Self::Output {
        &mut self[0]
    }
}
impl std::ops::IndexMut<crate::elements::e31> for AntiLineOnOrigin {
    fn index_mut(&mut self, _: crate::elements::e31) -> &mut Self::Output {
        &mut self[1]
    }
}
impl std::ops::IndexMut<crate::elements::e12> for AntiLineOnOrigin {
    fn index_mut(&mut self, _: crate::elements::e12) -> &mut Self::Output {
        &mut self[2]
    }
}
include!("./impls/anti_line_on_origin.rs");
