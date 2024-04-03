//
// AUTO-GENERATED - DO NOT MODIFY
//
// To contribute to this file, see the adjacent codegen package.
// https://github.com/AndrewBrownK/geometric_algebra/
//

use geometric_algebra::{simd::*, *};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

pub mod aspect_duals;
pub mod aspects;
pub mod characteristics;
pub mod involutions;
pub mod metrics;
pub mod norms;
pub mod unitize;
pub mod products {
    pub mod contractions;
    pub mod dot;
    pub mod expansions;
    pub mod exterior;
    pub mod geometric;
    pub mod isometries;
    pub mod projections;
}

#[derive(Clone, Copy)]
struct ScalarGroups {
    /// 1
    g0: f32,
}

#[derive(Clone, Copy)]
pub union Scalar {
    groups: ScalarGroups,
    /// 1
    elements: [f32; 1],
}

impl Scalar {
    #[allow(clippy::too_many_arguments)]
    pub const fn new(scalar: f32) -> Self {
        Self { elements: [scalar] }
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

impl std::convert::From<Scalar> for [f32; 1] {
    fn from(vector: Scalar) -> Self {
        unsafe { [vector.elements[0]] }
    }
}

impl std::convert::From<[f32; 1]> for Scalar {
    fn from(array: [f32; 1]) -> Self {
        Self { elements: [array[0]] }
    }
}

impl std::fmt::Debug for Scalar {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.debug_struct("Scalar").field("1", &self[0]).finish()
    }
}

#[derive(Clone, Copy)]
struct AntiScalarGroups {
    /// e12345
    g0: f32,
}

#[derive(Clone, Copy)]
pub union AntiScalar {
    groups: AntiScalarGroups,
    /// e12345
    elements: [f32; 1],
}

impl AntiScalar {
    #[allow(clippy::too_many_arguments)]
    pub const fn new(e12345: f32) -> Self {
        Self { elements: [e12345] }
    }
    pub const fn from_groups(g0: f32) -> Self {
        Self { groups: AntiScalarGroups { g0 } }
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

const ANTISCALAR_INDEX_REMAP: [usize; 1] = [0];

impl std::ops::Index<usize> for AntiScalar {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[ANTISCALAR_INDEX_REMAP[index]] }
    }
}

impl std::ops::IndexMut<usize> for AntiScalar {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[ANTISCALAR_INDEX_REMAP[index]] }
    }
}

impl std::convert::From<AntiScalar> for [f32; 1] {
    fn from(vector: AntiScalar) -> Self {
        unsafe { [vector.elements[0]] }
    }
}

impl std::convert::From<[f32; 1]> for AntiScalar {
    fn from(array: [f32; 1]) -> Self {
        Self { elements: [array[0]] }
    }
}

impl std::fmt::Debug for AntiScalar {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.debug_struct("AntiScalar").field("e12345", &self[0]).finish()
    }
}

#[derive(Clone, Copy)]
struct MagnitudeGroups {
    /// 1, e12345
    g0: Simd32x2,
}

#[derive(Clone, Copy)]
pub union Magnitude {
    groups: MagnitudeGroups,
    /// 1, e12345, 0, 0
    elements: [f32; 4],
}

impl Magnitude {
    #[allow(clippy::too_many_arguments)]
    pub const fn new(scalar: f32, e12345: f32) -> Self {
        Self {
            elements: [scalar, e12345, 0.0, 0.0],
        }
    }
    pub const fn from_groups(g0: Simd32x2) -> Self {
        Self { groups: MagnitudeGroups { g0 } }
    }
    #[inline(always)]
    pub fn group0(&self) -> Simd32x2 {
        unsafe { self.groups.g0 }
    }
    #[inline(always)]
    pub fn group0_mut(&mut self) -> &mut Simd32x2 {
        unsafe { &mut self.groups.g0 }
    }
}

const MAGNITUDE_INDEX_REMAP: [usize; 2] = [0, 1];

impl std::ops::Index<usize> for Magnitude {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[MAGNITUDE_INDEX_REMAP[index]] }
    }
}

impl std::ops::IndexMut<usize> for Magnitude {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[MAGNITUDE_INDEX_REMAP[index]] }
    }
}

impl std::convert::From<Magnitude> for [f32; 2] {
    fn from(vector: Magnitude) -> Self {
        unsafe { [vector.elements[0], vector.elements[1]] }
    }
}

impl std::convert::From<[f32; 2]> for Magnitude {
    fn from(array: [f32; 2]) -> Self {
        Self {
            elements: [array[0], array[1], 0.0, 0.0],
        }
    }
}

impl std::fmt::Debug for Magnitude {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.debug_struct("Magnitude").field("1", &self[0]).field("e12345", &self[1]).finish()
    }
}

#[derive(Clone, Copy)]
struct FlatPointGroups {
    /// e15, e25, e35, e45
    g0: Simd32x4,
}

#[derive(Clone, Copy)]
pub union FlatPoint {
    groups: FlatPointGroups,
    /// e15, e25, e35, e45
    elements: [f32; 4],
}

impl FlatPoint {
    #[allow(clippy::too_many_arguments)]
    pub const fn new(e15: f32, e25: f32, e35: f32, e45: f32) -> Self {
        Self { elements: [e15, e25, e35, e45] }
    }
    pub const fn from_groups(g0: Simd32x4) -> Self {
        Self { groups: FlatPointGroups { g0 } }
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

const FLATPOINT_INDEX_REMAP: [usize; 4] = [0, 1, 2, 3];

impl std::ops::Index<usize> for FlatPoint {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[FLATPOINT_INDEX_REMAP[index]] }
    }
}

impl std::ops::IndexMut<usize> for FlatPoint {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[FLATPOINT_INDEX_REMAP[index]] }
    }
}

impl std::convert::From<FlatPoint> for [f32; 4] {
    fn from(vector: FlatPoint) -> Self {
        unsafe { [vector.elements[0], vector.elements[1], vector.elements[2], vector.elements[3]] }
    }
}

impl std::convert::From<[f32; 4]> for FlatPoint {
    fn from(array: [f32; 4]) -> Self {
        Self {
            elements: [array[0], array[1], array[2], array[3]],
        }
    }
}

impl std::fmt::Debug for FlatPoint {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .debug_struct("FlatPoint")
            .field("e15", &self[0])
            .field("e25", &self[1])
            .field("e35", &self[2])
            .field("e45", &self[3])
            .finish()
    }
}

#[derive(Clone, Copy)]
struct PointAtOriginGroups {
    /// e45
    g0: f32,
}

#[derive(Clone, Copy)]
pub union PointAtOrigin {
    groups: PointAtOriginGroups,
    /// e45
    elements: [f32; 1],
}

impl PointAtOrigin {
    #[allow(clippy::too_many_arguments)]
    pub const fn new(e45: f32) -> Self {
        Self { elements: [e45] }
    }
    pub const fn from_groups(g0: f32) -> Self {
        Self {
            groups: PointAtOriginGroups { g0 },
        }
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

const POINTATORIGIN_INDEX_REMAP: [usize; 1] = [0];

impl std::ops::Index<usize> for PointAtOrigin {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[POINTATORIGIN_INDEX_REMAP[index]] }
    }
}

impl std::ops::IndexMut<usize> for PointAtOrigin {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[POINTATORIGIN_INDEX_REMAP[index]] }
    }
}

impl std::convert::From<PointAtOrigin> for [f32; 1] {
    fn from(vector: PointAtOrigin) -> Self {
        unsafe { [vector.elements[0]] }
    }
}

impl std::convert::From<[f32; 1]> for PointAtOrigin {
    fn from(array: [f32; 1]) -> Self {
        Self { elements: [array[0]] }
    }
}

impl std::fmt::Debug for PointAtOrigin {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.debug_struct("PointAtOrigin").field("e45", &self[0]).finish()
    }
}

#[derive(Clone, Copy)]
struct PointAtInfinityGroups {
    /// e15, e25, e35
    g0: Simd32x3,
}

#[derive(Clone, Copy)]
pub union PointAtInfinity {
    groups: PointAtInfinityGroups,
    /// e15, e25, e35, 0
    elements: [f32; 4],
}

impl PointAtInfinity {
    #[allow(clippy::too_many_arguments)]
    pub const fn new(e15: f32, e25: f32, e35: f32) -> Self {
        Self { elements: [e15, e25, e35, 0.0] }
    }
    pub const fn from_groups(g0: Simd32x3) -> Self {
        Self {
            groups: PointAtInfinityGroups { g0 },
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

const POINTATINFINITY_INDEX_REMAP: [usize; 3] = [0, 1, 2];

impl std::ops::Index<usize> for PointAtInfinity {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[POINTATINFINITY_INDEX_REMAP[index]] }
    }
}

impl std::ops::IndexMut<usize> for PointAtInfinity {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[POINTATINFINITY_INDEX_REMAP[index]] }
    }
}

impl std::convert::From<PointAtInfinity> for [f32; 3] {
    fn from(vector: PointAtInfinity) -> Self {
        unsafe { [vector.elements[0], vector.elements[1], vector.elements[2]] }
    }
}

impl std::convert::From<[f32; 3]> for PointAtInfinity {
    fn from(array: [f32; 3]) -> Self {
        Self {
            elements: [array[0], array[1], array[2], 0.0],
        }
    }
}

impl std::fmt::Debug for PointAtInfinity {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.debug_struct("PointAtInfinity").field("e15", &self[0]).field("e25", &self[1]).field("e35", &self[2]).finish()
    }
}

#[derive(Clone, Copy)]
struct LineGroups {
    /// -e145, -e245, -e345
    g0: Simd32x3,
    /// e235, -e135, e125
    g1: Simd32x3,
}

#[derive(Clone, Copy)]
pub union Line {
    groups: LineGroups,
    /// -e145, -e245, -e345, 0, e235, -e135, e125, 0
    elements: [f32; 8],
}

impl Line {
    #[allow(clippy::too_many_arguments)]
    pub const fn new(neg_e145: f32, neg_e245: f32, neg_e345: f32, e235: f32, neg_e135: f32, e125: f32) -> Self {
        Self {
            elements: [neg_e145, neg_e245, neg_e345, 0.0, e235, neg_e135, e125, 0.0],
        }
    }
    pub const fn from_groups(g0: Simd32x3, g1: Simd32x3) -> Self {
        Self { groups: LineGroups { g0, g1 } }
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
    pub fn group1(&self) -> Simd32x3 {
        unsafe { self.groups.g1 }
    }
    #[inline(always)]
    pub fn group1_mut(&mut self) -> &mut Simd32x3 {
        unsafe { &mut self.groups.g1 }
    }
}

const LINE_INDEX_REMAP: [usize; 6] = [0, 1, 2, 4, 5, 6];

impl std::ops::Index<usize> for Line {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[LINE_INDEX_REMAP[index]] }
    }
}

impl std::ops::IndexMut<usize> for Line {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[LINE_INDEX_REMAP[index]] }
    }
}

impl std::convert::From<Line> for [f32; 6] {
    fn from(vector: Line) -> Self {
        unsafe { [vector.elements[0], vector.elements[1], vector.elements[2], vector.elements[4], vector.elements[5], vector.elements[6]] }
    }
}

impl std::convert::From<[f32; 6]> for Line {
    fn from(array: [f32; 6]) -> Self {
        Self {
            elements: [array[0], array[1], array[2], 0.0, array[3], array[4], array[5], 0.0],
        }
    }
}

impl std::fmt::Debug for Line {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .debug_struct("Line")
            .field("-e145", &self[0])
            .field("-e245", &self[1])
            .field("-e345", &self[2])
            .field("e235", &self[3])
            .field("-e135", &self[4])
            .field("e125", &self[5])
            .finish()
    }
}

#[derive(Clone, Copy)]
struct LineAtOriginGroups {
    /// -e145, -e245, -e345
    g0: Simd32x3,
}

#[derive(Clone, Copy)]
pub union LineAtOrigin {
    groups: LineAtOriginGroups,
    /// -e145, -e245, -e345, 0
    elements: [f32; 4],
}

impl LineAtOrigin {
    #[allow(clippy::too_many_arguments)]
    pub const fn new(neg_e145: f32, neg_e245: f32, neg_e345: f32) -> Self {
        Self {
            elements: [neg_e145, neg_e245, neg_e345, 0.0],
        }
    }
    pub const fn from_groups(g0: Simd32x3) -> Self {
        Self {
            groups: LineAtOriginGroups { g0 },
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

const LINEATORIGIN_INDEX_REMAP: [usize; 3] = [0, 1, 2];

impl std::ops::Index<usize> for LineAtOrigin {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[LINEATORIGIN_INDEX_REMAP[index]] }
    }
}

impl std::ops::IndexMut<usize> for LineAtOrigin {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[LINEATORIGIN_INDEX_REMAP[index]] }
    }
}

impl std::convert::From<LineAtOrigin> for [f32; 3] {
    fn from(vector: LineAtOrigin) -> Self {
        unsafe { [vector.elements[0], vector.elements[1], vector.elements[2]] }
    }
}

impl std::convert::From<[f32; 3]> for LineAtOrigin {
    fn from(array: [f32; 3]) -> Self {
        Self {
            elements: [array[0], array[1], array[2], 0.0],
        }
    }
}

impl std::fmt::Debug for LineAtOrigin {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .debug_struct("LineAtOrigin")
            .field("-e145", &self[0])
            .field("-e245", &self[1])
            .field("-e345", &self[2])
            .finish()
    }
}

#[derive(Clone, Copy)]
struct LineAtInfinityGroups {
    /// e235, -e135, e125
    g0: Simd32x3,
}

#[derive(Clone, Copy)]
pub union LineAtInfinity {
    groups: LineAtInfinityGroups,
    /// e235, -e135, e125, 0
    elements: [f32; 4],
}

impl LineAtInfinity {
    #[allow(clippy::too_many_arguments)]
    pub const fn new(e235: f32, neg_e135: f32, e125: f32) -> Self {
        Self {
            elements: [e235, neg_e135, e125, 0.0],
        }
    }
    pub const fn from_groups(g0: Simd32x3) -> Self {
        Self {
            groups: LineAtInfinityGroups { g0 },
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

const LINEATINFINITY_INDEX_REMAP: [usize; 3] = [0, 1, 2];

impl std::ops::Index<usize> for LineAtInfinity {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[LINEATINFINITY_INDEX_REMAP[index]] }
    }
}

impl std::ops::IndexMut<usize> for LineAtInfinity {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[LINEATINFINITY_INDEX_REMAP[index]] }
    }
}

impl std::convert::From<LineAtInfinity> for [f32; 3] {
    fn from(vector: LineAtInfinity) -> Self {
        unsafe { [vector.elements[0], vector.elements[1], vector.elements[2]] }
    }
}

impl std::convert::From<[f32; 3]> for LineAtInfinity {
    fn from(array: [f32; 3]) -> Self {
        Self {
            elements: [array[0], array[1], array[2], 0.0],
        }
    }
}

impl std::fmt::Debug for LineAtInfinity {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .debug_struct("LineAtInfinity")
            .field("e235", &self[0])
            .field("-e135", &self[1])
            .field("e125", &self[2])
            .finish()
    }
}

#[derive(Clone, Copy)]
struct PlaneGroups {
    /// e2345, -e1345, e1245, -e1235
    g0: Simd32x4,
}

#[derive(Clone, Copy)]
pub union Plane {
    groups: PlaneGroups,
    /// e2345, -e1345, e1245, -e1235
    elements: [f32; 4],
}

impl Plane {
    #[allow(clippy::too_many_arguments)]
    pub const fn new(e2345: f32, neg_e1345: f32, e1245: f32, neg_e1235: f32) -> Self {
        Self {
            elements: [e2345, neg_e1345, e1245, neg_e1235],
        }
    }
    pub const fn from_groups(g0: Simd32x4) -> Self {
        Self { groups: PlaneGroups { g0 } }
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

const PLANE_INDEX_REMAP: [usize; 4] = [0, 1, 2, 3];

impl std::ops::Index<usize> for Plane {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[PLANE_INDEX_REMAP[index]] }
    }
}

impl std::ops::IndexMut<usize> for Plane {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[PLANE_INDEX_REMAP[index]] }
    }
}

impl std::convert::From<Plane> for [f32; 4] {
    fn from(vector: Plane) -> Self {
        unsafe { [vector.elements[0], vector.elements[1], vector.elements[2], vector.elements[3]] }
    }
}

impl std::convert::From<[f32; 4]> for Plane {
    fn from(array: [f32; 4]) -> Self {
        Self {
            elements: [array[0], array[1], array[2], array[3]],
        }
    }
}

impl std::fmt::Debug for Plane {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .debug_struct("Plane")
            .field("e2345", &self[0])
            .field("-e1345", &self[1])
            .field("e1245", &self[2])
            .field("-e1235", &self[3])
            .finish()
    }
}

#[derive(Clone, Copy)]
struct PlaneAtOriginGroups {
    /// e2345, -e1345, e1245
    g0: Simd32x3,
}

#[derive(Clone, Copy)]
pub union PlaneAtOrigin {
    groups: PlaneAtOriginGroups,
    /// e2345, -e1345, e1245, 0
    elements: [f32; 4],
}

impl PlaneAtOrigin {
    #[allow(clippy::too_many_arguments)]
    pub const fn new(e2345: f32, neg_e1345: f32, e1245: f32) -> Self {
        Self {
            elements: [e2345, neg_e1345, e1245, 0.0],
        }
    }
    pub const fn from_groups(g0: Simd32x3) -> Self {
        Self {
            groups: PlaneAtOriginGroups { g0 },
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

const PLANEATORIGIN_INDEX_REMAP: [usize; 3] = [0, 1, 2];

impl std::ops::Index<usize> for PlaneAtOrigin {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[PLANEATORIGIN_INDEX_REMAP[index]] }
    }
}

impl std::ops::IndexMut<usize> for PlaneAtOrigin {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[PLANEATORIGIN_INDEX_REMAP[index]] }
    }
}

impl std::convert::From<PlaneAtOrigin> for [f32; 3] {
    fn from(vector: PlaneAtOrigin) -> Self {
        unsafe { [vector.elements[0], vector.elements[1], vector.elements[2]] }
    }
}

impl std::convert::From<[f32; 3]> for PlaneAtOrigin {
    fn from(array: [f32; 3]) -> Self {
        Self {
            elements: [array[0], array[1], array[2], 0.0],
        }
    }
}

impl std::fmt::Debug for PlaneAtOrigin {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .debug_struct("PlaneAtOrigin")
            .field("e2345", &self[0])
            .field("-e1345", &self[1])
            .field("e1245", &self[2])
            .finish()
    }
}

type PlaneAtInfinity = Horizon;

#[derive(Clone, Copy)]
struct HorizonGroups {
    /// -e1235
    g0: f32,
}

#[derive(Clone, Copy)]
pub union Horizon {
    groups: HorizonGroups,
    /// -e1235
    elements: [f32; 1],
}

impl Horizon {
    #[allow(clippy::too_many_arguments)]
    pub const fn new(neg_e1235: f32) -> Self {
        Self { elements: [neg_e1235] }
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

impl std::convert::From<Horizon> for [f32; 1] {
    fn from(vector: Horizon) -> Self {
        unsafe { [vector.elements[0]] }
    }
}

impl std::convert::From<[f32; 1]> for Horizon {
    fn from(array: [f32; 1]) -> Self {
        Self { elements: [array[0]] }
    }
}

impl std::fmt::Debug for Horizon {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.debug_struct("Horizon").field("-e1235", &self[0]).finish()
    }
}

#[derive(Clone, Copy)]
struct RoundPointGroups {
    /// e1, e2, e3
    g0: Simd32x3,
    /// e4, e5
    g1: Simd32x2,
}

#[derive(Clone, Copy)]
pub union RoundPoint {
    groups: RoundPointGroups,
    /// e1, e2, e3, 0, e4, e5, 0, 0
    elements: [f32; 8],
}

impl RoundPoint {
    #[allow(clippy::too_many_arguments)]
    pub const fn new(e1: f32, e2: f32, e3: f32, e4: f32, e5: f32) -> Self {
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

const ROUNDPOINT_INDEX_REMAP: [usize; 5] = [0, 1, 2, 4, 5];

impl std::ops::Index<usize> for RoundPoint {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[ROUNDPOINT_INDEX_REMAP[index]] }
    }
}

impl std::ops::IndexMut<usize> for RoundPoint {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[ROUNDPOINT_INDEX_REMAP[index]] }
    }
}

impl std::convert::From<RoundPoint> for [f32; 5] {
    fn from(vector: RoundPoint) -> Self {
        unsafe { [vector.elements[0], vector.elements[1], vector.elements[2], vector.elements[4], vector.elements[5]] }
    }
}

impl std::convert::From<[f32; 5]> for RoundPoint {
    fn from(array: [f32; 5]) -> Self {
        Self {
            elements: [array[0], array[1], array[2], 0.0, array[3], array[4], 0.0, 0.0],
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

#[derive(Clone, Copy)]
struct DipoleGroups {
    /// -e14, -e24, -e34
    g0: Simd32x3,
    /// e23, -e13, e12
    g1: Simd32x3,
    /// e15, e25, e35, e45
    g2: Simd32x4,
}

#[derive(Clone, Copy)]
pub union Dipole {
    groups: DipoleGroups,
    /// -e14, -e24, -e34, 0, e23, -e13, e12, 0, e15, e25, e35, e45
    elements: [f32; 12],
}

impl Dipole {
    #[allow(clippy::too_many_arguments)]
    pub const fn new(neg_e14: f32, neg_e24: f32, neg_e34: f32, e23: f32, neg_e13: f32, e12: f32, e15: f32, e25: f32, e35: f32, e45: f32) -> Self {
        Self {
            elements: [neg_e14, neg_e24, neg_e34, 0.0, e23, neg_e13, e12, 0.0, e15, e25, e35, e45],
        }
    }
    pub const fn from_groups(g0: Simd32x3, g1: Simd32x3, g2: Simd32x4) -> Self {
        Self {
            groups: DipoleGroups { g0, g1, g2 },
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
    pub fn group1(&self) -> Simd32x3 {
        unsafe { self.groups.g1 }
    }
    #[inline(always)]
    pub fn group1_mut(&mut self) -> &mut Simd32x3 {
        unsafe { &mut self.groups.g1 }
    }
    #[inline(always)]
    pub fn group2(&self) -> Simd32x4 {
        unsafe { self.groups.g2 }
    }
    #[inline(always)]
    pub fn group2_mut(&mut self) -> &mut Simd32x4 {
        unsafe { &mut self.groups.g2 }
    }
}

const DIPOLE_INDEX_REMAP: [usize; 10] = [0, 1, 2, 4, 5, 6, 8, 9, 10, 11];

impl std::ops::Index<usize> for Dipole {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[DIPOLE_INDEX_REMAP[index]] }
    }
}

impl std::ops::IndexMut<usize> for Dipole {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[DIPOLE_INDEX_REMAP[index]] }
    }
}

impl std::convert::From<Dipole> for [f32; 10] {
    fn from(vector: Dipole) -> Self {
        unsafe {
            [
                vector.elements[0], vector.elements[1], vector.elements[2], vector.elements[4], vector.elements[5], vector.elements[6], vector.elements[8], vector.elements[9],
                vector.elements[10], vector.elements[11],
            ]
        }
    }
}

impl std::convert::From<[f32; 10]> for Dipole {
    fn from(array: [f32; 10]) -> Self {
        Self {
            elements: [array[0], array[1], array[2], 0.0, array[3], array[4], array[5], 0.0, array[6], array[7], array[8], array[9]],
        }
    }
}

impl std::fmt::Debug for Dipole {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .debug_struct("Dipole")
            .field("-e14", &self[0])
            .field("-e24", &self[1])
            .field("-e34", &self[2])
            .field("e23", &self[3])
            .field("-e13", &self[4])
            .field("e12", &self[5])
            .field("e15", &self[6])
            .field("e25", &self[7])
            .field("e35", &self[8])
            .field("e45", &self[9])
            .finish()
    }
}

#[derive(Clone, Copy)]
struct CircleGroups {
    /// e234, -e134, e124, -e123
    g0: Simd32x4,
    /// -e145, -e245, -e345
    g1: Simd32x3,
    /// e235, -e135, e125
    g2: Simd32x3,
}

#[derive(Clone, Copy)]
pub union Circle {
    groups: CircleGroups,
    /// e234, -e134, e124, -e123, -e145, -e245, -e345, 0, e235, -e135, e125, 0
    elements: [f32; 12],
}

impl Circle {
    #[allow(clippy::too_many_arguments)]
    pub const fn new(e234: f32, neg_e134: f32, e124: f32, neg_e123: f32, neg_e145: f32, neg_e245: f32, neg_e345: f32, e235: f32, neg_e135: f32, e125: f32) -> Self {
        Self {
            elements: [e234, neg_e134, e124, neg_e123, neg_e145, neg_e245, neg_e345, 0.0, e235, neg_e135, e125, 0.0],
        }
    }
    pub const fn from_groups(g0: Simd32x4, g1: Simd32x3, g2: Simd32x3) -> Self {
        Self {
            groups: CircleGroups { g0, g1, g2 },
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
    #[inline(always)]
    pub fn group2(&self) -> Simd32x3 {
        unsafe { self.groups.g2 }
    }
    #[inline(always)]
    pub fn group2_mut(&mut self) -> &mut Simd32x3 {
        unsafe { &mut self.groups.g2 }
    }
}

const CIRCLE_INDEX_REMAP: [usize; 10] = [0, 1, 2, 3, 4, 5, 6, 8, 9, 10];

impl std::ops::Index<usize> for Circle {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[CIRCLE_INDEX_REMAP[index]] }
    }
}

impl std::ops::IndexMut<usize> for Circle {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[CIRCLE_INDEX_REMAP[index]] }
    }
}

impl std::convert::From<Circle> for [f32; 10] {
    fn from(vector: Circle) -> Self {
        unsafe {
            [
                vector.elements[0], vector.elements[1], vector.elements[2], vector.elements[3], vector.elements[4], vector.elements[5], vector.elements[6], vector.elements[8],
                vector.elements[9], vector.elements[10],
            ]
        }
    }
}

impl std::convert::From<[f32; 10]> for Circle {
    fn from(array: [f32; 10]) -> Self {
        Self {
            elements: [array[0], array[1], array[2], array[3], array[4], array[5], array[6], 0.0, array[7], array[8], array[9], 0.0],
        }
    }
}

impl std::fmt::Debug for Circle {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .debug_struct("Circle")
            .field("e234", &self[0])
            .field("-e134", &self[1])
            .field("e124", &self[2])
            .field("-e123", &self[3])
            .field("-e145", &self[4])
            .field("-e245", &self[5])
            .field("-e345", &self[6])
            .field("e235", &self[7])
            .field("-e135", &self[8])
            .field("e125", &self[9])
            .finish()
    }
}

#[derive(Clone, Copy)]
struct SphereGroups {
    /// e2345, -e1345, e1245
    g0: Simd32x3,
    /// e1234, -e1235
    g1: Simd32x2,
}

#[derive(Clone, Copy)]
pub union Sphere {
    groups: SphereGroups,
    /// e2345, -e1345, e1245, 0, e1234, -e1235, 0, 0
    elements: [f32; 8],
}

impl Sphere {
    #[allow(clippy::too_many_arguments)]
    pub const fn new(e2345: f32, neg_e1345: f32, e1245: f32, e1234: f32, neg_e1235: f32) -> Self {
        Self {
            elements: [e2345, neg_e1345, e1245, 0.0, e1234, neg_e1235, 0.0, 0.0],
        }
    }
    pub const fn from_groups(g0: Simd32x3, g1: Simd32x2) -> Self {
        Self { groups: SphereGroups { g0, g1 } }
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

const SPHERE_INDEX_REMAP: [usize; 5] = [0, 1, 2, 4, 5];

impl std::ops::Index<usize> for Sphere {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[SPHERE_INDEX_REMAP[index]] }
    }
}

impl std::ops::IndexMut<usize> for Sphere {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[SPHERE_INDEX_REMAP[index]] }
    }
}

impl std::convert::From<Sphere> for [f32; 5] {
    fn from(vector: Sphere) -> Self {
        unsafe { [vector.elements[0], vector.elements[1], vector.elements[2], vector.elements[4], vector.elements[5]] }
    }
}

impl std::convert::From<[f32; 5]> for Sphere {
    fn from(array: [f32; 5]) -> Self {
        Self {
            elements: [array[0], array[1], array[2], 0.0, array[3], array[4], 0.0, 0.0],
        }
    }
}

impl std::fmt::Debug for Sphere {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .debug_struct("Sphere")
            .field("e2345", &self[0])
            .field("-e1345", &self[1])
            .field("e1245", &self[2])
            .field("e1234", &self[3])
            .field("-e1235", &self[4])
            .finish()
    }
}

#[derive(Clone, Copy)]
struct InfinityGroups {
    /// e5
    g0: f32,
}

#[derive(Clone, Copy)]
pub union Infinity {
    groups: InfinityGroups,
    /// e5
    elements: [f32; 1],
}

impl Infinity {
    #[allow(clippy::too_many_arguments)]
    pub const fn new(e5: f32) -> Self {
        Self { elements: [e5] }
    }
    pub const fn from_groups(g0: f32) -> Self {
        Self { groups: InfinityGroups { g0 } }
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

const INFINITY_INDEX_REMAP: [usize; 1] = [0];

impl std::ops::Index<usize> for Infinity {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[INFINITY_INDEX_REMAP[index]] }
    }
}

impl std::ops::IndexMut<usize> for Infinity {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[INFINITY_INDEX_REMAP[index]] }
    }
}

impl std::convert::From<Infinity> for [f32; 1] {
    fn from(vector: Infinity) -> Self {
        unsafe { [vector.elements[0]] }
    }
}

impl std::convert::From<[f32; 1]> for Infinity {
    fn from(array: [f32; 1]) -> Self {
        Self { elements: [array[0]] }
    }
}

impl std::fmt::Debug for Infinity {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.debug_struct("Infinity").field("e5", &self[0]).finish()
    }
}

#[derive(Clone, Copy)]
struct OriginGroups {
    /// e4
    g0: f32,
}

