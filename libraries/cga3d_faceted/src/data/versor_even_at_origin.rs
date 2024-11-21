use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// VersorEvenAtOrigin.
/// This variant of VersorEven is centered on the Origin.
#[repr(C)]
#[derive(Clone, Copy)]
pub union VersorEvenAtOrigin {
    groups: VersorEvenAtOriginGroups,
    /// e423, e431, e412, e4, e235, e315, e125, e5
    elements: [f32; 8],
}
#[repr(C)]
#[derive(Clone, Copy, encase::ShaderType)]
pub struct VersorEvenAtOriginGroups {
    /// e423, e431, e412, e4
    g0: Simd32x4,
    /// e235, e315, e125, e5
    g1: Simd32x4,
}
impl VersorEvenAtOrigin {
    #[allow(clippy::too_many_arguments)]
    pub const fn from_elements(e423: f32, e431: f32, e412: f32, e4: f32, e235: f32, e315: f32, e125: f32, e5: f32) -> Self {
        Self {
            elements: [e423, e431, e412, e4, e235, e315, e125, e5],
        }
    }
    pub const fn from_groups(g0: Simd32x4, g1: Simd32x4) -> Self {
        Self {
            groups: VersorEvenAtOriginGroups { g0, g1 },
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
    pub fn group1(&self) -> Simd32x4 {
        unsafe { self.groups.g1 }
    }
    #[inline(always)]
    pub fn group1_mut(&mut self) -> &mut Simd32x4 {
        unsafe { &mut self.groups.g1 }
    }
}
const VERSOR_EVEN_AT_ORIGIN_INDEX_REMAP: [usize; 8] = [0, 1, 2, 3, 4, 5, 6, 7];
impl std::ops::Index<usize> for VersorEvenAtOrigin {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[VERSOR_EVEN_AT_ORIGIN_INDEX_REMAP[index]] }
    }
}
impl std::ops::IndexMut<usize> for VersorEvenAtOrigin {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[VERSOR_EVEN_AT_ORIGIN_INDEX_REMAP[index]] }
    }
}
impl From<VersorEvenAtOrigin> for [f32; 8] {
    fn from(vector: VersorEvenAtOrigin) -> Self {
        unsafe {
            [
                vector.elements[0], vector.elements[1], vector.elements[2], vector.elements[3], vector.elements[4], vector.elements[5], vector.elements[6], vector.elements[7],
            ]
        }
    }
}
impl From<[f32; 8]> for VersorEvenAtOrigin {
    fn from(array: [f32; 8]) -> Self {
        Self {
            elements: [array[0], array[1], array[2], array[3], array[1], array[2], array[3], array[4]],
        }
    }
}
impl std::fmt::Debug for VersorEvenAtOrigin {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .debug_struct("VersorEvenAtOrigin")
            .field("e423", &self[0])
            .field("e431", &self[1])
            .field("e412", &self[2])
            .field("e4", &self[3])
            .field("e235", &self[4])
            .field("e315", &self[5])
            .field("e125", &self[6])
            .field("e5", &self[7])
            .finish()
    }
}

impl VersorEvenAtOrigin {
    pub const LEN: usize = 8;
}

