//
// AUTO-GENERATED - DO NOT MODIFY
//
// To contribute to this file, see the adjacent codegen package.
// https://github.com/AndrewBrownK/projective_ga/
//

use projective_ga::{simd::*, *};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

pub mod aspect_duals;
pub mod aspects;
pub mod characteristics;
pub mod involutions;
pub mod metrics;
pub mod norms;
pub mod shaders;
#[cfg(test)]
pub mod test;
pub mod unitize;
pub mod products {
    pub mod contractions;
    pub mod dot;
    pub mod expansions;
    pub mod exterior;
    pub mod geometric;
    pub mod isometries;
    pub mod projections;
    pub mod quotients;
    pub mod rejections;
    pub mod supports;
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

type Magnitude = DualNum;

#[derive(Clone, Copy)]
struct DualNumGroups {
    /// 1, e12345
    g0: Simd32x2,
}

#[derive(Clone, Copy)]
pub union DualNum {
    groups: DualNumGroups,
    /// 1, e12345, 0, 0
    elements: [f32; 4],
}

impl DualNum {
    #[allow(clippy::too_many_arguments)]
    pub const fn new(scalar: f32, e12345: f32) -> Self {
        Self {
            elements: [scalar, e12345, 0.0, 0.0],
        }
    }
    pub const fn from_groups(g0: Simd32x2) -> Self {
        Self { groups: DualNumGroups { g0 } }
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

const DUALNUM_INDEX_REMAP: [usize; 2] = [0, 1];

impl std::ops::Index<usize> for DualNum {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[DUALNUM_INDEX_REMAP[index]] }
    }
}

impl std::ops::IndexMut<usize> for DualNum {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[DUALNUM_INDEX_REMAP[index]] }
    }
}

impl std::convert::From<DualNum> for [f32; 2] {
    fn from(vector: DualNum) -> Self {
        unsafe { [vector.elements[0], vector.elements[1]] }
    }
}

impl std::convert::From<[f32; 2]> for DualNum {
    fn from(array: [f32; 2]) -> Self {
        Self {
            elements: [array[0], array[1], 0.0, 0.0],
        }
    }
}