#[derive(Clone, Copy)]
pub union Origin {
    groups: OriginGroups,
    /// e4
    elements: [f32; 1],
}

impl Origin {
    #[allow(clippy::too_many_arguments)]
    pub const fn new(e4: f32) -> Self {
        Self { elements: [e4] }
    }
    pub const fn from_groups(g0: f32) -> Self {
        Self { groups: OriginGroups { g0 } }
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

const ORIGIN_INDEX_REMAP: [usize; 1] = [0];

impl std::ops::Index<usize> for Origin {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[ORIGIN_INDEX_REMAP[index]] }
    }
}

impl std::ops::IndexMut<usize> for Origin {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[ORIGIN_INDEX_REMAP[index]] }
    }
}

impl std::convert::From<Origin> for [f32; 1] {
    fn from(vector: Origin) -> Self {
        unsafe { [vector.elements[0]] }
    }
}

impl std::convert::From<[f32; 1]> for Origin {
    fn from(array: [f32; 1]) -> Self {
        Self { elements: [array[0]] }
    }
}

impl std::fmt::Debug for Origin {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.debug_struct("Origin").field("e4", &self[0]).finish()
    }
}

#[derive(Clone, Copy)]
struct MotorGroups {
    /// -e145, -e245, -e345, e12345
    g0: Simd32x4,
    /// e235, -e135, e125
    g1: Simd32x3,
}

#[derive(Clone, Copy)]
pub union Motor {
    groups: MotorGroups,
    /// -e145, -e245, -e345, e12345, e235, -e135, e125, 0
    elements: [f32; 8],
}

impl Motor {
    #[allow(clippy::too_many_arguments)]
    pub const fn new(neg_e145: f32, neg_e245: f32, neg_e345: f32, e12345: f32, e235: f32, neg_e135: f32, e125: f32) -> Self {
        Self {
            elements: [neg_e145, neg_e245, neg_e345, e12345, e235, neg_e135, e125, 0.0],
        }
    }
    pub const fn from_groups(g0: Simd32x4, g1: Simd32x3) -> Self {
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
    pub fn group1(&self) -> Simd32x3 {
        unsafe { self.groups.g1 }
    }
    #[inline(always)]
    pub fn group1_mut(&mut self) -> &mut Simd32x3 {
        unsafe { &mut self.groups.g1 }
    }
}

const MOTOR_INDEX_REMAP: [usize; 7] = [0, 1, 2, 3, 4, 5, 6];

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

impl std::convert::From<Motor> for [f32; 7] {
    fn from(vector: Motor) -> Self {
        unsafe {
            [
                vector.elements[0], vector.elements[1], vector.elements[2], vector.elements[3], vector.elements[4], vector.elements[5], vector.elements[6],
            ]
        }
    }
}

impl std::convert::From<[f32; 7]> for Motor {
    fn from(array: [f32; 7]) -> Self {
        Self {
            elements: [array[0], array[1], array[2], array[3], array[4], array[5], array[6], 0.0],
        }
    }
}

impl std::fmt::Debug for Motor {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .debug_struct("Motor")
            .field("-e145", &self[0])
            .field("-e245", &self[1])
            .field("-e345", &self[2])
            .field("e12345", &self[3])
            .field("e235", &self[4])
            .field("-e135", &self[5])
            .field("e125", &self[6])
            .finish()
    }
}

#[derive(Clone, Copy)]
struct RotorGroups {
    /// -e145, -e245, -e345, e12345
    g0: Simd32x4,
}

#[derive(Clone, Copy)]
pub union Rotor {
    groups: RotorGroups,
    /// -e145, -e245, -e345, e12345
    elements: [f32; 4],
}

impl Rotor {
    #[allow(clippy::too_many_arguments)]
    pub const fn new(neg_e145: f32, neg_e245: f32, neg_e345: f32, e12345: f32) -> Self {
        Self {
            elements: [neg_e145, neg_e245, neg_e345, e12345],
        }
    }
    pub const fn from_groups(g0: Simd32x4) -> Self {
        Self { groups: RotorGroups { g0 } }
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

const ROTOR_INDEX_REMAP: [usize; 4] = [0, 1, 2, 3];

impl std::ops::Index<usize> for Rotor {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[ROTOR_INDEX_REMAP[index]] }
    }
}

impl std::ops::IndexMut<usize> for Rotor {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[ROTOR_INDEX_REMAP[index]] }
    }
}

impl std::convert::From<Rotor> for [f32; 4] {
    fn from(vector: Rotor) -> Self {
        unsafe { [vector.elements[0], vector.elements[1], vector.elements[2], vector.elements[3]] }
    }
}

impl std::convert::From<[f32; 4]> for Rotor {
    fn from(array: [f32; 4]) -> Self {
        Self {
            elements: [array[0], array[1], array[2], array[3]],
        }
    }
}

impl std::fmt::Debug for Rotor {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .debug_struct("Rotor")
            .field("-e145", &self[0])
            .field("-e245", &self[1])
            .field("-e345", &self[2])
            .field("e12345", &self[3])
            .finish()
    }
}

#[derive(Clone, Copy)]
struct TranslatorGroups {
    /// e235, -e135, e125, e12345
    g0: Simd32x4,
}

#[derive(Clone, Copy)]
pub union Translator {
    groups: TranslatorGroups,
    /// e235, -e135, e125, e12345
    elements: [f32; 4],
}

impl Translator {
    #[allow(clippy::too_many_arguments)]
    pub const fn new(e235: f32, neg_e135: f32, e125: f32, e12345: f32) -> Self {
        Self {
            elements: [e235, neg_e135, e125, e12345],
        }
    }
    pub const fn from_groups(g0: Simd32x4) -> Self {
        Self { groups: TranslatorGroups { g0 } }
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

const TRANSLATOR_INDEX_REMAP: [usize; 4] = [0, 1, 2, 3];

impl std::ops::Index<usize> for Translator {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[TRANSLATOR_INDEX_REMAP[index]] }
    }
}

impl std::ops::IndexMut<usize> for Translator {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[TRANSLATOR_INDEX_REMAP[index]] }
    }
}

impl std::convert::From<Translator> for [f32; 4] {
    fn from(vector: Translator) -> Self {
        unsafe { [vector.elements[0], vector.elements[1], vector.elements[2], vector.elements[3]] }
    }
}

impl std::convert::From<[f32; 4]> for Translator {
    fn from(array: [f32; 4]) -> Self {
        Self {
            elements: [array[0], array[1], array[2], array[3]],
        }
    }
}

impl std::fmt::Debug for Translator {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .debug_struct("Translator")
            .field("e235", &self[0])
            .field("-e135", &self[1])
            .field("e125", &self[2])
            .field("e12345", &self[3])
            .finish()
    }
}

#[derive(Clone, Copy)]
struct FlectorGroups {
    /// e15, e25, e35, e45
    g0: Simd32x4,
    /// e2345, -e1345, e1245, -e1235
    g1: Simd32x4,
}

#[derive(Clone, Copy)]
pub union Flector {
    groups: FlectorGroups,
    /// e15, e25, e35, e45, e2345, -e1345, e1245, -e1235
    elements: [f32; 8],
}

impl Flector {
    #[allow(clippy::too_many_arguments)]
    pub const fn new(e15: f32, e25: f32, e35: f32, e45: f32, e2345: f32, neg_e1345: f32, e1245: f32, neg_e1235: f32) -> Self {
        Self {
            elements: [e15, e25, e35, e45, e2345, neg_e1345, e1245, neg_e1235],
        }
    }
    pub const fn from_groups(g0: Simd32x4, g1: Simd32x4) -> Self {
        Self { groups: FlectorGroups { g0, g1 } }
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

const FLECTOR_INDEX_REMAP: [usize; 8] = [0, 1, 2, 3, 4, 5, 6, 7];

impl std::ops::Index<usize> for Flector {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[FLECTOR_INDEX_REMAP[index]] }
    }
}

impl std::ops::IndexMut<usize> for Flector {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[FLECTOR_INDEX_REMAP[index]] }
    }
}

impl std::convert::From<Flector> for [f32; 8] {
    fn from(vector: Flector) -> Self {
        unsafe {
            [
                vector.elements[0], vector.elements[1], vector.elements[2], vector.elements[3], vector.elements[4], vector.elements[5], vector.elements[6], vector.elements[7],
            ]
        }
    }
}

impl std::convert::From<[f32; 8]> for Flector {
    fn from(array: [f32; 8]) -> Self {
        Self {
            elements: [array[0], array[1], array[2], array[3], array[4], array[5], array[6], array[7]],
        }
    }
}

impl std::fmt::Debug for Flector {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .debug_struct("Flector")
            .field("e15", &self[0])
            .field("e25", &self[1])
            .field("e35", &self[2])
            .field("e45", &self[3])
            .field("e2345", &self[4])
            .field("-e1345", &self[5])
            .field("e1245", &self[6])
            .field("-e1235", &self[7])
            .finish()
    }
}

#[derive(Clone, Copy)]
struct MultiVectorGroups {
    /// 1, e12345
    g0: Simd32x2,
    /// e1, e2, e3
    g1: Simd32x3,
    /// e4, e5
    g2: Simd32x2,
    /// -e14, -e24, -e34
    g3: Simd32x3,
    /// e23, -e13, e12
    g4: Simd32x3,
    /// e15, e25, e35, e45
    g5: Simd32x4,
    /// e234, -e134, e124, -e123
    g6: Simd32x4,
    /// -e145, -e245, -e345
    g7: Simd32x3,
    /// e235, -e135, e125
    g8: Simd32x3,
    /// e2345, -e1345, e1245
    g9: Simd32x3,
    /// e1234, -e1235
    g10: Simd32x2,
}

#[derive(Clone, Copy)]
pub union MultiVector {
    groups: MultiVectorGroups,
    /// 1, e12345, 0, 0, e1, e2, e3, 0, e4, e5, 0, 0, -e14, -e24, -e34, 0, e23, -e13, e12, 0, e15, e25, e35, e45, e234, -e134, e124, -e123, -e145, -e245, -e345, 0, e235, -e135, e125, 0, e2345, -e1345, e1245, 0, e1234, -e1235, 0, 0
    elements: [f32; 44],
}

impl MultiVector {
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        scalar: f32,
        e12345: f32,
        e1: f32,
        e2: f32,
        e3: f32,
        e4: f32,
        e5: f32,
        neg_e14: f32,
        neg_e24: f32,
        neg_e34: f32,
        e23: f32,
        neg_e13: f32,
        e12: f32,
        e15: f32,
        e25: f32,
        e35: f32,
        e45: f32,
        e234: f32,
        neg_e134: f32,
        e124: f32,
        neg_e123: f32,
        neg_e145: f32,
        neg_e245: f32,
        neg_e345: f32,
        e235: f32,
        neg_e135: f32,
        e125: f32,
        e2345: f32,
        neg_e1345: f32,
        e1245: f32,
        e1234: f32,
        neg_e1235: f32,
    ) -> Self {
        Self {
            elements: [
                scalar, e12345, 0.0, 0.0, e1, e2, e3, 0.0, e4, e5, 0.0, 0.0, neg_e14, neg_e24, neg_e34, 0.0, e23, neg_e13, e12, 0.0, e15, e25, e35, e45, e234, neg_e134, e124,
                neg_e123, neg_e145, neg_e245, neg_e345, 0.0, e235, neg_e135, e125, 0.0, e2345, neg_e1345, e1245, 0.0, e1234, neg_e1235, 0.0, 0.0,
            ],
        }
    }
    pub const fn from_groups(
        g0: Simd32x2,
        g1: Simd32x3,
        g2: Simd32x2,
        g3: Simd32x3,
        g4: Simd32x3,
        g5: Simd32x4,
        g6: Simd32x4,
        g7: Simd32x3,
        g8: Simd32x3,
        g9: Simd32x3,
        g10: Simd32x2,
    ) -> Self {
        Self {
            groups: MultiVectorGroups {
                g0,
                g1,
                g2,
                g3,
                g4,
                g5,
                g6,
                g7,
                g8,
                g9,
                g10,
            },
        }
    }
    #[inline(always)]
    pub fn group0(&self) -> Simd32x2 {
        unsafe { self.groups.g0 }
    }
    #[inline(always)]
    pub fn group0_mut(&mut self) -> &mut Simd32x2 {
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
    #[inline(always)]
    pub fn group2(&self) -> Simd32x2 {
        unsafe { self.groups.g2 }
    }
    #[inline(always)]
    pub fn group2_mut(&mut self) -> &mut Simd32x2 {
        unsafe { &mut self.groups.g2 }
    }
    #[inline(always)]
    pub fn group3(&self) -> Simd32x3 {
        unsafe { self.groups.g3 }
    }
    #[inline(always)]
    pub fn group3_mut(&mut self) -> &mut Simd32x3 {
        unsafe { &mut self.groups.g3 }
    }
    #[inline(always)]
    pub fn group4(&self) -> Simd32x3 {
        unsafe { self.groups.g4 }
    }
    #[inline(always)]
    pub fn group4_mut(&mut self) -> &mut Simd32x3 {
        unsafe { &mut self.groups.g4 }
    }
    #[inline(always)]
    pub fn group5(&self) -> Simd32x4 {
        unsafe { self.groups.g5 }
    }
    #[inline(always)]
    pub fn group5_mut(&mut self) -> &mut Simd32x4 {
        unsafe { &mut self.groups.g5 }
    }
    #[inline(always)]
    pub fn group6(&self) -> Simd32x4 {
        unsafe { self.groups.g6 }
    }
    #[inline(always)]
    pub fn group6_mut(&mut self) -> &mut Simd32x4 {
        unsafe { &mut self.groups.g6 }
    }
    #[inline(always)]
    pub fn group7(&self) -> Simd32x3 {
        unsafe { self.groups.g7 }
    }
    #[inline(always)]
    pub fn group7_mut(&mut self) -> &mut Simd32x3 {
        unsafe { &mut self.groups.g7 }
    }
    #[inline(always)]
    pub fn group8(&self) -> Simd32x3 {
        unsafe { self.groups.g8 }
    }
    #[inline(always)]
    pub fn group8_mut(&mut self) -> &mut Simd32x3 {
        unsafe { &mut self.groups.g8 }
    }
    #[inline(always)]
    pub fn group9(&self) -> Simd32x3 {
        unsafe { self.groups.g9 }
    }
    #[inline(always)]
    pub fn group9_mut(&mut self) -> &mut Simd32x3 {
        unsafe { &mut self.groups.g9 }
    }
    #[inline(always)]
    pub fn group10(&self) -> Simd32x2 {
        unsafe { self.groups.g10 }
    }
    #[inline(always)]
    pub fn group10_mut(&mut self) -> &mut Simd32x2 {
        unsafe { &mut self.groups.g10 }
    }
}

const MULTIVECTOR_INDEX_REMAP: [usize; 32] = [0, 1, 4, 5, 6, 8, 9, 12, 13, 14, 16, 17, 18, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 32, 33, 34, 36, 37, 38, 40, 41];

impl std::ops::Index<usize> for MultiVector {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[MULTIVECTOR_INDEX_REMAP[index]] }
    }
}

impl std::ops::IndexMut<usize> for MultiVector {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[MULTIVECTOR_INDEX_REMAP[index]] }
    }
}

impl std::convert::From<MultiVector> for [f32; 32] {
    fn from(vector: MultiVector) -> Self {
        unsafe {
            [
                vector.elements[0], vector.elements[1], vector.elements[4], vector.elements[5], vector.elements[6], vector.elements[8], vector.elements[9], vector.elements[12],
                vector.elements[13], vector.elements[14], vector.elements[16], vector.elements[17], vector.elements[18], vector.elements[20], vector.elements[21],
                vector.elements[22], vector.elements[23], vector.elements[24], vector.elements[25], vector.elements[26], vector.elements[27], vector.elements[28],
                vector.elements[29], vector.elements[30], vector.elements[32], vector.elements[33], vector.elements[34], vector.elements[36], vector.elements[37],
                vector.elements[38], vector.elements[40], vector.elements[41],
            ]
        }
    }
}

impl std::convert::From<[f32; 32]> for MultiVector {
    fn from(array: [f32; 32]) -> Self {
        Self {
            elements: [
                array[0], array[1], 0.0, 0.0, array[2], array[3], array[4], 0.0, array[5], array[6], 0.0, 0.0, array[7], array[8], array[9], 0.0, array[10], array[11], array[12],
                0.0, array[13], array[14], array[15], array[16], array[17], array[18], array[19], array[20], array[21], array[22], array[23], 0.0, array[24], array[25], array[26],
                0.0, array[27], array[28], array[29], 0.0, array[30], array[31], 0.0, 0.0,
            ],
        }
    }
}

impl std::fmt::Debug for MultiVector {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .debug_struct("MultiVector")
            .field("1", &self[0])
            .field("e12345", &self[1])
            .field("e1", &self[2])
            .field("e2", &self[3])
            .field("e3", &self[4])
            .field("e4", &self[5])
            .field("e5", &self[6])
            .field("-e14", &self[7])
            .field("-e24", &self[8])
            .field("-e34", &self[9])
            .field("e23", &self[10])
            .field("-e13", &self[11])
            .field("e12", &self[12])
            .field("e15", &self[13])
            .field("e25", &self[14])
            .field("e35", &self[15])
            .field("e45", &self[16])
            .field("e234", &self[17])
            .field("-e134", &self[18])
            .field("e124", &self[19])
            .field("-e123", &self[20])
            .field("-e145", &self[21])
            .field("-e245", &self[22])
            .field("-e345", &self[23])
            .field("e235", &self[24])
            .field("-e135", &self[25])
            .field("e125", &self[26])
            .field("e2345", &self[27])
            .field("-e1345", &self[28])
            .field("e1245", &self[29])
            .field("e1234", &self[30])
            .field("-e1235", &self[31])
            .finish()
    }
}

impl One for AntiScalar {
    fn one() -> Self {
        AntiScalar {
            groups: AntiScalarGroups { g0: 0.0 },
        }
    }
}

impl One for Circle {
    fn one() -> Self {
        Circle {
            groups: CircleGroups {
                g0: Simd32x4::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x3::from(0.0),
            },
        }
    }
}

impl One for Dipole {
    fn one() -> Self {
        Dipole {
            groups: DipoleGroups {
                g0: Simd32x3::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x4::from(0.0),
            },
        }
    }
}

impl One for FlatPoint {
    fn one() -> Self {
        FlatPoint {
            groups: FlatPointGroups { g0: Simd32x4::from(0.0) },
        }
    }
}

impl One for Flector {
    fn one() -> Self {
        Flector {
            groups: FlectorGroups {
                g0: Simd32x4::from(0.0),
                g1: Simd32x4::from(0.0),
            },
        }
    }
}

impl One for Horizon {
    fn one() -> Self {
        Horizon {
            groups: HorizonGroups { g0: 0.0 },
        }
    }
}

impl One for Infinity {
    fn one() -> Self {
        Infinity {
            groups: InfinityGroups { g0: 0.0 },
        }
    }
}

impl One for Line {
    fn one() -> Self {
        Line {
            groups: LineGroups {
                g0: Simd32x3::from(0.0),
                g1: Simd32x3::from(0.0),
            },
        }
    }
}

impl One for LineAtInfinity {
    fn one() -> Self {
        LineAtInfinity {
            groups: LineAtInfinityGroups { g0: Simd32x3::from(0.0) },
        }
    }
}

impl One for LineAtOrigin {
    fn one() -> Self {
        LineAtOrigin {
            groups: LineAtOriginGroups { g0: Simd32x3::from(0.0) },
        }
    }
}

impl One for Magnitude {
    fn one() -> Self {
        Magnitude {
            groups: MagnitudeGroups { g0: Simd32x2::from([1.0, 0.0]) },
        }
    }
}

impl One for Motor {
    fn one() -> Self {
        Motor {
            groups: MotorGroups {
                g0: Simd32x4::from(0.0),
                g1: Simd32x3::from(0.0),
            },
        }
    }
}

impl One for MultiVector {
    fn one() -> Self {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from([1.0, 0.0]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl One for Origin {
    fn one() -> Self {
        Origin { groups: OriginGroups { g0: 0.0 } }
    }
}

impl One for Plane {
    fn one() -> Self {
        Plane {
            groups: PlaneGroups { g0: Simd32x4::from(0.0) },
        }
    }
}

impl One for PlaneAtOrigin {
    fn one() -> Self {
        PlaneAtOrigin {
            groups: PlaneAtOriginGroups { g0: Simd32x3::from(0.0) },
        }
    }
}

impl One for PointAtInfinity {
    fn one() -> Self {
        PointAtInfinity {
            groups: PointAtInfinityGroups { g0: Simd32x3::from(0.0) },
        }
    }
}

impl One for PointAtOrigin {
    fn one() -> Self {
        PointAtOrigin {
            groups: PointAtOriginGroups { g0: 0.0 },
        }
    }
}

impl One for Rotor {
    fn one() -> Self {
        Rotor {
            groups: RotorGroups { g0: Simd32x4::from(0.0) },
        }
    }
}

impl One for RoundPoint {
    fn one() -> Self {
        RoundPoint {
            groups: RoundPointGroups {
                g0: Simd32x3::from(0.0),
                g1: Simd32x2::from(0.0),
            },
        }
    }
}

impl One for Scalar {
    fn one() -> Self {
        Scalar { groups: ScalarGroups { g0: 1.0 } }
    }
}

impl One for Sphere {
    fn one() -> Self {
        Sphere {
            groups: SphereGroups {
                g0: Simd32x3::from(0.0),
                g1: Simd32x2::from(0.0),
            },
        }
    }
}

impl One for Translator {
    fn one() -> Self {
        Translator {
            groups: TranslatorGroups { g0: Simd32x4::from(0.0) },
        }
    }
}

impl Zero for AntiScalar {
    fn zero() -> Self {
        AntiScalar {
            groups: AntiScalarGroups { g0: 0.0 },
        }
    }
}

impl Zero for Circle {
    fn zero() -> Self {
        Circle {
            groups: CircleGroups {
                g0: Simd32x4::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x3::from(0.0),
            },
        }
    }
}

impl Zero for Dipole {
    fn zero() -> Self {
        Dipole {
            groups: DipoleGroups {
                g0: Simd32x3::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x4::from(0.0),
            },
        }
    }
}

impl Zero for FlatPoint {
    fn zero() -> Self {
        FlatPoint {
            groups: FlatPointGroups { g0: Simd32x4::from(0.0) },
        }
    }
}

impl Zero for Flector {
    fn zero() -> Self {
        Flector {
            groups: FlectorGroups {
                g0: Simd32x4::from(0.0),
                g1: Simd32x4::from(0.0),
            },
        }
    }
}

impl Zero for Horizon {
    fn zero() -> Self {
        Horizon {
            groups: HorizonGroups { g0: 0.0 },
        }
    }
}

impl Zero for Infinity {
    fn zero() -> Self {
        Infinity {
            groups: InfinityGroups { g0: 0.0 },
        }
    }
}

impl Zero for Line {
    fn zero() -> Self {
        Line {
            groups: LineGroups {
                g0: Simd32x3::from(0.0),
                g1: Simd32x3::from(0.0),
            },
        }
    }
}

impl Zero for LineAtInfinity {
    fn zero() -> Self {
        LineAtInfinity {
            groups: LineAtInfinityGroups { g0: Simd32x3::from(0.0) },
        }
    }
}

impl Zero for LineAtOrigin {
    fn zero() -> Self {
        LineAtOrigin {
            groups: LineAtOriginGroups { g0: Simd32x3::from(0.0) },
        }
    }
}

impl Zero for Magnitude {
    fn zero() -> Self {
        Magnitude {
            groups: MagnitudeGroups { g0: Simd32x2::from(0.0) },
        }
    }
}

impl Zero for Motor {
    fn zero() -> Self {
        Motor {
            groups: MotorGroups {
                g0: Simd32x4::from(0.0),
                g1: Simd32x3::from(0.0),
            },
        }
    }
}

impl Zero for MultiVector {
    fn zero() -> Self {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Zero for Origin {
    fn zero() -> Self {
        Origin { groups: OriginGroups { g0: 0.0 } }
    }
}

impl Zero for Plane {
    fn zero() -> Self {
        Plane {
            groups: PlaneGroups { g0: Simd32x4::from(0.0) },
        }
    }
}

impl Zero for PlaneAtOrigin {
    fn zero() -> Self {
        PlaneAtOrigin {
            groups: PlaneAtOriginGroups { g0: Simd32x3::from(0.0) },
        }
    }
}

impl Zero for PointAtInfinity {
    fn zero() -> Self {
        PointAtInfinity {
            groups: PointAtInfinityGroups { g0: Simd32x3::from(0.0) },
        }
    }
}

impl Zero for PointAtOrigin {
    fn zero() -> Self {
        PointAtOrigin {
            groups: PointAtOriginGroups { g0: 0.0 },
        }
    }
}

impl Zero for Rotor {
    fn zero() -> Self {
        Rotor {
            groups: RotorGroups { g0: Simd32x4::from(0.0) },
        }
    }
}

impl Zero for RoundPoint {
    fn zero() -> Self {
        RoundPoint {
            groups: RoundPointGroups {
                g0: Simd32x3::from(0.0),
                g1: Simd32x2::from(0.0),
            },
        }
    }
}

impl Zero for Scalar {
    fn zero() -> Self {
        Scalar { groups: ScalarGroups { g0: 0.0 } }
    }
}

impl Zero for Sphere {
    fn zero() -> Self {
        Sphere {
            groups: SphereGroups {
                g0: Simd32x3::from(0.0),
                g1: Simd32x2::from(0.0),
            },
        }
    }
}

impl Zero for Translator {
    fn zero() -> Self {
        Translator {
            groups: TranslatorGroups { g0: Simd32x4::from(0.0) },
        }
    }
}

impl Neg for AntiScalar {
    type Output = AntiScalar;

    fn neg(self) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups { g0: self.group0() },
        }
    }
}

impl Neg for Circle {
    type Output = Circle;

    fn neg(self) -> Circle {
        Circle {
            groups: CircleGroups {
                g0: self.group0() * Simd32x4::from([1.0, -1.0, 1.0, -1.0]),
                g1: self.group1() * Simd32x3::from([1.0, -1.0, 1.0]),
                g2: self.group2() * Simd32x3::from([-1.0, 1.0, -1.0]),
            },
        }
    }
}

impl Neg for Dipole {
    type Output = Dipole;

    fn neg(self) -> Dipole {
        Dipole {
            groups: DipoleGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
                g1: self.group1() * Simd32x3::from(-1.0),
                g2: self.group2() * Simd32x4::from(-1.0),
            },
        }
    }
}

impl Neg for FlatPoint {
    type Output = FlatPoint;

    fn neg(self) -> FlatPoint {
        FlatPoint {
            groups: FlatPointGroups {
                g0: self.group0() * Simd32x4::from(-1.0),
            },
        }
    }
}

impl Neg for Flector {
    type Output = Flector;

    fn neg(self) -> Flector {
        Flector {
            groups: FlectorGroups {
                g0: self.group0() * Simd32x4::from(-1.0),
                g1: self.group1() * Simd32x4::from([1.0, -1.0, 1.0, 1.0]),
            },
        }
    }
}

impl Neg for Horizon {
    type Output = Horizon;

    fn neg(self) -> Horizon {
        Horizon {
            groups: HorizonGroups { g0: self.group0() },
        }
    }
}

impl Neg for Infinity {
    type Output = Infinity;

    fn neg(self) -> Infinity {
        Infinity {
            groups: InfinityGroups { g0: -self.group0() },
        }
    }
}

impl Neg for Line {
    type Output = Line;

    fn neg(self) -> Line {
        Line {
            groups: LineGroups {
                g0: self.group0() * Simd32x3::from([1.0, -1.0, 1.0]),
                g1: self.group1() * Simd32x3::from([-1.0, 1.0, -1.0]),
            },
        }
    }
}

impl Neg for LineAtInfinity {
    type Output = LineAtInfinity;

    fn neg(self) -> LineAtInfinity {
        LineAtInfinity {
            groups: LineAtInfinityGroups {
                g0: self.group0() * Simd32x3::from([-1.0, 1.0, -1.0]),
            },
        }
    }
}

impl Neg for LineAtOrigin {
    type Output = LineAtOrigin;

    fn neg(self) -> LineAtOrigin {
        LineAtOrigin {
            groups: LineAtOriginGroups {
                g0: self.group0() * Simd32x3::from([1.0, -1.0, 1.0]),
            },
        }
    }
}

impl Neg for Magnitude {
    type Output = Magnitude;

    fn neg(self) -> Magnitude {
        Magnitude {
            groups: MagnitudeGroups {
                g0: self.group0() * Simd32x2::from([-1.0, 1.0]),
            },
        }
    }
}

impl Neg for Motor {
    type Output = Motor;

    fn neg(self) -> Motor {
        Motor {
            groups: MotorGroups {
                g0: self.group0() * Simd32x4::from([1.0, -1.0, 1.0, 1.0]),
                g1: self.group1() * Simd32x3::from([-1.0, 1.0, -1.0]),
            },
        }
    }
}

impl Neg for MultiVector {
    type Output = MultiVector;

    fn neg(self) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: self.group0() * Simd32x2::from([-1.0, 1.0]),
                g1: self.group1() * Simd32x3::from(-1.0),
                g2: self.group2() * Simd32x2::from(-1.0),
                g3: self.group3() * Simd32x3::from(-1.0),
                g4: self.group4() * Simd32x3::from(-1.0),
                g5: self.group5() * Simd32x4::from(-1.0),
                g6: self.group6() * Simd32x4::from([1.0, -1.0, 1.0, -1.0]),
                g7: self.group7() * Simd32x3::from([1.0, -1.0, 1.0]),
                g8: self.group8() * Simd32x3::from([-1.0, 1.0, -1.0]),
                g9: self.group9() * Simd32x3::from([1.0, -1.0, 1.0]),
                g10: self.group10() * Simd32x2::from([-1.0, 1.0]),
            },
        }
    }
}

impl Neg for Origin {
    type Output = Origin;