impl nearly::NearlyEqEps<VersorEvenAtOrigin, f32, f32> for VersorEvenAtOrigin {
    fn nearly_eq_eps(&self, other: &VersorEvenAtOrigin, eps: &nearly::EpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyEqUlps<VersorEvenAtOrigin, f32, f32> for VersorEvenAtOrigin {
    fn nearly_eq_ulps(&self, other: &VersorEvenAtOrigin, ulps: &nearly::UlpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyEqTol<VersorEvenAtOrigin, f32, f32> for VersorEvenAtOrigin {}
impl nearly::NearlyEq<VersorEvenAtOrigin, f32, f32> for VersorEvenAtOrigin {}
impl nearly::NearlyOrdUlps<VersorEvenAtOrigin, f32, f32> for VersorEvenAtOrigin {
    fn nearly_lt_ulps(&self, other: &VersorEvenAtOrigin, ulps: &nearly::UlpsToleranceType<f32, f32>) -> bool {
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

    fn nearly_gt_ulps(&self, other: &VersorEvenAtOrigin, ulps: &nearly::UlpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyOrdEps<VersorEvenAtOrigin, f32, f32> for VersorEvenAtOrigin {
    fn nearly_lt_eps(&self, other: &VersorEvenAtOrigin, eps: &nearly::EpsToleranceType<f32, f32>) -> bool {
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

    fn nearly_gt_eps(&self, other: &VersorEvenAtOrigin, eps: &nearly::EpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyOrdTol<VersorEvenAtOrigin, f32, f32> for VersorEvenAtOrigin {}
impl nearly::NearlyOrd<VersorEvenAtOrigin, f32, f32> for VersorEvenAtOrigin {}

impl VersorEvenAtOrigin {
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

impl PartialOrd for VersorEvenAtOrigin {
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
impl Ord for VersorEvenAtOrigin {
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
impl PartialEq for VersorEvenAtOrigin {
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
impl Eq for VersorEvenAtOrigin {}
impl std::hash::Hash for VersorEvenAtOrigin {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for i in 0..Self::LEN {
            self[i].to_bits().hash(state);
        }
    }
}

unsafe impl bytemuck::Zeroable for VersorEvenAtOrigin {}
unsafe impl bytemuck::Pod for VersorEvenAtOrigin {}
impl encase::ShaderType for VersorEvenAtOrigin {
    type ExtraMetadata = <VersorEvenAtOriginGroups as encase::ShaderType>::ExtraMetadata;
    const METADATA: encase::private::Metadata<Self::ExtraMetadata> = <VersorEvenAtOriginGroups as encase::ShaderType>::METADATA;
    fn min_size() -> std::num::NonZeroU64 {
        return <VersorEvenAtOriginGroups as encase::ShaderType>::min_size();
    }
    fn size(&self) -> std::num::NonZeroU64 {
        return encase::ShaderType::size(unsafe { &self.groups });
    }
    const UNIFORM_COMPAT_ASSERT: fn() = <VersorEvenAtOriginGroups as encase::ShaderType>::UNIFORM_COMPAT_ASSERT;
    fn assert_uniform_compat() {
        return <VersorEvenAtOriginGroups as encase::ShaderType>::assert_uniform_compat();
    }
}

impl serde::Serialize for VersorEvenAtOrigin {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("VersorEvenAtOrigin", 8)?;
        state.serialize_field("e423", &self[crate::elements::e423])?;
        state.serialize_field("e431", &self[crate::elements::e431])?;
        state.serialize_field("e412", &self[crate::elements::e412])?;
        state.serialize_field("e4", &self[crate::elements::e4])?;
        state.serialize_field("e235", &self[crate::elements::e235])?;
        state.serialize_field("e315", &self[crate::elements::e315])?;
        state.serialize_field("e125", &self[crate::elements::e125])?;
        state.serialize_field("e5", &self[crate::elements::e5])?;
        state.end()
    }
}
impl<'de> serde::Deserialize<'de> for VersorEvenAtOrigin {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::{MapAccess, Visitor};
        use std::fmt;
        #[allow(non_camel_case_types)]
        #[derive(serde::Deserialize)]
        enum VersorEvenAtOriginField {
            e423,
            e431,
            e412,
            e4,
            e235,
            e315,
            e125,
            e5,
        }
        struct VersorEvenAtOriginVisitor;
        impl<'de> Visitor<'de> for VersorEvenAtOriginVisitor {
            type Value = VersorEvenAtOrigin;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct VersorEvenAtOrigin")
            }
            fn visit_map<V>(self, mut map: V) -> Result<VersorEvenAtOrigin, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut e423 = None;
                let mut e431 = None;
                let mut e412 = None;
                let mut e4 = None;
                let mut e235 = None;
                let mut e315 = None;
                let mut e125 = None;
                let mut e5 = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        VersorEvenAtOriginField::e423 => {
                            if e423.is_some() {
                                return Err(serde::de::Error::duplicate_field("e423"));
                            }
                            e423 = Some(map.next_value()?);
                        }

                        VersorEvenAtOriginField::e431 => {
                            if e431.is_some() {
                                return Err(serde::de::Error::duplicate_field("e431"));
                            }
                            e431 = Some(map.next_value()?);
                        }

                        VersorEvenAtOriginField::e412 => {
                            if e412.is_some() {
                                return Err(serde::de::Error::duplicate_field("e412"));
                            }
                            e412 = Some(map.next_value()?);
                        }

                        VersorEvenAtOriginField::e4 => {
                            if e4.is_some() {
                                return Err(serde::de::Error::duplicate_field("e4"));
                            }
                            e4 = Some(map.next_value()?);
                        }

                        VersorEvenAtOriginField::e235 => {
                            if e235.is_some() {
                                return Err(serde::de::Error::duplicate_field("e235"));
                            }
                            e235 = Some(map.next_value()?);
                        }

                        VersorEvenAtOriginField::e315 => {
                            if e315.is_some() {
                                return Err(serde::de::Error::duplicate_field("e315"));
                            }
                            e315 = Some(map.next_value()?);
                        }

                        VersorEvenAtOriginField::e125 => {
                            if e125.is_some() {
                                return Err(serde::de::Error::duplicate_field("e125"));
                            }
                            e125 = Some(map.next_value()?);
                        }

                        VersorEvenAtOriginField::e5 => {
                            if e5.is_some() {
                                return Err(serde::de::Error::duplicate_field("e5"));
                            }
                            e5 = Some(map.next_value()?);
                        }
                    }
                }
                let mut result = VersorEvenAtOrigin::from([0.0; 8]);
                result[crate::elements::e423] = e423.ok_or_else(|| serde::de::Error::missing_field("e423"))?;
                result[crate::elements::e431] = e431.ok_or_else(|| serde::de::Error::missing_field("e431"))?;
                result[crate::elements::e412] = e412.ok_or_else(|| serde::de::Error::missing_field("e412"))?;
                result[crate::elements::e4] = e4.ok_or_else(|| serde::de::Error::missing_field("e4"))?;
                result[crate::elements::e235] = e235.ok_or_else(|| serde::de::Error::missing_field("e235"))?;
                result[crate::elements::e315] = e315.ok_or_else(|| serde::de::Error::missing_field("e315"))?;
                result[crate::elements::e125] = e125.ok_or_else(|| serde::de::Error::missing_field("e125"))?;
                result[crate::elements::e5] = e5.ok_or_else(|| serde::de::Error::missing_field("e5"))?;
                Ok(result)
            }
        }

        const FIELDS: &'static [&'static str] = &["e423", "e431", "e412", "e4", "e235", "e315", "e125", "e5"];
        deserializer.deserialize_struct("VersorEvenAtOrigin", FIELDS, VersorEvenAtOriginVisitor)
    }
}
impl std::ops::Index<crate::elements::e423> for VersorEvenAtOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e423) -> &Self::Output {
        &self[0]
    }
}
impl std::ops::Index<crate::elements::e431> for VersorEvenAtOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e431) -> &Self::Output {
        &self[1]
    }
}
impl std::ops::Index<crate::elements::e412> for VersorEvenAtOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e412) -> &Self::Output {
        &self[2]
    }
}
impl std::ops::Index<crate::elements::e4> for VersorEvenAtOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e4) -> &Self::Output {
        &self[3]
    }
}
impl std::ops::Index<crate::elements::e235> for VersorEvenAtOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e235) -> &Self::Output {
        &self[4]
    }
}
impl std::ops::Index<crate::elements::e315> for VersorEvenAtOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e315) -> &Self::Output {
        &self[5]
    }
}
impl std::ops::Index<crate::elements::e125> for VersorEvenAtOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e125) -> &Self::Output {
        &self[6]
    }
}
impl std::ops::Index<crate::elements::e5> for VersorEvenAtOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e5) -> &Self::Output {
        &self[7]
    }
}
impl std::ops::IndexMut<crate::elements::e423> for VersorEvenAtOrigin {
    fn index_mut(&mut self, _: crate::elements::e423) -> &mut Self::Output {
        &mut self[0]
    }
}
impl std::ops::IndexMut<crate::elements::e431> for VersorEvenAtOrigin {
    fn index_mut(&mut self, _: crate::elements::e431) -> &mut Self::Output {
        &mut self[1]
    }
}
impl std::ops::IndexMut<crate::elements::e412> for VersorEvenAtOrigin {
    fn index_mut(&mut self, _: crate::elements::e412) -> &mut Self::Output {
        &mut self[2]
    }
}
impl std::ops::IndexMut<crate::elements::e4> for VersorEvenAtOrigin {
    fn index_mut(&mut self, _: crate::elements::e4) -> &mut Self::Output {
        &mut self[3]
    }
}
impl std::ops::IndexMut<crate::elements::e235> for VersorEvenAtOrigin {
    fn index_mut(&mut self, _: crate::elements::e235) -> &mut Self::Output {
        &mut self[4]
    }
}
impl std::ops::IndexMut<crate::elements::e315> for VersorEvenAtOrigin {
    fn index_mut(&mut self, _: crate::elements::e315) -> &mut Self::Output {
        &mut self[5]
    }
}
impl std::ops::IndexMut<crate::elements::e125> for VersorEvenAtOrigin {
    fn index_mut(&mut self, _: crate::elements::e125) -> &mut Self::Output {
        &mut self[6]
    }
}
impl std::ops::IndexMut<crate::elements::e5> for VersorEvenAtOrigin {
    fn index_mut(&mut self, _: crate::elements::e5) -> &mut Self::Output {
        &mut self[7]
    }
}
include!("./impls/versor_even_at_origin.rs");