impl std::fmt::Debug for DualNum {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.debug_struct("DualNum").field("1", &self[0]).field("e12345", &self[1]).finish()
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

impl One for DualNum {
    fn one() -> Self {
        DualNum {
            groups: DualNumGroups { g0: Simd32x2::from([1.0, 0.0]) },
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

impl One for Plane {
    fn one() -> Self {
        Plane {
            groups: PlaneGroups { g0: Simd32x4::from(0.0) },
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

impl Unit for AntiScalar {
    fn unit() -> Self {
        AntiScalar {
            groups: AntiScalarGroups { g0: 1.0 },
        }
    }
}

impl Unit for Circle {
    fn unit() -> Self {
        Circle {
            groups: CircleGroups {
                g0: Simd32x4::from(1.0),
                g1: Simd32x3::from(1.0),
                g2: Simd32x3::from(1.0),
            },
        }
    }
}

impl Unit for Dipole {
    fn unit() -> Self {
        Dipole {
            groups: DipoleGroups {
                g0: Simd32x3::from(1.0),
                g1: Simd32x3::from(1.0),
                g2: Simd32x4::from(1.0),
            },
        }
    }
}

impl Unit for DualNum {
    fn unit() -> Self {
        DualNum {
            groups: DualNumGroups { g0: Simd32x2::from(1.0) },
        }
    }
}

impl Unit for FlatPoint {
    fn unit() -> Self {
        FlatPoint {
            groups: FlatPointGroups { g0: Simd32x4::from(1.0) },
        }
    }
}

impl Unit for Flector {
    fn unit() -> Self {
        Flector {
            groups: FlectorGroups {
                g0: Simd32x4::from(1.0),
                g1: Simd32x4::from(1.0),
            },
        }
    }
}

impl Unit for Line {
    fn unit() -> Self {
        Line {
            groups: LineGroups {
                g0: Simd32x3::from(1.0),
                g1: Simd32x3::from(1.0),
            },
        }
    }
}

impl Unit for Motor {
    fn unit() -> Self {
        Motor {
            groups: MotorGroups {
                g0: Simd32x4::from(1.0),
                g1: Simd32x3::from(1.0),
            },
        }
    }
}

impl Unit for MultiVector {
    fn unit() -> Self {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(1.0),
                g1: Simd32x3::from(1.0),
                g2: Simd32x2::from(1.0),
                g3: Simd32x3::from(1.0),
                g4: Simd32x3::from(1.0),
                g5: Simd32x4::from(1.0),
                g6: Simd32x4::from(1.0),
                g7: Simd32x3::from(1.0),
                g8: Simd32x3::from(1.0),
                g9: Simd32x3::from(1.0),
                g10: Simd32x2::from(1.0),
            },
        }
    }
}

impl Unit for Plane {
    fn unit() -> Self {
        Plane {
            groups: PlaneGroups { g0: Simd32x4::from(1.0) },
        }
    }
}

impl Unit for RoundPoint {
    fn unit() -> Self {
        RoundPoint {
            groups: RoundPointGroups {
                g0: Simd32x3::from(1.0),
                g1: Simd32x2::from(1.0),
            },
        }
    }
}

impl Unit for Scalar {
    fn unit() -> Self {
        Scalar { groups: ScalarGroups { g0: 1.0 } }
    }
}

impl Unit for Sphere {
    fn unit() -> Self {
        Sphere {
            groups: SphereGroups {
                g0: Simd32x3::from(1.0),
                g1: Simd32x2::from(1.0),
            },
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

impl Zero for DualNum {
    fn zero() -> Self {
        DualNum {
            groups: DualNumGroups { g0: Simd32x2::from(0.0) },
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

impl Zero for Plane {
    fn zero() -> Self {
        Plane {
            groups: PlaneGroups { g0: Simd32x4::from(0.0) },
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

impl Neg for AntiScalar {
    type Output = AntiScalar;

    fn neg(self) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups { g0: -self.group0() },
        }
    }
}

impl Neg for Circle {
    type Output = Circle;

    fn neg(self) -> Circle {
        Circle {
            groups: CircleGroups {
                g0: self.group0() * Simd32x4::from(-1.0),
                g1: self.group1() * Simd32x3::from(-1.0),
                g2: self.group2() * Simd32x3::from(-1.0),
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

impl Neg for DualNum {
    type Output = DualNum;

    fn neg(self) -> DualNum {
        DualNum {
            groups: DualNumGroups {
                g0: self.group0() * Simd32x2::from(-1.0),
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
                g1: self.group1() * Simd32x4::from(-1.0),
            },
        }
    }
}

impl Neg for Line {
    type Output = Line;

    fn neg(self) -> Line {
        Line {
            groups: LineGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
                g1: self.group1() * Simd32x3::from(-1.0),
            },
        }
    }
}

impl Neg for Motor {
    type Output = Motor;

    fn neg(self) -> Motor {
        Motor {
            groups: MotorGroups {
                g0: self.group0() * Simd32x4::from(-1.0),
                g1: self.group1() * Simd32x3::from(-1.0),
            },
        }
    }
}

impl Neg for MultiVector {
    type Output = MultiVector;

    fn neg(self) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: self.group0() * Simd32x2::from(-1.0),
                g1: self.group1() * Simd32x3::from(-1.0),
                g2: self.group2() * Simd32x2::from(-1.0),
                g3: self.group3() * Simd32x3::from(-1.0),
                g4: self.group4() * Simd32x3::from(-1.0),
                g5: self.group5() * Simd32x4::from(-1.0),
                g6: self.group6() * Simd32x4::from(-1.0),
                g7: self.group7() * Simd32x3::from(-1.0),
                g8: self.group8() * Simd32x3::from(-1.0),
                g9: self.group9() * Simd32x3::from(-1.0),
                g10: self.group10() * Simd32x2::from(-1.0),
            },
        }
    }
}

impl Neg for Plane {
    type Output = Plane;

    fn neg(self) -> Plane {
        Plane {
            groups: PlaneGroups {
                g0: self.group0() * Simd32x4::from(-1.0),
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
                g0: self.group0() * Simd32x3::from(-1.0),
                g1: self.group1() * Simd32x2::from(-1.0),
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

impl Add<Circle> for AntiScalar {
    type Output = MultiVector;

    fn add(self, other: Circle) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from([0.0, self.group0()]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: other.group0(),
                g7: other.group1(),
                g8: other.group2(),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Add<Dipole> for AntiScalar {
    type Output = MultiVector;

    fn add(self, other: Dipole) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from([0.0, self.group0()]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: other.group0(),
                g4: other.group1(),
                g5: other.group2(),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Add<DualNum> for AntiScalar {
    type Output = DualNum;

    fn add(self, other: DualNum) -> DualNum {
        DualNum {
            groups: DualNumGroups {
                g0: Simd32x2::from([0.0, self.group0()]) + other.group0(),
            },
        }
    }
}

impl Add<FlatPoint> for AntiScalar {
    type Output = MultiVector;

    fn add(self, other: FlatPoint) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from([0.0, self.group0()]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: other.group0(),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Add<Flector> for AntiScalar {
    type Output = MultiVector;

    fn add(self, other: Flector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from([0.0, self.group0()]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: other.group0(),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
                g10: Simd32x2::from([0.0, other.group1()[3]]),
            },
        }
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

impl Add<Plane> for AntiScalar {
    type Output = MultiVector;

    fn add(self, other: Plane) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from([0.0, self.group0()]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
                g10: Simd32x2::from([0.0, other.group0()[3]]),
            },
        }
    }
}

impl Add<RoundPoint> for AntiScalar {
    type Output = MultiVector;

    fn add(self, other: RoundPoint) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from([0.0, self.group0()]),
                g1: other.group0(),
                g2: other.group1(),
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

impl Add<Scalar> for AntiScalar {
    type Output = DualNum;

    fn add(self, other: Scalar) -> DualNum {
        DualNum {
            groups: DualNumGroups {
                g0: Simd32x2::from([0.0, self.group0()]) + Simd32x2::from([other.group0(), 0.0]),
            },
        }
    }
}

impl Add<Sphere> for AntiScalar {
    type Output = MultiVector;

    fn add(self, other: Sphere) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from([0.0, self.group0()]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: other.group0(),
                g10: other.group1(),
            },
        }
    }
}

impl Add<AntiScalar> for Circle {
    type Output = MultiVector;

    fn add(self, other: AntiScalar) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from([0.0, other.group0()]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: self.group0(),
                g7: self.group1(),
                g8: self.group2(),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
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

impl Add<Dipole> for Circle {
    type Output = MultiVector;

    fn add(self, other: Dipole) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: other.group0(),
                g4: other.group1(),
                g5: other.group2(),
                g6: self.group0(),
                g7: self.group1(),
                g8: self.group2(),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Add<DualNum> for Circle {
    type Output = MultiVector;

    fn add(self, other: DualNum) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: other.group0(),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: self.group0(),
                g7: self.group1(),
                g8: self.group2(),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Add<FlatPoint> for Circle {
    type Output = MultiVector;

    fn add(self, other: FlatPoint) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: other.group0(),
                g6: self.group0(),
                g7: self.group1(),
                g8: self.group2(),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Add<Flector> for Circle {
    type Output = MultiVector;

    fn add(self, other: Flector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: other.group0(),
                g6: self.group0(),
                g7: self.group1(),
                g8: self.group2(),
                g9: Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
                g10: Simd32x2::from([0.0, other.group1()[3]]),
            },
        }
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

impl Add<Motor> for Circle {
    type Output = MultiVector;

    fn add(self, other: Motor) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from([0.0, other.group0()[3]]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: self.group0(),
                g7: self.group1() + Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
                g8: self.group2() + other.group1(),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
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

impl Add<Plane> for Circle {
    type Output = MultiVector;

    fn add(self, other: Plane) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: self.group0(),
                g7: self.group1(),
                g8: self.group2(),
                g9: Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
                g10: Simd32x2::from([0.0, other.group0()[3]]),
            },
        }
    }
}

impl Add<RoundPoint> for Circle {
    type Output = MultiVector;

    fn add(self, other: RoundPoint) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: other.group0(),
                g2: other.group1(),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: self.group0(),
                g7: self.group1(),
                g8: self.group2(),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Add<Scalar> for Circle {
    type Output = MultiVector;

    fn add(self, other: Scalar) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from([other.group0(), 0.0]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: self.group0(),
                g7: self.group1(),
                g8: self.group2(),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Add<Sphere> for Circle {
    type Output = MultiVector;

    fn add(self, other: Sphere) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: self.group0(),
                g7: self.group1(),
                g8: self.group2(),
                g9: other.group0(),
                g10: other.group1(),
            },
        }
    }
}

impl Add<AntiScalar> for Dipole {
    type Output = MultiVector;

    fn add(self, other: AntiScalar) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from([0.0, other.group0()]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: self.group0(),
                g4: self.group1(),
                g5: self.group2(),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Add<Circle> for Dipole {
    type Output = MultiVector;

    fn add(self, other: Circle) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: self.group0(),
                g4: self.group1(),
                g5: self.group2(),
                g6: other.group0(),
                g7: other.group1(),
                g8: other.group2(),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
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

impl Add<DualNum> for Dipole {
    type Output = MultiVector;

    fn add(self, other: DualNum) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: other.group0(),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: self.group0(),
                g4: self.group1(),
                g5: self.group2(),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
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

impl Add<Flector> for Dipole {
    type Output = MultiVector;

    fn add(self, other: Flector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: self.group0(),
                g4: self.group1(),
                g5: self.group2() + other.group0(),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
                g10: Simd32x2::from([0.0, other.group1()[3]]),
            },
        }
    }
}

impl Add<Line> for Dipole {
    type Output = MultiVector;

    fn add(self, other: Line) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: self.group0(),
                g4: self.group1(),
                g5: self.group2(),
                g6: Simd32x4::from(0.0),
                g7: other.group0(),
                g8: other.group1(),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Add<Motor> for Dipole {
    type Output = MultiVector;

    fn add(self, other: Motor) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from([0.0, other.group0()[3]]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: self.group0(),
                g4: self.group1(),
                g5: self.group2(),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
                g8: other.group1(),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
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

impl Add<Plane> for Dipole {
    type Output = MultiVector;

    fn add(self, other: Plane) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: self.group0(),
                g4: self.group1(),
                g5: self.group2(),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
                g10: Simd32x2::from([0.0, other.group0()[3]]),
            },
        }
    }
}

impl Add<RoundPoint> for Dipole {
    type Output = MultiVector;

    fn add(self, other: RoundPoint) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: other.group0(),
                g2: other.group1(),
                g3: self.group0(),
                g4: self.group1(),
                g5: self.group2(),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Add<Scalar> for Dipole {
    type Output = MultiVector;

    fn add(self, other: Scalar) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from([other.group0(), 0.0]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: self.group0(),
                g4: self.group1(),
                g5: self.group2(),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Add<Sphere> for Dipole {
    type Output = MultiVector;

    fn add(self, other: Sphere) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: self.group0(),
                g4: self.group1(),
                g5: self.group2(),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: other.group0(),
                g10: other.group1(),
            },
        }
    }
}

impl Add<AntiScalar> for DualNum {
    type Output = DualNum;

    fn add(self, other: AntiScalar) -> DualNum {
        DualNum {
            groups: DualNumGroups {
                g0: self.group0() + Simd32x2::from([0.0, other.group0()]),
            },
        }
    }
}

impl AddAssign<AntiScalar> for DualNum {
    fn add_assign(&mut self, other: AntiScalar) {
        *self = (*self).add(other);
    }
}

impl Add<Circle> for DualNum {
    type Output = MultiVector;

    fn add(self, other: Circle) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: self.group0(),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: other.group0(),
                g7: other.group1(),
                g8: other.group2(),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Add<Dipole> for DualNum {
    type Output = MultiVector;

    fn add(self, other: Dipole) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: self.group0(),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: other.group0(),
                g4: other.group1(),
                g5: other.group2(),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Add<DualNum> for DualNum {
    type Output = DualNum;

    fn add(self, other: DualNum) -> DualNum {
        DualNum {
            groups: DualNumGroups {
                g0: self.group0() + other.group0(),
            },
        }
    }
}

impl AddAssign<DualNum> for DualNum {
    fn add_assign(&mut self, other: DualNum) {
        *self = (*self).add(other);
    }
}

impl Add<FlatPoint> for DualNum {
    type Output = MultiVector;

    fn add(self, other: FlatPoint) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: self.group0(),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: other.group0(),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Add<Flector> for DualNum {
    type Output = MultiVector;

    fn add(self, other: Flector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: self.group0(),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: other.group0(),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
                g10: Simd32x2::from([0.0, other.group1()[3]]),
            },
        }
    }
}

impl Add<Line> for DualNum {
    type Output = MultiVector;

    fn add(self, other: Line) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: self.group0(),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: other.group0(),
                g8: other.group1(),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Add<Motor> for DualNum {
    type Output = MultiVector;

    fn add(self, other: Motor) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: self.group0() + Simd32x2::from([0.0, other.group0()[3]]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
                g8: other.group1(),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Add<MultiVector> for DualNum {
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

impl Add<Plane> for DualNum {
    type Output = MultiVector;

    fn add(self, other: Plane) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: self.group0(),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
                g10: Simd32x2::from([0.0, other.group0()[3]]),
            },
        }
    }
}

impl Add<RoundPoint> for DualNum {
    type Output = MultiVector;

    fn add(self, other: RoundPoint) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: self.group0(),
                g1: other.group0(),
                g2: other.group1(),
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

impl Add<Scalar> for DualNum {
    type Output = DualNum;

    fn add(self, other: Scalar) -> DualNum {
        DualNum {
            groups: DualNumGroups {
                g0: self.group0() + Simd32x2::from([other.group0(), 0.0]),
            },
        }
    }
}

impl AddAssign<Scalar> for DualNum {
    fn add_assign(&mut self, other: Scalar) {
        *self = (*self).add(other);
    }
}

impl Add<Sphere> for DualNum {
    type Output = MultiVector;

    fn add(self, other: Sphere) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: self.group0(),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: other.group0(),
                g10: other.group1(),
            },
        }
    }
}

impl Add<AntiScalar> for FlatPoint {
    type Output = MultiVector;

    fn add(self, other: AntiScalar) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from([0.0, other.group0()]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: self.group0(),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Add<Circle> for FlatPoint {
    type Output = MultiVector;

    fn add(self, other: Circle) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: self.group0(),
                g6: other.group0(),
                g7: other.group1(),
                g8: other.group2(),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
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

impl Add<DualNum> for FlatPoint {
    type Output = MultiVector;

    fn add(self, other: DualNum) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: other.group0(),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: self.group0(),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
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

impl Add<Line> for FlatPoint {
    type Output = MultiVector;

    fn add(self, other: Line) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: self.group0(),
                g6: Simd32x4::from(0.0),
                g7: other.group0(),
                g8: other.group1(),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Add<Motor> for FlatPoint {
    type Output = MultiVector;

    fn add(self, other: Motor) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from([0.0, other.group0()[3]]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: self.group0(),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
                g8: other.group1(),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
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

impl Add<RoundPoint> for FlatPoint {
    type Output = MultiVector;

    fn add(self, other: RoundPoint) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: other.group0(),
                g2: other.group1(),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: self.group0(),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Add<Scalar> for FlatPoint {
    type Output = MultiVector;

    fn add(self, other: Scalar) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from([other.group0(), 0.0]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: self.group0(),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Add<Sphere> for FlatPoint {
    type Output = MultiVector;

    fn add(self, other: Sphere) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: self.group0(),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: other.group0(),
                g10: other.group1(),
            },
        }
    }
}

impl Add<AntiScalar> for Flector {
    type Output = MultiVector;

    fn add(self, other: AntiScalar) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from([0.0, other.group0()]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: self.group0(),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
                g10: Simd32x2::from([0.0, self.group1()[3]]),
            },
        }
    }
}

impl Add<Circle> for Flector {
    type Output = MultiVector;

    fn add(self, other: Circle) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: self.group0(),
                g6: other.group0(),
                g7: other.group1(),
                g8: other.group2(),
                g9: Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
                g10: Simd32x2::from([0.0, self.group1()[3]]),
            },
        }
    }
}

impl Add<Dipole> for Flector {
    type Output = MultiVector;

    fn add(self, other: Dipole) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: other.group0(),
                g4: other.group1(),
                g5: self.group0() + other.group2(),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
                g10: Simd32x2::from([0.0, self.group1()[3]]),
            },
        }
    }
}

impl Add<DualNum> for Flector {
    type Output = MultiVector;

    fn add(self, other: DualNum) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: other.group0(),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: self.group0(),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
                g10: Simd32x2::from([0.0, self.group1()[3]]),
            },
        }
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

impl Add<Line> for Flector {
    type Output = MultiVector;

    fn add(self, other: Line) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: self.group0(),
                g6: Simd32x4::from(0.0),
                g7: other.group0(),
                g8: other.group1(),
                g9: Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
                g10: Simd32x2::from([0.0, self.group1()[3]]),
            },
        }
    }
}

impl Add<Motor> for Flector {
    type Output = MultiVector;

    fn add(self, other: Motor) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from([0.0, other.group0()[3]]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: self.group0(),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
                g8: other.group1(),
                g9: Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
                g10: Simd32x2::from([0.0, self.group1()[3]]),
            },
        }
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

impl Add<RoundPoint> for Flector {
    type Output = MultiVector;

    fn add(self, other: RoundPoint) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: other.group0(),
                g2: other.group1(),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: self.group0(),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
                g10: Simd32x2::from([0.0, self.group1()[3]]),
            },
        }
    }
}

impl Add<Scalar> for Flector {
    type Output = MultiVector;

    fn add(self, other: Scalar) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from([other.group0(), 0.0]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: self.group0(),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
                g10: Simd32x2::from([0.0, self.group1()[3]]),
            },
        }
    }
}

impl Add<Sphere> for Flector {
    type Output = MultiVector;

    fn add(self, other: Sphere) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: self.group0(),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]) + other.group0(),
                g10: Simd32x2::from([0.0, self.group1()[3]]) + other.group1(),
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

impl Add<Dipole> for Line {
    type Output = MultiVector;

    fn add(self, other: Dipole) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: other.group0(),
                g4: other.group1(),
                g5: other.group2(),
                g6: Simd32x4::from(0.0),
                g7: self.group0(),
                g8: self.group1(),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Add<DualNum> for Line {
    type Output = MultiVector;

    fn add(self, other: DualNum) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: other.group0(),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: self.group0(),
                g8: self.group1(),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Add<FlatPoint> for Line {
    type Output = MultiVector;

    fn add(self, other: FlatPoint) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: other.group0(),
                g6: Simd32x4::from(0.0),
                g7: self.group0(),
                g8: self.group1(),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Add<Flector> for Line {
    type Output = MultiVector;

    fn add(self, other: Flector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: other.group0(),
                g6: Simd32x4::from(0.0),
                g7: self.group0(),
                g8: self.group1(),
                g9: Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
                g10: Simd32x2::from([0.0, other.group1()[3]]),
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

impl Add<Plane> for Line {
    type Output = MultiVector;

    fn add(self, other: Plane) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: self.group0(),
                g8: self.group1(),
                g9: Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
                g10: Simd32x2::from([0.0, other.group0()[3]]),
            },
        }
    }
}

impl Add<RoundPoint> for Line {
    type Output = MultiVector;

    fn add(self, other: RoundPoint) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: other.group0(),
                g2: other.group1(),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: self.group0(),
                g8: self.group1(),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Add<Scalar> for Line {
    type Output = MultiVector;

    fn add(self, other: Scalar) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from([other.group0(), 0.0]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: self.group0(),
                g8: self.group1(),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Add<Sphere> for Line {
    type Output = MultiVector;

    fn add(self, other: Sphere) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: self.group0(),
                g8: self.group1(),
                g9: other.group0(),
                g10: other.group1(),
            },
        }
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

impl Add<Circle> for Motor {
    type Output = MultiVector;

    fn add(self, other: Circle) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from([0.0, self.group0()[3]]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: other.group0(),
                g7: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) + other.group1(),
                g8: self.group1() + other.group2(),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Add<Dipole> for Motor {
    type Output = MultiVector;

    fn add(self, other: Dipole) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from([0.0, self.group0()[3]]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: other.group0(),
                g4: other.group1(),
                g5: other.group2(),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
                g8: self.group1(),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Add<DualNum> for Motor {
    type Output = MultiVector;

    fn add(self, other: DualNum) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from([0.0, self.group0()[3]]) + other.group0(),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
                g8: self.group1(),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Add<FlatPoint> for Motor {
    type Output = MultiVector;

    fn add(self, other: FlatPoint) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from([0.0, self.group0()[3]]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: other.group0(),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
                g8: self.group1(),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Add<Flector> for Motor {
    type Output = MultiVector;

    fn add(self, other: Flector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from([0.0, self.group0()[3]]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: other.group0(),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
                g8: self.group1(),
                g9: Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
                g10: Simd32x2::from([0.0, other.group1()[3]]),
            },
        }
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

impl Add<Plane> for Motor {
    type Output = MultiVector;

    fn add(self, other: Plane) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from([0.0, self.group0()[3]]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
                g8: self.group1(),
                g9: Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
                g10: Simd32x2::from([0.0, other.group0()[3]]),
            },
        }
    }
}

impl Add<RoundPoint> for Motor {
    type Output = MultiVector;

    fn add(self, other: RoundPoint) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from([0.0, self.group0()[3]]),
                g1: other.group0(),
                g2: other.group1(),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
                g8: self.group1(),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Add<Scalar> for Motor {
    type Output = MultiVector;

    fn add(self, other: Scalar) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from([0.0, self.group0()[3]]) + Simd32x2::from([other.group0(), 0.0]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
                g8: self.group1(),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Add<Sphere> for Motor {
    type Output = MultiVector;

    fn add(self, other: Sphere) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from([0.0, self.group0()[3]]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
                g8: self.group1(),
                g9: other.group0(),
                g10: other.group1(),
            },
        }
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

impl Add<DualNum> for MultiVector {
    type Output = MultiVector;

    fn add(self, other: DualNum) -> MultiVector {
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

impl AddAssign<DualNum> for MultiVector {
    fn add_assign(&mut self, other: DualNum) {
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

impl Add<AntiScalar> for Plane {
    type Output = MultiVector;

    fn add(self, other: AntiScalar) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from([0.0, other.group0()]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
                g10: Simd32x2::from([0.0, self.group0()[3]]),
            },
        }
    }
}

impl Add<Circle> for Plane {
    type Output = MultiVector;

    fn add(self, other: Circle) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: other.group0(),
                g7: other.group1(),
                g8: other.group2(),
                g9: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
                g10: Simd32x2::from([0.0, self.group0()[3]]),
            },
        }
    }
}

impl Add<Dipole> for Plane {
    type Output = MultiVector;

    fn add(self, other: Dipole) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: other.group0(),
                g4: other.group1(),
                g5: other.group2(),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
                g10: Simd32x2::from([0.0, self.group0()[3]]),
            },
        }
    }
}

impl Add<DualNum> for Plane {
    type Output = MultiVector;

    fn add(self, other: DualNum) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: other.group0(),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
                g10: Simd32x2::from([0.0, self.group0()[3]]),
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

impl Add<Line> for Plane {
    type Output = MultiVector;

    fn add(self, other: Line) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: other.group0(),
                g8: other.group1(),
                g9: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
                g10: Simd32x2::from([0.0, self.group0()[3]]),
            },
        }
    }
}

impl Add<Motor> for Plane {
    type Output = MultiVector;

    fn add(self, other: Motor) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from([0.0, other.group0()[3]]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
                g8: other.group1(),
                g9: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
                g10: Simd32x2::from([0.0, self.group0()[3]]),
            },
        }
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

impl Add<RoundPoint> for Plane {
    type Output = MultiVector;

    fn add(self, other: RoundPoint) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: other.group0(),
                g2: other.group1(),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
                g10: Simd32x2::from([0.0, self.group0()[3]]),
            },
        }
    }
}

impl Add<Scalar> for Plane {
    type Output = MultiVector;

    fn add(self, other: Scalar) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from([other.group0(), 0.0]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
                g10: Simd32x2::from([0.0, self.group0()[3]]),
            },
        }
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

impl Add<AntiScalar> for RoundPoint {
    type Output = MultiVector;

    fn add(self, other: AntiScalar) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from([0.0, other.group0()]),
                g1: self.group0(),
                g2: self.group1(),
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

impl Add<Circle> for RoundPoint {
    type Output = MultiVector;

    fn add(self, other: Circle) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: self.group0(),
                g2: self.group1(),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: other.group0(),
                g7: other.group1(),
                g8: other.group2(),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Add<Dipole> for RoundPoint {
    type Output = MultiVector;

    fn add(self, other: Dipole) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: self.group0(),
                g2: self.group1(),
                g3: other.group0(),
                g4: other.group1(),
                g5: other.group2(),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Add<DualNum> for RoundPoint {
    type Output = MultiVector;

    fn add(self, other: DualNum) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: other.group0(),
                g1: self.group0(),
                g2: self.group1(),
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

impl Add<FlatPoint> for RoundPoint {
    type Output = MultiVector;

    fn add(self, other: FlatPoint) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: self.group0(),
                g2: self.group1(),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: other.group0(),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Add<Flector> for RoundPoint {
    type Output = MultiVector;

    fn add(self, other: Flector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: self.group0(),
                g2: self.group1(),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: other.group0(),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
                g10: Simd32x2::from([0.0, other.group1()[3]]),
            },
        }
    }
}

impl Add<Line> for RoundPoint {
    type Output = MultiVector;

    fn add(self, other: Line) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: self.group0(),
                g2: self.group1(),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: other.group0(),
                g8: other.group1(),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Add<Motor> for RoundPoint {
    type Output = MultiVector;

    fn add(self, other: Motor) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from([0.0, other.group0()[3]]),
                g1: self.group0(),
                g2: self.group1(),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
                g8: other.group1(),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
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

impl Add<Plane> for RoundPoint {
    type Output = MultiVector;

    fn add(self, other: Plane) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: self.group0(),
                g2: self.group1(),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
                g10: Simd32x2::from([0.0, other.group0()[3]]),
            },
        }
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

impl Add<Scalar> for RoundPoint {
    type Output = MultiVector;

    fn add(self, other: Scalar) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from([other.group0(), 0.0]),
                g1: self.group0(),
                g2: self.group1(),
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

impl Add<Sphere> for RoundPoint {
    type Output = MultiVector;

    fn add(self, other: Sphere) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: self.group0(),
                g2: self.group1(),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: other.group0(),
                g10: other.group1(),
            },
        }
    }
}

impl Add<AntiScalar> for Scalar {
    type Output = DualNum;

    fn add(self, other: AntiScalar) -> DualNum {
        DualNum {
            groups: DualNumGroups {
                g0: Simd32x2::from([self.group0(), 0.0]) + Simd32x2::from([0.0, other.group0()]),
            },
        }
    }
}

impl Add<Circle> for Scalar {
    type Output = MultiVector;

    fn add(self, other: Circle) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from([self.group0(), 0.0]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: other.group0(),
                g7: other.group1(),
                g8: other.group2(),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Add<Dipole> for Scalar {
    type Output = MultiVector;

    fn add(self, other: Dipole) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from([self.group0(), 0.0]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: other.group0(),
                g4: other.group1(),
                g5: other.group2(),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Add<DualNum> for Scalar {
    type Output = DualNum;

    fn add(self, other: DualNum) -> DualNum {
        DualNum {
            groups: DualNumGroups {
                g0: Simd32x2::from([self.group0(), 0.0]) + other.group0(),
            },
        }
    }
}

impl Add<FlatPoint> for Scalar {
    type Output = MultiVector;

    fn add(self, other: FlatPoint) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from([self.group0(), 0.0]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: other.group0(),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Add<Flector> for Scalar {
    type Output = MultiVector;

    fn add(self, other: Flector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from([self.group0(), 0.0]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: other.group0(),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
                g10: Simd32x2::from([0.0, other.group1()[3]]),
            },
        }
    }
}

impl Add<Line> for Scalar {
    type Output = MultiVector;

    fn add(self, other: Line) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from([self.group0(), 0.0]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: other.group0(),
                g8: other.group1(),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Add<Motor> for Scalar {
    type Output = MultiVector;

    fn add(self, other: Motor) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from([self.group0(), 0.0]) + Simd32x2::from([0.0, other.group0()[3]]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
                g8: other.group1(),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
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

impl Add<Plane> for Scalar {
    type Output = MultiVector;

    fn add(self, other: Plane) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from([self.group0(), 0.0]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
                g10: Simd32x2::from([0.0, other.group0()[3]]),
            },
        }
    }
}

impl Add<RoundPoint> for Scalar {
    type Output = MultiVector;

    fn add(self, other: RoundPoint) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from([self.group0(), 0.0]),
                g1: other.group0(),
                g2: other.group1(),
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

impl Add<Sphere> for Scalar {
    type Output = MultiVector;

    fn add(self, other: Sphere) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from([self.group0(), 0.0]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: other.group0(),
                g10: other.group1(),
            },
        }
    }
}

impl Add<AntiScalar> for Sphere {
    type Output = MultiVector;

    fn add(self, other: AntiScalar) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from([0.0, other.group0()]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: self.group0(),
                g10: self.group1(),
            },
        }
    }
}

impl Add<Circle> for Sphere {
    type Output = MultiVector;

    fn add(self, other: Circle) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: other.group0(),
                g7: other.group1(),
                g8: other.group2(),
                g9: self.group0(),
                g10: self.group1(),
            },
        }
    }
}

impl Add<Dipole> for Sphere {
    type Output = MultiVector;

    fn add(self, other: Dipole) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: other.group0(),
                g4: other.group1(),
                g5: other.group2(),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: self.group0(),
                g10: self.group1(),
            },
        }
    }
}

impl Add<DualNum> for Sphere {
    type Output = MultiVector;

    fn add(self, other: DualNum) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: other.group0(),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: self.group0(),
                g10: self.group1(),
            },
        }
    }
}

impl Add<FlatPoint> for Sphere {
    type Output = MultiVector;

    fn add(self, other: FlatPoint) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: other.group0(),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: self.group0(),
                g10: self.group1(),
            },
        }
    }
}

impl Add<Flector> for Sphere {
    type Output = MultiVector;

    fn add(self, other: Flector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: other.group0(),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: self.group0() + Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
                g10: self.group1() + Simd32x2::from([0.0, other.group1()[3]]),
            },
        }
    }
}

impl Add<Line> for Sphere {
    type Output = MultiVector;

    fn add(self, other: Line) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: other.group0(),
                g8: other.group1(),
                g9: self.group0(),
                g10: self.group1(),
            },
        }
    }
}

impl Add<Motor> for Sphere {
    type Output = MultiVector;

    fn add(self, other: Motor) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from([0.0, other.group0()[3]]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
                g8: other.group1(),
                g9: self.group0(),
                g10: self.group1(),
            },
        }
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

impl Add<RoundPoint> for Sphere {
    type Output = MultiVector;

    fn add(self, other: RoundPoint) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: other.group0(),
                g2: other.group1(),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: self.group0(),
                g10: self.group1(),
            },
        }
    }
}

impl Add<Scalar> for Sphere {
    type Output = MultiVector;

    fn add(self, other: Scalar) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from([other.group0(), 0.0]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: self.group0(),
                g10: self.group1(),
            },
        }
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

impl Div<AntiScalar> for AntiScalar {
    type Output = AntiScalar;

    fn div(self, other: AntiScalar) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: self.group0() / other.group0(),
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
                g0: self.group0() / other.group0(),
                g1: self.group1() / other.group1(),
                g2: self.group2() / other.group2(),
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
                g0: self.group0() / other.group0(),
                g1: self.group1() / other.group1(),
                g2: self.group2() / other.group2(),
            },
        }
    }
}

impl DivAssign<Dipole> for Dipole {
    fn div_assign(&mut self, other: Dipole) {
        *self = (*self).div(other);
    }
}

impl Div<DualNum> for DualNum {
    type Output = DualNum;