    fn neg(self) -> Origin {
        Origin {
            groups: OriginGroups { g0: -self.group0() },
        }
    }
}

impl Neg for Plane {
    type Output = Plane;

    fn neg(self) -> Plane {
        Plane {
            groups: PlaneGroups {
                g0: self.group0() * Simd32x4::from([1.0, -1.0, 1.0, 1.0]),
            },
        }
    }
}

impl Neg for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn neg(self) -> PlaneAtOrigin {
        PlaneAtOrigin {
            groups: PlaneAtOriginGroups {
                g0: self.group0() * Simd32x3::from([1.0, -1.0, 1.0]),
            },
        }
    }
}

impl Neg for PointAtInfinity {
    type Output = PointAtInfinity;

    fn neg(self) -> PointAtInfinity {
        PointAtInfinity {
            groups: PointAtInfinityGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
            },
        }
    }
}

impl Neg for PointAtOrigin {
    type Output = PointAtOrigin;

    fn neg(self) -> PointAtOrigin {
        PointAtOrigin {
            groups: PointAtOriginGroups { g0: -self.group0() },
        }
    }
}

impl Neg for Rotor {
    type Output = Rotor;

    fn neg(self) -> Rotor {
        Rotor {
            groups: RotorGroups {
                g0: self.group0() * Simd32x4::from([1.0, -1.0, 1.0, 1.0]),
            },
        }
    }
}

impl Neg for RoundPoint {
    type Output = RoundPoint;

    fn neg(self) -> RoundPoint {
        RoundPoint {
            groups: RoundPointGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
                g1: self.group1() * Simd32x2::from(-1.0),
            },
        }
    }
}

impl Neg for Scalar {
    type Output = Scalar;

    fn neg(self) -> Scalar {
        Scalar {
            groups: ScalarGroups { g0: -self.group0() },
        }
    }
}

impl Neg for Sphere {
    type Output = Sphere;

    fn neg(self) -> Sphere {
        Sphere {
            groups: SphereGroups {
                g0: self.group0() * Simd32x3::from([1.0, -1.0, 1.0]),
                g1: self.group1() * Simd32x2::from([-1.0, 1.0]),
            },
        }
    }
}

impl Neg for Translator {
    type Output = Translator;

    fn neg(self) -> Translator {
        Translator {
            groups: TranslatorGroups {
                g0: self.group0() * Simd32x4::from([-1.0, 1.0, -1.0, 1.0]),
            },
        }
    }
}

impl Add<AntiScalar> for AntiScalar {
    type Output = AntiScalar;

    fn add(self, other: AntiScalar) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0() + other.group0(),
            },
        }
    }
}

impl AddAssign<AntiScalar> for AntiScalar {
    fn add_assign(&mut self, other: AntiScalar) {
        *self = (*self).add(other);
    }
}

impl Add<Line> for AntiScalar {
    type Output = Motor;

    fn add(self, other: Line) -> Motor {
        Motor {
            groups: MotorGroups {
                g0: Simd32x4::from([0.0, 0.0, 0.0, self.group0()]) + Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
                g1: other.group1(),
            },
        }
    }
}

impl Add<LineAtInfinity> for AntiScalar {
    type Output = Translator;

    fn add(self, other: LineAtInfinity) -> Translator {
        Translator {
            groups: TranslatorGroups {
                g0: Simd32x4::from([0.0, 0.0, 0.0, self.group0()]) + Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            },
        }
    }
}

impl Add<LineAtOrigin> for AntiScalar {
    type Output = Rotor;

    fn add(self, other: LineAtOrigin) -> Rotor {
        Rotor {
            groups: RotorGroups {
                g0: Simd32x4::from([0.0, 0.0, 0.0, self.group0()]) + Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            },
        }
    }
}

impl Add<Magnitude> for AntiScalar {
    type Output = Magnitude;

    fn add(self, other: Magnitude) -> Magnitude {
        Magnitude {
            groups: MagnitudeGroups {
                g0: Simd32x2::from([0.0, self.group0()]) + other.group0(),
            },
        }
    }
}

impl Add<Motor> for AntiScalar {
    type Output = Motor;

    fn add(self, other: Motor) -> Motor {
        Motor {
            groups: MotorGroups {
                g0: Simd32x4::from([0.0, 0.0, 0.0, self.group0()]) + other.group0(),
                g1: other.group1(),
            },
        }
    }
}

impl Add<MultiVector> for AntiScalar {
    type Output = MultiVector;

    fn add(self, other: MultiVector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from([0.0, self.group0()]) + other.group0(),
                g1: other.group1(),
                g2: other.group2(),
                g3: other.group3(),
                g4: other.group4(),
                g5: other.group5(),
                g6: other.group6(),
                g7: other.group7(),
                g8: other.group8(),
                g9: other.group9(),
                g10: other.group10(),
            },
        }
    }
}

impl Add<Rotor> for AntiScalar {
    type Output = Rotor;

    fn add(self, other: Rotor) -> Rotor {
        Rotor {
            groups: RotorGroups {
                g0: Simd32x4::from([0.0, 0.0, 0.0, self.group0()]) + other.group0(),
            },
        }
    }
}

impl Add<Scalar> for AntiScalar {
    type Output = Magnitude;

    fn add(self, other: Scalar) -> Magnitude {
        Magnitude {
            groups: MagnitudeGroups {
                g0: Simd32x2::from([0.0, self.group0()]) + Simd32x2::from([other.group0(), 0.0]),
            },
        }
    }
}

impl Add<Translator> for AntiScalar {
    type Output = Translator;

    fn add(self, other: Translator) -> Translator {
        Translator {
            groups: TranslatorGroups {
                g0: Simd32x4::from([0.0, 0.0, 0.0, self.group0()]) + other.group0(),
            },
        }
    }
}

impl Add<Circle> for Circle {
    type Output = Circle;

    fn add(self, other: Circle) -> Circle {
        Circle {
            groups: CircleGroups {
                g0: self.group0() + other.group0(),
                g1: self.group1() + other.group1(),
                g2: self.group2() + other.group2(),
            },
        }
    }
}

impl AddAssign<Circle> for Circle {
    fn add_assign(&mut self, other: Circle) {
        *self = (*self).add(other);
    }
}

impl Add<Line> for Circle {
    type Output = Circle;

    fn add(self, other: Line) -> Circle {
        Circle {
            groups: CircleGroups {
                g0: self.group0(),
                g1: self.group1() + other.group0(),
                g2: self.group2() + other.group1(),
            },
        }
    }
}

impl AddAssign<Line> for Circle {
    fn add_assign(&mut self, other: Line) {
        *self = (*self).add(other);
    }
}

impl Add<LineAtInfinity> for Circle {
    type Output = Circle;

    fn add(self, other: LineAtInfinity) -> Circle {
        Circle {
            groups: CircleGroups {
                g0: self.group0(),
                g1: self.group1(),
                g2: self.group2() + other.group0(),
            },
        }
    }
}

impl AddAssign<LineAtInfinity> for Circle {
    fn add_assign(&mut self, other: LineAtInfinity) {
        *self = (*self).add(other);
    }
}

impl Add<LineAtOrigin> for Circle {
    type Output = Circle;

    fn add(self, other: LineAtOrigin) -> Circle {
        Circle {
            groups: CircleGroups {
                g0: self.group0(),
                g1: self.group1() + other.group0(),
                g2: self.group2(),
            },
        }
    }
}

impl AddAssign<LineAtOrigin> for Circle {
    fn add_assign(&mut self, other: LineAtOrigin) {
        *self = (*self).add(other);
    }
}

impl Add<MultiVector> for Circle {
    type Output = MultiVector;

    fn add(self, other: MultiVector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: other.group0(),
                g1: other.group1(),
                g2: other.group2(),
                g3: other.group3(),
                g4: other.group4(),
                g5: other.group5(),
                g6: self.group0() + other.group6(),
                g7: self.group1() + other.group7(),
                g8: self.group2() + other.group8(),
                g9: other.group9(),
                g10: other.group10(),
            },
        }
    }
}

impl Add<Dipole> for Dipole {
    type Output = Dipole;

    fn add(self, other: Dipole) -> Dipole {
        Dipole {
            groups: DipoleGroups {
                g0: self.group0() + other.group0(),
                g1: self.group1() + other.group1(),
                g2: self.group2() + other.group2(),
            },
        }
    }
}

impl AddAssign<Dipole> for Dipole {
    fn add_assign(&mut self, other: Dipole) {
        *self = (*self).add(other);
    }
}

impl Add<FlatPoint> for Dipole {
    type Output = Dipole;

    fn add(self, other: FlatPoint) -> Dipole {
        Dipole {
            groups: DipoleGroups {
                g0: self.group0(),
                g1: self.group1(),
                g2: self.group2() + other.group0(),
            },
        }
    }
}

impl AddAssign<FlatPoint> for Dipole {
    fn add_assign(&mut self, other: FlatPoint) {
        *self = (*self).add(other);
    }
}

impl Add<MultiVector> for Dipole {
    type Output = MultiVector;

    fn add(self, other: MultiVector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: other.group0(),
                g1: other.group1(),
                g2: other.group2(),
                g3: self.group0() + other.group3(),
                g4: self.group1() + other.group4(),
                g5: self.group2() + other.group5(),
                g6: other.group6(),
                g7: other.group7(),
                g8: other.group8(),
                g9: other.group9(),
                g10: other.group10(),
            },
        }
    }
}

impl Add<PointAtInfinity> for Dipole {
    type Output = Dipole;

    fn add(self, other: PointAtInfinity) -> Dipole {
        Dipole {
            groups: DipoleGroups {
                g0: self.group0(),
                g1: self.group1(),
                g2: self.group2() + Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            },
        }
    }
}

impl AddAssign<PointAtInfinity> for Dipole {
    fn add_assign(&mut self, other: PointAtInfinity) {
        *self = (*self).add(other);
    }
}

impl Add<PointAtOrigin> for Dipole {
    type Output = Dipole;

    fn add(self, other: PointAtOrigin) -> Dipole {
        Dipole {
            groups: DipoleGroups {
                g0: self.group0(),
                g1: self.group1(),
                g2: self.group2() + Simd32x4::from([0.0, 0.0, 0.0, other.group0()]),
            },
        }
    }
}

impl AddAssign<PointAtOrigin> for Dipole {
    fn add_assign(&mut self, other: PointAtOrigin) {
        *self = (*self).add(other);
    }
}

impl Add<Dipole> for FlatPoint {
    type Output = Dipole;

    fn add(self, other: Dipole) -> Dipole {
        Dipole {
            groups: DipoleGroups {
                g0: other.group0(),
                g1: other.group1(),
                g2: self.group0() + other.group2(),
            },
        }
    }
}

impl Add<FlatPoint> for FlatPoint {
    type Output = FlatPoint;

    fn add(self, other: FlatPoint) -> FlatPoint {
        FlatPoint {
            groups: FlatPointGroups {
                g0: self.group0() + other.group0(),
            },
        }
    }
}

impl AddAssign<FlatPoint> for FlatPoint {
    fn add_assign(&mut self, other: FlatPoint) {
        *self = (*self).add(other);
    }
}

impl Add<Flector> for FlatPoint {
    type Output = Flector;

    fn add(self, other: Flector) -> Flector {
        Flector {
            groups: FlectorGroups {
                g0: self.group0() + other.group0(),
                g1: other.group1(),
            },
        }
    }
}

impl Add<MultiVector> for FlatPoint {
    type Output = MultiVector;

    fn add(self, other: MultiVector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: other.group0(),
                g1: other.group1(),
                g2: other.group2(),
                g3: other.group3(),
                g4: other.group4(),
                g5: self.group0() + other.group5(),
                g6: other.group6(),
                g7: other.group7(),
                g8: other.group8(),
                g9: other.group9(),
                g10: other.group10(),
            },
        }
    }
}

impl Add<Plane> for FlatPoint {
    type Output = Flector;

    fn add(self, other: Plane) -> Flector {
        Flector {
            groups: FlectorGroups {
                g0: self.group0(),
                g1: other.group0(),
            },
        }
    }
}

impl Add<PointAtInfinity> for FlatPoint {
    type Output = FlatPoint;

    fn add(self, other: PointAtInfinity) -> FlatPoint {
        FlatPoint {
            groups: FlatPointGroups {
                g0: self.group0() + Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            },
        }
    }
}

impl AddAssign<PointAtInfinity> for FlatPoint {
    fn add_assign(&mut self, other: PointAtInfinity) {
        *self = (*self).add(other);
    }
}

impl Add<PointAtOrigin> for FlatPoint {
    type Output = FlatPoint;

    fn add(self, other: PointAtOrigin) -> FlatPoint {
        FlatPoint {
            groups: FlatPointGroups {
                g0: self.group0() + Simd32x4::from([0.0, 0.0, 0.0, other.group0()]),
            },
        }
    }
}

impl AddAssign<PointAtOrigin> for FlatPoint {
    fn add_assign(&mut self, other: PointAtOrigin) {
        *self = (*self).add(other);
    }
}

impl Add<FlatPoint> for Flector {
    type Output = Flector;

    fn add(self, other: FlatPoint) -> Flector {
        Flector {
            groups: FlectorGroups {
                g0: self.group0() + other.group0(),
                g1: self.group1(),
            },
        }
    }
}

impl AddAssign<FlatPoint> for Flector {
    fn add_assign(&mut self, other: FlatPoint) {
        *self = (*self).add(other);
    }
}

impl Add<Flector> for Flector {
    type Output = Flector;

    fn add(self, other: Flector) -> Flector {
        Flector {
            groups: FlectorGroups {
                g0: self.group0() + other.group0(),
                g1: self.group1() + other.group1(),
            },
        }
    }
}

impl AddAssign<Flector> for Flector {
    fn add_assign(&mut self, other: Flector) {
        *self = (*self).add(other);
    }
}

impl Add<Horizon> for Flector {
    type Output = Flector;

    fn add(self, other: Horizon) -> Flector {
        Flector {
            groups: FlectorGroups {
                g0: self.group0(),
                g1: self.group1() + Simd32x4::from([0.0, 0.0, 0.0, other.group0()]),
            },
        }
    }
}

impl AddAssign<Horizon> for Flector {
    fn add_assign(&mut self, other: Horizon) {
        *self = (*self).add(other);
    }
}

impl Add<MultiVector> for Flector {
    type Output = MultiVector;

    fn add(self, other: MultiVector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: other.group0(),
                g1: other.group1(),
                g2: other.group2(),
                g3: other.group3(),
                g4: other.group4(),
                g5: self.group0() + other.group5(),
                g6: other.group6(),
                g7: other.group7(),
                g8: other.group8(),
                g9: Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]) + other.group9(),
                g10: Simd32x2::from([0.0, self.group1()[3]]) + other.group10(),
            },
        }
    }
}

impl Add<Plane> for Flector {
    type Output = Flector;

    fn add(self, other: Plane) -> Flector {
        Flector {
            groups: FlectorGroups {
                g0: self.group0(),
                g1: self.group1() + other.group0(),
            },
        }
    }
}

impl AddAssign<Plane> for Flector {
    fn add_assign(&mut self, other: Plane) {
        *self = (*self).add(other);
    }
}

impl Add<PlaneAtOrigin> for Flector {
    type Output = Flector;

    fn add(self, other: PlaneAtOrigin) -> Flector {
        Flector {
            groups: FlectorGroups {
                g0: self.group0(),
                g1: self.group1() + Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            },
        }
    }
}

impl AddAssign<PlaneAtOrigin> for Flector {
    fn add_assign(&mut self, other: PlaneAtOrigin) {
        *self = (*self).add(other);
    }
}

impl Add<PointAtInfinity> for Flector {
    type Output = Flector;

    fn add(self, other: PointAtInfinity) -> Flector {
        Flector {
            groups: FlectorGroups {
                g0: self.group0() + Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
                g1: self.group1(),
            },
        }
    }
}

impl AddAssign<PointAtInfinity> for Flector {
    fn add_assign(&mut self, other: PointAtInfinity) {
        *self = (*self).add(other);
    }
}

impl Add<PointAtOrigin> for Flector {
    type Output = Flector;

    fn add(self, other: PointAtOrigin) -> Flector {
        Flector {
            groups: FlectorGroups {
                g0: self.group0() + Simd32x4::from([0.0, 0.0, 0.0, other.group0()]),
                g1: self.group1(),
            },
        }
    }
}

impl AddAssign<PointAtOrigin> for Flector {
    fn add_assign(&mut self, other: PointAtOrigin) {
        *self = (*self).add(other);
    }
}

impl Add<Flector> for Horizon {
    type Output = Flector;

    fn add(self, other: Flector) -> Flector {
        Flector {
            groups: FlectorGroups {
                g0: other.group0(),
                g1: Simd32x4::from([0.0, 0.0, 0.0, self.group0()]) + other.group1(),
            },
        }
    }
}

impl Add<Horizon> for Horizon {
    type Output = Horizon;

    fn add(self, other: Horizon) -> Horizon {
        Horizon {
            groups: HorizonGroups {
                g0: self.group0() + other.group0(),
            },
        }
    }
}

impl AddAssign<Horizon> for Horizon {
    fn add_assign(&mut self, other: Horizon) {
        *self = (*self).add(other);
    }
}

impl Add<MultiVector> for Horizon {
    type Output = MultiVector;

    fn add(self, other: MultiVector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: other.group0(),
                g1: other.group1(),
                g2: other.group2(),
                g3: other.group3(),
                g4: other.group4(),
                g5: other.group5(),
                g6: other.group6(),
                g7: other.group7(),
                g8: other.group8(),
                g9: other.group9(),
                g10: Simd32x2::from([0.0, self.group0()]) + other.group10(),
            },
        }
    }
}

impl Add<Plane> for Horizon {
    type Output = Plane;

    fn add(self, other: Plane) -> Plane {
        Plane {
            groups: PlaneGroups {
                g0: Simd32x4::from([0.0, 0.0, 0.0, self.group0()]) + other.group0(),
            },
        }
    }
}

impl Add<PlaneAtOrigin> for Horizon {
    type Output = Plane;

    fn add(self, other: PlaneAtOrigin) -> Plane {
        Plane {
            groups: PlaneGroups {
                g0: Simd32x4::from([0.0, 0.0, 0.0, self.group0()]) + Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            },
        }
    }
}

impl Add<Sphere> for Horizon {
    type Output = Sphere;

    fn add(self, other: Sphere) -> Sphere {
        Sphere {
            groups: SphereGroups {
                g0: other.group0(),
                g1: Simd32x2::from([0.0, self.group0()]) + other.group1(),
            },
        }
    }
}

impl Add<Infinity> for Infinity {
    type Output = Infinity;

    fn add(self, other: Infinity) -> Infinity {
        Infinity {
            groups: InfinityGroups {
                g0: self.group0() + other.group0(),
            },
        }
    }
}

impl AddAssign<Infinity> for Infinity {
    fn add_assign(&mut self, other: Infinity) {
        *self = (*self).add(other);
    }
}

impl Add<MultiVector> for Infinity {
    type Output = MultiVector;

    fn add(self, other: MultiVector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: other.group0(),
                g1: other.group1(),
                g2: Simd32x2::from([0.0, self.group0()]) + other.group2(),
                g3: other.group3(),
                g4: other.group4(),
                g5: other.group5(),
                g6: other.group6(),
                g7: other.group7(),
                g8: other.group8(),
                g9: other.group9(),
                g10: other.group10(),
            },
        }
    }
}

impl Add<RoundPoint> for Infinity {
    type Output = RoundPoint;

    fn add(self, other: RoundPoint) -> RoundPoint {
        RoundPoint {
            groups: RoundPointGroups {
                g0: other.group0(),
                g1: Simd32x2::from([0.0, self.group0()]) + other.group1(),
            },
        }
    }
}

impl Add<AntiScalar> for Line {
    type Output = Motor;

    fn add(self, other: AntiScalar) -> Motor {
        Motor {
            groups: MotorGroups {
                g0: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]) + Simd32x4::from([0.0, 0.0, 0.0, other.group0()]),
                g1: self.group1(),
            },
        }
    }
}

impl Add<Circle> for Line {
    type Output = Circle;

    fn add(self, other: Circle) -> Circle {
        Circle {
            groups: CircleGroups {
                g0: other.group0(),
                g1: self.group0() + other.group1(),
                g2: self.group1() + other.group2(),
            },
        }
    }
}

impl Add<Line> for Line {
    type Output = Line;

    fn add(self, other: Line) -> Line {
        Line {
            groups: LineGroups {
                g0: self.group0() + other.group0(),
                g1: self.group1() + other.group1(),
            },
        }
    }
}

impl AddAssign<Line> for Line {
    fn add_assign(&mut self, other: Line) {
        *self = (*self).add(other);
    }
}

impl Add<LineAtInfinity> for Line {
    type Output = Line;

    fn add(self, other: LineAtInfinity) -> Line {
        Line {
            groups: LineGroups {
                g0: self.group0(),
                g1: self.group1() + other.group0(),
            },
        }
    }
}

impl AddAssign<LineAtInfinity> for Line {
    fn add_assign(&mut self, other: LineAtInfinity) {
        *self = (*self).add(other);
    }
}

impl Add<LineAtOrigin> for Line {
    type Output = Line;

    fn add(self, other: LineAtOrigin) -> Line {
        Line {
            groups: LineGroups {
                g0: self.group0() + other.group0(),
                g1: self.group1(),
            },
        }
    }
}

impl AddAssign<LineAtOrigin> for Line {
    fn add_assign(&mut self, other: LineAtOrigin) {
        *self = (*self).add(other);
    }
}

impl Add<Motor> for Line {
    type Output = Motor;

    fn add(self, other: Motor) -> Motor {
        Motor {
            groups: MotorGroups {
                g0: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]) + other.group0(),
                g1: self.group1() + other.group1(),
            },
        }
    }
}

impl Add<MultiVector> for Line {
    type Output = MultiVector;

    fn add(self, other: MultiVector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: other.group0(),
                g1: other.group1(),
                g2: other.group2(),
                g3: other.group3(),
                g4: other.group4(),
                g5: other.group5(),
                g6: other.group6(),
                g7: self.group0() + other.group7(),
                g8: self.group1() + other.group8(),
                g9: other.group9(),
                g10: other.group10(),
            },
        }
    }
}

impl Add<Rotor> for Line {
    type Output = Motor;

    fn add(self, other: Rotor) -> Motor {
        Motor {
            groups: MotorGroups {
                g0: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]) + other.group0(),
                g1: self.group1(),
            },
        }
    }
}

impl Add<Translator> for Line {
    type Output = Motor;

    fn add(self, other: Translator) -> Motor {
        Motor {
            groups: MotorGroups {
                g0: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]) + Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]),
                g1: self.group1() + Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            },
        }
    }
}

impl Add<AntiScalar> for LineAtInfinity {
    type Output = Translator;

    fn add(self, other: AntiScalar) -> Translator {
        Translator {
            groups: TranslatorGroups {
                g0: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]) + Simd32x4::from([0.0, 0.0, 0.0, other.group0()]),
            },
        }
    }
}

impl Add<Circle> for LineAtInfinity {
    type Output = Circle;

    fn add(self, other: Circle) -> Circle {
        Circle {
            groups: CircleGroups {
                g0: other.group0(),
                g1: other.group1(),
                g2: self.group0() + other.group2(),
            },
        }
    }
}

impl Add<Line> for LineAtInfinity {
    type Output = Line;

    fn add(self, other: Line) -> Line {
        Line {
            groups: LineGroups {
                g0: other.group0(),
                g1: self.group0() + other.group1(),
            },
        }
    }
}

impl Add<LineAtInfinity> for LineAtInfinity {
    type Output = LineAtInfinity;

    fn add(self, other: LineAtInfinity) -> LineAtInfinity {
        LineAtInfinity {
            groups: LineAtInfinityGroups {
                g0: self.group0() + other.group0(),
            },
        }
    }
}

impl AddAssign<LineAtInfinity> for LineAtInfinity {
    fn add_assign(&mut self, other: LineAtInfinity) {
        *self = (*self).add(other);
    }
}

impl Add<LineAtOrigin> for LineAtInfinity {
    type Output = Line;

    fn add(self, other: LineAtOrigin) -> Line {
        Line {
            groups: LineGroups {
                g0: other.group0(),
                g1: self.group0(),
            },
        }
    }
}

impl Add<Motor> for LineAtInfinity {
    type Output = Motor;

    fn add(self, other: Motor) -> Motor {
        Motor {
            groups: MotorGroups {
                g0: other.group0(),
                g1: self.group0() + other.group1(),
            },
        }
    }
}

impl Add<MultiVector> for LineAtInfinity {
    type Output = MultiVector;

    fn add(self, other: MultiVector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: other.group0(),
                g1: other.group1(),
                g2: other.group2(),
                g3: other.group3(),
                g4: other.group4(),
                g5: other.group5(),
                g6: other.group6(),
                g7: other.group7(),
                g8: self.group0() + other.group8(),
                g9: other.group9(),
                g10: other.group10(),
            },
        }
    }
}

impl Add<Rotor> for LineAtInfinity {
    type Output = Motor;

    fn add(self, other: Rotor) -> Motor {
        Motor {
            groups: MotorGroups {
                g0: other.group0(),
                g1: self.group0(),
            },
        }
    }
}

impl Add<Translator> for LineAtInfinity {
    type Output = Translator;

    fn add(self, other: Translator) -> Translator {
        Translator {
            groups: TranslatorGroups {
                g0: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]) + other.group0(),
            },
        }
    }
}

impl Add<AntiScalar> for LineAtOrigin {
    type Output = Rotor;

    fn add(self, other: AntiScalar) -> Rotor {
        Rotor {
            groups: RotorGroups {
                g0: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]) + Simd32x4::from([0.0, 0.0, 0.0, other.group0()]),
            },
        }
    }
}

impl Add<Circle> for LineAtOrigin {
    type Output = Circle;

    fn add(self, other: Circle) -> Circle {
        Circle {
            groups: CircleGroups {
                g0: other.group0(),
                g1: self.group0() + other.group1(),
                g2: other.group2(),
            },
        }
    }
}

impl Add<Line> for LineAtOrigin {
    type Output = Line;

    fn add(self, other: Line) -> Line {
        Line {
            groups: LineGroups {
                g0: self.group0() + other.group0(),
                g1: other.group1(),
            },
        }
    }
}

impl Add<LineAtInfinity> for LineAtOrigin {
    type Output = Line;

    fn add(self, other: LineAtInfinity) -> Line {
        Line {
            groups: LineGroups {
                g0: self.group0(),
                g1: other.group0(),
            },
        }
    }
}

impl Add<LineAtOrigin> for LineAtOrigin {
    type Output = LineAtOrigin;

    fn add(self, other: LineAtOrigin) -> LineAtOrigin {
        LineAtOrigin {
            groups: LineAtOriginGroups {
                g0: self.group0() + other.group0(),
            },
        }
    }
}

impl AddAssign<LineAtOrigin> for LineAtOrigin {
    fn add_assign(&mut self, other: LineAtOrigin) {
        *self = (*self).add(other);
    }
}

impl Add<Motor> for LineAtOrigin {
    type Output = Motor;

    fn add(self, other: Motor) -> Motor {
        Motor {
            groups: MotorGroups {
                g0: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]) + other.group0(),
                g1: other.group1(),
            },
        }
    }
}

impl Add<MultiVector> for LineAtOrigin {
    type Output = MultiVector;

    fn add(self, other: MultiVector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: other.group0(),
                g1: other.group1(),
                g2: other.group2(),
                g3: other.group3(),
                g4: other.group4(),
                g5: other.group5(),
                g6: other.group6(),
                g7: self.group0() + other.group7(),
                g8: other.group8(),
                g9: other.group9(),
                g10: other.group10(),
            },
        }
    }
}

impl Add<Rotor> for LineAtOrigin {
    type Output = Rotor;

    fn add(self, other: Rotor) -> Rotor {
        Rotor {
            groups: RotorGroups {
                g0: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]) + other.group0(),
            },
        }
    }
}

impl Add<Translator> for LineAtOrigin {
    type Output = Motor;

    fn add(self, other: Translator) -> Motor {
        Motor {
            groups: MotorGroups {
                g0: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]) + Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]),
                g1: Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            },
        }
    }
}

impl Add<AntiScalar> for Magnitude {
    type Output = Magnitude;

    fn add(self, other: AntiScalar) -> Magnitude {
        Magnitude {
            groups: MagnitudeGroups {
                g0: self.group0() + Simd32x2::from([0.0, other.group0()]),
            },
        }
    }
}

impl AddAssign<AntiScalar> for Magnitude {
    fn add_assign(&mut self, other: AntiScalar) {
        *self = (*self).add(other);
    }
}

impl Add<Magnitude> for Magnitude {
    type Output = Magnitude;

    fn add(self, other: Magnitude) -> Magnitude {
        Magnitude {
            groups: MagnitudeGroups {
                g0: self.group0() + other.group0(),
            },
        }
    }
}

impl AddAssign<Magnitude> for Magnitude {
    fn add_assign(&mut self, other: Magnitude) {
        *self = (*self).add(other);
    }
}

impl Add<MultiVector> for Magnitude {
    type Output = MultiVector;

    fn add(self, other: MultiVector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: self.group0() + other.group0(),
                g1: other.group1(),
                g2: other.group2(),
                g3: other.group3(),
                g4: other.group4(),
                g5: other.group5(),
                g6: other.group6(),
                g7: other.group7(),
                g8: other.group8(),
                g9: other.group9(),
                g10: other.group10(),
            },
        }
    }
}

impl Add<Scalar> for Magnitude {
    type Output = Magnitude;

    fn add(self, other: Scalar) -> Magnitude {
        Magnitude {
            groups: MagnitudeGroups {
                g0: self.group0() + Simd32x2::from([other.group0(), 0.0]),
            },
        }
    }
}

impl AddAssign<Scalar> for Magnitude {
    fn add_assign(&mut self, other: Scalar) {
        *self = (*self).add(other);
    }
}

impl Add<AntiScalar> for Motor {
    type Output = Motor;

    fn add(self, other: AntiScalar) -> Motor {
        Motor {
            groups: MotorGroups {
                g0: self.group0() + Simd32x4::from([0.0, 0.0, 0.0, other.group0()]),
                g1: self.group1(),
            },
        }
    }
}

