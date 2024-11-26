#[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
pub use std::arch::aarch64::*;
#[cfg(all(target_arch = "arm", target_feature = "neon"))]
pub use std::arch::arm::*;
#[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
pub use std::arch::wasm32::*;
#[cfg(all(target_arch = "x86", target_feature = "sse2"))]
pub use std::arch::x86::*;
#[cfg(all(target_arch = "x86_64", target_feature = "sse2"))]
pub use std::arch::x86_64::*;
use std::num::NonZeroU64;
use encase::internal::{BufferMut, BufferRef, Reader, Writer};
use encase::private::Metadata;
use nearly::{EpsToleranceType, NearlyEqEps, NearlyEqTol, NearlyEqUlps, UlpsToleranceType};

#[derive(Clone, Copy)]
#[repr(C)]
pub union Simd32x4 {
    // Intel / AMD
    #[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2"))]
    pub f128: __m128,
    #[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2"))]
    pub i128: __m128i,

    // ARM
    #[cfg(all(any(target_arch = "arm", target_arch = "aarch64"), target_feature = "neon"))]
    pub f128: float32x4_t,
    #[cfg(all(any(target_arch = "arm", target_arch = "aarch64"), target_feature = "neon"))]
    pub i128: int32x4_t,
    #[cfg(all(any(target_arch = "arm", target_arch = "aarch64"), target_feature = "neon"))]
    pub u128: uint32x4_t,

    // Web
    #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
    pub v128: v128,

    // Fallback
    pub f32x4: [f32; 4],
    pub i32x4: [i32; 4],
    pub u32x4: [u32; 4],
}

#[derive(Clone, Copy)]
#[repr(C)]
pub union Simd32x3 {
    pub v32x4: Simd32x4,

    // Fallback
    pub f32x3: [f32; 3],
    pub i32x3: [i32; 3],
    pub u32x3: [u32; 3],
}

#[derive(Clone, Copy)]
#[repr(C)]
pub union Simd32x2 {
    pub v32x4: Simd32x4,
    pub v32x3: Simd32x3,

    // Fallback
    pub f32x2: [f32; 2],
    pub i32x2: [i32; 2],
    pub u32x2: [u32; 2],
}

#[macro_export]
macro_rules! match_architecture {
    ($Simd:ident, $native:tt, $fallback:tt,) => {{
        #[cfg(any(
            all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2"),
            all(any(target_arch = "arm", target_arch = "aarch64"), target_feature = "neon"),
            all(target_arch = "wasm32", target_feature = "simd128"),
        ))]
        { $Simd $native }
        #[cfg(not(any(
            all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2"),
            all(any(target_arch = "arm", target_arch = "aarch64"), target_feature = "neon"),
            all(target_arch = "wasm32", target_feature = "simd128"),
        )))]
        unsafe { $Simd $fallback }
    }};
    ($Simd:ident, $x86:tt, $arm:tt, $web:tt, $fallback:tt,) => {{
        #[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2"))]
        unsafe { $Simd $x86 }
        #[cfg(all(any(target_arch = "arm", target_arch = "aarch64"), target_feature = "neon"))]
        unsafe { $Simd $arm }
        #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
        unsafe { $Simd $web }
        #[cfg(not(any(
            all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2"),
            all(any(target_arch = "arm", target_arch = "aarch64"), target_feature = "neon"),
            all(target_arch = "wasm32", target_feature = "simd128"),
        )))]
        unsafe { $Simd $fallback }
    }};
}

// TODO truncated swizzling, vector extension, then more AST simplifications