    fn div(self, other: DualNum) -> DualNum {
        DualNum {
            groups: DualNumGroups {
                g0: self.group0() / other.group0(),
            },
        }
    }
}

impl DivAssign<DualNum> for DualNum {
    fn div_assign(&mut self, other: DualNum) {
        *self = (*self).div(other);
    }
}

impl Div<FlatPoint> for FlatPoint {
    type Output = FlatPoint;

    fn div(self, other: FlatPoint) -> FlatPoint {
        FlatPoint {
            groups: FlatPointGroups {
                g0: self.group0() / other.group0(),
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
                g0: self.group0() / other.group0(),
                g1: self.group1() / other.group1(),
            },
        }
    }
}

impl DivAssign<Flector> for Flector {
    fn div_assign(&mut self, other: Flector) {
        *self = (*self).div(other);
    }
}

impl Div<Line> for Line {
    type Output = Line;

    fn div(self, other: Line) -> Line {
        Line {
            groups: LineGroups {
                g0: self.group0() / other.group0(),
                g1: self.group1() / other.group1(),
            },
        }
    }
}

impl DivAssign<Line> for Line {
    fn div_assign(&mut self, other: Line) {
        *self = (*self).div(other);
    }
}

impl Div<Motor> for Motor {
    type Output = Motor;