impl AddAssign<AntiScalar> for Motor {
    fn add_assign(&mut self, other: AntiScalar) {
        *self = (*self).add(other);
    }
}

impl Add<Line> for Motor {
    type Output = Motor;

    fn add(self, other: Line) -> Motor {
        Motor {
            groups: MotorGroups {
                g0: self.group0() + Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
                g1: self.group1() + other.group1(),
            },
        }
    }
}

impl AddAssign<Line> for Motor {
    fn add_assign(&mut self, other: Line) {
        *self = (*self).add(other);
    }
}

impl Add<LineAtInfinity> for Motor {
    type Output = Motor;

    fn add(self, other: LineAtInfinity) -> Motor {
        Motor {
            groups: MotorGroups {
                g0: self.group0(),
                g1: self.group1() + other.group0(),
            },
        }
    }
}

impl AddAssign<LineAtInfinity> for Motor {
    fn add_assign(&mut self, other: LineAtInfinity) {
        *self = (*self).add(other);
    }
}

impl Add<LineAtOrigin> for Motor {
    type Output = Motor;

    fn add(self, other: LineAtOrigin) -> Motor {
        Motor {
            groups: MotorGroups {
                g0: self.group0() + Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
                g1: self.group1(),
            },
        }
    }
}

impl AddAssign<LineAtOrigin> for Motor {
    fn add_assign(&mut self, other: LineAtOrigin) {
        *self = (*self).add(other);
    }
}

impl Add<Motor> for Motor {
    type Output = Motor;

    fn add(self, other: Motor) -> Motor {
        Motor {
            groups: MotorGroups {
                g0: self.group0() + other.group0(),
                g1: self.group1() + other.group1(),
            },
        }
    }
}

impl AddAssign<Motor> for Motor {
    fn add_assign(&mut self, other: Motor) {
        *self = (*self).add(other);
    }
}

impl Add<MultiVector> for Motor {
    type Output = MultiVector;

    fn add(self, other: MultiVector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from([0.0, self.group0()[3]]) + other.group0(),
                g1: other.group1(),
                g2: other.group2(),
                g3: other.group3(),
                g4: other.group4(),
                g5: other.group5(),
                g6: other.group6(),
                g7: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) + other.group7(),
                g8: self.group1() + other.group8(),
                g9: other.group9(),
                g10: other.group10(),
            },
        }
    }
}

impl Add<Rotor> for Motor {
    type Output = Motor;

    fn add(self, other: Rotor) -> Motor {
        Motor {
            groups: MotorGroups {
                g0: self.group0() + other.group0(),
                g1: self.group1(),
            },
        }
    }
}

impl AddAssign<Rotor> for Motor {
    fn add_assign(&mut self, other: Rotor) {
        *self = (*self).add(other);
    }
}

impl Add<Translator> for Motor {
    type Output = Motor;

    fn add(self, other: Translator) -> Motor {
        Motor {
            groups: MotorGroups {
                g0: self.group0() + Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]),
                g1: self.group1() + Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            },
        }
    }
}

impl AddAssign<Translator> for Motor {
    fn add_assign(&mut self, other: Translator) {
        *self = (*self).add(other);
    }
}

impl Add<AntiScalar> for MultiVector {
    type Output = MultiVector;

    fn add(self, other: AntiScalar) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: self.group0() + Simd32x2::from([0.0, other.group0()]),
                g1: self.group1(),
                g2: self.group2(),
                g3: self.group3(),
                g4: self.group4(),
                g5: self.group5(),
                g6: self.group6(),
                g7: self.group7(),
                g8: self.group8(),
                g9: self.group9(),
                g10: self.group10(),
            },
        }
    }
}

impl AddAssign<AntiScalar> for MultiVector {
    fn add_assign(&mut self, other: AntiScalar) {
        *self = (*self).add(other);
    }
}

impl Add<Circle> for MultiVector {
    type Output = MultiVector;

    fn add(self, other: Circle) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: self.group0(),
                g1: self.group1(),
                g2: self.group2(),
                g3: self.group3(),
                g4: self.group4(),
                g5: self.group5(),
                g6: self.group6() + other.group0(),
                g7: self.group7() + other.group1(),
                g8: self.group8() + other.group2(),
                g9: self.group9(),
                g10: self.group10(),
            },
        }
    }
}

impl AddAssign<Circle> for MultiVector {
    fn add_assign(&mut self, other: Circle) {
        *self = (*self).add(other);
    }
}

impl Add<Dipole> for MultiVector {
    type Output = MultiVector;

    fn add(self, other: Dipole) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: self.group0(),
                g1: self.group1(),
                g2: self.group2(),
                g3: self.group3() + other.group0(),
                g4: self.group4() + other.group1(),
                g5: self.group5() + other.group2(),
                g6: self.group6(),
                g7: self.group7(),
                g8: self.group8(),
                g9: self.group9(),
                g10: self.group10(),
            },
        }
    }
}

impl AddAssign<Dipole> for MultiVector {
    fn add_assign(&mut self, other: Dipole) {
        *self = (*self).add(other);
    }
}

impl Add<FlatPoint> for MultiVector {
    type Output = MultiVector;

    fn add(self, other: FlatPoint) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: self.group0(),
                g1: self.group1(),
                g2: self.group2(),
                g3: self.group3(),
                g4: self.group4(),
                g5: self.group5() + other.group0(),
                g6: self.group6(),
                g7: self.group7(),
                g8: self.group8(),
                g9: self.group9(),
                g10: self.group10(),
            },
        }
    }
}

impl AddAssign<FlatPoint> for MultiVector {
    fn add_assign(&mut self, other: FlatPoint) {
        *self = (*self).add(other);
    }
}

impl Add<Flector> for MultiVector {
    type Output = MultiVector;

    fn add(self, other: Flector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: self.group0(),
                g1: self.group1(),
                g2: self.group2(),
                g3: self.group3(),
                g4: self.group4(),
                g5: self.group5() + other.group0(),
                g6: self.group6(),
                g7: self.group7(),
                g8: self.group8(),
                g9: self.group9() + Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
                g10: self.group10() + Simd32x2::from([0.0, other.group1()[3]]),
            },
        }
    }
}

impl AddAssign<Flector> for MultiVector {
    fn add_assign(&mut self, other: Flector) {
        *self = (*self).add(other);
    }
}

impl Add<Horizon> for MultiVector {
    type Output = MultiVector;

    fn add(self, other: Horizon) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: self.group0(),
                g1: self.group1(),
                g2: self.group2(),
                g3: self.group3(),
                g4: self.group4(),
                g5: self.group5(),
                g6: self.group6(),
                g7: self.group7(),
                g8: self.group8(),
                g9: self.group9(),
                g10: self.group10() + Simd32x2::from([0.0, other.group0()]),
            },
        }
    }
}

impl AddAssign<Horizon> for MultiVector {
    fn add_assign(&mut self, other: Horizon) {
        *self = (*self).add(other);
    }
}

impl Add<Infinity> for MultiVector {
    type Output = MultiVector;

    fn add(self, other: Infinity) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: self.group0(),
                g1: self.group1(),
                g2: self.group2() + Simd32x2::from([0.0, other.group0()]),
                g3: self.group3(),
                g4: self.group4(),
                g5: self.group5(),
                g6: self.group6(),
                g7: self.group7(),
                g8: self.group8(),
                g9: self.group9(),
                g10: self.group10(),
            },
        }
    }
}

impl AddAssign<Infinity> for MultiVector {
    fn add_assign(&mut self, other: Infinity) {
        *self = (*self).add(other);
    }
}

impl Add<Line> for MultiVector {
    type Output = MultiVector;

    fn add(self, other: Line) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: self.group0(),
                g1: self.group1(),
                g2: self.group2(),
                g3: self.group3(),
                g4: self.group4(),
                g5: self.group5(),
                g6: self.group6(),
                g7: self.group7() + other.group0(),
                g8: self.group8() + other.group1(),
                g9: self.group9(),
                g10: self.group10(),
            },
        }
    }
}

impl AddAssign<Line> for MultiVector {
    fn add_assign(&mut self, other: Line) {
        *self = (*self).add(other);
    }
}

impl Add<LineAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn add(self, other: LineAtInfinity) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: self.group0(),
                g1: self.group1(),
                g2: self.group2(),
                g3: self.group3(),
                g4: self.group4(),
                g5: self.group5(),
                g6: self.group6(),
                g7: self.group7(),
                g8: self.group8() + other.group0(),
                g9: self.group9(),
                g10: self.group10(),
            },
        }
    }
}

impl AddAssign<LineAtInfinity> for MultiVector {
    fn add_assign(&mut self, other: LineAtInfinity) {
        *self = (*self).add(other);
    }
}

impl Add<LineAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn add(self, other: LineAtOrigin) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: self.group0(),
                g1: self.group1(),
                g2: self.group2(),
                g3: self.group3(),
                g4: self.group4(),
                g5: self.group5(),
                g6: self.group6(),
                g7: self.group7() + other.group0(),
                g8: self.group8(),
                g9: self.group9(),
                g10: self.group10(),
            },
        }
    }
}

impl AddAssign<LineAtOrigin> for MultiVector {
    fn add_assign(&mut self, other: LineAtOrigin) {
        *self = (*self).add(other);
    }
}

impl Add<Magnitude> for MultiVector {
    type Output = MultiVector;

    fn add(self, other: Magnitude) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: self.group0() + other.group0(),
                g1: self.group1(),
                g2: self.group2(),
                g3: self.group3(),
                g4: self.group4(),
                g5: self.group5(),
                g6: self.group6(),
                g7: self.group7(),
                g8: self.group8(),
                g9: self.group9(),
                g10: self.group10(),
            },
        }
    }
}

impl AddAssign<Magnitude> for MultiVector {
    fn add_assign(&mut self, other: Magnitude) {
        *self = (*self).add(other);
    }
}

impl Add<Motor> for MultiVector {
    type Output = MultiVector;

    fn add(self, other: Motor) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: self.group0() + Simd32x2::from([0.0, other.group0()[3]]),
                g1: self.group1(),
                g2: self.group2(),
                g3: self.group3(),
                g4: self.group4(),
                g5: self.group5(),
                g6: self.group6(),
                g7: self.group7() + Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
                g8: self.group8() + other.group1(),
                g9: self.group9(),
                g10: self.group10(),
            },
        }
    }
}

impl AddAssign<Motor> for MultiVector {
    fn add_assign(&mut self, other: Motor) {
        *self = (*self).add(other);
    }
}

impl Add<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn add(self, other: MultiVector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: self.group0() + other.group0(),
                g1: self.group1() + other.group1(),
                g2: self.group2() + other.group2(),
                g3: self.group3() + other.group3(),
                g4: self.group4() + other.group4(),
                g5: self.group5() + other.group5(),
                g6: self.group6() + other.group6(),
                g7: self.group7() + other.group7(),
                g8: self.group8() + other.group8(),
                g9: self.group9() + other.group9(),
                g10: self.group10() + other.group10(),
            },
        }
    }
}

impl AddAssign<MultiVector> for MultiVector {
    fn add_assign(&mut self, other: MultiVector) {
        *self = (*self).add(other);
    }
}

impl Add<Origin> for MultiVector {
    type Output = MultiVector;

    fn add(self, other: Origin) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: self.group0(),
                g1: self.group1(),
                g2: self.group2() + Simd32x2::from([other.group0(), 0.0]),
                g3: self.group3(),
                g4: self.group4(),
                g5: self.group5(),
                g6: self.group6(),
                g7: self.group7(),
                g8: self.group8(),
                g9: self.group9(),
                g10: self.group10(),
            },
        }
    }
}

impl AddAssign<Origin> for MultiVector {
    fn add_assign(&mut self, other: Origin) {
        *self = (*self).add(other);
    }
}

impl Add<Plane> for MultiVector {
    type Output = MultiVector;

    fn add(self, other: Plane) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: self.group0(),
                g1: self.group1(),
                g2: self.group2(),
                g3: self.group3(),
                g4: self.group4(),
                g5: self.group5(),
                g6: self.group6(),
                g7: self.group7(),
                g8: self.group8(),
                g9: self.group9() + Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
                g10: self.group10() + Simd32x2::from([0.0, other.group0()[3]]),
            },
        }
    }
}

impl AddAssign<Plane> for MultiVector {
    fn add_assign(&mut self, other: Plane) {
        *self = (*self).add(other);
    }
}

impl Add<PlaneAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn add(self, other: PlaneAtOrigin) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: self.group0(),
                g1: self.group1(),
                g2: self.group2(),
                g3: self.group3(),
                g4: self.group4(),
                g5: self.group5(),
                g6: self.group6(),
                g7: self.group7(),
                g8: self.group8(),
                g9: self.group9() + other.group0(),
                g10: self.group10(),
            },
        }
    }
}

impl AddAssign<PlaneAtOrigin> for MultiVector {
    fn add_assign(&mut self, other: PlaneAtOrigin) {
        *self = (*self).add(other);
    }
}

impl Add<PointAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn add(self, other: PointAtInfinity) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: self.group0(),
                g1: self.group1(),
                g2: self.group2(),
                g3: self.group3(),
                g4: self.group4(),
                g5: self.group5() + Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
                g6: self.group6(),
                g7: self.group7(),
                g8: self.group8(),
                g9: self.group9(),
                g10: self.group10(),
            },
        }
    }
}

impl AddAssign<PointAtInfinity> for MultiVector {
    fn add_assign(&mut self, other: PointAtInfinity) {
        *self = (*self).add(other);
    }
}

impl Add<PointAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn add(self, other: PointAtOrigin) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: self.group0(),
                g1: self.group1(),
                g2: self.group2(),
                g3: self.group3(),
                g4: self.group4(),
                g5: self.group5() + Simd32x4::from([0.0, 0.0, 0.0, other.group0()]),
                g6: self.group6(),
                g7: self.group7(),
                g8: self.group8(),
                g9: self.group9(),
                g10: self.group10(),
            },
        }
    }
}

impl AddAssign<PointAtOrigin> for MultiVector {
    fn add_assign(&mut self, other: PointAtOrigin) {
        *self = (*self).add(other);
    }
}

impl Add<Rotor> for MultiVector {
    type Output = MultiVector;

    fn add(self, other: Rotor) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: self.group0() + Simd32x2::from([0.0, other.group0()[3]]),
                g1: self.group1(),
                g2: self.group2(),
                g3: self.group3(),
                g4: self.group4(),
                g5: self.group5(),
                g6: self.group6(),
                g7: self.group7() + Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
                g8: self.group8(),
                g9: self.group9(),
                g10: self.group10(),
            },
        }
    }
}

impl AddAssign<Rotor> for MultiVector {
    fn add_assign(&mut self, other: Rotor) {
        *self = (*self).add(other);
    }
}

impl Add<RoundPoint> for MultiVector {
    type Output = MultiVector;

    fn add(self, other: RoundPoint) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: self.group0(),
                g1: self.group1() + other.group0(),
                g2: self.group2() + other.group1(),
                g3: self.group3(),
                g4: self.group4(),
                g5: self.group5(),
                g6: self.group6(),
                g7: self.group7(),
                g8: self.group8(),
                g9: self.group9(),
                g10: self.group10(),
            },
        }
    }
}

impl AddAssign<RoundPoint> for MultiVector {
    fn add_assign(&mut self, other: RoundPoint) {
        *self = (*self).add(other);
    }
}

impl Add<Scalar> for MultiVector {
    type Output = MultiVector;

    fn add(self, other: Scalar) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: self.group0() + Simd32x2::from([other.group0(), 0.0]),
                g1: self.group1(),
                g2: self.group2(),
                g3: self.group3(),
                g4: self.group4(),
                g5: self.group5(),
                g6: self.group6(),
                g7: self.group7(),
                g8: self.group8(),
                g9: self.group9(),
                g10: self.group10(),
            },
        }
    }
}

impl AddAssign<Scalar> for MultiVector {
    fn add_assign(&mut self, other: Scalar) {
        *self = (*self).add(other);
    }
}

impl Add<Sphere> for MultiVector {
    type Output = MultiVector;

    fn add(self, other: Sphere) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: self.group0(),
                g1: self.group1(),
                g2: self.group2(),
                g3: self.group3(),
                g4: self.group4(),
                g5: self.group5(),
                g6: self.group6(),
                g7: self.group7(),
                g8: self.group8(),
                g9: self.group9() + other.group0(),
                g10: self.group10() + other.group1(),
            },
        }
    }
}

impl AddAssign<Sphere> for MultiVector {
    fn add_assign(&mut self, other: Sphere) {
        *self = (*self).add(other);
    }
}

impl Add<Translator> for MultiVector {
    type Output = MultiVector;

    fn add(self, other: Translator) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: self.group0() + Simd32x2::from([0.0, other.group0()[3]]),
                g1: self.group1(),
                g2: self.group2(),
                g3: self.group3(),
                g4: self.group4(),
                g5: self.group5(),
                g6: self.group6(),
                g7: self.group7(),
                g8: self.group8() + Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
                g9: self.group9(),
                g10: self.group10(),
            },
        }
    }
}

impl AddAssign<Translator> for MultiVector {
    fn add_assign(&mut self, other: Translator) {
        *self = (*self).add(other);
    }
}

impl Add<MultiVector> for Origin {
    type Output = MultiVector;

    fn add(self, other: MultiVector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: other.group0(),
                g1: other.group1(),
                g2: Simd32x2::from([self.group0(), 0.0]) + other.group2(),
                g3: other.group3(),
                g4: other.group4(),
                g5: other.group5(),
                g6: other.group6(),
                g7: other.group7(),
                g8: other.group8(),
                g9: other.group9(),
                g10: other.group10(),
            },
        }
    }
}

impl Add<Origin> for Origin {
    type Output = Origin;

    fn add(self, other: Origin) -> Origin {
        Origin {
            groups: OriginGroups {
                g0: self.group0() + other.group0(),
            },
        }
    }
}

impl AddAssign<Origin> for Origin {
    fn add_assign(&mut self, other: Origin) {
        *self = (*self).add(other);
    }
}

impl Add<RoundPoint> for Origin {
    type Output = RoundPoint;

    fn add(self, other: RoundPoint) -> RoundPoint {
        RoundPoint {
            groups: RoundPointGroups {
                g0: other.group0(),
                g1: Simd32x2::from([self.group0(), 0.0]) + other.group1(),
            },
        }
    }
}

impl Add<FlatPoint> for Plane {
    type Output = Flector;

    fn add(self, other: FlatPoint) -> Flector {
        Flector {
            groups: FlectorGroups {
                g0: other.group0(),
                g1: self.group0(),
            },
        }
    }
}

impl Add<Flector> for Plane {
    type Output = Flector;

    fn add(self, other: Flector) -> Flector {
        Flector {
            groups: FlectorGroups {
                g0: other.group0(),
                g1: self.group0() + other.group1(),
            },
        }
    }
}

impl Add<Horizon> for Plane {
    type Output = Plane;

    fn add(self, other: Horizon) -> Plane {
        Plane {
            groups: PlaneGroups {
                g0: self.group0() + Simd32x4::from([0.0, 0.0, 0.0, other.group0()]),
            },
        }
    }
}

impl AddAssign<Horizon> for Plane {
    fn add_assign(&mut self, other: Horizon) {
        *self = (*self).add(other);
    }
}

impl Add<MultiVector> for Plane {
    type Output = MultiVector;

    fn add(self, other: MultiVector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: other.group0(),
                g1: other.group1(),
                g2: other.group2(),
                g3: other.group3(),
                g4: other.group4(),
                g5: other.group5(),
                g6: other.group6(),
                g7: other.group7(),
                g8: other.group8(),
                g9: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) + other.group9(),
                g10: Simd32x2::from([0.0, self.group0()[3]]) + other.group10(),
            },
        }
    }
}

impl Add<Plane> for Plane {
    type Output = Plane;

    fn add(self, other: Plane) -> Plane {
        Plane {
            groups: PlaneGroups {
                g0: self.group0() + other.group0(),
            },
        }
    }
}

impl AddAssign<Plane> for Plane {
    fn add_assign(&mut self, other: Plane) {
        *self = (*self).add(other);
    }
}

impl Add<PlaneAtOrigin> for Plane {
    type Output = Plane;

    fn add(self, other: PlaneAtOrigin) -> Plane {
        Plane {
            groups: PlaneGroups {
                g0: self.group0() + Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            },
        }
    }
}

impl AddAssign<PlaneAtOrigin> for Plane {
    fn add_assign(&mut self, other: PlaneAtOrigin) {
        *self = (*self).add(other);
    }
}

impl Add<Sphere> for Plane {
    type Output = Sphere;

    fn add(self, other: Sphere) -> Sphere {
        Sphere {
            groups: SphereGroups {
                g0: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) + other.group0(),
                g1: Simd32x2::from([0.0, self.group0()[3]]) + other.group1(),
            },
        }
    }
}

impl Add<Flector> for PlaneAtOrigin {
    type Output = Flector;

    fn add(self, other: Flector) -> Flector {
        Flector {
            groups: FlectorGroups {
                g0: other.group0(),
                g1: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]) + other.group1(),
            },
        }
    }
}

impl Add<Horizon> for PlaneAtOrigin {
    type Output = Plane;

    fn add(self, other: Horizon) -> Plane {
        Plane {
            groups: PlaneGroups {
                g0: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]) + Simd32x4::from([0.0, 0.0, 0.0, other.group0()]),
            },
        }
    }
}

impl Add<MultiVector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn add(self, other: MultiVector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: other.group0(),
                g1: other.group1(),
                g2: other.group2(),
                g3: other.group3(),
                g4: other.group4(),
                g5: other.group5(),
                g6: other.group6(),
                g7: other.group7(),
                g8: other.group8(),
                g9: self.group0() + other.group9(),
                g10: other.group10(),
            },
        }
    }
}

impl Add<Plane> for PlaneAtOrigin {
    type Output = Plane;

    fn add(self, other: Plane) -> Plane {
        Plane {
            groups: PlaneGroups {
                g0: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]) + other.group0(),
            },
        }
    }
}

impl Add<PlaneAtOrigin> for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn add(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        PlaneAtOrigin {
            groups: PlaneAtOriginGroups {
                g0: self.group0() + other.group0(),
            },
        }
    }
}

impl AddAssign<PlaneAtOrigin> for PlaneAtOrigin {
    fn add_assign(&mut self, other: PlaneAtOrigin) {
        *self = (*self).add(other);
    }
}

impl Add<Sphere> for PlaneAtOrigin {
    type Output = Sphere;

    fn add(self, other: Sphere) -> Sphere {
        Sphere {
            groups: SphereGroups {
                g0: self.group0() + other.group0(),
                g1: other.group1(),
            },
        }
    }
}

impl Add<Dipole> for PointAtInfinity {
    type Output = Dipole;

    fn add(self, other: Dipole) -> Dipole {
        Dipole {
            groups: DipoleGroups {
                g0: other.group0(),
                g1: other.group1(),
                g2: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]) + other.group2(),
            },
        }
    }
}

impl Add<FlatPoint> for PointAtInfinity {
    type Output = FlatPoint;

    fn add(self, other: FlatPoint) -> FlatPoint {
        FlatPoint {
            groups: FlatPointGroups {
                g0: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]) + other.group0(),
            },
        }
    }
}

impl Add<Flector> for PointAtInfinity {
    type Output = Flector;

    fn add(self, other: Flector) -> Flector {
        Flector {
            groups: FlectorGroups {
                g0: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]) + other.group0(),
                g1: other.group1(),
            },
        }
    }
}

impl Add<MultiVector> for PointAtInfinity {
    type Output = MultiVector;

    fn add(self, other: MultiVector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: other.group0(),
                g1: other.group1(),
                g2: other.group2(),
                g3: other.group3(),
                g4: other.group4(),
                g5: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]) + other.group5(),
                g6: other.group6(),
                g7: other.group7(),
                g8: other.group8(),
                g9: other.group9(),
                g10: other.group10(),
            },
        }
    }
}

impl Add<PointAtInfinity> for PointAtInfinity {
    type Output = PointAtInfinity;

    fn add(self, other: PointAtInfinity) -> PointAtInfinity {
        PointAtInfinity {
            groups: PointAtInfinityGroups {
                g0: self.group0() + other.group0(),
            },
        }
    }
}

impl AddAssign<PointAtInfinity> for PointAtInfinity {
    fn add_assign(&mut self, other: PointAtInfinity) {
        *self = (*self).add(other);
    }
}

impl Add<PointAtOrigin> for PointAtInfinity {
    type Output = FlatPoint;

    fn add(self, other: PointAtOrigin) -> FlatPoint {
        FlatPoint {
            groups: FlatPointGroups {
                g0: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]) + Simd32x4::from([0.0, 0.0, 0.0, other.group0()]),
            },
        }
    }
}

impl Add<Dipole> for PointAtOrigin {
    type Output = Dipole;

    fn add(self, other: Dipole) -> Dipole {
        Dipole {
            groups: DipoleGroups {
                g0: other.group0(),
                g1: other.group1(),
                g2: Simd32x4::from([0.0, 0.0, 0.0, self.group0()]) + other.group2(),
            },
        }
    }
}

impl Add<FlatPoint> for PointAtOrigin {
    type Output = FlatPoint;

    fn add(self, other: FlatPoint) -> FlatPoint {
        FlatPoint {
            groups: FlatPointGroups {
                g0: Simd32x4::from([0.0, 0.0, 0.0, self.group0()]) + other.group0(),
            },
        }
    }
}

impl Add<Flector> for PointAtOrigin {
    type Output = Flector;

    fn add(self, other: Flector) -> Flector {
        Flector {
            groups: FlectorGroups {
                g0: Simd32x4::from([0.0, 0.0, 0.0, self.group0()]) + other.group0(),
                g1: other.group1(),
            },
        }
    }
}

impl Add<MultiVector> for PointAtOrigin {
    type Output = MultiVector;

    fn add(self, other: MultiVector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: other.group0(),
                g1: other.group1(),
                g2: other.group2(),
                g3: other.group3(),
                g4: other.group4(),
                g5: Simd32x4::from([0.0, 0.0, 0.0, self.group0()]) + other.group5(),
                g6: other.group6(),
                g7: other.group7(),
                g8: other.group8(),
                g9: other.group9(),
                g10: other.group10(),
            },
        }
    }
}

impl Add<PointAtInfinity> for PointAtOrigin {
    type Output = FlatPoint;

    fn add(self, other: PointAtInfinity) -> FlatPoint {
        FlatPoint {
            groups: FlatPointGroups {
                g0: Simd32x4::from([0.0, 0.0, 0.0, self.group0()]) + Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            },
        }
    }
}

impl Add<PointAtOrigin> for PointAtOrigin {
    type Output = PointAtOrigin;

    fn add(self, other: PointAtOrigin) -> PointAtOrigin {
        PointAtOrigin {
            groups: PointAtOriginGroups {
                g0: self.group0() + other.group0(),
            },
        }
    }
}

impl AddAssign<PointAtOrigin> for PointAtOrigin {
    fn add_assign(&mut self, other: PointAtOrigin) {
        *self = (*self).add(other);
    }
}

impl Add<AntiScalar> for Rotor {
    type Output = Rotor;

    fn add(self, other: AntiScalar) -> Rotor {
        Rotor {
            groups: RotorGroups {
                g0: self.group0() + Simd32x4::from([0.0, 0.0, 0.0, other.group0()]),
            },
        }
    }
}

impl AddAssign<AntiScalar> for Rotor {
    fn add_assign(&mut self, other: AntiScalar) {
        *self = (*self).add(other);
    }
}

impl Add<Line> for Rotor {
    type Output = Motor;

    fn add(self, other: Line) -> Motor {
        Motor {
            groups: MotorGroups {
                g0: self.group0() + Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
                g1: other.group1(),
            },
        }
    }
}

impl Add<LineAtInfinity> for Rotor {
    type Output = Motor;

    fn add(self, other: LineAtInfinity) -> Motor {
        Motor {
            groups: MotorGroups {
                g0: self.group0(),
                g1: other.group0(),
            },
        }
    }
}

impl Add<LineAtOrigin> for Rotor {
    type Output = Rotor;

    fn add(self, other: LineAtOrigin) -> Rotor {
        Rotor {
            groups: RotorGroups {
                g0: self.group0() + Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            },
        }
    }
}

impl AddAssign<LineAtOrigin> for Rotor {
    fn add_assign(&mut self, other: LineAtOrigin) {
        *self = (*self).add(other);
    }
}

impl Add<Motor> for Rotor {
    type Output = Motor;

    fn add(self, other: Motor) -> Motor {
        Motor {
            groups: MotorGroups {
                g0: self.group0() + other.group0(),
                g1: other.group1(),
            },
        }
    }
}

impl Add<MultiVector> for Rotor {
    type Output = MultiVector;

    fn add(self, other: MultiVector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from([0.0, self.group0()[3]]) + other.group0(),
                g1: other.group1(),
                g2: other.group2(),
                g3: other.group3(),
                g4: other.group4(),
                g5: other.group5(),
                g6: other.group6(),
                g7: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) + other.group7(),
                g8: other.group8(),
                g9: other.group9(),
                g10: other.group10(),
            },
        }
    }
}

impl Add<Rotor> for Rotor {
    type Output = Rotor;

    fn add(self, other: Rotor) -> Rotor {
        Rotor {
            groups: RotorGroups {
                g0: self.group0() + other.group0(),
            },
        }
    }
}

impl AddAssign<Rotor> for Rotor {
    fn add_assign(&mut self, other: Rotor) {
        *self = (*self).add(other);
    }
}

impl Add<Translator> for Rotor {
    type Output = Motor;

    fn add(self, other: Translator) -> Motor {
        Motor {
            groups: MotorGroups {
                g0: self.group0() + Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]),
                g1: Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            },
        }
    }
}

impl Add<Infinity> for RoundPoint {
    type Output = RoundPoint;

    fn add(self, other: Infinity) -> RoundPoint {
        RoundPoint {
            groups: RoundPointGroups {
                g0: self.group0(),
                g1: self.group1() + Simd32x2::from([0.0, other.group0()]),
            },
        }
    }
}