#[macro_export]
macro_rules! swizzle {
    ($self:expr, $x:literal, $y:literal, _, _) => {
        unsafe { Simd32x2 { v32x4: $crate::swizzle!($self.v32x4, $x, $y, 2, 3) } }
    };
    ($self:expr, $x:literal, $y:literal, $z:literal, _) => {
        unsafe { Simd32x3 { v32x4: $crate::swizzle!($self.v32x4, $x, $y, $z, 3) } }
    };
    ($self:expr, $x:literal, $y:literal, $z:literal, $w:literal) => {
        $crate::match_architecture!(
            Simd32x4,
            // x86
            { f128: $crate::simd::_mm_permute_ps($self.f128, ($x as i32) | (($y as i32) << 2) | (($z as i32) << 4) | (($w as i32) << 6)) },
            // arm
            { f32x4: [
                $self.f32x4[$x],
                $self.f32x4[$y],
                $self.f32x4[$z],
                $self.f32x4[$w],
            ] },
            // web
            { v128: $crate::simd::i32x4_shuffle::<$x, $y, $z, $w>($self.v128, $self.v128) },
            // fallback
            { f32x4: [
                $self.f32x4[$x],
                $self.f32x4[$y],
                $self.f32x4[$z],
                $self.f32x4[$w],
            ] },
        )
    };
    ($self:expr, $x:literal, $y:literal, _) => {
        unsafe { Simd32x2 { $crate::swizzle!($self.v32x4, $x, $y, 2, 3) } }
    };
    ($self:expr, $x:literal, $y:literal, $z:literal) => {
        $crate::match_architecture!(
            Simd32x3,
            // native
            { v32x4: $crate::swizzle!($self.v32x4, $x, $y, $z, 3) },
            // fallback
            { f32x3: [
                $self.f32x3[$x],
                $self.f32x3[$y],
                $self.f32x3[$z],
            ] },
        )
    };
    ($self:expr, $x:literal, $y:literal) => {
        $crate::match_architecture!(
            Simd32x2,
            // native
            { v32x4: $crate::swizzle!($self.v32x4, $x, $y, 2, 3) },
            // fallback
            { f32x2: [
                $self.f32x2[$x],
                $self.f32x2[$y],
            ] },
        )
    };
}

impl Simd32x2 {
    pub fn powi(mut self, exponent: i32) -> Self {
        // TODO simd-ify
        self[0] = f32::powi(self[0], exponent);
        self[1] = f32::powi(self[1], exponent);
        self
    }
    pub fn powf(mut self, exponent: f32) -> Self {
        // TODO simd-ify
        self[0] = f32::powf(self[0], exponent);
        self[1] = f32::powf(self[1], exponent);
        self
    }
    pub fn extend_to_3(self, z: f32) -> Simd32x3 {
        let mut s = unsafe { self.v32x3 };
        s[2] = z;
        return s;
    }
    pub fn extend_to_4(self, z: f32, w: f32) -> Simd32x4 {
        let mut s = unsafe { self.v32x4 };
        s[2] = z;
        s[3] = w;
        return s;
    }
}
impl Simd32x3 {
    pub fn powi(mut self, exponent: i32) -> Self {
        // TODO simd-ify
        self[0] = f32::powi(self[0], exponent);
        self[1] = f32::powi(self[1], exponent);
        self[2] = f32::powi(self[2], exponent);
        self
    }
    pub fn powf(mut self, exponent: f32) -> Self {
        // TODO simd-ify
        self[0] = f32::powf(self[0], exponent);
        self[1] = f32::powf(self[1], exponent);
        self[2] = f32::powf(self[2], exponent);
        self
    }
    pub fn extend_to_4(self, w: f32) -> Simd32x4 {
        let mut s = unsafe { self.v32x4 };
        s[3] = w;
        return s;
    }
    pub fn truncate_to_2(self) -> Simd32x2 {
        unsafe { Simd32x2 { v32x4: self.v32x4 } }
    }
}
impl Simd32x4 {
    pub fn powi(mut self, exponent: i32) -> Self {
        // TODO simd-ify
        self[0] = f32::powi(self[0], exponent);
        self[1] = f32::powi(self[1], exponent);
        self[2] = f32::powi(self[2], exponent);
        self[3] = f32::powi(self[3], exponent);
        self
    }
    pub fn powf(mut self, exponent: f32) -> Self {
        // TODO simd-ify
        self[0] = f32::powf(self[0], exponent);
        self[1] = f32::powf(self[1], exponent);
        self[2] = f32::powf(self[2], exponent);
        self[3] = f32::powf(self[3], exponent);
        self
    }
    pub fn truncate_to_3(self) -> Simd32x3 {
        unsafe { Simd32x3 { v32x4: self } }
    }
    pub fn truncate_to_2(self) -> Simd32x2 {
        unsafe { Simd32x2 { v32x4: self } }
    }
}