    fn div(self, other: Motor) -> Motor {
        Motor {
            groups: MotorGroups {
                g0: self.group0() / other.group0(),
                g1: self.group1() / other.group1(),
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
                g0: self.group0() / other.group0(),
                g1: self.group1() / other.group1(),
                g2: self.group2() / other.group2(),
                g3: self.group3() / other.group3(),
                g4: self.group4() / other.group4(),
                g5: self.group5() / other.group5(),
                g6: self.group6() / other.group6(),
                g7: self.group7() / other.group7(),
                g8: self.group8() / other.group8(),
                g9: self.group9() / other.group9(),
                g10: self.group10() / other.group10(),
            },
        }
    }
}

impl DivAssign<MultiVector> for MultiVector {
    fn div_assign(&mut self, other: MultiVector) {
        *self = (*self).div(other);
    }
}

impl Div<Plane> for Plane {
    type Output = Plane;

    fn div(self, other: Plane) -> Plane {
        Plane {
            groups: PlaneGroups {
                g0: self.group0() / other.group0(),
            },
        }
    }
}

impl DivAssign<Plane> for Plane {
    fn div_assign(&mut self, other: Plane) {
        *self = (*self).div(other);
    }
}

impl Div<RoundPoint> for RoundPoint {
    type Output = RoundPoint;

