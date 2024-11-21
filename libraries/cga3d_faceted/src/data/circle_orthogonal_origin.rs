use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// CircleOrthogonalOrigin.
/// This variant of Circle has a CoCarrier that intersects the Origin.
#[repr(C)]
#[derive(Clone, Copy)]
pub union CircleOrthogonalOrigin {
    groups: CircleOrthogonalOriginGroups,
    /// e423, e431, e412, e321, e235, e315, e125, 0
    elements: [f32; 8],
}
#[repr(C)]
#[derive(Clone, Copy, encase::ShaderType)]
pub struct CircleOrthogonalOriginGroups {
    /// e423, e431, e412, e321
    g0: Simd32x4,
    /// e235, e315, e125
    g1: Simd32x3,
}
impl CircleOrthogonalOrigin {
    #[allow(clippy::too_many_arguments)]
    pub const fn from_elements(e423: f32, e431: f32, e412: f32, e321: f32, e235: f32, e315: f32, e125: f32) -> Self {
        Self {
            elements: [e423, e431, e412, e321, e235, e315, e125, 0.0],
        }
    }
    pub const fn from_groups(g0: Simd32x4, g1: Simd32x3) -> Self {
        Self {
            groups: CircleOrthogonalOriginGroups { g0, g1 },
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
    pub fn group1(&self) -> Simd32x3 {
        unsafe { self.groups.g1 }
    }
    #[inline(always)]
    pub fn group1_mut(&mut self) -> &mut Simd32x3 {
        unsafe { &mut self.groups.g1 }
    }
}
const CIRCLE_ORTHOGONAL_ORIGIN_INDEX_REMAP: [usize; 7] = [0, 1, 2, 3, 4, 5, 6];
impl std::ops::Index<usize> for CircleOrthogonalOrigin {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[CIRCLE_ORTHOGONAL_ORIGIN_INDEX_REMAP[index]] }
    }
}
impl std::ops::IndexMut<usize> for CircleOrthogonalOrigin {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[CIRCLE_ORTHOGONAL_ORIGIN_INDEX_REMAP[index]] }
    }
}
impl From<CircleOrthogonalOrigin> for [f32; 7] {
    fn from(vector: CircleOrthogonalOrigin) -> Self {
        unsafe {
            [
                vector.elements[0], vector.elements[1], vector.elements[2], vector.elements[3], vector.elements[4], vector.elements[5], vector.elements[6],
            ]
        }
    }
}
impl From<[f32; 7]> for CircleOrthogonalOrigin {
    fn from(array: [f32; 7]) -> Self {
        Self {
            elements: [array[0], array[1], array[2], array[3], array[1], array[2], array[3], 0.0],
        }
    }
}
impl std::fmt::Debug for CircleOrthogonalOrigin {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .debug_struct("CircleOrthogonalOrigin")
            .field("e423", &self[0])
            .field("e431", &self[1])
            .field("e412", &self[2])
            .field("e321", &self[3])
            .field("e235", &self[4])
            .field("e315", &self[5])
            .field("e125", &self[6])
            .finish()
    }
}

impl CircleOrthogonalOrigin {
    pub const LEN: usize = 7;
}