impl std::ops::Index<usize> for Simd32x4 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.f32x4[index] }
    }
}

impl std::ops::Index<usize> for Simd32x3 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.f32x3[index] }
    }
}

impl std::ops::Index<usize> for Simd32x2 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.f32x2[index] }
    }
}

impl std::ops::IndexMut<usize> for Simd32x4 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.f32x4[index] }
    }
}

impl std::ops::IndexMut<usize> for Simd32x3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.f32x3[index] }
    }
}

impl std::ops::IndexMut<usize> for Simd32x2 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.f32x2[index] }
    }
}

impl From<Simd32x4> for [f32; 4] {
    fn from(simd: Simd32x4) -> Self {
        unsafe { simd.f32x4 }
    }
}

impl From<Simd32x3> for [f32; 3] {
    fn from(simd: Simd32x3) -> Self {
        unsafe { simd.f32x3 }
    }
}

impl From<Simd32x2> for [f32; 2] {
    fn from(simd: Simd32x2) -> Self {
        unsafe { simd.f32x2 }
    }
}

impl From<[f32; 4]> for Simd32x4 {
    fn from(f32x4: [f32; 4]) -> Self {
        Self { f32x4 }
    }
}

impl From<[f32; 3]> for Simd32x3 {
    fn from(f32x3: [f32; 3]) -> Self {
        Self { f32x3 }
    }
}

impl From<[f32; 2]> for Simd32x2 {
    fn from(f32x2: [f32; 2]) -> Self {
        Self { f32x2 }
    }
}

impl From<f32> for Simd32x4 {
    fn from(value: f32) -> Self {
        Self {
            f32x4: [value, value, value, value],
        }
    }
}

impl From<f32> for Simd32x3 {
    fn from(value: f32) -> Self {
        Self { f32x3: [value, value, value] }
    }
}

impl From<f32> for Simd32x2 {
    fn from(value: f32) -> Self {
        Self { f32x2: [value, value] }
    }
}

impl std::fmt::Debug for Simd32x4 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.debug_list().entries([self[0], self[1], self[2], self[3]].iter()).finish()
    }
}

impl std::fmt::Debug for Simd32x3 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.debug_list().entries([self[0], self[1], self[2]].iter()).finish()
    }
}

impl std::fmt::Debug for Simd32x2 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.debug_list().entries([self[0], self[1]].iter()).finish()
    }
}

impl std::ops::Add<Simd32x4> for Simd32x4 {
    type Output = Simd32x4;

    fn add(self, other: Self) -> Self {
        match_architecture!(
            Self,
            { f128: _mm_add_ps(self.f128, other.f128) },
            { f128: vaddq_f32(self.f128, other.f128) },
            { v128: f32x4_add(self.v128, other.v128) },
            { f32x4: [
                self.f32x4[0] + other.f32x4[0],
                self.f32x4[1] + other.f32x4[1],
                self.f32x4[2] + other.f32x4[2],
                self.f32x4[3] + other.f32x4[3],
            ] },
        )
    }
}

impl std::ops::Add<Simd32x3> for Simd32x3 {
    type Output = Simd32x3;

    fn add(self, other: Self) -> Self {
        match_architecture!(
            Self,
            { v32x4: unsafe { self.v32x4 + other.v32x4 } },
            { f32x3: [
                self.f32x3[0] + other.f32x3[0],
                self.f32x3[1] + other.f32x3[1],
                self.f32x3[2] + other.f32x3[2],
            ] },
        )
    }
}