    fn div(self, other: RoundPoint) -> RoundPoint {
        RoundPoint {
            groups: RoundPointGroups {
                g0: self.group0() / other.group0(),
                g1: self.group1() / other.group1(),
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
                g0: self.group0() / other.group0(),
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
                g0: self.group0() / other.group0(),
                g1: self.group1() / other.group1(),
            },
        }
    }
}

impl DivAssign<Sphere> for Sphere {
    fn div_assign(&mut self, other: Sphere) {
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

impl Into<FlatPoint> for Dipole {
    fn into(self) -> FlatPoint {
        FlatPoint {
            groups: FlatPointGroups { g0: self.group2() },
        }
    }
}

impl Into<AntiScalar> for DualNum {
    fn into(self) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups { g0: self.group0()[1] },
        }
    }
}

impl Into<Scalar> for DualNum {
    fn into(self) -> Scalar {
        Scalar {
            groups: ScalarGroups { g0: self.group0()[0] },
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

impl Into<Plane> for Flector {
    fn into(self) -> Plane {
        Plane {
            groups: PlaneGroups { g0: self.group1() },
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

impl Into<DualNum> for MultiVector {
    fn into(self) -> DualNum {
        DualNum {
            groups: DualNumGroups { g0: self.group0() },
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

impl Into<Plane> for MultiVector {
    fn into(self) -> Plane {
        Plane {
            groups: PlaneGroups {
                g0: Simd32x4::from([self.group9()[0], self.group9()[1], self.group9()[2], self.group10()[1]]),
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

impl Into<Plane> for Sphere {
    fn into(self) -> Plane {
        Plane {
            groups: PlaneGroups {
                g0: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[1]]),
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

impl Mul<DualNum> for DualNum {
    type Output = DualNum;

    fn mul(self, other: DualNum) -> DualNum {
        DualNum {
            groups: DualNumGroups {
                g0: self.group0() * other.group0(),
            },
        }
    }
}

impl MulAssign<DualNum> for DualNum {
    fn mul_assign(&mut self, other: DualNum) {
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

impl Sub<Circle> for AntiScalar {
    type Output = MultiVector;

    fn sub(self, other: Circle) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from([0.0, self.group0()]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0) - other.group0(),
                g7: Simd32x3::from(0.0) - other.group1(),
                g8: Simd32x3::from(0.0) - other.group2(),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Sub<Dipole> for AntiScalar {
    type Output = MultiVector;

    fn sub(self, other: Dipole) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from([0.0, self.group0()]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0) - other.group0(),
                g4: Simd32x3::from(0.0) - other.group1(),
                g5: Simd32x4::from(0.0) - other.group2(),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Sub<DualNum> for AntiScalar {
    type Output = DualNum;

    fn sub(self, other: DualNum) -> DualNum {
        DualNum {
            groups: DualNumGroups {
                g0: Simd32x2::from([0.0, self.group0()]) - other.group0(),
            },
        }
    }
}

impl Sub<FlatPoint> for AntiScalar {
    type Output = MultiVector;

    fn sub(self, other: FlatPoint) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from([0.0, self.group0()]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0) - other.group0(),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Sub<Flector> for AntiScalar {
    type Output = MultiVector;

    fn sub(self, other: Flector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from([0.0, self.group0()]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0) - other.group0(),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(0.0) - Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
                g10: Simd32x2::from(0.0) - Simd32x2::from([0.0, other.group1()[3]]),
            },
        }
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

impl Sub<Plane> for AntiScalar {
    type Output = MultiVector;

    fn sub(self, other: Plane) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from([0.0, self.group0()]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(0.0) - Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
                g10: Simd32x2::from(0.0) - Simd32x2::from([0.0, other.group0()[3]]),
            },
        }
    }
}

impl Sub<RoundPoint> for AntiScalar {
    type Output = MultiVector;

    fn sub(self, other: RoundPoint) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from([0.0, self.group0()]),
                g1: Simd32x3::from(0.0) - other.group0(),
                g2: Simd32x2::from(0.0) - other.group1(),
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

impl Sub<Scalar> for AntiScalar {
    type Output = DualNum;

    fn sub(self, other: Scalar) -> DualNum {
        DualNum {
            groups: DualNumGroups {
                g0: Simd32x2::from([0.0, self.group0()]) - Simd32x2::from([other.group0(), 0.0]),
            },
        }
    }
}

impl Sub<Sphere> for AntiScalar {
    type Output = MultiVector;

    fn sub(self, other: Sphere) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from([0.0, self.group0()]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(0.0) - other.group0(),
                g10: Simd32x2::from(0.0) - other.group1(),
            },
        }
    }
}

impl Sub<AntiScalar> for Circle {
    type Output = MultiVector;

    fn sub(self, other: AntiScalar) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0) - Simd32x2::from([0.0, other.group0()]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: self.group0(),
                g7: self.group1(),
                g8: self.group2(),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
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

impl Sub<Dipole> for Circle {
    type Output = MultiVector;

    fn sub(self, other: Dipole) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0) - other.group0(),
                g4: Simd32x3::from(0.0) - other.group1(),
                g5: Simd32x4::from(0.0) - other.group2(),
                g6: self.group0(),
                g7: self.group1(),
                g8: self.group2(),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Sub<DualNum> for Circle {
    type Output = MultiVector;

    fn sub(self, other: DualNum) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0) - other.group0(),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: self.group0(),
                g7: self.group1(),
                g8: self.group2(),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Sub<FlatPoint> for Circle {
    type Output = MultiVector;

    fn sub(self, other: FlatPoint) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0) - other.group0(),
                g6: self.group0(),
                g7: self.group1(),
                g8: self.group2(),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Sub<Flector> for Circle {
    type Output = MultiVector;

    fn sub(self, other: Flector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0) - other.group0(),
                g6: self.group0(),
                g7: self.group1(),
                g8: self.group2(),
                g9: Simd32x3::from(0.0) - Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
                g10: Simd32x2::from(0.0) - Simd32x2::from([0.0, other.group1()[3]]),
            },
        }
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

impl Sub<Motor> for Circle {
    type Output = MultiVector;

    fn sub(self, other: Motor) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0) - Simd32x2::from([0.0, other.group0()[3]]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: self.group0(),
                g7: self.group1() - Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
                g8: self.group2() - other.group1(),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
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

impl Sub<Plane> for Circle {
    type Output = MultiVector;

    fn sub(self, other: Plane) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: self.group0(),
                g7: self.group1(),
                g8: self.group2(),
                g9: Simd32x3::from(0.0) - Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
                g10: Simd32x2::from(0.0) - Simd32x2::from([0.0, other.group0()[3]]),
            },
        }
    }
}

impl Sub<RoundPoint> for Circle {
    type Output = MultiVector;

    fn sub(self, other: RoundPoint) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0) - other.group0(),
                g2: Simd32x2::from(0.0) - other.group1(),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: self.group0(),
                g7: self.group1(),
                g8: self.group2(),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Sub<Scalar> for Circle {
    type Output = MultiVector;

    fn sub(self, other: Scalar) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0) - Simd32x2::from([other.group0(), 0.0]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: self.group0(),
                g7: self.group1(),
                g8: self.group2(),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Sub<Sphere> for Circle {
    type Output = MultiVector;

    fn sub(self, other: Sphere) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: self.group0(),
                g7: self.group1(),
                g8: self.group2(),
                g9: Simd32x3::from(0.0) - other.group0(),
                g10: Simd32x2::from(0.0) - other.group1(),
            },
        }
    }
}

impl Sub<AntiScalar> for Dipole {
    type Output = MultiVector;

    fn sub(self, other: AntiScalar) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0) - Simd32x2::from([0.0, other.group0()]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: self.group0(),
                g4: self.group1(),
                g5: self.group2(),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Sub<Circle> for Dipole {
    type Output = MultiVector;

    fn sub(self, other: Circle) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: self.group0(),
                g4: self.group1(),
                g5: self.group2(),
                g6: Simd32x4::from(0.0) - other.group0(),
                g7: Simd32x3::from(0.0) - other.group1(),
                g8: Simd32x3::from(0.0) - other.group2(),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
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

impl Sub<DualNum> for Dipole {
    type Output = MultiVector;

    fn sub(self, other: DualNum) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0) - other.group0(),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: self.group0(),
                g4: self.group1(),
                g5: self.group2(),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
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

impl Sub<Flector> for Dipole {
    type Output = MultiVector;

    fn sub(self, other: Flector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: self.group0(),
                g4: self.group1(),
                g5: self.group2() - other.group0(),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(0.0) - Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
                g10: Simd32x2::from(0.0) - Simd32x2::from([0.0, other.group1()[3]]),
            },
        }
    }
}

impl Sub<Line> for Dipole {
    type Output = MultiVector;

    fn sub(self, other: Line) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: self.group0(),
                g4: self.group1(),
                g5: self.group2(),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0) - other.group0(),
                g8: Simd32x3::from(0.0) - other.group1(),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Sub<Motor> for Dipole {
    type Output = MultiVector;

    fn sub(self, other: Motor) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0) - Simd32x2::from([0.0, other.group0()[3]]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: self.group0(),
                g4: self.group1(),
                g5: self.group2(),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0) - Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
                g8: Simd32x3::from(0.0) - other.group1(),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
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

impl Sub<Plane> for Dipole {
    type Output = MultiVector;

    fn sub(self, other: Plane) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: self.group0(),
                g4: self.group1(),
                g5: self.group2(),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(0.0) - Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
                g10: Simd32x2::from(0.0) - Simd32x2::from([0.0, other.group0()[3]]),
            },
        }
    }
}

impl Sub<RoundPoint> for Dipole {
    type Output = MultiVector;

    fn sub(self, other: RoundPoint) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0) - other.group0(),
                g2: Simd32x2::from(0.0) - other.group1(),
                g3: self.group0(),
                g4: self.group1(),
                g5: self.group2(),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Sub<Scalar> for Dipole {
    type Output = MultiVector;

    fn sub(self, other: Scalar) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0) - Simd32x2::from([other.group0(), 0.0]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: self.group0(),
                g4: self.group1(),
                g5: self.group2(),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Sub<Sphere> for Dipole {
    type Output = MultiVector;

    fn sub(self, other: Sphere) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: self.group0(),
                g4: self.group1(),
                g5: self.group2(),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(0.0) - other.group0(),
                g10: Simd32x2::from(0.0) - other.group1(),
            },
        }
    }
}

impl Sub<AntiScalar> for DualNum {
    type Output = DualNum;

    fn sub(self, other: AntiScalar) -> DualNum {
        DualNum {
            groups: DualNumGroups {
                g0: self.group0() - Simd32x2::from([0.0, other.group0()]),
            },
        }
    }
}

impl SubAssign<AntiScalar> for DualNum {
    fn sub_assign(&mut self, other: AntiScalar) {
        *self = (*self).sub(other);
    }
}

impl Sub<Circle> for DualNum {
    type Output = MultiVector;

    fn sub(self, other: Circle) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: self.group0(),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0) - other.group0(),
                g7: Simd32x3::from(0.0) - other.group1(),
                g8: Simd32x3::from(0.0) - other.group2(),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Sub<Dipole> for DualNum {
    type Output = MultiVector;

    fn sub(self, other: Dipole) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: self.group0(),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0) - other.group0(),
                g4: Simd32x3::from(0.0) - other.group1(),
                g5: Simd32x4::from(0.0) - other.group2(),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Sub<DualNum> for DualNum {
    type Output = DualNum;

    fn sub(self, other: DualNum) -> DualNum {
        DualNum {
            groups: DualNumGroups {
                g0: self.group0() - other.group0(),
            },
        }
    }
}

impl SubAssign<DualNum> for DualNum {
    fn sub_assign(&mut self, other: DualNum) {
        *self = (*self).sub(other);
    }
}

impl Sub<FlatPoint> for DualNum {
    type Output = MultiVector;

    fn sub(self, other: FlatPoint) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: self.group0(),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0) - other.group0(),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Sub<Flector> for DualNum {
    type Output = MultiVector;

    fn sub(self, other: Flector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: self.group0(),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0) - other.group0(),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(0.0) - Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
                g10: Simd32x2::from(0.0) - Simd32x2::from([0.0, other.group1()[3]]),
            },
        }
    }
}

impl Sub<Line> for DualNum {
    type Output = MultiVector;

    fn sub(self, other: Line) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: self.group0(),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0) - other.group0(),
                g8: Simd32x3::from(0.0) - other.group1(),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Sub<Motor> for DualNum {
    type Output = MultiVector;

    fn sub(self, other: Motor) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: self.group0() - Simd32x2::from([0.0, other.group0()[3]]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0) - Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
                g8: Simd32x3::from(0.0) - other.group1(),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Sub<MultiVector> for DualNum {
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

impl Sub<Plane> for DualNum {
    type Output = MultiVector;

    fn sub(self, other: Plane) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: self.group0(),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(0.0) - Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
                g10: Simd32x2::from(0.0) - Simd32x2::from([0.0, other.group0()[3]]),
            },
        }
    }
}

impl Sub<RoundPoint> for DualNum {
    type Output = MultiVector;

    fn sub(self, other: RoundPoint) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: self.group0(),
                g1: Simd32x3::from(0.0) - other.group0(),
                g2: Simd32x2::from(0.0) - other.group1(),
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

impl Sub<Scalar> for DualNum {
    type Output = DualNum;

    fn sub(self, other: Scalar) -> DualNum {
        DualNum {
            groups: DualNumGroups {
                g0: self.group0() - Simd32x2::from([other.group0(), 0.0]),
            },
        }
    }
}

impl SubAssign<Scalar> for DualNum {
    fn sub_assign(&mut self, other: Scalar) {
        *self = (*self).sub(other);
    }
}

impl Sub<Sphere> for DualNum {
    type Output = MultiVector;

    fn sub(self, other: Sphere) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: self.group0(),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(0.0) - other.group0(),
                g10: Simd32x2::from(0.0) - other.group1(),
            },
        }
    }
}

impl Sub<AntiScalar> for FlatPoint {
    type Output = MultiVector;

    fn sub(self, other: AntiScalar) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0) - Simd32x2::from([0.0, other.group0()]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: self.group0(),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Sub<Circle> for FlatPoint {
    type Output = MultiVector;

    fn sub(self, other: Circle) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: self.group0(),
                g6: Simd32x4::from(0.0) - other.group0(),
                g7: Simd32x3::from(0.0) - other.group1(),
                g8: Simd32x3::from(0.0) - other.group2(),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
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

impl Sub<DualNum> for FlatPoint {
    type Output = MultiVector;

    fn sub(self, other: DualNum) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0) - other.group0(),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: self.group0(),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
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

impl Sub<Line> for FlatPoint {
    type Output = MultiVector;

    fn sub(self, other: Line) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: self.group0(),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0) - other.group0(),
                g8: Simd32x3::from(0.0) - other.group1(),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Sub<Motor> for FlatPoint {
    type Output = MultiVector;

    fn sub(self, other: Motor) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0) - Simd32x2::from([0.0, other.group0()[3]]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: self.group0(),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0) - Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
                g8: Simd32x3::from(0.0) - other.group1(),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
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

impl Sub<RoundPoint> for FlatPoint {
    type Output = MultiVector;

    fn sub(self, other: RoundPoint) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0) - other.group0(),
                g2: Simd32x2::from(0.0) - other.group1(),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: self.group0(),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Sub<Scalar> for FlatPoint {
    type Output = MultiVector;

    fn sub(self, other: Scalar) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0) - Simd32x2::from([other.group0(), 0.0]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: self.group0(),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Sub<Sphere> for FlatPoint {
    type Output = MultiVector;

    fn sub(self, other: Sphere) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: self.group0(),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(0.0) - other.group0(),
                g10: Simd32x2::from(0.0) - other.group1(),
            },
        }
    }
}

impl Sub<AntiScalar> for Flector {
    type Output = MultiVector;

    fn sub(self, other: AntiScalar) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0) - Simd32x2::from([0.0, other.group0()]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: self.group0(),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
                g10: Simd32x2::from([0.0, self.group1()[3]]),
            },
        }
    }
}