impl nearly::NearlyEqEps<CircleOrthogonalOrigin, f32, f32> for CircleOrthogonalOrigin {
    fn nearly_eq_eps(&self, other: &CircleOrthogonalOrigin, eps: &nearly::EpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyEqUlps<CircleOrthogonalOrigin, f32, f32> for CircleOrthogonalOrigin {
    fn nearly_eq_ulps(&self, other: &CircleOrthogonalOrigin, ulps: &nearly::UlpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyEqTol<CircleOrthogonalOrigin, f32, f32> for CircleOrthogonalOrigin {}
impl nearly::NearlyEq<CircleOrthogonalOrigin, f32, f32> for CircleOrthogonalOrigin {}
impl nearly::NearlyOrdUlps<CircleOrthogonalOrigin, f32, f32> for CircleOrthogonalOrigin {
    fn nearly_lt_ulps(&self, other: &CircleOrthogonalOrigin, ulps: &nearly::UlpsToleranceType<f32, f32>) -> bool {
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

    fn nearly_gt_ulps(&self, other: &CircleOrthogonalOrigin, ulps: &nearly::UlpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyOrdEps<CircleOrthogonalOrigin, f32, f32> for CircleOrthogonalOrigin {
    fn nearly_lt_eps(&self, other: &CircleOrthogonalOrigin, eps: &nearly::EpsToleranceType<f32, f32>) -> bool {
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

    fn nearly_gt_eps(&self, other: &CircleOrthogonalOrigin, eps: &nearly::EpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyOrdTol<CircleOrthogonalOrigin, f32, f32> for CircleOrthogonalOrigin {}
impl nearly::NearlyOrd<CircleOrthogonalOrigin, f32, f32> for CircleOrthogonalOrigin {}

impl CircleOrthogonalOrigin {
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

impl PartialOrd for CircleOrthogonalOrigin {
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
impl Ord for CircleOrthogonalOrigin {
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
impl PartialEq for CircleOrthogonalOrigin {
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
impl Eq for CircleOrthogonalOrigin {}
impl std::hash::Hash for CircleOrthogonalOrigin {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for i in 0..Self::LEN {
            self[i].to_bits().hash(state);
        }
    }
}

unsafe impl bytemuck::Zeroable for CircleOrthogonalOrigin {}
unsafe impl bytemuck::Pod for CircleOrthogonalOrigin {}
impl encase::ShaderType for CircleOrthogonalOrigin {
    type ExtraMetadata = <CircleOrthogonalOriginGroups as encase::ShaderType>::ExtraMetadata;
    const METADATA: encase::private::Metadata<Self::ExtraMetadata> = <CircleOrthogonalOriginGroups as encase::ShaderType>::METADATA;
    fn min_size() -> std::num::NonZeroU64 {
        return <CircleOrthogonalOriginGroups as encase::ShaderType>::min_size();
    }
    fn size(&self) -> std::num::NonZeroU64 {
        return encase::ShaderType::size(unsafe { &self.groups });
    }
    const UNIFORM_COMPAT_ASSERT: fn() = <CircleOrthogonalOriginGroups as encase::ShaderType>::UNIFORM_COMPAT_ASSERT;
    fn assert_uniform_compat() {
        return <CircleOrthogonalOriginGroups as encase::ShaderType>::assert_uniform_compat();
    }
}

impl serde::Serialize for CircleOrthogonalOrigin {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("CircleOrthogonalOrigin", 7)?;
        state.serialize_field("e423", &self[crate::elements::e423])?;
        state.serialize_field("e431", &self[crate::elements::e431])?;
        state.serialize_field("e412", &self[crate::elements::e412])?;
        state.serialize_field("e321", &self[crate::elements::e321])?;
        state.serialize_field("e235", &self[crate::elements::e235])?;
        state.serialize_field("e315", &self[crate::elements::e315])?;
        state.serialize_field("e125", &self[crate::elements::e125])?;
        state.end()
    }
}
impl<'de> serde::Deserialize<'de> for CircleOrthogonalOrigin {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::{MapAccess, Visitor};
        use std::fmt;
        #[allow(non_camel_case_types)]
        #[derive(serde::Deserialize)]
        enum CircleOrthogonalOriginField {
            e423,
            e431,
            e412,
            e321,
            e235,
            e315,
            e125,
        }
        struct CircleOrthogonalOriginVisitor;
        impl<'de> Visitor<'de> for CircleOrthogonalOriginVisitor {
            type Value = CircleOrthogonalOrigin;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct CircleOrthogonalOrigin")
            }
            fn visit_map<V>(self, mut map: V) -> Result<CircleOrthogonalOrigin, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut e423 = None;
                let mut e431 = None;
                let mut e412 = None;
                let mut e321 = None;
                let mut e235 = None;
                let mut e315 = None;
                let mut e125 = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        CircleOrthogonalOriginField::e423 => {
                            if e423.is_some() {
                                return Err(serde::de::Error::duplicate_field("e423"));
                            }
                            e423 = Some(map.next_value()?);
                        }

                        CircleOrthogonalOriginField::e431 => {
                            if e431.is_some() {
                                return Err(serde::de::Error::duplicate_field("e431"));
                            }
                            e431 = Some(map.next_value()?);
                        }

                        CircleOrthogonalOriginField::e412 => {
                            if e412.is_some() {
                                return Err(serde::de::Error::duplicate_field("e412"));
                            }
                            e412 = Some(map.next_value()?);
                        }

                        CircleOrthogonalOriginField::e321 => {
                            if e321.is_some() {
                                return Err(serde::de::Error::duplicate_field("e321"));
                            }
                            e321 = Some(map.next_value()?);
                        }

                        CircleOrthogonalOriginField::e235 => {
                            if e235.is_some() {
                                return Err(serde::de::Error::duplicate_field("e235"));
                            }
                            e235 = Some(map.next_value()?);
                        }

                        CircleOrthogonalOriginField::e315 => {
                            if e315.is_some() {
                                return Err(serde::de::Error::duplicate_field("e315"));
                            }
                            e315 = Some(map.next_value()?);
                        }

                        CircleOrthogonalOriginField::e125 => {
                            if e125.is_some() {
                                return Err(serde::de::Error::duplicate_field("e125"));
                            }
                            e125 = Some(map.next_value()?);
                        }
                    }
                }
                let mut result = CircleOrthogonalOrigin::from([0.0; 7]);
                result[crate::elements::e423] = e423.ok_or_else(|| serde::de::Error::missing_field("e423"))?;
                result[crate::elements::e431] = e431.ok_or_else(|| serde::de::Error::missing_field("e431"))?;
                result[crate::elements::e412] = e412.ok_or_else(|| serde::de::Error::missing_field("e412"))?;
                result[crate::elements::e321] = e321.ok_or_else(|| serde::de::Error::missing_field("e321"))?;
                result[crate::elements::e235] = e235.ok_or_else(|| serde::de::Error::missing_field("e235"))?;
                result[crate::elements::e315] = e315.ok_or_else(|| serde::de::Error::missing_field("e315"))?;
                result[crate::elements::e125] = e125.ok_or_else(|| serde::de::Error::missing_field("e125"))?;
                Ok(result)
            }
        }

        const FIELDS: &'static [&'static str] = &["e423", "e431", "e412", "e321", "e235", "e315", "e125"];
        deserializer.deserialize_struct("CircleOrthogonalOrigin", FIELDS, CircleOrthogonalOriginVisitor)
    }
}
impl std::ops::Index<crate::elements::e423> for CircleOrthogonalOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e423) -> &Self::Output {
        &self[0]
    }
}
impl std::ops::Index<crate::elements::e431> for CircleOrthogonalOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e431) -> &Self::Output {
        &self[1]
    }
}
impl std::ops::Index<crate::elements::e412> for CircleOrthogonalOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e412) -> &Self::Output {
        &self[2]
    }
}
impl std::ops::Index<crate::elements::e321> for CircleOrthogonalOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e321) -> &Self::Output {
        &self[3]
    }
}
impl std::ops::Index<crate::elements::e235> for CircleOrthogonalOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e235) -> &Self::Output {
        &self[4]
    }
}
impl std::ops::Index<crate::elements::e315> for CircleOrthogonalOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e315) -> &Self::Output {
        &self[5]
    }
}
impl std::ops::Index<crate::elements::e125> for CircleOrthogonalOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e125) -> &Self::Output {
        &self[6]
    }
}
impl std::ops::IndexMut<crate::elements::e423> for CircleOrthogonalOrigin {
    fn index_mut(&mut self, _: crate::elements::e423) -> &mut Self::Output {
        &mut self[0]
    }
}
impl std::ops::IndexMut<crate::elements::e431> for CircleOrthogonalOrigin {
    fn index_mut(&mut self, _: crate::elements::e431) -> &mut Self::Output {
        &mut self[1]
    }
}
impl std::ops::IndexMut<crate::elements::e412> for CircleOrthogonalOrigin {
    fn index_mut(&mut self, _: crate::elements::e412) -> &mut Self::Output {
        &mut self[2]
    }
}
impl std::ops::IndexMut<crate::elements::e321> for CircleOrthogonalOrigin {
    fn index_mut(&mut self, _: crate::elements::e321) -> &mut Self::Output {
        &mut self[3]
    }
}
impl std::ops::IndexMut<crate::elements::e235> for CircleOrthogonalOrigin {
    fn index_mut(&mut self, _: crate::elements::e235) -> &mut Self::Output {
        &mut self[4]
    }
}
impl std::ops::IndexMut<crate::elements::e315> for CircleOrthogonalOrigin {
    fn index_mut(&mut self, _: crate::elements::e315) -> &mut Self::Output {
        &mut self[5]
    }
}
impl std::ops::IndexMut<crate::elements::e125> for CircleOrthogonalOrigin {
    fn index_mut(&mut self, _: crate::elements::e125) -> &mut Self::Output {
        &mut self[6]
    }
}
include!("./impls/circle_orthogonal_origin.rs");