impl std::ops::Add<Simd32x2> for Simd32x2 {
    type Output = Simd32x2;

    fn add(self, other: Self) -> Self {
        match_architecture!(
            Self,
            { v32x4: unsafe { self.v32x4 + other.v32x4 } },
            { f32x2: [
                self.f32x2[0] + other.f32x2[0],
                self.f32x2[1] + other.f32x2[1],
            ] },
        )
    }
}

impl std::ops::Sub<Simd32x4> for Simd32x4 {
    type Output = Simd32x4;

    fn sub(self, other: Self) -> Self {
        match_architecture!(
            Self,
            { f128: _mm_sub_ps(self.f128, other.f128) },
            { f128: vsubq_f32(self.f128, other.f128) },
            { v128: f32x4_sub(self.v128, other.v128) },
            { f32x4: [
                self.f32x4[0] - other.f32x4[0],
                self.f32x4[1] - other.f32x4[1],
                self.f32x4[2] - other.f32x4[2],
                self.f32x4[3] - other.f32x4[3],
            ] },
        )
    }
}

impl std::ops::Sub<Simd32x3> for Simd32x3 {
    type Output = Simd32x3;

    fn sub(self, other: Self) -> Self {
        match_architecture!(
            Self,
            { v32x4: unsafe { self.v32x4 - other.v32x4 } },
            { f32x3: [
                self.f32x3[0] - other.f32x3[0],
                self.f32x3[1] - other.f32x3[1],
                self.f32x3[2] - other.f32x3[2],
            ] },
        )
    }
}

impl std::ops::Sub<Simd32x2> for Simd32x2 {
    type Output = Simd32x2;

    fn sub(self, other: Self) -> Self {
        match_architecture!(
            Self,
            { v32x4: unsafe { self.v32x4 - other.v32x4 } },
            { f32x2: [
                self.f32x2[0] - other.f32x2[0],
                self.f32x2[1] - other.f32x2[1],
            ] },
        )
    }
}

impl std::ops::Neg for Simd32x4 {
    type Output = Simd32x4;

    fn neg(self) -> Self {
        Self::from(0.0) - self
    }
}

impl std::ops::Neg for Simd32x3 {
    type Output = Simd32x3;

    fn neg(self) -> Self {
        Self::from(0.0) - self
    }
}

impl std::ops::Neg for Simd32x2 {
    type Output = Simd32x2;

    fn neg(self) -> Self {
        Self::from(0.0) - self
    }
}

impl std::ops::Mul<Simd32x4> for Simd32x4 {
    type Output = Simd32x4;

    fn mul(self, other: Self) -> Self {
        match_architecture!(
            Self,
            { f128: _mm_mul_ps(self.f128, other.f128) },
            { f128: vmulq_f32(self.f128, other.f128) },
            { v128: f32x4_mul(self.v128, other.v128) },
            { f32x4: [
                self.f32x4[0] * other.f32x4[0],
                self.f32x4[1] * other.f32x4[1],
                self.f32x4[2] * other.f32x4[2],
                self.f32x4[3] * other.f32x4[3],
            ] },
        )
    }
}

impl std::ops::Mul<Simd32x3> for Simd32x3 {
    type Output = Simd32x3;

    fn mul(self, other: Self) -> Self {
        match_architecture!(
            Self,
            { v32x4: unsafe { self.v32x4 * other.v32x4 } },
            { f32x3: [
                self.f32x3[0] * other.f32x3[0],
                self.f32x3[1] * other.f32x3[1],
                self.f32x3[2] * other.f32x3[2],
            ] },
        )
    }
}

impl std::ops::Mul<Simd32x2> for Simd32x2 {
    type Output = Simd32x2;

    fn mul(self, other: Self) -> Self {
        match_architecture!(
            Self,
            { v32x4: unsafe { self.v32x4 * other.v32x4 } },
            { f32x2: [
                self.f32x2[0] * other.f32x2[0],
                self.f32x2[1] * other.f32x2[1],
            ] },
        )
    }
}