impl Sub<Circle> for Flector {
    type Output = MultiVector;

    fn sub(self, other: Circle) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: self.group0(),
                g6: Simd32x4::from(0.0) - other.group0(),
                g7: Simd32x3::from(0.0) - other.group1(),
                g8: Simd32x3::from(0.0) - other.group2(),
                g9: Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
                g10: Simd32x2::from([0.0, self.group1()[3]]),
            },
        }
    }
}

impl Sub<Dipole> for Flector {
    type Output = MultiVector;

    fn sub(self, other: Dipole) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0) - other.group0(),
                g4: Simd32x3::from(0.0) - other.group1(),
                g5: self.group0() - other.group2(),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
                g10: Simd32x2::from([0.0, self.group1()[3]]),
            },
        }
    }
}

impl Sub<DualNum> for Flector {
    type Output = MultiVector;

    fn sub(self, other: DualNum) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0) - other.group0(),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: self.group0(),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
                g10: Simd32x2::from([0.0, self.group1()[3]]),
            },
        }
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

impl Sub<Line> for Flector {
    type Output = MultiVector;

    fn sub(self, other: Line) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: self.group0(),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0) - other.group0(),
                g8: Simd32x3::from(0.0) - other.group1(),
                g9: Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
                g10: Simd32x2::from([0.0, self.group1()[3]]),
            },
        }
    }
}