impl AddAssign<Infinity> for RoundPoint {
    fn add_assign(&mut self, other: Infinity) {
        *self = (*self).add(other);
    }
}

impl Add<MultiVector> for RoundPoint {
    type Output = MultiVector;

    fn add(self, other: MultiVector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: other.group0(),
                g1: self.group0() + other.group1(),
                g2: self.group1() + other.group2(),
                g3: other.group3(),
                g4: other.group4(),
                g5: other.group5(),
                g6: other.group6(),
                g7: other.group7(),
                g8: other.group8(),
                g9: other.group9(),
                g10: other.group10(),
            },
        }
    }
}

impl Add<Origin> for RoundPoint {
    type Output = RoundPoint;

    fn add(self, other: Origin) -> RoundPoint {
        RoundPoint {
            groups: RoundPointGroups {
                g0: self.group0(),
                g1: self.group1() + Simd32x2::from([other.group0(), 0.0]),
            },
        }
    }
}

impl AddAssign<Origin> for RoundPoint {
    fn add_assign(&mut self, other: Origin) {
        *self = (*self).add(other);
    }
}

impl Add<RoundPoint> for RoundPoint {
    type Output = RoundPoint;

    fn add(self, other: RoundPoint) -> RoundPoint {
        RoundPoint {
            groups: RoundPointGroups {
                g0: self.group0() + other.group0(),
                g1: self.group1() + other.group1(),
            },
        }
    }
}

impl AddAssign<RoundPoint> for RoundPoint {
    fn add_assign(&mut self, other: RoundPoint) {
        *self = (*self).add(other);
    }
}

impl Add<AntiScalar> for Scalar {
    type Output = Magnitude;

    fn add(self, other: AntiScalar) -> Magnitude {
        Magnitude {
            groups: MagnitudeGroups {
                g0: Simd32x2::from([self.group0(), 0.0]) + Simd32x2::from([0.0, other.group0()]),
            },
        }
    }
}

impl Add<Magnitude> for Scalar {
    type Output = Magnitude;

    fn add(self, other: Magnitude) -> Magnitude {
        Magnitude {
            groups: MagnitudeGroups {
                g0: Simd32x2::from([self.group0(), 0.0]) + other.group0(),
            },
        }
    }
}

impl Add<MultiVector> for Scalar {
    type Output = MultiVector;

    fn add(self, other: MultiVector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from([self.group0(), 0.0]) + other.group0(),
                g1: other.group1(),
                g2: other.group2(),
                g3: other.group3(),
                g4: other.group4(),
                g5: other.group5(),
                g6: other.group6(),
                g7: other.group7(),
                g8: other.group8(),
                g9: other.group9(),
                g10: other.group10(),
            },
        }
    }
}

impl Add<Scalar> for Scalar {
    type Output = Scalar;

    fn add(self, other: Scalar) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0() + other.group0(),
            },
        }
    }
}

impl AddAssign<Scalar> for Scalar {
    fn add_assign(&mut self, other: Scalar) {
        *self = (*self).add(other);
    }
}

impl Add<Horizon> for Sphere {
    type Output = Sphere;

    fn add(self, other: Horizon) -> Sphere {
        Sphere {
            groups: SphereGroups {
                g0: self.group0(),
                g1: self.group1() + Simd32x2::from([0.0, other.group0()]),
            },
        }
    }
}

impl AddAssign<Horizon> for Sphere {
    fn add_assign(&mut self, other: Horizon) {
        *self = (*self).add(other);
    }
}

impl Add<MultiVector> for Sphere {
    type Output = MultiVector;

    fn add(self, other: MultiVector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: other.group0(),
                g1: other.group1(),
                g2: other.group2(),
                g3: other.group3(),
                g4: other.group4(),
                g5: other.group5(),
                g6: other.group6(),
                g7: other.group7(),
                g8: other.group8(),
                g9: self.group0() + other.group9(),
                g10: self.group1() + other.group10(),
            },
        }
    }
}

impl Add<Plane> for Sphere {
    type Output = Sphere;

    fn add(self, other: Plane) -> Sphere {
        Sphere {
            groups: SphereGroups {
                g0: self.group0() + Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
                g1: self.group1() + Simd32x2::from([0.0, other.group0()[3]]),
            },
        }
    }
}

impl AddAssign<Plane> for Sphere {
    fn add_assign(&mut self, other: Plane) {
        *self = (*self).add(other);
    }
}

impl Add<PlaneAtOrigin> for Sphere {
    type Output = Sphere;

    fn add(self, other: PlaneAtOrigin) -> Sphere {
        Sphere {
            groups: SphereGroups {
                g0: self.group0() + other.group0(),
                g1: self.group1(),
            },
        }
    }
}

impl AddAssign<PlaneAtOrigin> for Sphere {
    fn add_assign(&mut self, other: PlaneAtOrigin) {
        *self = (*self).add(other);
    }
}

impl Add<Sphere> for Sphere {
    type Output = Sphere;

    fn add(self, other: Sphere) -> Sphere {
        Sphere {
            groups: SphereGroups {
                g0: self.group0() + other.group0(),
                g1: self.group1() + other.group1(),
            },
        }
    }
}

impl AddAssign<Sphere> for Sphere {
    fn add_assign(&mut self, other: Sphere) {
        *self = (*self).add(other);
    }
}

impl Add<AntiScalar> for Translator {
    type Output = Translator;

    fn add(self, other: AntiScalar) -> Translator {
        Translator {
            groups: TranslatorGroups {
                g0: self.group0() + Simd32x4::from([0.0, 0.0, 0.0, other.group0()]),
            },
        }
    }
}

impl AddAssign<AntiScalar> for Translator {
    fn add_assign(&mut self, other: AntiScalar) {
        *self = (*self).add(other);
    }
}

impl Add<Line> for Translator {
    type Output = Motor;

    fn add(self, other: Line) -> Motor {
        Motor {
            groups: MotorGroups {
                g0: Simd32x4::from([0.0, 0.0, 0.0, self.group0()[3]]) + Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
                g1: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) + other.group1(),
            },
        }
    }
}

impl Add<LineAtInfinity> for Translator {
    type Output = Translator;

    fn add(self, other: LineAtInfinity) -> Translator {
        Translator {
            groups: TranslatorGroups {
                g0: self.group0() + Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            },
        }
    }
}

impl AddAssign<LineAtInfinity> for Translator {
    fn add_assign(&mut self, other: LineAtInfinity) {
        *self = (*self).add(other);
    }
}

impl Add<LineAtOrigin> for Translator {
    type Output = Motor;

    fn add(self, other: LineAtOrigin) -> Motor {
        Motor {
            groups: MotorGroups {
                g0: Simd32x4::from([0.0, 0.0, 0.0, self.group0()[3]]) + Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
                g1: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
            },
        }
    }
}

impl Add<Motor> for Translator {
    type Output = Motor;

    fn add(self, other: Motor) -> Motor {
        Motor {
            groups: MotorGroups {
                g0: Simd32x4::from([0.0, 0.0, 0.0, self.group0()[3]]) + other.group0(),
                g1: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) + other.group1(),
            },
        }
    }
}

impl Add<MultiVector> for Translator {
    type Output = MultiVector;

    fn add(self, other: MultiVector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from([0.0, self.group0()[3]]) + other.group0(),
                g1: other.group1(),
                g2: other.group2(),
                g3: other.group3(),
                g4: other.group4(),
                g5: other.group5(),
                g6: other.group6(),
                g7: other.group7(),
                g8: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) + other.group8(),
                g9: other.group9(),
                g10: other.group10(),
            },
        }
    }
}

impl Add<Rotor> for Translator {
    type Output = Motor;

    fn add(self, other: Rotor) -> Motor {
        Motor {
            groups: MotorGroups {
                g0: Simd32x4::from([0.0, 0.0, 0.0, self.group0()[3]]) + other.group0(),
                g1: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
            },
        }
    }
}

impl Add<Translator> for Translator {
    type Output = Translator;

    fn add(self, other: Translator) -> Translator {
        Translator {
            groups: TranslatorGroups {
                g0: self.group0() + other.group0(),
            },
        }
    }
}

impl AddAssign<Translator> for Translator {
    fn add_assign(&mut self, other: Translator) {
        *self = (*self).add(other);
    }
}

impl Div<AntiScalar> for AntiScalar {
    type Output = AntiScalar;

    fn div(self, other: AntiScalar) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0() * 1.0 / other.group0() * 1.0,
            },
        }
    }
}

impl DivAssign<AntiScalar> for AntiScalar {
    fn div_assign(&mut self, other: AntiScalar) {
        *self = (*self).div(other);
    }
}

impl Div<Circle> for Circle {
    type Output = Circle;

    fn div(self, other: Circle) -> Circle {
        Circle {
            groups: CircleGroups {
                g0: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group0()[3]]) * Simd32x4::from([1.0, 1.0, 1.0, 1.0])
                    / Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group0()[3]])
                    * Simd32x4::from([1.0, 1.0, 1.0, 1.0]),
                g1: Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]) * Simd32x3::from([1.0, 1.0, 1.0])
                    / Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]])
                    * Simd32x3::from([1.0, 1.0, 1.0]),
                g2: Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]) * Simd32x3::from([1.0, 1.0, 1.0])
                    / Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]])
                    * Simd32x3::from([1.0, 1.0, 1.0]),
            },
        }
    }
}

impl DivAssign<Circle> for Circle {
    fn div_assign(&mut self, other: Circle) {
        *self = (*self).div(other);
    }
}

impl Div<Dipole> for Dipole {
    type Output = Dipole;

    fn div(self, other: Dipole) -> Dipole {
        Dipole {
            groups: DipoleGroups {
                g0: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from([1.0, 1.0, 1.0])
                    / Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])
                    * Simd32x3::from([1.0, 1.0, 1.0]),
                g1: Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]) * Simd32x3::from([1.0, 1.0, 1.0])
                    / Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]])
                    * Simd32x3::from([1.0, 1.0, 1.0]),
                g2: Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group2()[3]]) * Simd32x4::from([1.0, 1.0, 1.0, 1.0])
                    / Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], other.group2()[3]])
                    * Simd32x4::from([1.0, 1.0, 1.0, 1.0]),
            },
        }
    }
}

impl DivAssign<Dipole> for Dipole {
    fn div_assign(&mut self, other: Dipole) {
        *self = (*self).div(other);
    }
}

impl Div<FlatPoint> for FlatPoint {
    type Output = FlatPoint;

    fn div(self, other: FlatPoint) -> FlatPoint {
        FlatPoint {
            groups: FlatPointGroups {
                g0: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group0()[3]]) * Simd32x4::from([1.0, 1.0, 1.0, 1.0])
                    / Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group0()[3]])
                    * Simd32x4::from([1.0, 1.0, 1.0, 1.0]),
            },
        }
    }
}

impl DivAssign<FlatPoint> for FlatPoint {
    fn div_assign(&mut self, other: FlatPoint) {
        *self = (*self).div(other);
    }
}

impl Div<Flector> for Flector {
    type Output = Flector;

    fn div(self, other: Flector) -> Flector {
        Flector {
            groups: FlectorGroups {
                g0: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group0()[3]]) * Simd32x4::from([1.0, 1.0, 1.0, 1.0])
                    / Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group0()[3]])
                    * Simd32x4::from([1.0, 1.0, 1.0, 1.0]),
                g1: Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group1()[3]]) * Simd32x4::from([1.0, 1.0, 1.0, 1.0])
                    / Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group1()[3]])
                    * Simd32x4::from([1.0, 1.0, 1.0, 1.0]),
            },
        }
    }
}

impl DivAssign<Flector> for Flector {
    fn div_assign(&mut self, other: Flector) {
        *self = (*self).div(other);
    }
}

impl Div<Horizon> for Horizon {
    type Output = Horizon;

    fn div(self, other: Horizon) -> Horizon {
        Horizon {
            groups: HorizonGroups {
                g0: self.group0() * 1.0 / other.group0() * 1.0,
            },
        }
    }
}

impl DivAssign<Horizon> for Horizon {
    fn div_assign(&mut self, other: Horizon) {
        *self = (*self).div(other);
    }
}

impl Div<Infinity> for Infinity {
    type Output = Infinity;

    fn div(self, other: Infinity) -> Infinity {
        Infinity {
            groups: InfinityGroups {
                g0: self.group0() * 1.0 / other.group0() * 1.0,
            },
        }
    }
}

impl DivAssign<Infinity> for Infinity {
    fn div_assign(&mut self, other: Infinity) {
        *self = (*self).div(other);
    }
}

impl Div<Line> for Line {
    type Output = Line;

    fn div(self, other: Line) -> Line {
        Line {
            groups: LineGroups {
                g0: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from([1.0, 1.0, 1.0])
                    / Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])
                    * Simd32x3::from([1.0, 1.0, 1.0]),
                g1: Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]) * Simd32x3::from([1.0, 1.0, 1.0])
                    / Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]])
                    * Simd32x3::from([1.0, 1.0, 1.0]),
            },
        }
    }
}

impl DivAssign<Line> for Line {
    fn div_assign(&mut self, other: Line) {
        *self = (*self).div(other);
    }
}

impl Div<LineAtInfinity> for LineAtInfinity {
    type Output = LineAtInfinity;

    fn div(self, other: LineAtInfinity) -> LineAtInfinity {
        LineAtInfinity {
            groups: LineAtInfinityGroups {
                g0: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from([1.0, 1.0, 1.0])
                    / Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])
                    * Simd32x3::from([1.0, 1.0, 1.0]),
            },
        }
    }
}

impl DivAssign<LineAtInfinity> for LineAtInfinity {
    fn div_assign(&mut self, other: LineAtInfinity) {
        *self = (*self).div(other);
    }
}

impl Div<LineAtOrigin> for LineAtOrigin {
    type Output = LineAtOrigin;

    fn div(self, other: LineAtOrigin) -> LineAtOrigin {
        LineAtOrigin {
            groups: LineAtOriginGroups {
                g0: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from([1.0, 1.0, 1.0])
                    / Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])
                    * Simd32x3::from([1.0, 1.0, 1.0]),
            },
        }
    }
}

impl DivAssign<LineAtOrigin> for LineAtOrigin {
    fn div_assign(&mut self, other: LineAtOrigin) {
        *self = (*self).div(other);
    }
}

impl Div<Magnitude> for Magnitude {
    type Output = Magnitude;

    fn div(self, other: Magnitude) -> Magnitude {
        Magnitude {
            groups: MagnitudeGroups {
                g0: Simd32x2::from([self.group0()[0], self.group0()[1]]) * Simd32x2::from([1.0, 1.0]) / Simd32x2::from([other.group0()[0], other.group0()[1]])
                    * Simd32x2::from([1.0, 1.0]),
            },
        }
    }
}

impl DivAssign<Magnitude> for Magnitude {
    fn div_assign(&mut self, other: Magnitude) {
        *self = (*self).div(other);
    }
}

impl Div<Motor> for Motor {
    type Output = Motor;

    fn div(self, other: Motor) -> Motor {
        Motor {
            groups: MotorGroups {
                g0: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group0()[3]]) * Simd32x4::from([1.0, 1.0, 1.0, 1.0])
                    / Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group0()[3]])
                    * Simd32x4::from([1.0, 1.0, 1.0, 1.0]),
                g1: Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]) * Simd32x3::from([1.0, 1.0, 1.0])
                    / Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]])
                    * Simd32x3::from([1.0, 1.0, 1.0]),
            },
        }
    }
}

impl DivAssign<Motor> for Motor {
    fn div_assign(&mut self, other: Motor) {
        *self = (*self).div(other);
    }
}

impl Div<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn div(self, other: MultiVector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from([self.group0()[0], self.group0()[1]]) * Simd32x2::from([1.0, 1.0]) / Simd32x2::from([other.group0()[0], other.group0()[1]])
                    * Simd32x2::from([1.0, 1.0]),
                g1: Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]) * Simd32x3::from([1.0, 1.0, 1.0])
                    / Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]])
                    * Simd32x3::from([1.0, 1.0, 1.0]),
                g2: Simd32x2::from([self.group2()[0], self.group2()[1]]) * Simd32x2::from([1.0, 1.0]) / Simd32x2::from([other.group2()[0], other.group2()[1]])
                    * Simd32x2::from([1.0, 1.0]),
                g3: Simd32x3::from([self.group3()[0], self.group3()[1], self.group3()[2]]) * Simd32x3::from([1.0, 1.0, 1.0])
                    / Simd32x3::from([other.group3()[0], other.group3()[1], other.group3()[2]])
                    * Simd32x3::from([1.0, 1.0, 1.0]),
                g4: Simd32x3::from([self.group4()[0], self.group4()[1], self.group4()[2]]) * Simd32x3::from([1.0, 1.0, 1.0])
                    / Simd32x3::from([other.group4()[0], other.group4()[1], other.group4()[2]])
                    * Simd32x3::from([1.0, 1.0, 1.0]),
                g5: Simd32x4::from([self.group5()[0], self.group5()[1], self.group5()[2], self.group5()[3]]) * Simd32x4::from([1.0, 1.0, 1.0, 1.0])
                    / Simd32x4::from([other.group5()[0], other.group5()[1], other.group5()[2], other.group5()[3]])
                    * Simd32x4::from([1.0, 1.0, 1.0, 1.0]),
                g6: Simd32x4::from([self.group6()[0], self.group6()[1], self.group6()[2], self.group6()[3]]) * Simd32x4::from([1.0, 1.0, 1.0, 1.0])
                    / Simd32x4::from([other.group6()[0], other.group6()[1], other.group6()[2], other.group6()[3]])
                    * Simd32x4::from([1.0, 1.0, 1.0, 1.0]),
                g7: Simd32x3::from([self.group7()[0], self.group7()[1], self.group7()[2]]) * Simd32x3::from([1.0, 1.0, 1.0])
                    / Simd32x3::from([other.group7()[0], other.group7()[1], other.group7()[2]])
                    * Simd32x3::from([1.0, 1.0, 1.0]),
                g8: Simd32x3::from([self.group8()[0], self.group8()[1], self.group8()[2]]) * Simd32x3::from([1.0, 1.0, 1.0])
                    / Simd32x3::from([other.group8()[0], other.group8()[1], other.group8()[2]])
                    * Simd32x3::from([1.0, 1.0, 1.0]),
                g9: Simd32x3::from([self.group9()[0], self.group9()[1], self.group9()[2]]) * Simd32x3::from([1.0, 1.0, 1.0])
                    / Simd32x3::from([other.group9()[0], other.group9()[1], other.group9()[2]])
                    * Simd32x3::from([1.0, 1.0, 1.0]),
                g10: Simd32x2::from([self.group10()[0], self.group10()[1]]) * Simd32x2::from([1.0, 1.0]) / Simd32x2::from([other.group10()[0], other.group10()[1]])
                    * Simd32x2::from([1.0, 1.0]),
            },
        }
    }
}

impl DivAssign<MultiVector> for MultiVector {
    fn div_assign(&mut self, other: MultiVector) {
        *self = (*self).div(other);
    }
}

impl Div<Origin> for Origin {
    type Output = Origin;

    fn div(self, other: Origin) -> Origin {
        Origin {
            groups: OriginGroups {
                g0: self.group0() * 1.0 / other.group0() * 1.0,
            },
        }
    }
}

impl DivAssign<Origin> for Origin {
    fn div_assign(&mut self, other: Origin) {
        *self = (*self).div(other);
    }
}

impl Div<Plane> for Plane {
    type Output = Plane;

    fn div(self, other: Plane) -> Plane {
        Plane {
            groups: PlaneGroups {
                g0: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group0()[3]]) * Simd32x4::from([1.0, 1.0, 1.0, 1.0])
                    / Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group0()[3]])
                    * Simd32x4::from([1.0, 1.0, 1.0, 1.0]),
            },
        }
    }
}

impl DivAssign<Plane> for Plane {
    fn div_assign(&mut self, other: Plane) {
        *self = (*self).div(other);
    }
}

impl Div<PlaneAtOrigin> for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn div(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        PlaneAtOrigin {
            groups: PlaneAtOriginGroups {
                g0: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from([1.0, 1.0, 1.0])
                    / Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])
                    * Simd32x3::from([1.0, 1.0, 1.0]),
            },
        }
    }
}

impl DivAssign<PlaneAtOrigin> for PlaneAtOrigin {
    fn div_assign(&mut self, other: PlaneAtOrigin) {
        *self = (*self).div(other);
    }
}

impl Div<PointAtInfinity> for PointAtInfinity {
    type Output = PointAtInfinity;

    fn div(self, other: PointAtInfinity) -> PointAtInfinity {
        PointAtInfinity {
            groups: PointAtInfinityGroups {
                g0: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from([1.0, 1.0, 1.0])
                    / Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])
                    * Simd32x3::from([1.0, 1.0, 1.0]),
            },
        }
    }
}

impl DivAssign<PointAtInfinity> for PointAtInfinity {
    fn div_assign(&mut self, other: PointAtInfinity) {
        *self = (*self).div(other);
    }
}

impl Div<PointAtOrigin> for PointAtOrigin {
    type Output = PointAtOrigin;

    fn div(self, other: PointAtOrigin) -> PointAtOrigin {
        PointAtOrigin {
            groups: PointAtOriginGroups {
                g0: self.group0() * 1.0 / other.group0() * 1.0,
            },
        }
    }
}

impl DivAssign<PointAtOrigin> for PointAtOrigin {
    fn div_assign(&mut self, other: PointAtOrigin) {
        *self = (*self).div(other);
    }
}

impl Div<Rotor> for Rotor {
    type Output = Rotor;

    fn div(self, other: Rotor) -> Rotor {
        Rotor {
            groups: RotorGroups {
                g0: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group0()[3]]) * Simd32x4::from([1.0, 1.0, 1.0, 1.0])
                    / Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group0()[3]])
                    * Simd32x4::from([1.0, 1.0, 1.0, 1.0]),
            },
        }
    }
}

impl DivAssign<Rotor> for Rotor {
    fn div_assign(&mut self, other: Rotor) {
        *self = (*self).div(other);
    }
}

impl Div<RoundPoint> for RoundPoint {
    type Output = RoundPoint;

    fn div(self, other: RoundPoint) -> RoundPoint {
        RoundPoint {
            groups: RoundPointGroups {
                g0: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from([1.0, 1.0, 1.0])
                    / Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])
                    * Simd32x3::from([1.0, 1.0, 1.0]),
                g1: Simd32x2::from([self.group1()[0], self.group1()[1]]) * Simd32x2::from([1.0, 1.0]) / Simd32x2::from([other.group1()[0], other.group1()[1]])
                    * Simd32x2::from([1.0, 1.0]),
            },
        }
    }
}

impl DivAssign<RoundPoint> for RoundPoint {
    fn div_assign(&mut self, other: RoundPoint) {
        *self = (*self).div(other);
    }
}

impl Div<Scalar> for Scalar {
    type Output = Scalar;

    fn div(self, other: Scalar) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0() * 1.0 / other.group0() * 1.0,
            },
        }
    }
}

impl DivAssign<Scalar> for Scalar {
    fn div_assign(&mut self, other: Scalar) {
        *self = (*self).div(other);
    }
}

impl Div<Sphere> for Sphere {
    type Output = Sphere;

    fn div(self, other: Sphere) -> Sphere {
        Sphere {
            groups: SphereGroups {
                g0: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from([1.0, 1.0, 1.0])
                    / Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])
                    * Simd32x3::from([1.0, 1.0, 1.0]),
                g1: Simd32x2::from([self.group1()[0], self.group1()[1]]) * Simd32x2::from([1.0, 1.0]) / Simd32x2::from([other.group1()[0], other.group1()[1]])
                    * Simd32x2::from([1.0, 1.0]),
            },
        }
    }
}

impl DivAssign<Sphere> for Sphere {
    fn div_assign(&mut self, other: Sphere) {
        *self = (*self).div(other);
    }
}

impl Div<Translator> for Translator {
    type Output = Translator;

    fn div(self, other: Translator) -> Translator {
        Translator {
            groups: TranslatorGroups {
                g0: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group0()[3]]) * Simd32x4::from([1.0, 1.0, 1.0, 1.0])
                    / Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group0()[3]])
                    * Simd32x4::from([1.0, 1.0, 1.0, 1.0]),
            },
        }
    }
}

impl DivAssign<Translator> for Translator {
    fn div_assign(&mut self, other: Translator) {
        *self = (*self).div(other);
    }
}

impl Into<Line> for Circle {
    fn into(self) -> Line {
        Line {
            groups: LineGroups {
                g0: self.group1(),
                g1: self.group2(),
            },
        }
    }
}

impl Into<LineAtInfinity> for Circle {
    fn into(self) -> LineAtInfinity {
        LineAtInfinity {
            groups: LineAtInfinityGroups { g0: self.group2() },
        }
    }
}

impl Into<LineAtOrigin> for Circle {
    fn into(self) -> LineAtOrigin {
        LineAtOrigin {
            groups: LineAtOriginGroups { g0: self.group1() },
        }
    }
}

impl Into<FlatPoint> for Dipole {
    fn into(self) -> FlatPoint {
        FlatPoint {
            groups: FlatPointGroups { g0: self.group2() },
        }
    }
}

impl Into<PointAtInfinity> for Dipole {
    fn into(self) -> PointAtInfinity {
        PointAtInfinity {
            groups: PointAtInfinityGroups {
                g0: Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            },
        }
    }
}

impl Into<PointAtOrigin> for Dipole {
    fn into(self) -> PointAtOrigin {
        PointAtOrigin {
            groups: PointAtOriginGroups { g0: self.group2()[3] },
        }
    }
}

impl Into<PointAtInfinity> for FlatPoint {
    fn into(self) -> PointAtInfinity {
        PointAtInfinity {
            groups: PointAtInfinityGroups {
                g0: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
            },
        }
    }
}

impl Into<PointAtOrigin> for FlatPoint {
    fn into(self) -> PointAtOrigin {
        PointAtOrigin {
            groups: PointAtOriginGroups { g0: self.group0()[3] },
        }
    }
}

impl Into<FlatPoint> for Flector {
    fn into(self) -> FlatPoint {
        FlatPoint {
            groups: FlatPointGroups { g0: self.group0() },
        }
    }
}

impl Into<Horizon> for Flector {
    fn into(self) -> Horizon {
        Horizon {
            groups: HorizonGroups { g0: self.group1()[3] },
        }
    }
}

impl Into<Plane> for Flector {
    fn into(self) -> Plane {
        Plane {
            groups: PlaneGroups { g0: self.group1() },
        }
    }
}

impl Into<PlaneAtOrigin> for Flector {
    fn into(self) -> PlaneAtOrigin {
        PlaneAtOrigin {
            groups: PlaneAtOriginGroups {
                g0: Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            },
        }
    }
}

impl Into<PointAtInfinity> for Flector {
    fn into(self) -> PointAtInfinity {
        PointAtInfinity {
            groups: PointAtInfinityGroups {
                g0: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
            },
        }
    }
}

impl Into<PointAtOrigin> for Flector {
    fn into(self) -> PointAtOrigin {
        PointAtOrigin {
            groups: PointAtOriginGroups { g0: self.group0()[3] },
        }
    }
}

impl Into<LineAtInfinity> for Line {
    fn into(self) -> LineAtInfinity {
        LineAtInfinity {
            groups: LineAtInfinityGroups { g0: self.group1() },
        }
    }
}

impl Into<LineAtOrigin> for Line {
    fn into(self) -> LineAtOrigin {
        LineAtOrigin {
            groups: LineAtOriginGroups { g0: self.group0() },
        }
    }
}

impl Into<AntiScalar> for Magnitude {
    fn into(self) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups { g0: self.group0()[1] },
        }
    }
}

impl Into<Scalar> for Magnitude {
    fn into(self) -> Scalar {
        Scalar {
            groups: ScalarGroups { g0: self.group0()[0] },
        }
    }
}

impl Into<AntiScalar> for Motor {
    fn into(self) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups { g0: self.group0()[3] },
        }
    }
}

impl Into<Line> for Motor {
    fn into(self) -> Line {
        Line {
            groups: LineGroups {
                g0: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
                g1: self.group1(),
            },
        }
    }
}

impl Into<LineAtInfinity> for Motor {
    fn into(self) -> LineAtInfinity {
        LineAtInfinity {
            groups: LineAtInfinityGroups { g0: self.group1() },
        }
    }
}

impl Into<LineAtOrigin> for Motor {
    fn into(self) -> LineAtOrigin {
        LineAtOrigin {
            groups: LineAtOriginGroups {
                g0: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
            },
        }
    }
}

impl Into<Rotor> for Motor {
    fn into(self) -> Rotor {
        Rotor {
            groups: RotorGroups { g0: self.group0() },
        }
    }
}

impl Into<Translator> for Motor {
    fn into(self) -> Translator {
        Translator {
            groups: TranslatorGroups {
                g0: Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group0()[3]]),
            },
        }
    }
}

impl Into<AntiScalar> for MultiVector {
    fn into(self) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups { g0: self.group0()[1] },
        }
    }
}

impl Into<Circle> for MultiVector {
    fn into(self) -> Circle {
        Circle {
            groups: CircleGroups {
                g0: self.group6(),
                g1: self.group7(),
                g2: self.group8(),
            },
        }
    }
}

impl Into<Dipole> for MultiVector {
    fn into(self) -> Dipole {
        Dipole {
            groups: DipoleGroups {
                g0: self.group3(),
                g1: self.group4(),
                g2: self.group5(),
            },
        }
    }
}

impl Into<FlatPoint> for MultiVector {
    fn into(self) -> FlatPoint {
        FlatPoint {
            groups: FlatPointGroups { g0: self.group5() },
        }
    }
}

impl Into<Flector> for MultiVector {
    fn into(self) -> Flector {
        Flector {
            groups: FlectorGroups {
                g0: self.group5(),
                g1: Simd32x4::from([self.group9()[0], self.group9()[1], self.group9()[2], self.group10()[1]]),
            },
        }
    }
}

impl Into<Horizon> for MultiVector {
    fn into(self) -> Horizon {
        Horizon {
            groups: HorizonGroups { g0: self.group10()[1] },
        }
    }
}

impl Into<Infinity> for MultiVector {
    fn into(self) -> Infinity {
        Infinity {
            groups: InfinityGroups { g0: self.group2()[1] },
        }
    }
}