impl std::ops::Div<Simd32x4> for Simd32x4 {
    type Output = Simd32x4;

    fn div(self, other: Self) -> Self {
        match_architecture!(
            Self,
            { f128: _mm_div_ps(self.f128, other.f128) },
            { f128: vdivq_f32(self.f128, other.f128) },
            { v128: f32x4_div(self.v128, other.v128) },
            { f32x4: [
                self.f32x4[0] / other.f32x4[0],
                self.f32x4[1] / other.f32x4[1],
                self.f32x4[2] / other.f32x4[2],
                self.f32x4[3] / other.f32x4[3],
            ] },
        )
    }
}

impl std::ops::Div<Simd32x3> for Simd32x3 {
    type Output = Simd32x3;

    fn div(self, other: Self) -> Self {
        match_architecture!(
            Self,
            { v32x4: unsafe { self.v32x4 / other.v32x4 } },
            { f32x3: [
                self.f32x3[0] / other.f32x3[0],
                self.f32x3[1] / other.f32x3[1],
                self.f32x3[2] / other.f32x3[2],
            ] },
        )
    }
}

impl std::ops::Div<Simd32x2> for Simd32x2 {
    type Output = Simd32x2;

    fn div(self, other: Self) -> Self {
        match_architecture!(
            Self,
            { v32x4: unsafe { self.v32x4 / other.v32x4 } },
            { f32x2: [
                self.f32x2[0] / other.f32x2[0],
                self.f32x2[1] / other.f32x2[1],
            ] },
        )
    }
}








// encase implementations are based on this code
//  https://github.com/teoxoy/encase/blob/main/src/types/vector.rs



impl encase::private::AsRefVectorParts<f32, 4> for Simd32x4 {
    #[inline]
    fn as_ref_parts(&self) -> &[f32; 4] {
        unsafe { &self.f32x4 }
    }
}
impl encase::private::AsMutVectorParts<f32, 4> for Simd32x4 {
    #[inline]
    fn as_mut_parts(&mut self) -> &mut [f32; 4] {
        unsafe { &mut self.f32x4 }
    }
}
impl encase::private::FromVectorParts<f32, 4> for Simd32x4 {
    #[inline]
    fn from_parts(parts: [f32; 4]) -> Self {
        Simd32x4::from(parts)
    }
}
impl encase::ShaderType for Simd32x4 {
    type ExtraMetadata = ();
    const METADATA: Metadata<Self::ExtraMetadata> = {
        let f32_size = <f32 as encase::private::ShaderSize>::SHADER_SIZE;
        let four = NonZeroU64::new(4u64);
        // const eval can be annoying
        let four_f32s = match four {
            Some(four) => f32_size.saturating_mul(four),
            None => f32_size,
        };
        let size = encase::private::SizeValue::from(four_f32s);
        let alignment = encase::private::AlignmentValue::from_next_power_of_two_size(size);
        Metadata {
            alignment,
            has_uniform_min_alignment: false,
            min_size: size,
            is_pod: <[f32; 4] as encase::ShaderType>::METADATA.is_pod(),
            extra: (),
        }
    };
}
impl encase::ShaderSize for Simd32x4 {}
impl encase::private::WriteInto for Simd32x4 {
    #[inline]
    fn write_into<B>(&self, writer: &mut Writer<B>) where B: BufferMut
    {
        let elements = encase::private::AsRefVectorParts::<f32, 4>::as_ref_parts(self);
        encase::private::WriteInto::write_into(elements, writer);
    }
}
impl encase::private::ReadFrom for Simd32x4 {
    #[inline]
    fn read_from<B>(&mut self, reader: &mut Reader<B>) where B: BufferRef
    {
        let elements = encase::private::AsMutVectorParts::<f32, 4>::as_mut_parts(self);
        encase::private::ReadFrom::read_from(elements, reader);
    }
}
impl encase::private::CreateFrom for Simd32x4 {
    #[inline]
    fn create_from<B>(reader: &mut Reader<B>) -> Self where B: BufferRef
    {
        let elements = encase::private::CreateFrom::create_from(reader);
        encase::private::FromVectorParts::<f32, 4>::from_parts(elements)
    }
}