impl Sub<Motor> for Flector {
    type Output = MultiVector;

    fn sub(self, other: Motor) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0) - Simd32x2::from([0.0, other.group0()[3]]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: self.group0(),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0) - Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
                g8: Simd32x3::from(0.0) - other.group1(),
                g9: Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
                g10: Simd32x2::from([0.0, self.group1()[3]]),
            },
        }
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

impl Sub<RoundPoint> for Flector {
    type Output = MultiVector;

    fn sub(self, other: RoundPoint) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0) - other.group0(),
                g2: Simd32x2::from(0.0) - other.group1(),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: self.group0(),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
                g10: Simd32x2::from([0.0, self.group1()[3]]),
            },
        }
    }
}

impl Sub<Scalar> for Flector {
    type Output = MultiVector;

    fn sub(self, other: Scalar) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0) - Simd32x2::from([other.group0(), 0.0]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: self.group0(),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
                g10: Simd32x2::from([0.0, self.group1()[3]]),
            },
        }
    }
}

impl Sub<Sphere> for Flector {
    type Output = MultiVector;

    fn sub(self, other: Sphere) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: self.group0(),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]) - other.group0(),
                g10: Simd32x2::from([0.0, self.group1()[3]]) - other.group1(),
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

impl Sub<Dipole> for Line {
    type Output = MultiVector;

    fn sub(self, other: Dipole) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0) - other.group0(),
                g4: Simd32x3::from(0.0) - other.group1(),
                g5: Simd32x4::from(0.0) - other.group2(),
                g6: Simd32x4::from(0.0),
                g7: self.group0(),
                g8: self.group1(),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Sub<DualNum> for Line {
    type Output = MultiVector;

    fn sub(self, other: DualNum) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0) - other.group0(),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: self.group0(),
                g8: self.group1(),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Sub<FlatPoint> for Line {
    type Output = MultiVector;

    fn sub(self, other: FlatPoint) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0) - other.group0(),
                g6: Simd32x4::from(0.0),
                g7: self.group0(),
                g8: self.group1(),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Sub<Flector> for Line {
    type Output = MultiVector;

    fn sub(self, other: Flector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0) - other.group0(),
                g6: Simd32x4::from(0.0),
                g7: self.group0(),
                g8: self.group1(),
                g9: Simd32x3::from(0.0) - Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
                g10: Simd32x2::from(0.0) - Simd32x2::from([0.0, other.group1()[3]]),
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

impl Sub<Plane> for Line {
    type Output = MultiVector;

    fn sub(self, other: Plane) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: self.group0(),
                g8: self.group1(),
                g9: Simd32x3::from(0.0) - Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
                g10: Simd32x2::from(0.0) - Simd32x2::from([0.0, other.group0()[3]]),
            },
        }
    }
}

impl Sub<RoundPoint> for Line {
    type Output = MultiVector;

    fn sub(self, other: RoundPoint) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0) - other.group0(),
                g2: Simd32x2::from(0.0) - other.group1(),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: self.group0(),
                g8: self.group1(),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Sub<Scalar> for Line {
    type Output = MultiVector;

    fn sub(self, other: Scalar) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0) - Simd32x2::from([other.group0(), 0.0]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: self.group0(),
                g8: self.group1(),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Sub<Sphere> for Line {
    type Output = MultiVector;

    fn sub(self, other: Sphere) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: self.group0(),
                g8: self.group1(),
                g9: Simd32x3::from(0.0) - other.group0(),
                g10: Simd32x2::from(0.0) - other.group1(),
            },
        }
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

impl Sub<Circle> for Motor {
    type Output = MultiVector;

    fn sub(self, other: Circle) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from([0.0, self.group0()[3]]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0) - other.group0(),
                g7: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) - other.group1(),
                g8: self.group1() - other.group2(),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Sub<Dipole> for Motor {
    type Output = MultiVector;

    fn sub(self, other: Dipole) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from([0.0, self.group0()[3]]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0) - other.group0(),
                g4: Simd32x3::from(0.0) - other.group1(),
                g5: Simd32x4::from(0.0) - other.group2(),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
                g8: self.group1(),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Sub<DualNum> for Motor {
    type Output = MultiVector;

    fn sub(self, other: DualNum) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from([0.0, self.group0()[3]]) - other.group0(),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
                g8: self.group1(),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Sub<FlatPoint> for Motor {
    type Output = MultiVector;

    fn sub(self, other: FlatPoint) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from([0.0, self.group0()[3]]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0) - other.group0(),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
                g8: self.group1(),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Sub<Flector> for Motor {
    type Output = MultiVector;

    fn sub(self, other: Flector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from([0.0, self.group0()[3]]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0) - other.group0(),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
                g8: self.group1(),
                g9: Simd32x3::from(0.0) - Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
                g10: Simd32x2::from(0.0) - Simd32x2::from([0.0, other.group1()[3]]),
            },
        }
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

impl Sub<Plane> for Motor {
    type Output = MultiVector;

    fn sub(self, other: Plane) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from([0.0, self.group0()[3]]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
                g8: self.group1(),
                g9: Simd32x3::from(0.0) - Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
                g10: Simd32x2::from(0.0) - Simd32x2::from([0.0, other.group0()[3]]),
            },
        }
    }
}

impl Sub<RoundPoint> for Motor {
    type Output = MultiVector;

    fn sub(self, other: RoundPoint) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from([0.0, self.group0()[3]]),
                g1: Simd32x3::from(0.0) - other.group0(),
                g2: Simd32x2::from(0.0) - other.group1(),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
                g8: self.group1(),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Sub<Scalar> for Motor {
    type Output = MultiVector;

    fn sub(self, other: Scalar) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from([0.0, self.group0()[3]]) - Simd32x2::from([other.group0(), 0.0]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
                g8: self.group1(),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Sub<Sphere> for Motor {
    type Output = MultiVector;

    fn sub(self, other: Sphere) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from([0.0, self.group0()[3]]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
                g8: self.group1(),
                g9: Simd32x3::from(0.0) - other.group0(),
                g10: Simd32x2::from(0.0) - other.group1(),
            },
        }
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

impl Sub<DualNum> for MultiVector {
    type Output = MultiVector;

    fn sub(self, other: DualNum) -> MultiVector {
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

impl SubAssign<DualNum> for MultiVector {
    fn sub_assign(&mut self, other: DualNum) {
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

impl Sub<AntiScalar> for Plane {
    type Output = MultiVector;

    fn sub(self, other: AntiScalar) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0) - Simd32x2::from([0.0, other.group0()]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
                g10: Simd32x2::from([0.0, self.group0()[3]]),
            },
        }
    }
}

impl Sub<Circle> for Plane {
    type Output = MultiVector;

    fn sub(self, other: Circle) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0) - other.group0(),
                g7: Simd32x3::from(0.0) - other.group1(),
                g8: Simd32x3::from(0.0) - other.group2(),
                g9: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
                g10: Simd32x2::from([0.0, self.group0()[3]]),
            },
        }
    }
}

impl Sub<Dipole> for Plane {
    type Output = MultiVector;

    fn sub(self, other: Dipole) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0) - other.group0(),
                g4: Simd32x3::from(0.0) - other.group1(),
                g5: Simd32x4::from(0.0) - other.group2(),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
                g10: Simd32x2::from([0.0, self.group0()[3]]),
            },
        }
    }
}

impl Sub<DualNum> for Plane {
    type Output = MultiVector;

    fn sub(self, other: DualNum) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0) - other.group0(),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
                g10: Simd32x2::from([0.0, self.group0()[3]]),
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

impl Sub<Line> for Plane {
    type Output = MultiVector;

    fn sub(self, other: Line) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0) - other.group0(),
                g8: Simd32x3::from(0.0) - other.group1(),
                g9: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
                g10: Simd32x2::from([0.0, self.group0()[3]]),
            },
        }
    }
}

impl Sub<Motor> for Plane {
    type Output = MultiVector;

    fn sub(self, other: Motor) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0) - Simd32x2::from([0.0, other.group0()[3]]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0) - Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
                g8: Simd32x3::from(0.0) - other.group1(),
                g9: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
                g10: Simd32x2::from([0.0, self.group0()[3]]),
            },
        }
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

impl Sub<RoundPoint> for Plane {
    type Output = MultiVector;