impl Into<Line> for MultiVector {
    fn into(self) -> Line {
        Line {
            groups: LineGroups {
                g0: self.group7(),
                g1: self.group8(),
            },
        }
    }
}

impl Into<LineAtInfinity> for MultiVector {
    fn into(self) -> LineAtInfinity {
        LineAtInfinity {
            groups: LineAtInfinityGroups { g0: self.group8() },
        }
    }
}

impl Into<LineAtOrigin> for MultiVector {
    fn into(self) -> LineAtOrigin {
        LineAtOrigin {
            groups: LineAtOriginGroups { g0: self.group7() },
        }
    }
}

impl Into<Magnitude> for MultiVector {
    fn into(self) -> Magnitude {
        Magnitude {
            groups: MagnitudeGroups { g0: self.group0() },
        }
    }
}

impl Into<Motor> for MultiVector {
    fn into(self) -> Motor {
        Motor {
            groups: MotorGroups {
                g0: Simd32x4::from([self.group7()[0], self.group7()[1], self.group7()[2], self.group0()[1]]),
                g1: self.group8(),
            },
        }
    }
}

impl Into<Origin> for MultiVector {
    fn into(self) -> Origin {
        Origin {
            groups: OriginGroups { g0: self.group2()[0] },
        }
    }
}

impl Into<Plane> for MultiVector {
    fn into(self) -> Plane {
        Plane {
            groups: PlaneGroups {
                g0: Simd32x4::from([self.group9()[0], self.group9()[1], self.group9()[2], self.group10()[1]]),
            },
        }
    }
}

impl Into<PlaneAtOrigin> for MultiVector {
    fn into(self) -> PlaneAtOrigin {
        PlaneAtOrigin {
            groups: PlaneAtOriginGroups { g0: self.group9() },
        }
    }
}

impl Into<PointAtInfinity> for MultiVector {
    fn into(self) -> PointAtInfinity {
        PointAtInfinity {
            groups: PointAtInfinityGroups {
                g0: Simd32x3::from([self.group5()[0], self.group5()[1], self.group5()[2]]),
            },
        }
    }
}

impl Into<PointAtOrigin> for MultiVector {
    fn into(self) -> PointAtOrigin {
        PointAtOrigin {
            groups: PointAtOriginGroups { g0: self.group5()[3] },
        }
    }
}

impl Into<Rotor> for MultiVector {
    fn into(self) -> Rotor {
        Rotor {
            groups: RotorGroups {
                g0: Simd32x4::from([self.group7()[0], self.group7()[1], self.group7()[2], self.group0()[1]]),
            },
        }
    }
}

impl Into<RoundPoint> for MultiVector {
    fn into(self) -> RoundPoint {
        RoundPoint {
            groups: RoundPointGroups {
                g0: self.group1(),
                g1: self.group2(),
            },
        }
    }
}

impl Into<Scalar> for MultiVector {
    fn into(self) -> Scalar {
        Scalar {
            groups: ScalarGroups { g0: self.group0()[0] },
        }
    }
}

impl Into<Sphere> for MultiVector {
    fn into(self) -> Sphere {
        Sphere {
            groups: SphereGroups {
                g0: self.group9(),
                g1: self.group10(),
            },
        }
    }
}

impl Into<Translator> for MultiVector {
    fn into(self) -> Translator {
        Translator {
            groups: TranslatorGroups {
                g0: Simd32x4::from([self.group8()[0], self.group8()[1], self.group8()[2], self.group0()[1]]),
            },
        }
    }
}

impl Into<Horizon> for Plane {
    fn into(self) -> Horizon {
        Horizon {
            groups: HorizonGroups { g0: self.group0()[3] },
        }
    }
}

impl Into<PlaneAtOrigin> for Plane {
    fn into(self) -> PlaneAtOrigin {
        PlaneAtOrigin {
            groups: PlaneAtOriginGroups {
                g0: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
            },
        }
    }
}

impl Into<AntiScalar> for Rotor {
    fn into(self) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups { g0: self.group0()[3] },
        }
    }
}

impl Into<LineAtOrigin> for Rotor {
    fn into(self) -> LineAtOrigin {
        LineAtOrigin {
            groups: LineAtOriginGroups {
                g0: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
            },
        }
    }
}

impl Into<Infinity> for RoundPoint {
    fn into(self) -> Infinity {
        Infinity {
            groups: InfinityGroups { g0: self.group1()[1] },
        }
    }
}

impl Into<Origin> for RoundPoint {
    fn into(self) -> Origin {
        Origin {
            groups: OriginGroups { g0: self.group1()[0] },
        }
    }
}

impl Into<Horizon> for Sphere {
    fn into(self) -> Horizon {
        Horizon {
            groups: HorizonGroups { g0: self.group1()[1] },
        }
    }
}

impl Into<Plane> for Sphere {
    fn into(self) -> Plane {
        Plane {
            groups: PlaneGroups {
                g0: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[1]]),
            },
        }
    }
}

impl Into<PlaneAtOrigin> for Sphere {
    fn into(self) -> PlaneAtOrigin {
        PlaneAtOrigin {
            groups: PlaneAtOriginGroups { g0: self.group0() },
        }
    }
}

impl Into<AntiScalar> for Translator {
    fn into(self) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups { g0: self.group0()[3] },
        }
    }
}

impl Into<LineAtInfinity> for Translator {
    fn into(self) -> LineAtInfinity {
        LineAtInfinity {
            groups: LineAtInfinityGroups {
                g0: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
            },
        }
    }
}

impl Mul<AntiScalar> for AntiScalar {
    type Output = AntiScalar;

    fn mul(self, other: AntiScalar) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0() * other.group0(),
            },
        }
    }
}

impl MulAssign<AntiScalar> for AntiScalar {
    fn mul_assign(&mut self, other: AntiScalar) {
        *self = (*self).mul(other);
    }
}

impl Mul<Circle> for Circle {
    type Output = Circle;

    fn mul(self, other: Circle) -> Circle {
        Circle {
            groups: CircleGroups {
                g0: self.group0() * other.group0(),
                g1: self.group1() * other.group1(),
                g2: self.group2() * other.group2(),
            },
        }
    }
}

impl MulAssign<Circle> for Circle {
    fn mul_assign(&mut self, other: Circle) {
        *self = (*self).mul(other);
    }
}

impl Mul<Dipole> for Dipole {
    type Output = Dipole;

    fn mul(self, other: Dipole) -> Dipole {
        Dipole {
            groups: DipoleGroups {
                g0: self.group0() * other.group0(),
                g1: self.group1() * other.group1(),
                g2: self.group2() * other.group2(),
            },
        }
    }
}

impl MulAssign<Dipole> for Dipole {
    fn mul_assign(&mut self, other: Dipole) {
        *self = (*self).mul(other);
    }
}

impl Mul<FlatPoint> for FlatPoint {
    type Output = FlatPoint;

    fn mul(self, other: FlatPoint) -> FlatPoint {
        FlatPoint {
            groups: FlatPointGroups {
                g0: self.group0() * other.group0(),
            },
        }
    }
}

impl MulAssign<FlatPoint> for FlatPoint {
    fn mul_assign(&mut self, other: FlatPoint) {
        *self = (*self).mul(other);
    }
}

impl Mul<Flector> for Flector {
    type Output = Flector;

    fn mul(self, other: Flector) -> Flector {
        Flector {
            groups: FlectorGroups {
                g0: self.group0() * other.group0(),
                g1: self.group1() * other.group1(),
            },
        }
    }
}

impl MulAssign<Flector> for Flector {
    fn mul_assign(&mut self, other: Flector) {
        *self = (*self).mul(other);
    }
}

impl Mul<Horizon> for Horizon {
    type Output = Horizon;

    fn mul(self, other: Horizon) -> Horizon {
        Horizon {
            groups: HorizonGroups {
                g0: self.group0() * other.group0(),
            },
        }
    }
}

impl MulAssign<Horizon> for Horizon {
    fn mul_assign(&mut self, other: Horizon) {
        *self = (*self).mul(other);
    }
}

impl Mul<Infinity> for Infinity {
    type Output = Infinity;

    fn mul(self, other: Infinity) -> Infinity {
        Infinity {
            groups: InfinityGroups {
                g0: self.group0() * other.group0(),
            },
        }
    }
}

impl MulAssign<Infinity> for Infinity {
    fn mul_assign(&mut self, other: Infinity) {
        *self = (*self).mul(other);
    }
}

impl Mul<Line> for Line {
    type Output = Line;

    fn mul(self, other: Line) -> Line {
        Line {
            groups: LineGroups {
                g0: self.group0() * other.group0(),
                g1: self.group1() * other.group1(),
            },
        }
    }
}

impl MulAssign<Line> for Line {
    fn mul_assign(&mut self, other: Line) {
        *self = (*self).mul(other);
    }
}

impl Mul<LineAtInfinity> for LineAtInfinity {
    type Output = LineAtInfinity;

    fn mul(self, other: LineAtInfinity) -> LineAtInfinity {
        LineAtInfinity {
            groups: LineAtInfinityGroups {
                g0: self.group0() * other.group0(),
            },
        }
    }
}

impl MulAssign<LineAtInfinity> for LineAtInfinity {
    fn mul_assign(&mut self, other: LineAtInfinity) {
        *self = (*self).mul(other);
    }
}

impl Mul<LineAtOrigin> for LineAtOrigin {
    type Output = LineAtOrigin;

    fn mul(self, other: LineAtOrigin) -> LineAtOrigin {
        LineAtOrigin {
            groups: LineAtOriginGroups {
                g0: self.group0() * other.group0(),
            },
        }
    }
}

impl MulAssign<LineAtOrigin> for LineAtOrigin {
    fn mul_assign(&mut self, other: LineAtOrigin) {
        *self = (*self).mul(other);
    }
}

impl Mul<Magnitude> for Magnitude {
    type Output = Magnitude;

    fn mul(self, other: Magnitude) -> Magnitude {
        Magnitude {
            groups: MagnitudeGroups {
                g0: self.group0() * other.group0(),
            },
        }
    }
}

impl MulAssign<Magnitude> for Magnitude {
    fn mul_assign(&mut self, other: Magnitude) {
        *self = (*self).mul(other);
    }
}

impl Mul<Motor> for Motor {
    type Output = Motor;

    fn mul(self, other: Motor) -> Motor {
        Motor {
            groups: MotorGroups {
                g0: self.group0() * other.group0(),
                g1: self.group1() * other.group1(),
            },
        }
    }
}

impl MulAssign<Motor> for Motor {
    fn mul_assign(&mut self, other: Motor) {
        *self = (*self).mul(other);
    }
}

impl Mul<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn mul(self, other: MultiVector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: self.group0() * other.group0(),
                g1: self.group1() * other.group1(),
                g2: self.group2() * other.group2(),
                g3: self.group3() * other.group3(),
                g4: self.group4() * other.group4(),
                g5: self.group5() * other.group5(),
                g6: self.group6() * other.group6(),
                g7: self.group7() * other.group7(),
                g8: self.group8() * other.group8(),
                g9: self.group9() * other.group9(),
                g10: self.group10() * other.group10(),
            },
        }
    }
}

impl MulAssign<MultiVector> for MultiVector {
    fn mul_assign(&mut self, other: MultiVector) {
        *self = (*self).mul(other);
    }
}

impl Mul<Origin> for Origin {
    type Output = Origin;

    fn mul(self, other: Origin) -> Origin {
        Origin {
            groups: OriginGroups {
                g0: self.group0() * other.group0(),
            },
        }
    }
}

impl MulAssign<Origin> for Origin {
    fn mul_assign(&mut self, other: Origin) {
        *self = (*self).mul(other);
    }
}

impl Mul<Plane> for Plane {
    type Output = Plane;

    fn mul(self, other: Plane) -> Plane {
        Plane {
            groups: PlaneGroups {
                g0: self.group0() * other.group0(),
            },
        }
    }
}

impl MulAssign<Plane> for Plane {
    fn mul_assign(&mut self, other: Plane) {
        *self = (*self).mul(other);
    }
}

impl Mul<PlaneAtOrigin> for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn mul(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        PlaneAtOrigin {
            groups: PlaneAtOriginGroups {
                g0: self.group0() * other.group0(),
            },
        }
    }
}

impl MulAssign<PlaneAtOrigin> for PlaneAtOrigin {
    fn mul_assign(&mut self, other: PlaneAtOrigin) {
        *self = (*self).mul(other);
    }
}

impl Mul<PointAtInfinity> for PointAtInfinity {
    type Output = PointAtInfinity;

    fn mul(self, other: PointAtInfinity) -> PointAtInfinity {
        PointAtInfinity {
            groups: PointAtInfinityGroups {
                g0: self.group0() * other.group0(),
            },
        }
    }
}

impl MulAssign<PointAtInfinity> for PointAtInfinity {
    fn mul_assign(&mut self, other: PointAtInfinity) {
        *self = (*self).mul(other);
    }
}

impl Mul<PointAtOrigin> for PointAtOrigin {
    type Output = PointAtOrigin;

    fn mul(self, other: PointAtOrigin) -> PointAtOrigin {
        PointAtOrigin {
            groups: PointAtOriginGroups {
                g0: self.group0() * other.group0(),
            },
        }
    }
}

impl MulAssign<PointAtOrigin> for PointAtOrigin {
    fn mul_assign(&mut self, other: PointAtOrigin) {
        *self = (*self).mul(other);
    }
}

impl Mul<Rotor> for Rotor {
    type Output = Rotor;

    fn mul(self, other: Rotor) -> Rotor {
        Rotor {
            groups: RotorGroups {
                g0: self.group0() * other.group0(),
            },
        }
    }
}

impl MulAssign<Rotor> for Rotor {
    fn mul_assign(&mut self, other: Rotor) {
        *self = (*self).mul(other);
    }
}

impl Mul<RoundPoint> for RoundPoint {
    type Output = RoundPoint;

    fn mul(self, other: RoundPoint) -> RoundPoint {
        RoundPoint {
            groups: RoundPointGroups {
                g0: self.group0() * other.group0(),
                g1: self.group1() * other.group1(),
            },
        }
    }
}

impl MulAssign<RoundPoint> for RoundPoint {
    fn mul_assign(&mut self, other: RoundPoint) {
        *self = (*self).mul(other);
    }
}

impl Mul<Scalar> for Scalar {
    type Output = Scalar;

    fn mul(self, other: Scalar) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0() * other.group0(),
            },
        }
    }
}

impl MulAssign<Scalar> for Scalar {
    fn mul_assign(&mut self, other: Scalar) {
        *self = (*self).mul(other);
    }
}

impl Mul<Sphere> for Sphere {
    type Output = Sphere;

    fn mul(self, other: Sphere) -> Sphere {
        Sphere {
            groups: SphereGroups {
                g0: self.group0() * other.group0(),
                g1: self.group1() * other.group1(),
            },
        }
    }
}

impl MulAssign<Sphere> for Sphere {
    fn mul_assign(&mut self, other: Sphere) {
        *self = (*self).mul(other);
    }
}

impl Mul<Translator> for Translator {
    type Output = Translator;

    fn mul(self, other: Translator) -> Translator {
        Translator {
            groups: TranslatorGroups {
                g0: self.group0() * other.group0(),
            },
        }
    }
}

impl MulAssign<Translator> for Translator {
    fn mul_assign(&mut self, other: Translator) {
        *self = (*self).mul(other);
    }
}

impl Sub<AntiScalar> for AntiScalar {
    type Output = AntiScalar;

    fn sub(self, other: AntiScalar) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0() - other.group0(),
            },
        }
    }
}

impl SubAssign<AntiScalar> for AntiScalar {
    fn sub_assign(&mut self, other: AntiScalar) {
        *self = (*self).sub(other);
    }
}

impl Sub<Line> for AntiScalar {
    type Output = Motor;

    fn sub(self, other: Line) -> Motor {
        Motor {
            groups: MotorGroups {
                g0: Simd32x4::from([0.0, 0.0, 0.0, self.group0()]) - Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
                g1: Simd32x3::from(0.0) - other.group1(),
            },
        }
    }
}

impl Sub<LineAtInfinity> for AntiScalar {
    type Output = Translator;

    fn sub(self, other: LineAtInfinity) -> Translator {
        Translator {
            groups: TranslatorGroups {
                g0: Simd32x4::from([0.0, 0.0, 0.0, self.group0()]) - Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            },
        }
    }
}

impl Sub<LineAtOrigin> for AntiScalar {
    type Output = Rotor;

    fn sub(self, other: LineAtOrigin) -> Rotor {
        Rotor {
            groups: RotorGroups {
                g0: Simd32x4::from([0.0, 0.0, 0.0, self.group0()]) - Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            },
        }
    }
}

impl Sub<Magnitude> for AntiScalar {
    type Output = Magnitude;

    fn sub(self, other: Magnitude) -> Magnitude {
        Magnitude {
            groups: MagnitudeGroups {
                g0: Simd32x2::from([0.0, self.group0()]) - other.group0(),
            },
        }
    }
}

impl Sub<Motor> for AntiScalar {
    type Output = Motor;

    fn sub(self, other: Motor) -> Motor {
        Motor {
            groups: MotorGroups {
                g0: Simd32x4::from([0.0, 0.0, 0.0, self.group0()]) - other.group0(),
                g1: Simd32x3::from(0.0) - other.group1(),
            },
        }
    }
}

impl Sub<MultiVector> for AntiScalar {
    type Output = MultiVector;

    fn sub(self, other: MultiVector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from([0.0, self.group0()]) - other.group0(),
                g1: Simd32x3::from(0.0) - other.group1(),
                g2: Simd32x2::from(0.0) - other.group2(),
                g3: Simd32x3::from(0.0) - other.group3(),
                g4: Simd32x3::from(0.0) - other.group4(),
                g5: Simd32x4::from(0.0) - other.group5(),
                g6: Simd32x4::from(0.0) - other.group6(),
                g7: Simd32x3::from(0.0) - other.group7(),
                g8: Simd32x3::from(0.0) - other.group8(),
                g9: Simd32x3::from(0.0) - other.group9(),
                g10: Simd32x2::from(0.0) - other.group10(),
            },
        }
    }
}

impl Sub<Rotor> for AntiScalar {
    type Output = Rotor;

    fn sub(self, other: Rotor) -> Rotor {
        Rotor {
            groups: RotorGroups {
                g0: Simd32x4::from([0.0, 0.0, 0.0, self.group0()]) - other.group0(),
            },
        }
    }
}

impl Sub<Scalar> for AntiScalar {
    type Output = Magnitude;

    fn sub(self, other: Scalar) -> Magnitude {
        Magnitude {
            groups: MagnitudeGroups {
                g0: Simd32x2::from([0.0, self.group0()]) - Simd32x2::from([other.group0(), 0.0]),
            },
        }
    }
}

impl Sub<Translator> for AntiScalar {
    type Output = Translator;

    fn sub(self, other: Translator) -> Translator {
        Translator {
            groups: TranslatorGroups {
                g0: Simd32x4::from([0.0, 0.0, 0.0, self.group0()]) - other.group0(),
            },
        }
    }
}

impl Sub<Circle> for Circle {
    type Output = Circle;

    fn sub(self, other: Circle) -> Circle {
        Circle {
            groups: CircleGroups {
                g0: self.group0() - other.group0(),
                g1: self.group1() - other.group1(),
                g2: self.group2() - other.group2(),
            },
        }
    }
}

impl SubAssign<Circle> for Circle {
    fn sub_assign(&mut self, other: Circle) {
        *self = (*self).sub(other);
    }
}

impl Sub<Line> for Circle {
    type Output = Circle;

    fn sub(self, other: Line) -> Circle {
        Circle {
            groups: CircleGroups {
                g0: self.group0(),
                g1: self.group1() - other.group0(),
                g2: self.group2() - other.group1(),
            },
        }
    }
}

impl SubAssign<Line> for Circle {
    fn sub_assign(&mut self, other: Line) {
        *self = (*self).sub(other);
    }
}

impl Sub<LineAtInfinity> for Circle {
    type Output = Circle;

    fn sub(self, other: LineAtInfinity) -> Circle {
        Circle {
            groups: CircleGroups {
                g0: self.group0(),
                g1: self.group1(),
                g2: self.group2() - other.group0(),
            },
        }
    }
}

impl SubAssign<LineAtInfinity> for Circle {
    fn sub_assign(&mut self, other: LineAtInfinity) {
        *self = (*self).sub(other);
    }
}

impl Sub<LineAtOrigin> for Circle {
    type Output = Circle;

    fn sub(self, other: LineAtOrigin) -> Circle {
        Circle {
            groups: CircleGroups {
                g0: self.group0(),
                g1: self.group1() - other.group0(),
                g2: self.group2(),
            },
        }
    }
}

impl SubAssign<LineAtOrigin> for Circle {
    fn sub_assign(&mut self, other: LineAtOrigin) {
        *self = (*self).sub(other);
    }
}

impl Sub<MultiVector> for Circle {
    type Output = MultiVector;

    fn sub(self, other: MultiVector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0) - other.group0(),
                g1: Simd32x3::from(0.0) - other.group1(),
                g2: Simd32x2::from(0.0) - other.group2(),
                g3: Simd32x3::from(0.0) - other.group3(),
                g4: Simd32x3::from(0.0) - other.group4(),
                g5: Simd32x4::from(0.0) - other.group5(),
                g6: self.group0() - other.group6(),
                g7: self.group1() - other.group7(),
                g8: self.group2() - other.group8(),
                g9: Simd32x3::from(0.0) - other.group9(),
                g10: Simd32x2::from(0.0) - other.group10(),
            },
        }
    }
}

impl Sub<Dipole> for Dipole {
    type Output = Dipole;

    fn sub(self, other: Dipole) -> Dipole {
        Dipole {
            groups: DipoleGroups {
                g0: self.group0() - other.group0(),
                g1: self.group1() - other.group1(),
                g2: self.group2() - other.group2(),
            },
        }
    }
}

impl SubAssign<Dipole> for Dipole {
    fn sub_assign(&mut self, other: Dipole) {
        *self = (*self).sub(other);
    }
}

impl Sub<FlatPoint> for Dipole {
    type Output = Dipole;

    fn sub(self, other: FlatPoint) -> Dipole {
        Dipole {
            groups: DipoleGroups {
                g0: self.group0(),
                g1: self.group1(),
                g2: self.group2() - other.group0(),
            },
        }
    }
}

impl SubAssign<FlatPoint> for Dipole {
    fn sub_assign(&mut self, other: FlatPoint) {
        *self = (*self).sub(other);
    }
}

impl Sub<MultiVector> for Dipole {
    type Output = MultiVector;

    fn sub(self, other: MultiVector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0) - other.group0(),
                g1: Simd32x3::from(0.0) - other.group1(),
                g2: Simd32x2::from(0.0) - other.group2(),
                g3: self.group0() - other.group3(),
                g4: self.group1() - other.group4(),
                g5: self.group2() - other.group5(),
                g6: Simd32x4::from(0.0) - other.group6(),
                g7: Simd32x3::from(0.0) - other.group7(),
                g8: Simd32x3::from(0.0) - other.group8(),
                g9: Simd32x3::from(0.0) - other.group9(),
                g10: Simd32x2::from(0.0) - other.group10(),
            },
        }
    }
}

impl Sub<PointAtInfinity> for Dipole {
    type Output = Dipole;

    fn sub(self, other: PointAtInfinity) -> Dipole {
        Dipole {
            groups: DipoleGroups {
                g0: self.group0(),
                g1: self.group1(),
                g2: self.group2() - Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            },
        }
    }
}

impl SubAssign<PointAtInfinity> for Dipole {
    fn sub_assign(&mut self, other: PointAtInfinity) {
        *self = (*self).sub(other);
    }
}

impl Sub<PointAtOrigin> for Dipole {
    type Output = Dipole;

    fn sub(self, other: PointAtOrigin) -> Dipole {
        Dipole {
            groups: DipoleGroups {
                g0: self.group0(),
                g1: self.group1(),
                g2: self.group2() - Simd32x4::from([0.0, 0.0, 0.0, other.group0()]),
            },
        }
    }
}

impl SubAssign<PointAtOrigin> for Dipole {
    fn sub_assign(&mut self, other: PointAtOrigin) {
        *self = (*self).sub(other);
    }
}

impl Sub<Dipole> for FlatPoint {
    type Output = Dipole;

    fn sub(self, other: Dipole) -> Dipole {
        Dipole {
            groups: DipoleGroups {
                g0: Simd32x3::from(0.0) - other.group0(),
                g1: Simd32x3::from(0.0) - other.group1(),
                g2: self.group0() - other.group2(),
            },
        }
    }
}

impl Sub<FlatPoint> for FlatPoint {
    type Output = FlatPoint;

    fn sub(self, other: FlatPoint) -> FlatPoint {
        FlatPoint {
            groups: FlatPointGroups {
                g0: self.group0() - other.group0(),
            },
        }
    }
}

impl SubAssign<FlatPoint> for FlatPoint {
    fn sub_assign(&mut self, other: FlatPoint) {
        *self = (*self).sub(other);
    }
}

impl Sub<Flector> for FlatPoint {
    type Output = Flector;

    fn sub(self, other: Flector) -> Flector {
        Flector {
            groups: FlectorGroups {
                g0: self.group0() - other.group0(),
                g1: Simd32x4::from(0.0) - other.group1(),
            },
        }
    }
}

impl Sub<MultiVector> for FlatPoint {
    type Output = MultiVector;

    fn sub(self, other: MultiVector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0) - other.group0(),
                g1: Simd32x3::from(0.0) - other.group1(),
                g2: Simd32x2::from(0.0) - other.group2(),
                g3: Simd32x3::from(0.0) - other.group3(),
                g4: Simd32x3::from(0.0) - other.group4(),
                g5: self.group0() - other.group5(),
                g6: Simd32x4::from(0.0) - other.group6(),
                g7: Simd32x3::from(0.0) - other.group7(),
                g8: Simd32x3::from(0.0) - other.group8(),
                g9: Simd32x3::from(0.0) - other.group9(),
                g10: Simd32x2::from(0.0) - other.group10(),
            },
        }
    }
}

impl Sub<Plane> for FlatPoint {
    type Output = Flector;

    fn sub(self, other: Plane) -> Flector {
        Flector {
            groups: FlectorGroups {
                g0: self.group0(),
                g1: Simd32x4::from(0.0) - other.group0(),
            },
        }
    }
}

impl Sub<PointAtInfinity> for FlatPoint {
    type Output = FlatPoint;

    fn sub(self, other: PointAtInfinity) -> FlatPoint {
        FlatPoint {
            groups: FlatPointGroups {
                g0: self.group0() - Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            },
        }
    }
}

impl SubAssign<PointAtInfinity> for FlatPoint {
    fn sub_assign(&mut self, other: PointAtInfinity) {
        *self = (*self).sub(other);
    }
}

impl Sub<PointAtOrigin> for FlatPoint {
    type Output = FlatPoint;

    fn sub(self, other: PointAtOrigin) -> FlatPoint {
        FlatPoint {
            groups: FlatPointGroups {
                g0: self.group0() - Simd32x4::from([0.0, 0.0, 0.0, other.group0()]),
            },
        }
    }
}

impl SubAssign<PointAtOrigin> for FlatPoint {
    fn sub_assign(&mut self, other: PointAtOrigin) {
        *self = (*self).sub(other);
    }
}

impl Sub<FlatPoint> for Flector {
    type Output = Flector;

    fn sub(self, other: FlatPoint) -> Flector {
        Flector {
            groups: FlectorGroups {
                g0: self.group0() - other.group0(),
                g1: self.group1(),
            },
        }
    }
}

impl SubAssign<FlatPoint> for Flector {
    fn sub_assign(&mut self, other: FlatPoint) {
        *self = (*self).sub(other);
    }
}

impl Sub<Flector> for Flector {
    type Output = Flector;

    fn sub(self, other: Flector) -> Flector {
        Flector {
            groups: FlectorGroups {
                g0: self.group0() - other.group0(),
                g1: self.group1() - other.group1(),
            },
        }
    }
}

impl SubAssign<Flector> for Flector {
    fn sub_assign(&mut self, other: Flector) {
        *self = (*self).sub(other);
    }
}

impl Sub<Horizon> for Flector {
    type Output = Flector;

    fn sub(self, other: Horizon) -> Flector {
        Flector {
            groups: FlectorGroups {
                g0: self.group0(),
                g1: self.group1() - Simd32x4::from([0.0, 0.0, 0.0, other.group0()]),
            },
        }
    }
}

impl SubAssign<Horizon> for Flector {
    fn sub_assign(&mut self, other: Horizon) {
        *self = (*self).sub(other);
    }
}

impl Sub<MultiVector> for Flector {
    type Output = MultiVector;

    fn sub(self, other: MultiVector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0) - other.group0(),
                g1: Simd32x3::from(0.0) - other.group1(),
                g2: Simd32x2::from(0.0) - other.group2(),
                g3: Simd32x3::from(0.0) - other.group3(),
                g4: Simd32x3::from(0.0) - other.group4(),
                g5: self.group0() - other.group5(),
                g6: Simd32x4::from(0.0) - other.group6(),
                g7: Simd32x3::from(0.0) - other.group7(),
                g8: Simd32x3::from(0.0) - other.group8(),
                g9: Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]) - other.group9(),
                g10: Simd32x2::from([0.0, self.group1()[3]]) - other.group10(),
            },
        }
    }
}

impl Sub<Plane> for Flector {
    type Output = Flector;

    fn sub(self, other: Plane) -> Flector {
        Flector {
            groups: FlectorGroups {
                g0: self.group0(),
                g1: self.group1() - other.group0(),
            },
        }
    }
}

impl SubAssign<Plane> for Flector {
    fn sub_assign(&mut self, other: Plane) {
        *self = (*self).sub(other);
    }
}

impl Sub<PlaneAtOrigin> for Flector {
    type Output = Flector;

    fn sub(self, other: PlaneAtOrigin) -> Flector {
        Flector {
            groups: FlectorGroups {
                g0: self.group0(),
                g1: self.group1() - Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            },
        }
    }
}