impl encase::private::AsRefVectorParts<f32, 3> for Simd32x3 {
    #[inline]
    fn as_ref_parts(&self) -> &[f32; 3] {
        unsafe { &self.f32x3 }
    }
}
impl encase::private::AsMutVectorParts<f32, 3> for Simd32x3 {
    #[inline]
    fn as_mut_parts(&mut self) -> &mut [f32; 3] {
        unsafe { &mut self.f32x3 }
    }
}
impl encase::private::FromVectorParts<f32, 3> for Simd32x3 {
    #[inline]
    fn from_parts(parts: [f32; 3]) -> Self {
        Simd32x3::from(parts)
    }
}
impl encase::ShaderType for Simd32x3 {
    type ExtraMetadata = ();
    const METADATA: Metadata<Self::ExtraMetadata> = {
        let f32_size = <f32 as encase::private::ShaderSize>::SHADER_SIZE;
        let three = NonZeroU64::new(3u64);
        // const eval can be annoying
        let three_f32s = match three {
            Some(three) => f32_size.saturating_mul(three),
            None => f32_size,
        };
        let size = encase::private::SizeValue::from(three_f32s);
        let alignment = encase::private::AlignmentValue::from_next_power_of_two_size(size);
        Metadata {
            alignment,
            has_uniform_min_alignment: false,
            min_size: size,
            is_pod: <[f32; 3] as encase::ShaderType>::METADATA.is_pod(),
            extra: (),
        }
    };
}
impl encase::ShaderSize for Simd32x3 {}
impl encase::private::WriteInto for Simd32x3 {
    #[inline]
    fn write_into<B>(&self, writer: &mut Writer<B>) where B: BufferMut
    {
        let elements = encase::private::AsRefVectorParts::<f32, 3>::as_ref_parts(self);
        encase::private::WriteInto::write_into(elements, writer);
    }
}
impl encase::private::ReadFrom for Simd32x3 {
    #[inline]
    fn read_from<B>(&mut self, reader: &mut Reader<B>) where B: BufferRef
    {
        let elements = encase::private::AsMutVectorParts::<f32, 3>::as_mut_parts(self);
        encase::private::ReadFrom::read_from(elements, reader);
    }
}
impl encase::private::CreateFrom for Simd32x3 {
    #[inline]
    fn create_from<B>(reader: &mut Reader<B>) -> Self where B: BufferRef
    {
        let elements = encase::private::CreateFrom::create_from(reader);
        encase::private::FromVectorParts::<f32, 3>::from_parts(elements)
    }
}