    fn sub(self, other: RoundPoint) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0) - other.group0(),
                g2: Simd32x2::from(0.0) - other.group1(),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
                g10: Simd32x2::from([0.0, self.group0()[3]]),
            },
        }
    }
}

impl Sub<Scalar> for Plane {
    type Output = MultiVector;

    fn sub(self, other: Scalar) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0) - Simd32x2::from([other.group0(), 0.0]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
                g10: Simd32x2::from([0.0, self.group0()[3]]),
            },
        }
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

impl Sub<AntiScalar> for RoundPoint {
    type Output = MultiVector;

    fn sub(self, other: AntiScalar) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0) - Simd32x2::from([0.0, other.group0()]),
                g1: self.group0(),
                g2: self.group1(),
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

impl Sub<Circle> for RoundPoint {
    type Output = MultiVector;

    fn sub(self, other: Circle) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: self.group0(),
                g2: self.group1(),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0) - other.group0(),
                g7: Simd32x3::from(0.0) - other.group1(),
                g8: Simd32x3::from(0.0) - other.group2(),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Sub<Dipole> for RoundPoint {
    type Output = MultiVector;

    fn sub(self, other: Dipole) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: self.group0(),
                g2: self.group1(),
                g3: Simd32x3::from(0.0) - other.group0(),
                g4: Simd32x3::from(0.0) - other.group1(),
                g5: Simd32x4::from(0.0) - other.group2(),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Sub<DualNum> for RoundPoint {
    type Output = MultiVector;

    fn sub(self, other: DualNum) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0) - other.group0(),
                g1: self.group0(),
                g2: self.group1(),
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

impl Sub<FlatPoint> for RoundPoint {
    type Output = MultiVector;

    fn sub(self, other: FlatPoint) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: self.group0(),
                g2: self.group1(),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0) - other.group0(),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Sub<Flector> for RoundPoint {
    type Output = MultiVector;

    fn sub(self, other: Flector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: self.group0(),
                g2: self.group1(),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0) - other.group0(),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(0.0) - Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
                g10: Simd32x2::from(0.0) - Simd32x2::from([0.0, other.group1()[3]]),
            },
        }
    }
}

impl Sub<Line> for RoundPoint {
    type Output = MultiVector;

    fn sub(self, other: Line) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: self.group0(),
                g2: self.group1(),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0) - other.group0(),
                g8: Simd32x3::from(0.0) - other.group1(),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Sub<Motor> for RoundPoint {
    type Output = MultiVector;

    fn sub(self, other: Motor) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0) - Simd32x2::from([0.0, other.group0()[3]]),
                g1: self.group0(),
                g2: self.group1(),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0) - Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
                g8: Simd32x3::from(0.0) - other.group1(),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
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

impl Sub<Plane> for RoundPoint {
    type Output = MultiVector;

    fn sub(self, other: Plane) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: self.group0(),
                g2: self.group1(),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(0.0) - Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
                g10: Simd32x2::from(0.0) - Simd32x2::from([0.0, other.group0()[3]]),
            },
        }
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

impl Sub<Scalar> for RoundPoint {
    type Output = MultiVector;

    fn sub(self, other: Scalar) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0) - Simd32x2::from([other.group0(), 0.0]),
                g1: self.group0(),
                g2: self.group1(),
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

impl Sub<Sphere> for RoundPoint {
    type Output = MultiVector;

    fn sub(self, other: Sphere) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: self.group0(),
                g2: self.group1(),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(0.0) - other.group0(),
                g10: Simd32x2::from(0.0) - other.group1(),
            },
        }
    }
}

impl Sub<AntiScalar> for Scalar {
    type Output = DualNum;

    fn sub(self, other: AntiScalar) -> DualNum {
        DualNum {
            groups: DualNumGroups {
                g0: Simd32x2::from([self.group0(), 0.0]) - Simd32x2::from([0.0, other.group0()]),
            },
        }
    }
}

impl Sub<Circle> for Scalar {
    type Output = MultiVector;

    fn sub(self, other: Circle) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from([self.group0(), 0.0]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0) - other.group0(),
                g7: Simd32x3::from(0.0) - other.group1(),
                g8: Simd32x3::from(0.0) - other.group2(),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Sub<Dipole> for Scalar {
    type Output = MultiVector;

    fn sub(self, other: Dipole) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from([self.group0(), 0.0]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0) - other.group0(),
                g4: Simd32x3::from(0.0) - other.group1(),
                g5: Simd32x4::from(0.0) - other.group2(),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Sub<DualNum> for Scalar {
    type Output = DualNum;

    fn sub(self, other: DualNum) -> DualNum {
        DualNum {
            groups: DualNumGroups {
                g0: Simd32x2::from([self.group0(), 0.0]) - other.group0(),
            },
        }
    }
}

impl Sub<FlatPoint> for Scalar {
    type Output = MultiVector;

    fn sub(self, other: FlatPoint) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from([self.group0(), 0.0]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0) - other.group0(),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Sub<Flector> for Scalar {
    type Output = MultiVector;

    fn sub(self, other: Flector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from([self.group0(), 0.0]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0) - other.group0(),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(0.0) - Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
                g10: Simd32x2::from(0.0) - Simd32x2::from([0.0, other.group1()[3]]),
            },
        }
    }
}

impl Sub<Line> for Scalar {
    type Output = MultiVector;

    fn sub(self, other: Line) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from([self.group0(), 0.0]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0) - other.group0(),
                g8: Simd32x3::from(0.0) - other.group1(),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Sub<Motor> for Scalar {
    type Output = MultiVector;

    fn sub(self, other: Motor) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from([self.group0(), 0.0]) - Simd32x2::from([0.0, other.group0()[3]]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0) - Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
                g8: Simd32x3::from(0.0) - other.group1(),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
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

impl Sub<Plane> for Scalar {
    type Output = MultiVector;

    fn sub(self, other: Plane) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from([self.group0(), 0.0]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(0.0) - Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
                g10: Simd32x2::from(0.0) - Simd32x2::from([0.0, other.group0()[3]]),
            },
        }
    }
}

impl Sub<RoundPoint> for Scalar {
    type Output = MultiVector;

    fn sub(self, other: RoundPoint) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from([self.group0(), 0.0]),
                g1: Simd32x3::from(0.0) - other.group0(),
                g2: Simd32x2::from(0.0) - other.group1(),
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

impl Sub<Sphere> for Scalar {
    type Output = MultiVector;

    fn sub(self, other: Sphere) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from([self.group0(), 0.0]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(0.0) - other.group0(),
                g10: Simd32x2::from(0.0) - other.group1(),
            },
        }
    }
}

impl Sub<AntiScalar> for Sphere {
    type Output = MultiVector;

    fn sub(self, other: AntiScalar) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0) - Simd32x2::from([0.0, other.group0()]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: self.group0(),
                g10: self.group1(),
            },
        }
    }
}

impl Sub<Circle> for Sphere {
    type Output = MultiVector;

    fn sub(self, other: Circle) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0) - other.group0(),
                g7: Simd32x3::from(0.0) - other.group1(),
                g8: Simd32x3::from(0.0) - other.group2(),
                g9: self.group0(),
                g10: self.group1(),
            },
        }
    }
}

impl Sub<Dipole> for Sphere {
    type Output = MultiVector;

    fn sub(self, other: Dipole) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0) - other.group0(),
                g4: Simd32x3::from(0.0) - other.group1(),
                g5: Simd32x4::from(0.0) - other.group2(),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: self.group0(),
                g10: self.group1(),
            },
        }
    }
}

impl Sub<DualNum> for Sphere {
    type Output = MultiVector;

    fn sub(self, other: DualNum) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0) - other.group0(),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: self.group0(),
                g10: self.group1(),
            },
        }
    }
}

impl Sub<FlatPoint> for Sphere {
    type Output = MultiVector;

    fn sub(self, other: FlatPoint) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0) - other.group0(),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: self.group0(),
                g10: self.group1(),
            },
        }
    }
}

impl Sub<Flector> for Sphere {
    type Output = MultiVector;

    fn sub(self, other: Flector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0) - other.group0(),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: self.group0() - Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
                g10: self.group1() - Simd32x2::from([0.0, other.group1()[3]]),
            },
        }
    }
}

impl Sub<Line> for Sphere {
    type Output = MultiVector;

    fn sub(self, other: Line) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0) - other.group0(),
                g8: Simd32x3::from(0.0) - other.group1(),
                g9: self.group0(),
                g10: self.group1(),
            },
        }
    }
}

impl Sub<Motor> for Sphere {
    type Output = MultiVector;

    fn sub(self, other: Motor) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0) - Simd32x2::from([0.0, other.group0()[3]]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0) - Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
                g8: Simd32x3::from(0.0) - other.group1(),
                g9: self.group0(),
                g10: self.group1(),
            },
        }
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

impl Sub<RoundPoint> for Sphere {
    type Output = MultiVector;

    fn sub(self, other: RoundPoint) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0) - other.group0(),
                g2: Simd32x2::from(0.0) - other.group1(),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: self.group0(),
                g10: self.group1(),
            },
        }
    }
}

impl Sub<Scalar> for Sphere {
    type Output = MultiVector;

    fn sub(self, other: Scalar) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0) - Simd32x2::from([other.group0(), 0.0]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: self.group0(),
                g10: self.group1(),
            },
        }
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