impl SubAssign<PlaneAtOrigin> for Flector {
    fn sub_assign(&mut self, other: PlaneAtOrigin) {
        *self = (*self).sub(other);
    }
}

impl Sub<PointAtInfinity> for Flector {
    type Output = Flector;

    fn sub(self, other: PointAtInfinity) -> Flector {
        Flector {
            groups: FlectorGroups {
                g0: self.group0() - Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
                g1: self.group1(),
            },
        }
    }
}

impl SubAssign<PointAtInfinity> for Flector {
    fn sub_assign(&mut self, other: PointAtInfinity) {
        *self = (*self).sub(other);
    }
}

impl Sub<PointAtOrigin> for Flector {
    type Output = Flector;

    fn sub(self, other: PointAtOrigin) -> Flector {
        Flector {
            groups: FlectorGroups {
                g0: self.group0() - Simd32x4::from([0.0, 0.0, 0.0, other.group0()]),
                g1: self.group1(),
            },
        }
    }
}

impl SubAssign<PointAtOrigin> for Flector {
    fn sub_assign(&mut self, other: PointAtOrigin) {
        *self = (*self).sub(other);
    }
}

impl Sub<Flector> for Horizon {
    type Output = Flector;

    fn sub(self, other: Flector) -> Flector {
        Flector {
            groups: FlectorGroups {
                g0: Simd32x4::from(0.0) - other.group0(),
                g1: Simd32x4::from([0.0, 0.0, 0.0, self.group0()]) - other.group1(),
            },
        }
    }
}

impl Sub<Horizon> for Horizon {
    type Output = Horizon;

    fn sub(self, other: Horizon) -> Horizon {
        Horizon {
            groups: HorizonGroups {
                g0: self.group0() - other.group0(),
            },
        }
    }
}

impl SubAssign<Horizon> for Horizon {
    fn sub_assign(&mut self, other: Horizon) {
        *self = (*self).sub(other);
    }
}

impl Sub<MultiVector> for Horizon {
    type Output = MultiVector;

    fn sub(self, other: MultiVector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0) - other.group0(),
                g1: Simd32x3::from(0.0) - other.group1(),
                g2: Simd32x2::from(0.0) - other.group2(),
                g3: Simd32x3::from(0.0) - other.group3(),
                g4: Simd32x3::from(0.0) - other.group4(),
                g5: Simd32x4::from(0.0) - other.group5(),
                g6: Simd32x4::from(0.0) - other.group6(),
                g7: Simd32x3::from(0.0) - other.group7(),
                g8: Simd32x3::from(0.0) - other.group8(),
                g9: Simd32x3::from(0.0) - other.group9(),
                g10: Simd32x2::from([0.0, self.group0()]) - other.group10(),
            },
        }
    }
}

impl Sub<Plane> for Horizon {
    type Output = Plane;

    fn sub(self, other: Plane) -> Plane {
        Plane {
            groups: PlaneGroups {
                g0: Simd32x4::from([0.0, 0.0, 0.0, self.group0()]) - other.group0(),
            },
        }
    }
}

impl Sub<PlaneAtOrigin> for Horizon {
    type Output = Plane;

    fn sub(self, other: PlaneAtOrigin) -> Plane {
        Plane {
            groups: PlaneGroups {
                g0: Simd32x4::from([0.0, 0.0, 0.0, self.group0()]) - Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            },
        }
    }
}

impl Sub<Sphere> for Horizon {
    type Output = Sphere;

    fn sub(self, other: Sphere) -> Sphere {
        Sphere {
            groups: SphereGroups {
                g0: Simd32x3::from(0.0) - other.group0(),
                g1: Simd32x2::from([0.0, self.group0()]) - other.group1(),
            },
        }
    }
}

impl Sub<Infinity> for Infinity {
    type Output = Infinity;

    fn sub(self, other: Infinity) -> Infinity {
        Infinity {
            groups: InfinityGroups {
                g0: self.group0() - other.group0(),
            },
        }
    }
}

impl SubAssign<Infinity> for Infinity {
    fn sub_assign(&mut self, other: Infinity) {
        *self = (*self).sub(other);
    }
}

impl Sub<MultiVector> for Infinity {
    type Output = MultiVector;

    fn sub(self, other: MultiVector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0) - other.group0(),
                g1: Simd32x3::from(0.0) - other.group1(),
                g2: Simd32x2::from([0.0, self.group0()]) - other.group2(),
                g3: Simd32x3::from(0.0) - other.group3(),
                g4: Simd32x3::from(0.0) - other.group4(),
                g5: Simd32x4::from(0.0) - other.group5(),
                g6: Simd32x4::from(0.0) - other.group6(),
                g7: Simd32x3::from(0.0) - other.group7(),
                g8: Simd32x3::from(0.0) - other.group8(),
                g9: Simd32x3::from(0.0) - other.group9(),
                g10: Simd32x2::from(0.0) - other.group10(),
            },
        }
    }
}

impl Sub<RoundPoint> for Infinity {
    type Output = RoundPoint;

    fn sub(self, other: RoundPoint) -> RoundPoint {
        RoundPoint {
            groups: RoundPointGroups {
                g0: Simd32x3::from(0.0) - other.group0(),
                g1: Simd32x2::from([0.0, self.group0()]) - other.group1(),
            },
        }
    }
}

impl Sub<AntiScalar> for Line {
    type Output = Motor;

    fn sub(self, other: AntiScalar) -> Motor {
        Motor {
            groups: MotorGroups {
                g0: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]) - Simd32x4::from([0.0, 0.0, 0.0, other.group0()]),
                g1: self.group1(),
            },
        }
    }
}

impl Sub<Circle> for Line {
    type Output = Circle;

    fn sub(self, other: Circle) -> Circle {
        Circle {
            groups: CircleGroups {
                g0: Simd32x4::from(0.0) - other.group0(),
                g1: self.group0() - other.group1(),
                g2: self.group1() - other.group2(),
            },
        }
    }
}

impl Sub<Line> for Line {
    type Output = Line;

    fn sub(self, other: Line) -> Line {
        Line {
            groups: LineGroups {
                g0: self.group0() - other.group0(),
                g1: self.group1() - other.group1(),
            },
        }
    }
}

impl SubAssign<Line> for Line {
    fn sub_assign(&mut self, other: Line) {
        *self = (*self).sub(other);
    }
}

impl Sub<LineAtInfinity> for Line {
    type Output = Line;

    fn sub(self, other: LineAtInfinity) -> Line {
        Line {
            groups: LineGroups {
                g0: self.group0(),
                g1: self.group1() - other.group0(),
            },
        }
    }
}

impl SubAssign<LineAtInfinity> for Line {
    fn sub_assign(&mut self, other: LineAtInfinity) {
        *self = (*self).sub(other);
    }
}

impl Sub<LineAtOrigin> for Line {
    type Output = Line;

    fn sub(self, other: LineAtOrigin) -> Line {
        Line {
            groups: LineGroups {
                g0: self.group0() - other.group0(),
                g1: self.group1(),
            },
        }
    }
}

impl SubAssign<LineAtOrigin> for Line {
    fn sub_assign(&mut self, other: LineAtOrigin) {
        *self = (*self).sub(other);
    }
}

impl Sub<Motor> for Line {
    type Output = Motor;

    fn sub(self, other: Motor) -> Motor {
        Motor {
            groups: MotorGroups {
                g0: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]) - other.group0(),
                g1: self.group1() - other.group1(),
            },
        }
    }
}

impl Sub<MultiVector> for Line {
    type Output = MultiVector;

    fn sub(self, other: MultiVector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0) - other.group0(),
                g1: Simd32x3::from(0.0) - other.group1(),
                g2: Simd32x2::from(0.0) - other.group2(),
                g3: Simd32x3::from(0.0) - other.group3(),
                g4: Simd32x3::from(0.0) - other.group4(),
                g5: Simd32x4::from(0.0) - other.group5(),
                g6: Simd32x4::from(0.0) - other.group6(),
                g7: self.group0() - other.group7(),
                g8: self.group1() - other.group8(),
                g9: Simd32x3::from(0.0) - other.group9(),
                g10: Simd32x2::from(0.0) - other.group10(),
            },
        }
    }
}

impl Sub<Rotor> for Line {
    type Output = Motor;

    fn sub(self, other: Rotor) -> Motor {
        Motor {
            groups: MotorGroups {
                g0: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]) - other.group0(),
                g1: self.group1(),
            },
        }
    }
}

impl Sub<Translator> for Line {
    type Output = Motor;

    fn sub(self, other: Translator) -> Motor {
        Motor {
            groups: MotorGroups {
                g0: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]) - Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]),
                g1: self.group1() - Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            },
        }
    }
}

impl Sub<AntiScalar> for LineAtInfinity {
    type Output = Translator;

    fn sub(self, other: AntiScalar) -> Translator {
        Translator {
            groups: TranslatorGroups {
                g0: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]) - Simd32x4::from([0.0, 0.0, 0.0, other.group0()]),
            },
        }
    }
}

impl Sub<Circle> for LineAtInfinity {
    type Output = Circle;

    fn sub(self, other: Circle) -> Circle {
        Circle {
            groups: CircleGroups {
                g0: Simd32x4::from(0.0) - other.group0(),
                g1: Simd32x3::from(0.0) - other.group1(),
                g2: self.group0() - other.group2(),
            },
        }
    }
}

impl Sub<Line> for LineAtInfinity {
    type Output = Line;

    fn sub(self, other: Line) -> Line {
        Line {
            groups: LineGroups {
                g0: Simd32x3::from(0.0) - other.group0(),
                g1: self.group0() - other.group1(),
            },
        }
    }
}

impl Sub<LineAtInfinity> for LineAtInfinity {
    type Output = LineAtInfinity;

    fn sub(self, other: LineAtInfinity) -> LineAtInfinity {
        LineAtInfinity {
            groups: LineAtInfinityGroups {
                g0: self.group0() - other.group0(),
            },
        }
    }
}

impl SubAssign<LineAtInfinity> for LineAtInfinity {
    fn sub_assign(&mut self, other: LineAtInfinity) {
        *self = (*self).sub(other);
    }
}

impl Sub<LineAtOrigin> for LineAtInfinity {
    type Output = Line;

    fn sub(self, other: LineAtOrigin) -> Line {
        Line {
            groups: LineGroups {
                g0: Simd32x3::from(0.0) - other.group0(),
                g1: self.group0(),
            },
        }
    }
}

impl Sub<Motor> for LineAtInfinity {
    type Output = Motor;

    fn sub(self, other: Motor) -> Motor {
        Motor {
            groups: MotorGroups {
                g0: Simd32x4::from(0.0) - other.group0(),
                g1: self.group0() - other.group1(),
            },
        }
    }
}

impl Sub<MultiVector> for LineAtInfinity {
    type Output = MultiVector;

    fn sub(self, other: MultiVector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0) - other.group0(),
                g1: Simd32x3::from(0.0) - other.group1(),
                g2: Simd32x2::from(0.0) - other.group2(),
                g3: Simd32x3::from(0.0) - other.group3(),
                g4: Simd32x3::from(0.0) - other.group4(),
                g5: Simd32x4::from(0.0) - other.group5(),
                g6: Simd32x4::from(0.0) - other.group6(),
                g7: Simd32x3::from(0.0) - other.group7(),
                g8: self.group0() - other.group8(),
                g9: Simd32x3::from(0.0) - other.group9(),
                g10: Simd32x2::from(0.0) - other.group10(),
            },
        }
    }
}

impl Sub<Rotor> for LineAtInfinity {
    type Output = Motor;

    fn sub(self, other: Rotor) -> Motor {
        Motor {
            groups: MotorGroups {
                g0: Simd32x4::from(0.0) - other.group0(),
                g1: self.group0(),
            },
        }
    }
}

impl Sub<Translator> for LineAtInfinity {
    type Output = Translator;

    fn sub(self, other: Translator) -> Translator {
        Translator {
            groups: TranslatorGroups {
                g0: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]) - other.group0(),
            },
        }
    }
}

impl Sub<AntiScalar> for LineAtOrigin {
    type Output = Rotor;

    fn sub(self, other: AntiScalar) -> Rotor {
        Rotor {
            groups: RotorGroups {
                g0: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]) - Simd32x4::from([0.0, 0.0, 0.0, other.group0()]),
            },
        }
    }
}

impl Sub<Circle> for LineAtOrigin {
    type Output = Circle;

    fn sub(self, other: Circle) -> Circle {
        Circle {
            groups: CircleGroups {
                g0: Simd32x4::from(0.0) - other.group0(),
                g1: self.group0() - other.group1(),
                g2: Simd32x3::from(0.0) - other.group2(),
            },
        }
    }
}

impl Sub<Line> for LineAtOrigin {
    type Output = Line;

    fn sub(self, other: Line) -> Line {
        Line {
            groups: LineGroups {
                g0: self.group0() - other.group0(),
                g1: Simd32x3::from(0.0) - other.group1(),
            },
        }
    }
}

impl Sub<LineAtInfinity> for LineAtOrigin {
    type Output = Line;

    fn sub(self, other: LineAtInfinity) -> Line {
        Line {
            groups: LineGroups {
                g0: self.group0(),
                g1: Simd32x3::from(0.0) - other.group0(),
            },
        }
    }
}

impl Sub<LineAtOrigin> for LineAtOrigin {
    type Output = LineAtOrigin;

    fn sub(self, other: LineAtOrigin) -> LineAtOrigin {
        LineAtOrigin {
            groups: LineAtOriginGroups {
                g0: self.group0() - other.group0(),
            },
        }
    }
}

impl SubAssign<LineAtOrigin> for LineAtOrigin {
    fn sub_assign(&mut self, other: LineAtOrigin) {
        *self = (*self).sub(other);
    }
}

impl Sub<Motor> for LineAtOrigin {
    type Output = Motor;

    fn sub(self, other: Motor) -> Motor {
        Motor {
            groups: MotorGroups {
                g0: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]) - other.group0(),
                g1: Simd32x3::from(0.0) - other.group1(),
            },
        }
    }
}

impl Sub<MultiVector> for LineAtOrigin {
    type Output = MultiVector;

    fn sub(self, other: MultiVector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0) - other.group0(),
                g1: Simd32x3::from(0.0) - other.group1(),
                g2: Simd32x2::from(0.0) - other.group2(),
                g3: Simd32x3::from(0.0) - other.group3(),
                g4: Simd32x3::from(0.0) - other.group4(),
                g5: Simd32x4::from(0.0) - other.group5(),
                g6: Simd32x4::from(0.0) - other.group6(),
                g7: self.group0() - other.group7(),
                g8: Simd32x3::from(0.0) - other.group8(),
                g9: Simd32x3::from(0.0) - other.group9(),
                g10: Simd32x2::from(0.0) - other.group10(),
            },
        }
    }
}

impl Sub<Rotor> for LineAtOrigin {
    type Output = Rotor;

    fn sub(self, other: Rotor) -> Rotor {
        Rotor {
            groups: RotorGroups {
                g0: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]) - other.group0(),
            },
        }
    }
}

impl Sub<Translator> for LineAtOrigin {
    type Output = Motor;

    fn sub(self, other: Translator) -> Motor {
        Motor {
            groups: MotorGroups {
                g0: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]) - Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]),
                g1: Simd32x3::from(0.0) - Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            },
        }
    }
}

impl Sub<AntiScalar> for Magnitude {
    type Output = Magnitude;

    fn sub(self, other: AntiScalar) -> Magnitude {
        Magnitude {
            groups: MagnitudeGroups {
                g0: self.group0() - Simd32x2::from([0.0, other.group0()]),
            },
        }
    }
}

impl SubAssign<AntiScalar> for Magnitude {
    fn sub_assign(&mut self, other: AntiScalar) {
        *self = (*self).sub(other);
    }
}

impl Sub<Magnitude> for Magnitude {
    type Output = Magnitude;

    fn sub(self, other: Magnitude) -> Magnitude {
        Magnitude {
            groups: MagnitudeGroups {
                g0: self.group0() - other.group0(),
            },
        }
    }
}

impl SubAssign<Magnitude> for Magnitude {
    fn sub_assign(&mut self, other: Magnitude) {
        *self = (*self).sub(other);
    }
}

impl Sub<MultiVector> for Magnitude {
    type Output = MultiVector;

    fn sub(self, other: MultiVector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: self.group0() - other.group0(),
                g1: Simd32x3::from(0.0) - other.group1(),
                g2: Simd32x2::from(0.0) - other.group2(),
                g3: Simd32x3::from(0.0) - other.group3(),
                g4: Simd32x3::from(0.0) - other.group4(),
                g5: Simd32x4::from(0.0) - other.group5(),
                g6: Simd32x4::from(0.0) - other.group6(),
                g7: Simd32x3::from(0.0) - other.group7(),
                g8: Simd32x3::from(0.0) - other.group8(),
                g9: Simd32x3::from(0.0) - other.group9(),
                g10: Simd32x2::from(0.0) - other.group10(),
            },
        }
    }
}

impl Sub<Scalar> for Magnitude {
    type Output = Magnitude;

    fn sub(self, other: Scalar) -> Magnitude {
        Magnitude {
            groups: MagnitudeGroups {
                g0: self.group0() - Simd32x2::from([other.group0(), 0.0]),
            },
        }
    }
}

impl SubAssign<Scalar> for Magnitude {
    fn sub_assign(&mut self, other: Scalar) {
        *self = (*self).sub(other);
    }
}

impl Sub<AntiScalar> for Motor {
    type Output = Motor;

    fn sub(self, other: AntiScalar) -> Motor {
        Motor {
            groups: MotorGroups {
                g0: self.group0() - Simd32x4::from([0.0, 0.0, 0.0, other.group0()]),
                g1: self.group1(),
            },
        }
    }
}

impl SubAssign<AntiScalar> for Motor {
    fn sub_assign(&mut self, other: AntiScalar) {
        *self = (*self).sub(other);
    }
}

impl Sub<Line> for Motor {
    type Output = Motor;

    fn sub(self, other: Line) -> Motor {
        Motor {
            groups: MotorGroups {
                g0: self.group0() - Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
                g1: self.group1() - other.group1(),
            },
        }
    }
}

impl SubAssign<Line> for Motor {
    fn sub_assign(&mut self, other: Line) {
        *self = (*self).sub(other);
    }
}

impl Sub<LineAtInfinity> for Motor {
    type Output = Motor;

    fn sub(self, other: LineAtInfinity) -> Motor {
        Motor {
            groups: MotorGroups {
                g0: self.group0(),
                g1: self.group1() - other.group0(),
            },
        }
    }
}

impl SubAssign<LineAtInfinity> for Motor {
    fn sub_assign(&mut self, other: LineAtInfinity) {
        *self = (*self).sub(other);
    }
}

impl Sub<LineAtOrigin> for Motor {
    type Output = Motor;

    fn sub(self, other: LineAtOrigin) -> Motor {
        Motor {
            groups: MotorGroups {
                g0: self.group0() - Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
                g1: self.group1(),
            },
        }
    }
}

impl SubAssign<LineAtOrigin> for Motor {
    fn sub_assign(&mut self, other: LineAtOrigin) {
        *self = (*self).sub(other);
    }
}

impl Sub<Motor> for Motor {
    type Output = Motor;

    fn sub(self, other: Motor) -> Motor {
        Motor {
            groups: MotorGroups {
                g0: self.group0() - other.group0(),
                g1: self.group1() - other.group1(),
            },
        }
    }
}

impl SubAssign<Motor> for Motor {
    fn sub_assign(&mut self, other: Motor) {
        *self = (*self).sub(other);
    }
}

impl Sub<MultiVector> for Motor {
    type Output = MultiVector;

    fn sub(self, other: MultiVector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from([0.0, self.group0()[3]]) - other.group0(),
                g1: Simd32x3::from(0.0) - other.group1(),
                g2: Simd32x2::from(0.0) - other.group2(),
                g3: Simd32x3::from(0.0) - other.group3(),
                g4: Simd32x3::from(0.0) - other.group4(),
                g5: Simd32x4::from(0.0) - other.group5(),
                g6: Simd32x4::from(0.0) - other.group6(),
                g7: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) - other.group7(),
                g8: self.group1() - other.group8(),
                g9: Simd32x3::from(0.0) - other.group9(),
                g10: Simd32x2::from(0.0) - other.group10(),
            },
        }
    }
}

impl Sub<Rotor> for Motor {
    type Output = Motor;

    fn sub(self, other: Rotor) -> Motor {
        Motor {
            groups: MotorGroups {
                g0: self.group0() - other.group0(),
                g1: self.group1(),
            },
        }
    }
}

impl SubAssign<Rotor> for Motor {
    fn sub_assign(&mut self, other: Rotor) {
        *self = (*self).sub(other);
    }
}

impl Sub<Translator> for Motor {
    type Output = Motor;

    fn sub(self, other: Translator) -> Motor {
        Motor {
            groups: MotorGroups {
                g0: self.group0() - Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]),
                g1: self.group1() - Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            },
        }
    }
}

impl SubAssign<Translator> for Motor {
    fn sub_assign(&mut self, other: Translator) {
        *self = (*self).sub(other);
    }
}

impl Sub<AntiScalar> for MultiVector {
    type Output = MultiVector;

    fn sub(self, other: AntiScalar) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: self.group0() - Simd32x2::from([0.0, other.group0()]),
                g1: self.group1(),
                g2: self.group2(),
                g3: self.group3(),
                g4: self.group4(),
                g5: self.group5(),
                g6: self.group6(),
                g7: self.group7(),
                g8: self.group8(),
                g9: self.group9(),
                g10: self.group10(),
            },
        }
    }
}

impl SubAssign<AntiScalar> for MultiVector {
    fn sub_assign(&mut self, other: AntiScalar) {
        *self = (*self).sub(other);
    }
}

impl Sub<Circle> for MultiVector {
    type Output = MultiVector;

    fn sub(self, other: Circle) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: self.group0(),
                g1: self.group1(),
                g2: self.group2(),
                g3: self.group3(),
                g4: self.group4(),
                g5: self.group5(),
                g6: self.group6() - other.group0(),
                g7: self.group7() - other.group1(),
                g8: self.group8() - other.group2(),
                g9: self.group9(),
                g10: self.group10(),
            },
        }
    }
}

impl SubAssign<Circle> for MultiVector {
    fn sub_assign(&mut self, other: Circle) {
        *self = (*self).sub(other);
    }
}

impl Sub<Dipole> for MultiVector {
    type Output = MultiVector;

    fn sub(self, other: Dipole) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: self.group0(),
                g1: self.group1(),
                g2: self.group2(),
                g3: self.group3() - other.group0(),
                g4: self.group4() - other.group1(),
                g5: self.group5() - other.group2(),
                g6: self.group6(),
                g7: self.group7(),
                g8: self.group8(),
                g9: self.group9(),
                g10: self.group10(),
            },
        }
    }
}

impl SubAssign<Dipole> for MultiVector {
    fn sub_assign(&mut self, other: Dipole) {
        *self = (*self).sub(other);
    }
}

impl Sub<FlatPoint> for MultiVector {
    type Output = MultiVector;

    fn sub(self, other: FlatPoint) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: self.group0(),
                g1: self.group1(),
                g2: self.group2(),
                g3: self.group3(),
                g4: self.group4(),
                g5: self.group5() - other.group0(),
                g6: self.group6(),
                g7: self.group7(),
                g8: self.group8(),
                g9: self.group9(),
                g10: self.group10(),
            },
        }
    }
}

impl SubAssign<FlatPoint> for MultiVector {
    fn sub_assign(&mut self, other: FlatPoint) {
        *self = (*self).sub(other);
    }
}

impl Sub<Flector> for MultiVector {
    type Output = MultiVector;

    fn sub(self, other: Flector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: self.group0(),
                g1: self.group1(),
                g2: self.group2(),
                g3: self.group3(),
                g4: self.group4(),
                g5: self.group5() - other.group0(),
                g6: self.group6(),
                g7: self.group7(),
                g8: self.group8(),
                g9: self.group9() - Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
                g10: self.group10() - Simd32x2::from([0.0, other.group1()[3]]),
            },
        }
    }
}

impl SubAssign<Flector> for MultiVector {
    fn sub_assign(&mut self, other: Flector) {
        *self = (*self).sub(other);
    }
}

impl Sub<Horizon> for MultiVector {
    type Output = MultiVector;

    fn sub(self, other: Horizon) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: self.group0(),
                g1: self.group1(),
                g2: self.group2(),
                g3: self.group3(),
                g4: self.group4(),
                g5: self.group5(),
                g6: self.group6(),
                g7: self.group7(),
                g8: self.group8(),
                g9: self.group9(),
                g10: self.group10() - Simd32x2::from([0.0, other.group0()]),
            },
        }
    }
}

impl SubAssign<Horizon> for MultiVector {
    fn sub_assign(&mut self, other: Horizon) {
        *self = (*self).sub(other);
    }
}

impl Sub<Infinity> for MultiVector {
    type Output = MultiVector;

    fn sub(self, other: Infinity) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: self.group0(),
                g1: self.group1(),
                g2: self.group2() - Simd32x2::from([0.0, other.group0()]),
                g3: self.group3(),
                g4: self.group4(),
                g5: self.group5(),
                g6: self.group6(),
                g7: self.group7(),
                g8: self.group8(),
                g9: self.group9(),
                g10: self.group10(),
            },
        }
    }
}

impl SubAssign<Infinity> for MultiVector {
    fn sub_assign(&mut self, other: Infinity) {
        *self = (*self).sub(other);
    }
}

impl Sub<Line> for MultiVector {
    type Output = MultiVector;

    fn sub(self, other: Line) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: self.group0(),
                g1: self.group1(),
                g2: self.group2(),
                g3: self.group3(),
                g4: self.group4(),
                g5: self.group5(),
                g6: self.group6(),
                g7: self.group7() - other.group0(),
                g8: self.group8() - other.group1(),
                g9: self.group9(),
                g10: self.group10(),
            },
        }
    }
}

impl SubAssign<Line> for MultiVector {
    fn sub_assign(&mut self, other: Line) {
        *self = (*self).sub(other);
    }
}

impl Sub<LineAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn sub(self, other: LineAtInfinity) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: self.group0(),
                g1: self.group1(),
                g2: self.group2(),
                g3: self.group3(),
                g4: self.group4(),
                g5: self.group5(),
                g6: self.group6(),
                g7: self.group7(),
                g8: self.group8() - other.group0(),
                g9: self.group9(),
                g10: self.group10(),
            },
        }
    }
}

impl SubAssign<LineAtInfinity> for MultiVector {
    fn sub_assign(&mut self, other: LineAtInfinity) {
        *self = (*self).sub(other);
    }
}

impl Sub<LineAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn sub(self, other: LineAtOrigin) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: self.group0(),
                g1: self.group1(),
                g2: self.group2(),
                g3: self.group3(),
                g4: self.group4(),
                g5: self.group5(),
                g6: self.group6(),
                g7: self.group7() - other.group0(),
                g8: self.group8(),
                g9: self.group9(),
                g10: self.group10(),
            },
        }
    }
}

impl SubAssign<LineAtOrigin> for MultiVector {
    fn sub_assign(&mut self, other: LineAtOrigin) {
        *self = (*self).sub(other);
    }
}

impl Sub<Magnitude> for MultiVector {
    type Output = MultiVector;

    fn sub(self, other: Magnitude) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: self.group0() - other.group0(),
                g1: self.group1(),
                g2: self.group2(),
                g3: self.group3(),
                g4: self.group4(),
                g5: self.group5(),
                g6: self.group6(),
                g7: self.group7(),
                g8: self.group8(),
                g9: self.group9(),
                g10: self.group10(),
            },
        }
    }
}

impl SubAssign<Magnitude> for MultiVector {
    fn sub_assign(&mut self, other: Magnitude) {
        *self = (*self).sub(other);
    }
}

impl Sub<Motor> for MultiVector {
    type Output = MultiVector;

    fn sub(self, other: Motor) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: self.group0() - Simd32x2::from([0.0, other.group0()[3]]),
                g1: self.group1(),
                g2: self.group2(),
                g3: self.group3(),
                g4: self.group4(),
                g5: self.group5(),
                g6: self.group6(),
                g7: self.group7() - Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
                g8: self.group8() - other.group1(),
                g9: self.group9(),
                g10: self.group10(),
            },
        }
    }
}

impl SubAssign<Motor> for MultiVector {
    fn sub_assign(&mut self, other: Motor) {
        *self = (*self).sub(other);
    }
}

impl Sub<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn sub(self, other: MultiVector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: self.group0() - other.group0(),
                g1: self.group1() - other.group1(),
                g2: self.group2() - other.group2(),
                g3: self.group3() - other.group3(),
                g4: self.group4() - other.group4(),
                g5: self.group5() - other.group5(),
                g6: self.group6() - other.group6(),
                g7: self.group7() - other.group7(),
                g8: self.group8() - other.group8(),
                g9: self.group9() - other.group9(),
                g10: self.group10() - other.group10(),
            },
        }
    }
}

impl SubAssign<MultiVector> for MultiVector {
    fn sub_assign(&mut self, other: MultiVector) {
        *self = (*self).sub(other);
    }
}

impl Sub<Origin> for MultiVector {
    type Output = MultiVector;

    fn sub(self, other: Origin) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: self.group0(),
                g1: self.group1(),
                g2: self.group2() - Simd32x2::from([other.group0(), 0.0]),
                g3: self.group3(),
                g4: self.group4(),
                g5: self.group5(),
                g6: self.group6(),
                g7: self.group7(),
                g8: self.group8(),
                g9: self.group9(),
                g10: self.group10(),
            },
        }
    }
}

impl SubAssign<Origin> for MultiVector {
    fn sub_assign(&mut self, other: Origin) {
        *self = (*self).sub(other);
    }
}

impl Sub<Plane> for MultiVector {
    type Output = MultiVector;

    fn sub(self, other: Plane) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: self.group0(),
                g1: self.group1(),
                g2: self.group2(),
                g3: self.group3(),
                g4: self.group4(),
                g5: self.group5(),
                g6: self.group6(),
                g7: self.group7(),
                g8: self.group8(),
                g9: self.group9() - Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
                g10: self.group10() - Simd32x2::from([0.0, other.group0()[3]]),
            },
        }
    }
}