impl encase::private::AsRefVectorParts<f32, 2> for Simd32x2 {
    #[inline]
    fn as_ref_parts(&self) -> &[f32; 2] {
        unsafe { &self.f32x2 }
    }
}
impl encase::private::AsMutVectorParts<f32, 2> for Simd32x2 {
    #[inline]
    fn as_mut_parts(&mut self) -> &mut [f32; 2] {
        unsafe { &mut self.f32x2 }
    }
}
impl encase::private::FromVectorParts<f32, 2> for Simd32x2 {
    #[inline]
    fn from_parts(parts: [f32; 2]) -> Self {
        Simd32x2::from(parts)
    }
}
impl encase::ShaderType for Simd32x2 {
    type ExtraMetadata = ();
    const METADATA: Metadata<Self::ExtraMetadata> = {
        let f32_size = <f32 as encase::private::ShaderSize>::SHADER_SIZE;
        let two = NonZeroU64::new(2u64);
        // const eval can be annoying
        let two_f32s = match two {
            Some(two) => f32_size.saturating_mul(two),
            None => f32_size,
        };
        let size = encase::private::SizeValue::from(two_f32s);
        let alignment = encase::private::AlignmentValue::from_next_power_of_two_size(size);
        Metadata {
            alignment,
            has_uniform_min_alignment: false,
            min_size: size,
            is_pod: <[f32; 2] as encase::ShaderType>::METADATA.is_pod(),
            extra: (),
        }
    };
}
impl encase::ShaderSize for Simd32x2 {}
impl encase::private::WriteInto for Simd32x2 {
    #[inline]
    fn write_into<B>(&self, writer: &mut Writer<B>) where B: BufferMut
    {
        let elements = encase::private::AsRefVectorParts::<f32, 2>::as_ref_parts(self);
        encase::private::WriteInto::write_into(elements, writer);
    }
}
impl encase::private::ReadFrom for Simd32x2 {
    #[inline]
    fn read_from<B>(&mut self, reader: &mut Reader<B>) where B: BufferRef
    {
        let elements = encase::private::AsMutVectorParts::<f32, 2>::as_mut_parts(self);
        encase::private::ReadFrom::read_from(elements, reader);
    }
}
impl encase::private::CreateFrom for Simd32x2 {
    #[inline]
    fn create_from<B>(reader: &mut Reader<B>) -> Self where B: BufferRef
    {
        let elements = encase::private::CreateFrom::create_from(reader);
        encase::private::FromVectorParts::<f32, 2>::from_parts(elements)
    }
}










impl NearlyEqTol<Self, f32, f32> for Simd32x4 {}
impl NearlyEqEps<Self, f32, f32> for Simd32x4 {
    fn nearly_eq_eps(&self, other: &Self, eps: &EpsToleranceType<f32, f32>) -> bool {
        let s = unsafe { &self.f32x4 };
        let o = unsafe { &other.f32x4 };
        s.nearly_eq_eps(o, eps)
    }
}
impl NearlyEqUlps<Self, f32, f32> for Simd32x4 {
    fn nearly_eq_ulps(&self, other: &Self, ulps: &UlpsToleranceType<f32, f32>) -> bool {
        let s = unsafe { &self.f32x4 };
        let o = unsafe { &other.f32x4 };
        s.nearly_eq_ulps(o, ulps)
    }
}
impl nearly::NearlyEq<Self, f32, f32> for Simd32x4 {}











impl NearlyEqTol<Self, f32, f32> for Simd32x3 {}
impl NearlyEqEps<Self, f32, f32> for Simd32x3 {
    fn nearly_eq_eps(&self, other: &Self, eps: &EpsToleranceType<f32, f32>) -> bool {
        let s = unsafe { &self.f32x3 };
        let o = unsafe { &other.f32x3 };
        s.nearly_eq_eps(o, eps)
    }
}
impl NearlyEqUlps<Self, f32, f32> for Simd32x3 {
    fn nearly_eq_ulps(&self, other: &Self, ulps: &UlpsToleranceType<f32, f32>) -> bool {
        let s = unsafe { &self.f32x3 };
        let o = unsafe { &other.f32x3 };
        s.nearly_eq_ulps(o, ulps)
    }
}
impl nearly::NearlyEq<Self, f32, f32> for Simd32x3 {}











impl NearlyEqTol<Self, f32, f32> for Simd32x2 {}
impl NearlyEqEps<Self, f32, f32> for Simd32x2 {
    fn nearly_eq_eps(&self, other: &Self, eps: &EpsToleranceType<f32, f32>) -> bool {
        let s = unsafe { &self.f32x2 };
        let o = unsafe { &other.f32x2 };
        s.nearly_eq_eps(o, eps)
    }
}
impl NearlyEqUlps<Self, f32, f32> for Simd32x2 {
    fn nearly_eq_ulps(&self, other: &Self, ulps: &UlpsToleranceType<f32, f32>) -> bool {
        let s = unsafe { &self.f32x2 };
        let o = unsafe { &other.f32x2 };
        s.nearly_eq_ulps(o, ulps)
    }
}
impl nearly::NearlyEq<Self, f32, f32> for Simd32x2 {}




















//