impl SubAssign<Plane> for MultiVector {
    fn sub_assign(&mut self, other: Plane) {
        *self = (*self).sub(other);
    }
}

impl Sub<PlaneAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn sub(self, other: PlaneAtOrigin) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: self.group0(),
                g1: self.group1(),
                g2: self.group2(),
                g3: self.group3(),
                g4: self.group4(),
                g5: self.group5(),
                g6: self.group6(),
                g7: self.group7(),
                g8: self.group8(),
                g9: self.group9() - other.group0(),
                g10: self.group10(),
            },
        }
    }
}

impl SubAssign<PlaneAtOrigin> for MultiVector {
    fn sub_assign(&mut self, other: PlaneAtOrigin) {
        *self = (*self).sub(other);
    }
}

impl Sub<PointAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn sub(self, other: PointAtInfinity) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: self.group0(),
                g1: self.group1(),
                g2: self.group2(),
                g3: self.group3(),
                g4: self.group4(),
                g5: self.group5() - Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
                g6: self.group6(),
                g7: self.group7(),
                g8: self.group8(),
                g9: self.group9(),
                g10: self.group10(),
            },
        }
    }
}

impl SubAssign<PointAtInfinity> for MultiVector {
    fn sub_assign(&mut self, other: PointAtInfinity) {
        *self = (*self).sub(other);
    }
}

impl Sub<PointAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn sub(self, other: PointAtOrigin) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: self.group0(),
                g1: self.group1(),
                g2: self.group2(),
                g3: self.group3(),
                g4: self.group4(),
                g5: self.group5() - Simd32x4::from([0.0, 0.0, 0.0, other.group0()]),
                g6: self.group6(),
                g7: self.group7(),
                g8: self.group8(),
                g9: self.group9(),
                g10: self.group10(),
            },
        }
    }
}

impl SubAssign<PointAtOrigin> for MultiVector {
    fn sub_assign(&mut self, other: PointAtOrigin) {
        *self = (*self).sub(other);
    }
}

impl Sub<Rotor> for MultiVector {
    type Output = MultiVector;

    fn sub(self, other: Rotor) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: self.group0() - Simd32x2::from([0.0, other.group0()[3]]),
                g1: self.group1(),
                g2: self.group2(),
                g3: self.group3(),
                g4: self.group4(),
                g5: self.group5(),
                g6: self.group6(),
                g7: self.group7() - Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
                g8: self.group8(),
                g9: self.group9(),
                g10: self.group10(),
            },
        }
    }
}

impl SubAssign<Rotor> for MultiVector {
    fn sub_assign(&mut self, other: Rotor) {
        *self = (*self).sub(other);
    }
}

impl Sub<RoundPoint> for MultiVector {
    type Output = MultiVector;

    fn sub(self, other: RoundPoint) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: self.group0(),
                g1: self.group1() - other.group0(),
                g2: self.group2() - other.group1(),
                g3: self.group3(),
                g4: self.group4(),
                g5: self.group5(),
                g6: self.group6(),
                g7: self.group7(),
                g8: self.group8(),
                g9: self.group9(),
                g10: self.group10(),
            },
        }
    }
}

impl SubAssign<RoundPoint> for MultiVector {
    fn sub_assign(&mut self, other: RoundPoint) {
        *self = (*self).sub(other);
    }
}

impl Sub<Scalar> for MultiVector {
    type Output = MultiVector;

    fn sub(self, other: Scalar) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: self.group0() - Simd32x2::from([other.group0(), 0.0]),
                g1: self.group1(),
                g2: self.group2(),
                g3: self.group3(),
                g4: self.group4(),
                g5: self.group5(),
                g6: self.group6(),
                g7: self.group7(),
                g8: self.group8(),
                g9: self.group9(),
                g10: self.group10(),
            },
        }
    }
}

impl SubAssign<Scalar> for MultiVector {
    fn sub_assign(&mut self, other: Scalar) {
        *self = (*self).sub(other);
    }
}

impl Sub<Sphere> for MultiVector {
    type Output = MultiVector;

    fn sub(self, other: Sphere) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: self.group0(),
                g1: self.group1(),
                g2: self.group2(),
                g3: self.group3(),
                g4: self.group4(),
                g5: self.group5(),
                g6: self.group6(),
                g7: self.group7(),
                g8: self.group8(),
                g9: self.group9() - other.group0(),
                g10: self.group10() - other.group1(),
            },
        }
    }
}

impl SubAssign<Sphere> for MultiVector {
    fn sub_assign(&mut self, other: Sphere) {
        *self = (*self).sub(other);
    }
}

impl Sub<Translator> for MultiVector {
    type Output = MultiVector;

    fn sub(self, other: Translator) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: self.group0() - Simd32x2::from([0.0, other.group0()[3]]),
                g1: self.group1(),
                g2: self.group2(),
                g3: self.group3(),
                g4: self.group4(),
                g5: self.group5(),
                g6: self.group6(),
                g7: self.group7(),
                g8: self.group8() - Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
                g9: self.group9(),
                g10: self.group10(),
            },
        }
    }
}

impl SubAssign<Translator> for MultiVector {
    fn sub_assign(&mut self, other: Translator) {
        *self = (*self).sub(other);
    }
}

impl Sub<MultiVector> for Origin {
    type Output = MultiVector;

    fn sub(self, other: MultiVector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0) - other.group0(),
                g1: Simd32x3::from(0.0) - other.group1(),
                g2: Simd32x2::from([self.group0(), 0.0]) - other.group2(),
                g3: Simd32x3::from(0.0) - other.group3(),
                g4: Simd32x3::from(0.0) - other.group4(),
                g5: Simd32x4::from(0.0) - other.group5(),
                g6: Simd32x4::from(0.0) - other.group6(),
                g7: Simd32x3::from(0.0) - other.group7(),
                g8: Simd32x3::from(0.0) - other.group8(),
                g9: Simd32x3::from(0.0) - other.group9(),
                g10: Simd32x2::from(0.0) - other.group10(),
            },
        }
    }
}

impl Sub<Origin> for Origin {
    type Output = Origin;

    fn sub(self, other: Origin) -> Origin {
        Origin {
            groups: OriginGroups {
                g0: self.group0() - other.group0(),
            },
        }
    }
}

impl SubAssign<Origin> for Origin {
    fn sub_assign(&mut self, other: Origin) {
        *self = (*self).sub(other);
    }
}

impl Sub<RoundPoint> for Origin {
    type Output = RoundPoint;

    fn sub(self, other: RoundPoint) -> RoundPoint {
        RoundPoint {
            groups: RoundPointGroups {
                g0: Simd32x3::from(0.0) - other.group0(),
                g1: Simd32x2::from([self.group0(), 0.0]) - other.group1(),
            },
        }
    }
}

impl Sub<FlatPoint> for Plane {
    type Output = Flector;

    fn sub(self, other: FlatPoint) -> Flector {
        Flector {
            groups: FlectorGroups {
                g0: Simd32x4::from(0.0) - other.group0(),
                g1: self.group0(),
            },
        }
    }
}

impl Sub<Flector> for Plane {
    type Output = Flector;

    fn sub(self, other: Flector) -> Flector {
        Flector {
            groups: FlectorGroups {
                g0: Simd32x4::from(0.0) - other.group0(),
                g1: self.group0() - other.group1(),
            },
        }
    }
}

impl Sub<Horizon> for Plane {
    type Output = Plane;

    fn sub(self, other: Horizon) -> Plane {
        Plane {
            groups: PlaneGroups {
                g0: self.group0() - Simd32x4::from([0.0, 0.0, 0.0, other.group0()]),
            },
        }
    }
}

impl SubAssign<Horizon> for Plane {
    fn sub_assign(&mut self, other: Horizon) {
        *self = (*self).sub(other);
    }
}

impl Sub<MultiVector> for Plane {
    type Output = MultiVector;

    fn sub(self, other: MultiVector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0) - other.group0(),
                g1: Simd32x3::from(0.0) - other.group1(),
                g2: Simd32x2::from(0.0) - other.group2(),
                g3: Simd32x3::from(0.0) - other.group3(),
                g4: Simd32x3::from(0.0) - other.group4(),
                g5: Simd32x4::from(0.0) - other.group5(),
                g6: Simd32x4::from(0.0) - other.group6(),
                g7: Simd32x3::from(0.0) - other.group7(),
                g8: Simd32x3::from(0.0) - other.group8(),
                g9: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) - other.group9(),
                g10: Simd32x2::from([0.0, self.group0()[3]]) - other.group10(),
            },
        }
    }
}

impl Sub<Plane> for Plane {
    type Output = Plane;

    fn sub(self, other: Plane) -> Plane {
        Plane {
            groups: PlaneGroups {
                g0: self.group0() - other.group0(),
            },
        }
    }
}

impl SubAssign<Plane> for Plane {
    fn sub_assign(&mut self, other: Plane) {
        *self = (*self).sub(other);
    }
}

impl Sub<PlaneAtOrigin> for Plane {
    type Output = Plane;

    fn sub(self, other: PlaneAtOrigin) -> Plane {
        Plane {
            groups: PlaneGroups {
                g0: self.group0() - Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            },
        }
    }
}

impl SubAssign<PlaneAtOrigin> for Plane {
    fn sub_assign(&mut self, other: PlaneAtOrigin) {
        *self = (*self).sub(other);
    }
}

impl Sub<Sphere> for Plane {
    type Output = Sphere;

    fn sub(self, other: Sphere) -> Sphere {
        Sphere {
            groups: SphereGroups {
                g0: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) - other.group0(),
                g1: Simd32x2::from([0.0, self.group0()[3]]) - other.group1(),
            },
        }
    }
}

impl Sub<Flector> for PlaneAtOrigin {
    type Output = Flector;

    fn sub(self, other: Flector) -> Flector {
        Flector {
            groups: FlectorGroups {
                g0: Simd32x4::from(0.0) - other.group0(),
                g1: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]) - other.group1(),
            },
        }
    }
}

impl Sub<Horizon> for PlaneAtOrigin {
    type Output = Plane;

    fn sub(self, other: Horizon) -> Plane {
        Plane {
            groups: PlaneGroups {
                g0: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]) - Simd32x4::from([0.0, 0.0, 0.0, other.group0()]),
            },
        }
    }
}

impl Sub<MultiVector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn sub(self, other: MultiVector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0) - other.group0(),
                g1: Simd32x3::from(0.0) - other.group1(),
                g2: Simd32x2::from(0.0) - other.group2(),
                g3: Simd32x3::from(0.0) - other.group3(),
                g4: Simd32x3::from(0.0) - other.group4(),
                g5: Simd32x4::from(0.0) - other.group5(),
                g6: Simd32x4::from(0.0) - other.group6(),
                g7: Simd32x3::from(0.0) - other.group7(),
                g8: Simd32x3::from(0.0) - other.group8(),
                g9: self.group0() - other.group9(),
                g10: Simd32x2::from(0.0) - other.group10(),
            },
        }
    }
}

impl Sub<Plane> for PlaneAtOrigin {
    type Output = Plane;

    fn sub(self, other: Plane) -> Plane {
        Plane {
            groups: PlaneGroups {
                g0: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]) - other.group0(),
            },
        }
    }
}

impl Sub<PlaneAtOrigin> for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn sub(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        PlaneAtOrigin {
            groups: PlaneAtOriginGroups {
                g0: self.group0() - other.group0(),
            },
        }
    }
}

impl SubAssign<PlaneAtOrigin> for PlaneAtOrigin {
    fn sub_assign(&mut self, other: PlaneAtOrigin) {
        *self = (*self).sub(other);
    }
}

impl Sub<Sphere> for PlaneAtOrigin {
    type Output = Sphere;

    fn sub(self, other: Sphere) -> Sphere {
        Sphere {
            groups: SphereGroups {
                g0: self.group0() - other.group0(),
                g1: Simd32x2::from(0.0) - other.group1(),
            },
        }
    }
}

impl Sub<Dipole> for PointAtInfinity {
    type Output = Dipole;

    fn sub(self, other: Dipole) -> Dipole {
        Dipole {
            groups: DipoleGroups {
                g0: Simd32x3::from(0.0) - other.group0(),
                g1: Simd32x3::from(0.0) - other.group1(),
                g2: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]) - other.group2(),
            },
        }
    }
}

impl Sub<FlatPoint> for PointAtInfinity {
    type Output = FlatPoint;

    fn sub(self, other: FlatPoint) -> FlatPoint {
        FlatPoint {
            groups: FlatPointGroups {
                g0: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]) - other.group0(),
            },
        }
    }
}

impl Sub<Flector> for PointAtInfinity {
    type Output = Flector;

    fn sub(self, other: Flector) -> Flector {
        Flector {
            groups: FlectorGroups {
                g0: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]) - other.group0(),
                g1: Simd32x4::from(0.0) - other.group1(),
            },
        }
    }
}

impl Sub<MultiVector> for PointAtInfinity {
    type Output = MultiVector;

    fn sub(self, other: MultiVector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0) - other.group0(),
                g1: Simd32x3::from(0.0) - other.group1(),
                g2: Simd32x2::from(0.0) - other.group2(),
                g3: Simd32x3::from(0.0) - other.group3(),
                g4: Simd32x3::from(0.0) - other.group4(),
                g5: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]) - other.group5(),
                g6: Simd32x4::from(0.0) - other.group6(),
                g7: Simd32x3::from(0.0) - other.group7(),
                g8: Simd32x3::from(0.0) - other.group8(),
                g9: Simd32x3::from(0.0) - other.group9(),
                g10: Simd32x2::from(0.0) - other.group10(),
            },
        }
    }
}

impl Sub<PointAtInfinity> for PointAtInfinity {
    type Output = PointAtInfinity;

    fn sub(self, other: PointAtInfinity) -> PointAtInfinity {
        PointAtInfinity {
            groups: PointAtInfinityGroups {
                g0: self.group0() - other.group0(),
            },
        }
    }
}

impl SubAssign<PointAtInfinity> for PointAtInfinity {
    fn sub_assign(&mut self, other: PointAtInfinity) {
        *self = (*self).sub(other);
    }
}

impl Sub<PointAtOrigin> for PointAtInfinity {
    type Output = FlatPoint;

    fn sub(self, other: PointAtOrigin) -> FlatPoint {
        FlatPoint {
            groups: FlatPointGroups {
                g0: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]) - Simd32x4::from([0.0, 0.0, 0.0, other.group0()]),
            },
        }
    }
}

impl Sub<Dipole> for PointAtOrigin {
    type Output = Dipole;

    fn sub(self, other: Dipole) -> Dipole {
        Dipole {
            groups: DipoleGroups {
                g0: Simd32x3::from(0.0) - other.group0(),
                g1: Simd32x3::from(0.0) - other.group1(),
                g2: Simd32x4::from([0.0, 0.0, 0.0, self.group0()]) - other.group2(),
            },
        }
    }
}

impl Sub<FlatPoint> for PointAtOrigin {
    type Output = FlatPoint;

    fn sub(self, other: FlatPoint) -> FlatPoint {
        FlatPoint {
            groups: FlatPointGroups {
                g0: Simd32x4::from([0.0, 0.0, 0.0, self.group0()]) - other.group0(),
            },
        }
    }
}

impl Sub<Flector> for PointAtOrigin {
    type Output = Flector;

    fn sub(self, other: Flector) -> Flector {
        Flector {
            groups: FlectorGroups {
                g0: Simd32x4::from([0.0, 0.0, 0.0, self.group0()]) - other.group0(),
                g1: Simd32x4::from(0.0) - other.group1(),
            },
        }
    }
}

impl Sub<MultiVector> for PointAtOrigin {
    type Output = MultiVector;

    fn sub(self, other: MultiVector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0) - other.group0(),
                g1: Simd32x3::from(0.0) - other.group1(),
                g2: Simd32x2::from(0.0) - other.group2(),
                g3: Simd32x3::from(0.0) - other.group3(),
                g4: Simd32x3::from(0.0) - other.group4(),
                g5: Simd32x4::from([0.0, 0.0, 0.0, self.group0()]) - other.group5(),
                g6: Simd32x4::from(0.0) - other.group6(),
                g7: Simd32x3::from(0.0) - other.group7(),
                g8: Simd32x3::from(0.0) - other.group8(),
                g9: Simd32x3::from(0.0) - other.group9(),
                g10: Simd32x2::from(0.0) - other.group10(),
            },
        }
    }
}

impl Sub<PointAtInfinity> for PointAtOrigin {
    type Output = FlatPoint;

    fn sub(self, other: PointAtInfinity) -> FlatPoint {
        FlatPoint {
            groups: FlatPointGroups {
                g0: Simd32x4::from([0.0, 0.0, 0.0, self.group0()]) - Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            },
        }
    }
}

impl Sub<PointAtOrigin> for PointAtOrigin {
    type Output = PointAtOrigin;

    fn sub(self, other: PointAtOrigin) -> PointAtOrigin {
        PointAtOrigin {
            groups: PointAtOriginGroups {
                g0: self.group0() - other.group0(),
            },
        }
    }
}

impl SubAssign<PointAtOrigin> for PointAtOrigin {
    fn sub_assign(&mut self, other: PointAtOrigin) {
        *self = (*self).sub(other);
    }
}

impl Sub<AntiScalar> for Rotor {
    type Output = Rotor;

    fn sub(self, other: AntiScalar) -> Rotor {
        Rotor {
            groups: RotorGroups {
                g0: self.group0() - Simd32x4::from([0.0, 0.0, 0.0, other.group0()]),
            },
        }
    }
}

impl SubAssign<AntiScalar> for Rotor {
    fn sub_assign(&mut self, other: AntiScalar) {
        *self = (*self).sub(other);
    }
}

impl Sub<Line> for Rotor {
    type Output = Motor;

    fn sub(self, other: Line) -> Motor {
        Motor {
            groups: MotorGroups {
                g0: self.group0() - Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
                g1: Simd32x3::from(0.0) - other.group1(),
            },
        }
    }
}

impl Sub<LineAtInfinity> for Rotor {
    type Output = Motor;

    fn sub(self, other: LineAtInfinity) -> Motor {
        Motor {
            groups: MotorGroups {
                g0: self.group0(),
                g1: Simd32x3::from(0.0) - other.group0(),
            },
        }
    }
}

impl Sub<LineAtOrigin> for Rotor {
    type Output = Rotor;

    fn sub(self, other: LineAtOrigin) -> Rotor {
        Rotor {
            groups: RotorGroups {
                g0: self.group0() - Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            },
        }
    }
}

impl SubAssign<LineAtOrigin> for Rotor {
    fn sub_assign(&mut self, other: LineAtOrigin) {
        *self = (*self).sub(other);
    }
}

impl Sub<Motor> for Rotor {
    type Output = Motor;

    fn sub(self, other: Motor) -> Motor {
        Motor {
            groups: MotorGroups {
                g0: self.group0() - other.group0(),
                g1: Simd32x3::from(0.0) - other.group1(),
            },
        }
    }
}

impl Sub<MultiVector> for Rotor {
    type Output = MultiVector;

    fn sub(self, other: MultiVector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from([0.0, self.group0()[3]]) - other.group0(),
                g1: Simd32x3::from(0.0) - other.group1(),
                g2: Simd32x2::from(0.0) - other.group2(),
                g3: Simd32x3::from(0.0) - other.group3(),
                g4: Simd32x3::from(0.0) - other.group4(),
                g5: Simd32x4::from(0.0) - other.group5(),
                g6: Simd32x4::from(0.0) - other.group6(),
                g7: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) - other.group7(),
                g8: Simd32x3::from(0.0) - other.group8(),
                g9: Simd32x3::from(0.0) - other.group9(),
                g10: Simd32x2::from(0.0) - other.group10(),
            },
        }
    }
}

impl Sub<Rotor> for Rotor {
    type Output = Rotor;

    fn sub(self, other: Rotor) -> Rotor {
        Rotor {
            groups: RotorGroups {
                g0: self.group0() - other.group0(),
            },
        }
    }
}

impl SubAssign<Rotor> for Rotor {
    fn sub_assign(&mut self, other: Rotor) {
        *self = (*self).sub(other);
    }
}

impl Sub<Translator> for Rotor {
    type Output = Motor;

    fn sub(self, other: Translator) -> Motor {
        Motor {
            groups: MotorGroups {
                g0: self.group0() - Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]),
                g1: Simd32x3::from(0.0) - Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            },
        }
    }
}

impl Sub<Infinity> for RoundPoint {
    type Output = RoundPoint;

    fn sub(self, other: Infinity) -> RoundPoint {
        RoundPoint {
            groups: RoundPointGroups {
                g0: self.group0(),
                g1: self.group1() - Simd32x2::from([0.0, other.group0()]),
            },
        }
    }
}

impl SubAssign<Infinity> for RoundPoint {
    fn sub_assign(&mut self, other: Infinity) {
        *self = (*self).sub(other);
    }
}

impl Sub<MultiVector> for RoundPoint {
    type Output = MultiVector;

    fn sub(self, other: MultiVector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0) - other.group0(),
                g1: self.group0() - other.group1(),
                g2: self.group1() - other.group2(),
                g3: Simd32x3::from(0.0) - other.group3(),
                g4: Simd32x3::from(0.0) - other.group4(),
                g5: Simd32x4::from(0.0) - other.group5(),
                g6: Simd32x4::from(0.0) - other.group6(),
                g7: Simd32x3::from(0.0) - other.group7(),
                g8: Simd32x3::from(0.0) - other.group8(),
                g9: Simd32x3::from(0.0) - other.group9(),
                g10: Simd32x2::from(0.0) - other.group10(),
            },
        }
    }
}

impl Sub<Origin> for RoundPoint {
    type Output = RoundPoint;

    fn sub(self, other: Origin) -> RoundPoint {
        RoundPoint {
            groups: RoundPointGroups {
                g0: self.group0(),
                g1: self.group1() - Simd32x2::from([other.group0(), 0.0]),
            },
        }
    }
}

impl SubAssign<Origin> for RoundPoint {
    fn sub_assign(&mut self, other: Origin) {
        *self = (*self).sub(other);
    }
}

impl Sub<RoundPoint> for RoundPoint {
    type Output = RoundPoint;

    fn sub(self, other: RoundPoint) -> RoundPoint {
        RoundPoint {
            groups: RoundPointGroups {
                g0: self.group0() - other.group0(),
                g1: self.group1() - other.group1(),
            },
        }
    }
}

impl SubAssign<RoundPoint> for RoundPoint {
    fn sub_assign(&mut self, other: RoundPoint) {
        *self = (*self).sub(other);
    }
}

impl Sub<AntiScalar> for Scalar {
    type Output = Magnitude;

    fn sub(self, other: AntiScalar) -> Magnitude {
        Magnitude {
            groups: MagnitudeGroups {
                g0: Simd32x2::from([self.group0(), 0.0]) - Simd32x2::from([0.0, other.group0()]),
            },
        }
    }
}

impl Sub<Magnitude> for Scalar {
    type Output = Magnitude;

    fn sub(self, other: Magnitude) -> Magnitude {
        Magnitude {
            groups: MagnitudeGroups {
                g0: Simd32x2::from([self.group0(), 0.0]) - other.group0(),
            },
        }
    }
}

impl Sub<MultiVector> for Scalar {
    type Output = MultiVector;

    fn sub(self, other: MultiVector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from([self.group0(), 0.0]) - other.group0(),
                g1: Simd32x3::from(0.0) - other.group1(),
                g2: Simd32x2::from(0.0) - other.group2(),
                g3: Simd32x3::from(0.0) - other.group3(),
                g4: Simd32x3::from(0.0) - other.group4(),
                g5: Simd32x4::from(0.0) - other.group5(),
                g6: Simd32x4::from(0.0) - other.group6(),
                g7: Simd32x3::from(0.0) - other.group7(),
                g8: Simd32x3::from(0.0) - other.group8(),
                g9: Simd32x3::from(0.0) - other.group9(),
                g10: Simd32x2::from(0.0) - other.group10(),
            },
        }
    }
}

impl Sub<Scalar> for Scalar {
    type Output = Scalar;

    fn sub(self, other: Scalar) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: self.group0() - other.group0(),
            },
        }
    }
}

impl SubAssign<Scalar> for Scalar {
    fn sub_assign(&mut self, other: Scalar) {
        *self = (*self).sub(other);
    }
}

impl Sub<Horizon> for Sphere {
    type Output = Sphere;

    fn sub(self, other: Horizon) -> Sphere {
        Sphere {
            groups: SphereGroups {
                g0: self.group0(),
                g1: self.group1() - Simd32x2::from([0.0, other.group0()]),
            },
        }
    }
}

impl SubAssign<Horizon> for Sphere {
    fn sub_assign(&mut self, other: Horizon) {
        *self = (*self).sub(other);
    }
}

impl Sub<MultiVector> for Sphere {
    type Output = MultiVector;

    fn sub(self, other: MultiVector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0) - other.group0(),
                g1: Simd32x3::from(0.0) - other.group1(),
                g2: Simd32x2::from(0.0) - other.group2(),
                g3: Simd32x3::from(0.0) - other.group3(),
                g4: Simd32x3::from(0.0) - other.group4(),
                g5: Simd32x4::from(0.0) - other.group5(),
                g6: Simd32x4::from(0.0) - other.group6(),
                g7: Simd32x3::from(0.0) - other.group7(),
                g8: Simd32x3::from(0.0) - other.group8(),
                g9: self.group0() - other.group9(),
                g10: self.group1() - other.group10(),
            },
        }
    }
}

impl Sub<Plane> for Sphere {
    type Output = Sphere;

    fn sub(self, other: Plane) -> Sphere {
        Sphere {
            groups: SphereGroups {
                g0: self.group0() - Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
                g1: self.group1() - Simd32x2::from([0.0, other.group0()[3]]),
            },
        }
    }
}

impl SubAssign<Plane> for Sphere {
    fn sub_assign(&mut self, other: Plane) {
        *self = (*self).sub(other);
    }
}

impl Sub<PlaneAtOrigin> for Sphere {
    type Output = Sphere;

    fn sub(self, other: PlaneAtOrigin) -> Sphere {
        Sphere {
            groups: SphereGroups {
                g0: self.group0() - other.group0(),
                g1: self.group1(),
            },
        }
    }
}

impl SubAssign<PlaneAtOrigin> for Sphere {
    fn sub_assign(&mut self, other: PlaneAtOrigin) {
        *self = (*self).sub(other);
    }
}

impl Sub<Sphere> for Sphere {
    type Output = Sphere;

    fn sub(self, other: Sphere) -> Sphere {
        Sphere {
            groups: SphereGroups {
                g0: self.group0() - other.group0(),
                g1: self.group1() - other.group1(),
            },
        }
    }
}

impl SubAssign<Sphere> for Sphere {
    fn sub_assign(&mut self, other: Sphere) {
        *self = (*self).sub(other);
    }
}

impl Sub<AntiScalar> for Translator {
    type Output = Translator;

    fn sub(self, other: AntiScalar) -> Translator {
        Translator {
            groups: TranslatorGroups {
                g0: self.group0() - Simd32x4::from([0.0, 0.0, 0.0, other.group0()]),
            },
        }
    }
}

impl SubAssign<AntiScalar> for Translator {
    fn sub_assign(&mut self, other: AntiScalar) {
        *self = (*self).sub(other);
    }
}

impl Sub<Line> for Translator {
    type Output = Motor;

    fn sub(self, other: Line) -> Motor {
        Motor {
            groups: MotorGroups {
                g0: Simd32x4::from([0.0, 0.0, 0.0, self.group0()[3]]) - Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
                g1: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) - other.group1(),
            },
        }
    }
}

impl Sub<LineAtInfinity> for Translator {
    type Output = Translator;

    fn sub(self, other: LineAtInfinity) -> Translator {
        Translator {
            groups: TranslatorGroups {
                g0: self.group0() - Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            },
        }
    }
}

impl SubAssign<LineAtInfinity> for Translator {
    fn sub_assign(&mut self, other: LineAtInfinity) {
        *self = (*self).sub(other);
    }
}

impl Sub<LineAtOrigin> for Translator {
    type Output = Motor;

    fn sub(self, other: LineAtOrigin) -> Motor {
        Motor {
            groups: MotorGroups {
                g0: Simd32x4::from([0.0, 0.0, 0.0, self.group0()[3]]) - Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
                g1: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
            },
        }
    }
}

impl Sub<Motor> for Translator {
    type Output = Motor;

    fn sub(self, other: Motor) -> Motor {
        Motor {
            groups: MotorGroups {
                g0: Simd32x4::from([0.0, 0.0, 0.0, self.group0()[3]]) - other.group0(),
                g1: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) - other.group1(),
            },
        }
    }
}

impl Sub<MultiVector> for Translator {
    type Output = MultiVector;

    fn sub(self, other: MultiVector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from([0.0, self.group0()[3]]) - other.group0(),
                g1: Simd32x3::from(0.0) - other.group1(),
                g2: Simd32x2::from(0.0) - other.group2(),
                g3: Simd32x3::from(0.0) - other.group3(),
                g4: Simd32x3::from(0.0) - other.group4(),
                g5: Simd32x4::from(0.0) - other.group5(),
                g6: Simd32x4::from(0.0) - other.group6(),
                g7: Simd32x3::from(0.0) - other.group7(),
                g8: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) - other.group8(),
                g9: Simd32x3::from(0.0) - other.group9(),
                g10: Simd32x2::from(0.0) - other.group10(),
            },
        }
    }
}

impl Sub<Rotor> for Translator {
    type Output = Motor;

    fn sub(self, other: Rotor) -> Motor {
        Motor {
            groups: MotorGroups {
                g0: Simd32x4::from([0.0, 0.0, 0.0, self.group0()[3]]) - other.group0(),
                g1: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
            },
        }
    }
}

impl Sub<Translator> for Translator {
    type Output = Translator;

    fn sub(self, other: Translator) -> Translator {
        Translator {
            groups: TranslatorGroups {
                g0: self.group0() - other.group0(),
            },
        }
    }
}

impl SubAssign<Translator> for Translator {
    fn sub_assign(&mut self, other: Translator) {
        *self = (*self).sub(other);
    }
}
