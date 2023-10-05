#![allow(clippy::assign_op_pattern)]
use crate::{simd::*, *};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

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
    pub const fn new(element0: f32) -> Self {
        Self { elements: [element0] }
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
        formatter
            .debug_struct("Scalar")
            .field("1", &self[0])
            .finish()
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
    pub const fn new(element0: f32) -> Self {
        Self { elements: [element0] }
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
        formatter
            .debug_struct("AntiScalar")
            .field("e12345", &self[0])
            .finish()
    }
}

#[derive(Clone, Copy)]
struct RadialPointGroups {
    /// e1, e2, e3
    g0: Simd32x3,
    /// e4, e5
    g1: Simd32x2,
}

#[derive(Clone, Copy)]
pub union RadialPoint {
    groups: RadialPointGroups,
    /// e1, e2, e3, 0, e4, e5, 0, 0
    elements: [f32; 8],
}

impl RadialPoint {
    #[allow(clippy::too_many_arguments)]
    pub const fn new(element0: f32, element1: f32, element2: f32, element3: f32, element4: f32) -> Self {
        Self { elements: [element0, element1, element2, 0.0, element3, element4, 0.0, 0.0] }
    }
    pub const fn from_groups(g0: Simd32x3, g1: Simd32x2) -> Self {
        Self { groups: RadialPointGroups { g0, g1 } }
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

const RADIALPOINT_INDEX_REMAP: [usize; 5] = [0, 1, 2, 4, 5];

impl std::ops::Index<usize> for RadialPoint {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[RADIALPOINT_INDEX_REMAP[index]] }
    }
}

impl std::ops::IndexMut<usize> for RadialPoint {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[RADIALPOINT_INDEX_REMAP[index]] }
    }
}

impl std::convert::From<RadialPoint> for [f32; 5] {
    fn from(vector: RadialPoint) -> Self {
        unsafe { [vector.elements[0], vector.elements[1], vector.elements[2], vector.elements[4], vector.elements[5]] }
    }
}

impl std::convert::From<[f32; 5]> for RadialPoint {
    fn from(array: [f32; 5]) -> Self {
        Self { elements: [array[0], array[1], array[2], 0.0, array[3], array[4], 0.0, 0.0] }
    }
}

impl std::fmt::Debug for RadialPoint {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .debug_struct("RadialPoint")
            .field("e1", &self[0])
            .field("e2", &self[1])
            .field("e3", &self[2])
            .field("e4", &self[3])
            .field("e5", &self[4])
            .finish()
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
    pub const fn new(element0: f32, element1: f32, element2: f32, element3: f32) -> Self {
        Self { elements: [element0, element1, element2, element3] }
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
        Self { elements: [array[0], array[1], array[2], array[3]] }
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
    pub const fn new(element0: f32, element1: f32, element2: f32, element3: f32, element4: f32, element5: f32, element6: f32, element7: f32, element8: f32, element9: f32) -> Self {
        Self { elements: [element0, element1, element2, 0.0, element3, element4, element5, 0.0, element6, element7, element8, element9] }
    }
    pub const fn from_groups(g0: Simd32x3, g1: Simd32x3, g2: Simd32x4) -> Self {
        Self { groups: DipoleGroups { g0, g1, g2 } }
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
        unsafe { [vector.elements[0], vector.elements[1], vector.elements[2], vector.elements[4], vector.elements[5], vector.elements[6], vector.elements[8], vector.elements[9], vector.elements[10], vector.elements[11]] }
    }
}

impl std::convert::From<[f32; 10]> for Dipole {
    fn from(array: [f32; 10]) -> Self {
        Self { elements: [array[0], array[1], array[2], 0.0, array[3], array[4], array[5], 0.0, array[6], array[7], array[8], array[9]] }
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
    pub const fn new(element0: f32, element1: f32, element2: f32, element3: f32, element4: f32, element5: f32) -> Self {
        Self { elements: [element0, element1, element2, 0.0, element3, element4, element5, 0.0] }
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
        Self { elements: [array[0], array[1], array[2], 0.0, array[3], array[4], array[5], 0.0] }
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
    pub const fn new(element0: f32, element1: f32, element2: f32, element3: f32, element4: f32, element5: f32, element6: f32, element7: f32, element8: f32, element9: f32) -> Self {
        Self { elements: [element0, element1, element2, element3, element4, element5, element6, 0.0, element7, element8, element9, 0.0] }
    }
    pub const fn from_groups(g0: Simd32x4, g1: Simd32x3, g2: Simd32x3) -> Self {
        Self { groups: CircleGroups { g0, g1, g2 } }
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
        unsafe { [vector.elements[0], vector.elements[1], vector.elements[2], vector.elements[3], vector.elements[4], vector.elements[5], vector.elements[6], vector.elements[8], vector.elements[9], vector.elements[10]] }
    }
}

impl std::convert::From<[f32; 10]> for Circle {
    fn from(array: [f32; 10]) -> Self {
        Self { elements: [array[0], array[1], array[2], array[3], array[4], array[5], array[6], 0.0, array[7], array[8], array[9], 0.0] }
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
    pub const fn new(element0: f32, element1: f32, element2: f32, element3: f32) -> Self {
        Self { elements: [element0, element1, element2, element3] }
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
        Self { elements: [array[0], array[1], array[2], array[3]] }
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
struct SphereGroups {
    /// e1234
    g0: f32,
    /// e2345, -e1345, e1245, -e1235
    g1: Simd32x4,
}

#[derive(Clone, Copy)]
pub union Sphere {
    groups: SphereGroups,
    /// e1234, e2345, -e1345, e1245, -e1235
    elements: [f32; 5],
}

impl Sphere {
    #[allow(clippy::too_many_arguments)]
    pub const fn new(element0: f32, element1: f32, element2: f32, element3: f32, element4: f32) -> Self {
        Self { elements: [element0, element1, element2, element3, element4] }
    }
    pub const fn from_groups(g0: f32, g1: Simd32x4) -> Self {
        Self { groups: SphereGroups { g0, g1 } }
    }
    #[inline(always)]
    pub fn group0(&self) -> f32 {
        unsafe { self.groups.g0 }
    }
    #[inline(always)]
    pub fn group0_mut(&mut self) -> &mut f32 {
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

const SPHERE_INDEX_REMAP: [usize; 5] = [0, 1, 2, 3, 4];

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
        unsafe { [vector.elements[0], vector.elements[1], vector.elements[2], vector.elements[3], vector.elements[4]] }
    }
}

impl std::convert::From<[f32; 5]> for Sphere {
    fn from(array: [f32; 5]) -> Self {
        Self { elements: [array[0], array[1], array[2], array[3], array[4]] }
    }
}

impl std::fmt::Debug for Sphere {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .debug_struct("Sphere")
            .field("e1234", &self[0])
            .field("e2345", &self[1])
            .field("-e1345", &self[2])
            .field("e1245", &self[3])
            .field("-e1235", &self[4])
            .finish()
    }
}

#[derive(Clone, Copy)]
struct MotorGroups {
    /// -e145, -e245, -e345, e12345
    g0: Simd32x4,
    /// e235, -e135, e125, e5
    g1: Simd32x4,
}

#[derive(Clone, Copy)]
pub union Motor {
    groups: MotorGroups,
    /// -e145, -e245, -e345, e12345, e235, -e135, e125, e5
    elements: [f32; 8],
}

impl Motor {
    #[allow(clippy::too_many_arguments)]
    pub const fn new(element0: f32, element1: f32, element2: f32, element3: f32, element4: f32, element5: f32, element6: f32, element7: f32) -> Self {
        Self { elements: [element0, element1, element2, element3, element4, element5, element6, element7] }
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

impl std::convert::From<Motor> for [f32; 8] {
    fn from(vector: Motor) -> Self {
        unsafe { [vector.elements[0], vector.elements[1], vector.elements[2], vector.elements[3], vector.elements[4], vector.elements[5], vector.elements[6], vector.elements[7]] }
    }
}

impl std::convert::From<[f32; 8]> for Motor {
    fn from(array: [f32; 8]) -> Self {
        Self { elements: [array[0], array[1], array[2], array[3], array[4], array[5], array[6], array[7]] }
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
            .field("e5", &self[7])
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
    pub const fn new(element0: f32, element1: f32, element2: f32, element3: f32) -> Self {
        Self { elements: [element0, element1, element2, element3] }
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
        Self { elements: [array[0], array[1], array[2], array[3]] }
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
    pub const fn new(element0: f32, element1: f32, element2: f32, element3: f32) -> Self {
        Self { elements: [element0, element1, element2, element3] }
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
        Self { elements: [array[0], array[1], array[2], array[3]] }
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
    pub const fn new(element0: f32, element1: f32, element2: f32, element3: f32, element4: f32, element5: f32, element6: f32, element7: f32) -> Self {
        Self { elements: [element0, element1, element2, element3, element4, element5, element6, element7] }
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
        unsafe { [vector.elements[0], vector.elements[1], vector.elements[2], vector.elements[3], vector.elements[4], vector.elements[5], vector.elements[6], vector.elements[7]] }
    }
}

impl std::convert::From<[f32; 8]> for Flector {
    fn from(array: [f32; 8]) -> Self {
        Self { elements: [array[0], array[1], array[2], array[3], array[4], array[5], array[6], array[7]] }
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
    /// 1, e1234, e12345
    g0: Simd32x3,
    /// e1, e2, e3
    g1: Simd32x3,
    /// e4, e5
    g2: Simd32x2,
    /// e15, e25, e35, e45
    g3: Simd32x4,
    /// -e14, -e24, -e34
    g4: Simd32x3,
    /// e23, -e13, e12
    g5: Simd32x3,
    /// -e145, -e245, -e345
    g6: Simd32x3,
    /// e235, -e135, e125
    g7: Simd32x3,
    /// e234, -e134, e124, -e123
    g8: Simd32x4,
    /// e2345, -e1345, e1245, -e1235
    g9: Simd32x4,
}

#[derive(Clone, Copy)]
pub union MultiVector {
    groups: MultiVectorGroups,
    /// 1, e1234, e12345, 0, e1, e2, e3, 0, e4, e5, 0, 0, e15, e25, e35, e45, -e14, -e24, -e34, 0, e23, -e13, e12, 0, -e145, -e245, -e345, 0, e235, -e135, e125, 0, e234, -e134, e124, -e123, e2345, -e1345, e1245, -e1235
    elements: [f32; 40],
}

impl MultiVector {
    #[allow(clippy::too_many_arguments)]
    pub const fn new(element0: f32, element1: f32, element2: f32, element3: f32, element4: f32, element5: f32, element6: f32, element7: f32, element8: f32, element9: f32, element10: f32, element11: f32, element12: f32, element13: f32, element14: f32, element15: f32, element16: f32, element17: f32, element18: f32, element19: f32, element20: f32, element21: f32, element22: f32, element23: f32, element24: f32, element25: f32, element26: f32, element27: f32, element28: f32, element29: f32, element30: f32, element31: f32) -> Self {
        Self { elements: [element0, element1, element2, 0.0, element3, element4, element5, 0.0, element6, element7, 0.0, 0.0, element8, element9, element10, element11, element12, element13, element14, 0.0, element15, element16, element17, 0.0, element18, element19, element20, 0.0, element21, element22, element23, 0.0, element24, element25, element26, element27, element28, element29, element30, element31] }
    }
    pub const fn from_groups(g0: Simd32x3, g1: Simd32x3, g2: Simd32x2, g3: Simd32x4, g4: Simd32x3, g5: Simd32x3, g6: Simd32x3, g7: Simd32x3, g8: Simd32x4, g9: Simd32x4) -> Self {
        Self { groups: MultiVectorGroups { g0, g1, g2, g3, g4, g5, g6, g7, g8, g9 } }
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
    pub fn group2(&self) -> Simd32x2 {
        unsafe { self.groups.g2 }
    }
    #[inline(always)]
    pub fn group2_mut(&mut self) -> &mut Simd32x2 {
        unsafe { &mut self.groups.g2 }
    }
    #[inline(always)]
    pub fn group3(&self) -> Simd32x4 {
        unsafe { self.groups.g3 }
    }
    #[inline(always)]
    pub fn group3_mut(&mut self) -> &mut Simd32x4 {
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
    pub fn group5(&self) -> Simd32x3 {
        unsafe { self.groups.g5 }
    }
    #[inline(always)]
    pub fn group5_mut(&mut self) -> &mut Simd32x3 {
        unsafe { &mut self.groups.g5 }
    }
    #[inline(always)]
    pub fn group6(&self) -> Simd32x3 {
        unsafe { self.groups.g6 }
    }
    #[inline(always)]
    pub fn group6_mut(&mut self) -> &mut Simd32x3 {
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
    pub fn group8(&self) -> Simd32x4 {
        unsafe { self.groups.g8 }
    }
    #[inline(always)]
    pub fn group8_mut(&mut self) -> &mut Simd32x4 {
        unsafe { &mut self.groups.g8 }
    }
    #[inline(always)]
    pub fn group9(&self) -> Simd32x4 {
        unsafe { self.groups.g9 }
    }
    #[inline(always)]
    pub fn group9_mut(&mut self) -> &mut Simd32x4 {
        unsafe { &mut self.groups.g9 }
    }
}

const MULTIVECTOR_INDEX_REMAP: [usize; 32] = [0, 1, 2, 4, 5, 6, 8, 9, 12, 13, 14, 15, 16, 17, 18, 20, 21, 22, 24, 25, 26, 28, 29, 30, 32, 33, 34, 35, 36, 37, 38, 39];

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
        unsafe { [vector.elements[0], vector.elements[1], vector.elements[2], vector.elements[4], vector.elements[5], vector.elements[6], vector.elements[8], vector.elements[9], vector.elements[12], vector.elements[13], vector.elements[14], vector.elements[15], vector.elements[16], vector.elements[17], vector.elements[18], vector.elements[20], vector.elements[21], vector.elements[22], vector.elements[24], vector.elements[25], vector.elements[26], vector.elements[28], vector.elements[29], vector.elements[30], vector.elements[32], vector.elements[33], vector.elements[34], vector.elements[35], vector.elements[36], vector.elements[37], vector.elements[38], vector.elements[39]] }
    }
}

impl std::convert::From<[f32; 32]> for MultiVector {
    fn from(array: [f32; 32]) -> Self {
        Self { elements: [array[0], array[1], array[2], 0.0, array[3], array[4], array[5], 0.0, array[6], array[7], 0.0, 0.0, array[8], array[9], array[10], array[11], array[12], array[13], array[14], 0.0, array[15], array[16], array[17], 0.0, array[18], array[19], array[20], 0.0, array[21], array[22], array[23], 0.0, array[24], array[25], array[26], array[27], array[28], array[29], array[30], array[31]] }
    }
}

impl std::fmt::Debug for MultiVector {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .debug_struct("MultiVector")
            .field("1", &self[0])
            .field("e1234", &self[1])
            .field("e12345", &self[2])
            .field("e1", &self[3])
            .field("e2", &self[4])
            .field("e3", &self[5])
            .field("e4", &self[6])
            .field("e5", &self[7])
            .field("e15", &self[8])
            .field("e25", &self[9])
            .field("e35", &self[10])
            .field("e45", &self[11])
            .field("-e14", &self[12])
            .field("-e24", &self[13])
            .field("-e34", &self[14])
            .field("e23", &self[15])
            .field("-e13", &self[16])
            .field("e12", &self[17])
            .field("-e145", &self[18])
            .field("-e245", &self[19])
            .field("-e345", &self[20])
            .field("e235", &self[21])
            .field("-e135", &self[22])
            .field("e125", &self[23])
            .field("e234", &self[24])
            .field("-e134", &self[25])
            .field("e124", &self[26])
            .field("-e123", &self[27])
            .field("e2345", &self[28])
            .field("-e1345", &self[29])
            .field("e1245", &self[30])
            .field("-e1235", &self[31])
            .finish()
    }
}

impl Zero for Scalar {
    fn zero() -> Self {
        Scalar { groups: ScalarGroups { g0: 0.0 } }
    }
}

impl One for Scalar {
    fn one() -> Self {
        Scalar { groups: ScalarGroups { g0: 1.0 } }
    }
}

impl Neg for Scalar {
    type Output = Scalar;

    fn neg(self) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0() * -1.0 } }
    }
}

impl Automorphism for Scalar {
    type Output = Scalar;

    fn automorphism(self) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0() } }
    }
}

impl Reversal for Scalar {
    type Output = Scalar;

    fn reversal(self) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0() } }
    }
}

impl Conjugation for Scalar {
    type Output = Scalar;

    fn conjugation(self) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0() } }
    }
}

impl Add<Scalar> for Scalar {
    type Output = Scalar;

    fn add(self, other: Scalar) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0() + other.group0() } }
    }
}

impl AddAssign<Scalar> for Scalar {
    fn add_assign(&mut self, other: Scalar) {
        *self = (*self).add(other);
    }
}

impl Sub<Scalar> for Scalar {
    type Output = Scalar;

    fn sub(self, other: Scalar) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0() - other.group0() } }
    }
}

impl SubAssign<Scalar> for Scalar {
    fn sub_assign(&mut self, other: Scalar) {
        *self = (*self).sub(other);
    }
}

impl Mul<Scalar> for Scalar {
    type Output = Scalar;

    fn mul(self, other: Scalar) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0() * other.group0() } }
    }
}

impl MulAssign<Scalar> for Scalar {
    fn mul_assign(&mut self, other: Scalar) {
        *self = (*self).mul(other);
    }
}

impl Div<Scalar> for Scalar {
    type Output = Scalar;

    fn div(self, other: Scalar) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0() * 1.0 / other.group0() * 1.0 } }
    }
}

impl DivAssign<Scalar> for Scalar {
    fn div_assign(&mut self, other: Scalar) {
        *self = (*self).div(other);
    }
}

impl GeometricProduct<Scalar> for Scalar {
    type Output = Scalar;

    fn geometric_product(self, other: Scalar) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0() * other.group0() } }
    }
}

impl OuterProduct<Scalar> for Scalar {
    type Output = Scalar;

    fn outer_product(self, other: Scalar) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0() * other.group0() } }
    }
}

impl InnerProduct<Scalar> for Scalar {
    type Output = Scalar;

    fn inner_product(self, other: Scalar) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0() * other.group0() } }
    }
}

impl LeftContraction<Scalar> for Scalar {
    type Output = Scalar;

    fn left_contraction(self, other: Scalar) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0() * other.group0() } }
    }
}

impl RightContraction<Scalar> for Scalar {
    type Output = Scalar;

    fn right_contraction(self, other: Scalar) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0() * other.group0() } }
    }
}

impl ScalarProduct<Scalar> for Scalar {
    type Output = Scalar;

    fn scalar_product(self, other: Scalar) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0() * other.group0() } }
    }
}

impl GeometricProduct<AntiScalar> for Scalar {
    type Output = AntiScalar;

    fn geometric_product(self, other: AntiScalar) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0() * other.group0() } }
    }
}

impl OuterProduct<AntiScalar> for Scalar {
    type Output = AntiScalar;

    fn outer_product(self, other: AntiScalar) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0() * other.group0() } }
    }
}

impl InnerProduct<AntiScalar> for Scalar {
    type Output = AntiScalar;

    fn inner_product(self, other: AntiScalar) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0() * other.group0() } }
    }
}

impl LeftContraction<AntiScalar> for Scalar {
    type Output = AntiScalar;

    fn left_contraction(self, other: AntiScalar) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0() * other.group0() } }
    }
}

impl GeometricProduct<RadialPoint> for Scalar {
    type Output = RadialPoint;

    fn geometric_product(self, other: RadialPoint) -> RadialPoint {
        RadialPoint { groups: RadialPointGroups { g0: Simd32x3::from(self.group0()) * other.group0(), g1: Simd32x2::from(self.group0()) * other.group1() } }
    }
}

impl OuterProduct<RadialPoint> for Scalar {
    type Output = RadialPoint;

    fn outer_product(self, other: RadialPoint) -> RadialPoint {
        RadialPoint { groups: RadialPointGroups { g0: Simd32x3::from(self.group0()) * other.group0(), g1: Simd32x2::from(self.group0()) * other.group1() } }
    }
}

impl InnerProduct<RadialPoint> for Scalar {
    type Output = RadialPoint;

    fn inner_product(self, other: RadialPoint) -> RadialPoint {
        RadialPoint { groups: RadialPointGroups { g0: Simd32x3::from(self.group0()) * other.group0(), g1: Simd32x2::from(self.group0()) * other.group1() } }
    }
}

impl LeftContraction<RadialPoint> for Scalar {
    type Output = RadialPoint;

    fn left_contraction(self, other: RadialPoint) -> RadialPoint {
        RadialPoint { groups: RadialPointGroups { g0: Simd32x3::from(self.group0()) * other.group0(), g1: Simd32x2::from(self.group0()) * other.group1() } }
    }
}

impl GeometricProduct<FlatPoint> for Scalar {
    type Output = FlatPoint;

    fn geometric_product(self, other: FlatPoint) -> FlatPoint {
        FlatPoint { groups: FlatPointGroups { g0: Simd32x4::from(self.group0()) * other.group0() } }
    }
}

impl OuterProduct<FlatPoint> for Scalar {
    type Output = FlatPoint;

    fn outer_product(self, other: FlatPoint) -> FlatPoint {
        FlatPoint { groups: FlatPointGroups { g0: Simd32x4::from(self.group0()) * other.group0() } }
    }
}

impl InnerProduct<FlatPoint> for Scalar {
    type Output = FlatPoint;

    fn inner_product(self, other: FlatPoint) -> FlatPoint {
        FlatPoint { groups: FlatPointGroups { g0: Simd32x4::from(self.group0()) * other.group0() } }
    }
}

impl LeftContraction<FlatPoint> for Scalar {
    type Output = FlatPoint;

    fn left_contraction(self, other: FlatPoint) -> FlatPoint {
        FlatPoint { groups: FlatPointGroups { g0: Simd32x4::from(self.group0()) * other.group0() } }
    }
}

impl GeometricProduct<Dipole> for Scalar {
    type Output = Dipole;

    fn geometric_product(self, other: Dipole) -> Dipole {
        Dipole { groups: DipoleGroups { g0: Simd32x3::from(self.group0()) * other.group0(), g1: Simd32x3::from(self.group0()) * other.group1(), g2: Simd32x4::from(self.group0()) * other.group2() } }
    }
}

impl OuterProduct<Dipole> for Scalar {
    type Output = Dipole;

    fn outer_product(self, other: Dipole) -> Dipole {
        Dipole { groups: DipoleGroups { g0: Simd32x3::from(self.group0()) * other.group0(), g1: Simd32x3::from(self.group0()) * other.group1(), g2: Simd32x4::from(self.group0()) * other.group2() } }
    }
}

impl InnerProduct<Dipole> for Scalar {
    type Output = Dipole;

    fn inner_product(self, other: Dipole) -> Dipole {
        Dipole { groups: DipoleGroups { g0: Simd32x3::from(self.group0()) * other.group0(), g1: Simd32x3::from(self.group0()) * other.group1(), g2: Simd32x4::from(self.group0()) * other.group2() } }
    }
}

impl LeftContraction<Dipole> for Scalar {
    type Output = Dipole;

    fn left_contraction(self, other: Dipole) -> Dipole {
        Dipole { groups: DipoleGroups { g0: Simd32x3::from(self.group0()) * other.group0(), g1: Simd32x3::from(self.group0()) * other.group1(), g2: Simd32x4::from(self.group0()) * other.group2() } }
    }
}

impl GeometricProduct<Line> for Scalar {
    type Output = Line;

    fn geometric_product(self, other: Line) -> Line {
        Line { groups: LineGroups { g0: Simd32x3::from(self.group0()) * other.group0(), g1: Simd32x3::from(self.group0()) * other.group1() } }
    }
}

impl OuterProduct<Line> for Scalar {
    type Output = Line;

    fn outer_product(self, other: Line) -> Line {
        Line { groups: LineGroups { g0: Simd32x3::from(self.group0()) * other.group0(), g1: Simd32x3::from(self.group0()) * other.group1() } }
    }
}

impl InnerProduct<Line> for Scalar {
    type Output = Line;

    fn inner_product(self, other: Line) -> Line {
        Line { groups: LineGroups { g0: Simd32x3::from(self.group0()) * other.group0(), g1: Simd32x3::from(self.group0()) * other.group1() } }
    }
}

impl LeftContraction<Line> for Scalar {
    type Output = Line;

    fn left_contraction(self, other: Line) -> Line {
        Line { groups: LineGroups { g0: Simd32x3::from(self.group0()) * other.group0(), g1: Simd32x3::from(self.group0()) * other.group1() } }
    }
}

impl GeometricProduct<Circle> for Scalar {
    type Output = Circle;

    fn geometric_product(self, other: Circle) -> Circle {
        Circle { groups: CircleGroups { g0: Simd32x4::from(self.group0()) * other.group0(), g1: Simd32x3::from(self.group0()) * other.group1(), g2: Simd32x3::from(self.group0()) * other.group2() } }
    }
}

impl OuterProduct<Circle> for Scalar {
    type Output = Circle;

    fn outer_product(self, other: Circle) -> Circle {
        Circle { groups: CircleGroups { g0: Simd32x4::from(self.group0()) * other.group0(), g1: Simd32x3::from(self.group0()) * other.group1(), g2: Simd32x3::from(self.group0()) * other.group2() } }
    }
}

impl InnerProduct<Circle> for Scalar {
    type Output = Circle;

    fn inner_product(self, other: Circle) -> Circle {
        Circle { groups: CircleGroups { g0: Simd32x4::from(self.group0()) * other.group0(), g1: Simd32x3::from(self.group0()) * other.group1(), g2: Simd32x3::from(self.group0()) * other.group2() } }
    }
}

impl LeftContraction<Circle> for Scalar {
    type Output = Circle;

    fn left_contraction(self, other: Circle) -> Circle {
        Circle { groups: CircleGroups { g0: Simd32x4::from(self.group0()) * other.group0(), g1: Simd32x3::from(self.group0()) * other.group1(), g2: Simd32x3::from(self.group0()) * other.group2() } }
    }
}

impl GeometricProduct<Plane> for Scalar {
    type Output = Plane;

    fn geometric_product(self, other: Plane) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()) * other.group0() } }
    }
}

impl OuterProduct<Plane> for Scalar {
    type Output = Plane;

    fn outer_product(self, other: Plane) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()) * other.group0() } }
    }
}

impl InnerProduct<Plane> for Scalar {
    type Output = Plane;

    fn inner_product(self, other: Plane) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()) * other.group0() } }
    }
}

impl LeftContraction<Plane> for Scalar {
    type Output = Plane;

    fn left_contraction(self, other: Plane) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()) * other.group0() } }
    }
}

impl GeometricProduct<Sphere> for Scalar {
    type Output = Sphere;

    fn geometric_product(self, other: Sphere) -> Sphere {
        Sphere { groups: SphereGroups { g0: self.group0() * other.group0(), g1: Simd32x4::from(self.group0()) * other.group1() } }
    }
}

impl OuterProduct<Sphere> for Scalar {
    type Output = Sphere;

    fn outer_product(self, other: Sphere) -> Sphere {
        Sphere { groups: SphereGroups { g0: self.group0() * other.group0(), g1: Simd32x4::from(self.group0()) * other.group1() } }
    }
}

impl InnerProduct<Sphere> for Scalar {
    type Output = Sphere;

    fn inner_product(self, other: Sphere) -> Sphere {
        Sphere { groups: SphereGroups { g0: self.group0() * other.group0(), g1: Simd32x4::from(self.group0()) * other.group1() } }
    }
}

impl LeftContraction<Sphere> for Scalar {
    type Output = Sphere;

    fn left_contraction(self, other: Sphere) -> Sphere {
        Sphere { groups: SphereGroups { g0: self.group0() * other.group0(), g1: Simd32x4::from(self.group0()) * other.group1() } }
    }
}

impl GeometricProduct<Motor> for Scalar {
    type Output = Motor;

    fn geometric_product(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()) * other.group0(), g1: Simd32x4::from(self.group0()) * other.group1() } }
    }
}

impl OuterProduct<Motor> for Scalar {
    type Output = Motor;

    fn outer_product(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()) * other.group0(), g1: Simd32x4::from(self.group0()) * other.group1() } }
    }
}

impl InnerProduct<Motor> for Scalar {
    type Output = Motor;

    fn inner_product(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()) * other.group0(), g1: Simd32x4::from(self.group0()) * other.group1() } }
    }
}

impl LeftContraction<Motor> for Scalar {
    type Output = Motor;

    fn left_contraction(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()) * other.group0(), g1: Simd32x4::from(self.group0()) * other.group1() } }
    }
}

impl GeometricProduct<Rotor> for Scalar {
    type Output = Rotor;

    fn geometric_product(self, other: Rotor) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group0()) * other.group0() } }
    }
}

impl OuterProduct<Rotor> for Scalar {
    type Output = Rotor;

    fn outer_product(self, other: Rotor) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group0()) * other.group0() } }
    }
}

impl InnerProduct<Rotor> for Scalar {
    type Output = Rotor;

    fn inner_product(self, other: Rotor) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group0()) * other.group0() } }
    }
}

impl LeftContraction<Rotor> for Scalar {
    type Output = Rotor;

    fn left_contraction(self, other: Rotor) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group0()) * other.group0() } }
    }
}

impl GeometricProduct<Translator> for Scalar {
    type Output = Translator;

    fn geometric_product(self, other: Translator) -> Translator {
        Translator { groups: TranslatorGroups { g0: Simd32x4::from(self.group0()) * other.group0() } }
    }
}

impl OuterProduct<Translator> for Scalar {
    type Output = Translator;

    fn outer_product(self, other: Translator) -> Translator {
        Translator { groups: TranslatorGroups { g0: Simd32x4::from(self.group0()) * other.group0() } }
    }
}

impl InnerProduct<Translator> for Scalar {
    type Output = Translator;

    fn inner_product(self, other: Translator) -> Translator {
        Translator { groups: TranslatorGroups { g0: Simd32x4::from(self.group0()) * other.group0() } }
    }
}

impl LeftContraction<Translator> for Scalar {
    type Output = Translator;

    fn left_contraction(self, other: Translator) -> Translator {
        Translator { groups: TranslatorGroups { g0: Simd32x4::from(self.group0()) * other.group0() } }
    }
}

impl GeometricProduct<Flector> for Scalar {
    type Output = Flector;

    fn geometric_product(self, other: Flector) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()) * other.group0(), g1: Simd32x4::from(self.group0()) * other.group1() } }
    }
}

impl OuterProduct<Flector> for Scalar {
    type Output = Flector;

    fn outer_product(self, other: Flector) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()) * other.group0(), g1: Simd32x4::from(self.group0()) * other.group1() } }
    }
}

impl InnerProduct<Flector> for Scalar {
    type Output = Flector;

    fn inner_product(self, other: Flector) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()) * other.group0(), g1: Simd32x4::from(self.group0()) * other.group1() } }
    }
}

impl LeftContraction<Flector> for Scalar {
    type Output = Flector;

    fn left_contraction(self, other: Flector) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()) * other.group0(), g1: Simd32x4::from(self.group0()) * other.group1() } }
    }
}

impl Add<MultiVector> for Scalar {
    type Output = MultiVector;

    fn add(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x3::from(self.group0()) * Simd32x3::from([1.0, 0.0, 0.0]) + other.group0(), g1: other.group1(), g2: other.group2(), g3: other.group3(), g4: other.group4(), g5: other.group5(), g6: other.group6(), g7: other.group7(), g8: other.group8(), g9: other.group9() } }
    }
}

impl Sub<MultiVector> for Scalar {
    type Output = MultiVector;

    fn sub(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x3::from(self.group0()) * Simd32x3::from([1.0, 0.0, 0.0]) - other.group0(), g1: Simd32x3::from(0.0) - other.group1(), g2: Simd32x2::from(0.0) - other.group2(), g3: Simd32x4::from(0.0) - other.group3(), g4: Simd32x3::from(0.0) - other.group4(), g5: Simd32x3::from(0.0) - other.group5(), g6: Simd32x3::from(0.0) - other.group6(), g7: Simd32x3::from(0.0) - other.group7(), g8: Simd32x4::from(0.0) - other.group8(), g9: Simd32x4::from(0.0) - other.group9() } }
    }
}

impl GeometricProduct<MultiVector> for Scalar {
    type Output = MultiVector;

    fn geometric_product(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x3::from(self.group0()) * other.group0(), g1: Simd32x3::from(self.group0()) * other.group1(), g2: Simd32x2::from(self.group0()) * other.group2(), g3: Simd32x4::from(self.group0()) * other.group3(), g4: Simd32x3::from(self.group0()) * other.group4(), g5: Simd32x3::from(self.group0()) * other.group5(), g6: Simd32x3::from(self.group0()) * other.group6(), g7: Simd32x3::from(self.group0()) * other.group7(), g8: Simd32x4::from(self.group0()) * other.group8(), g9: Simd32x4::from(self.group0()) * other.group9() } }
    }
}

impl OuterProduct<MultiVector> for Scalar {
    type Output = MultiVector;

    fn outer_product(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x3::from(self.group0()) * other.group0(), g1: Simd32x3::from(self.group0()) * other.group1(), g2: Simd32x2::from(self.group0()) * other.group2(), g3: Simd32x4::from(self.group0()) * other.group3(), g4: Simd32x3::from(self.group0()) * other.group4(), g5: Simd32x3::from(self.group0()) * other.group5(), g6: Simd32x3::from(self.group0()) * other.group6(), g7: Simd32x3::from(self.group0()) * other.group7(), g8: Simd32x4::from(self.group0()) * other.group8(), g9: Simd32x4::from(self.group0()) * other.group9() } }
    }
}

impl InnerProduct<MultiVector> for Scalar {
    type Output = MultiVector;

    fn inner_product(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x3::from(self.group0()) * other.group0(), g1: Simd32x3::from(self.group0()) * other.group1(), g2: Simd32x2::from(self.group0()) * other.group2(), g3: Simd32x4::from(self.group0()) * other.group3(), g4: Simd32x3::from(self.group0()) * other.group4(), g5: Simd32x3::from(self.group0()) * other.group5(), g6: Simd32x3::from(self.group0()) * other.group6(), g7: Simd32x3::from(self.group0()) * other.group7(), g8: Simd32x4::from(self.group0()) * other.group8(), g9: Simd32x4::from(self.group0()) * other.group9() } }
    }
}

impl LeftContraction<MultiVector> for Scalar {
    type Output = MultiVector;

    fn left_contraction(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x3::from(self.group0()) * other.group0(), g1: Simd32x3::from(self.group0()) * other.group1(), g2: Simd32x2::from(self.group0()) * other.group2(), g3: Simd32x4::from(self.group0()) * other.group3(), g4: Simd32x3::from(self.group0()) * other.group4(), g5: Simd32x3::from(self.group0()) * other.group5(), g6: Simd32x3::from(self.group0()) * other.group6(), g7: Simd32x3::from(self.group0()) * other.group7(), g8: Simd32x4::from(self.group0()) * other.group8(), g9: Simd32x4::from(self.group0()) * other.group9() } }
    }
}

impl RightContraction<MultiVector> for Scalar {
    type Output = Scalar;

    fn right_contraction(self, other: MultiVector) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0() * other.group0()[0] } }
    }
}

impl ScalarProduct<MultiVector> for Scalar {
    type Output = Scalar;

    fn scalar_product(self, other: MultiVector) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0() * other.group0()[0] } }
    }
}

impl SquaredMagnitude for Scalar {
    type Output = Scalar;

    fn squared_magnitude(self) -> Scalar {
        self.scalar_product(self.reversal())
    }
}

impl Magnitude for Scalar {
    type Output = Scalar;

    fn magnitude(self) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.squared_magnitude().group0().sqrt() } }
    }
}

impl Scale for Scalar {
    type Output = Scalar;

    fn scale(self, other: f32) -> Scalar {
        self.geometric_product(Scalar { groups: ScalarGroups { g0: other } })
    }
}

impl Signum for Scalar {
    type Output = Scalar;

    fn signum(self) -> Scalar {
        self.geometric_product(Scalar { groups: ScalarGroups { g0: 1.0 / self.magnitude().group0() } })
    }
}

impl Inverse for Scalar {
    type Output = Scalar;

    fn inverse(self) -> Scalar {
        self.reversal().geometric_product(Scalar { groups: ScalarGroups { g0: 1.0 / self.squared_magnitude().group0() } })
    }
}

impl Zero for AntiScalar {
    fn zero() -> Self {
        AntiScalar { groups: AntiScalarGroups { g0: 0.0 } }
    }
}

impl One for AntiScalar {
    fn one() -> Self {
        AntiScalar { groups: AntiScalarGroups { g0: 0.0 } }
    }
}

impl Neg for AntiScalar {
    type Output = AntiScalar;

    fn neg(self) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0() * -1.0 } }
    }
}

impl Automorphism for AntiScalar {
    type Output = AntiScalar;

    fn automorphism(self) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0() * -1.0 } }
    }
}

impl Reversal for AntiScalar {
    type Output = AntiScalar;

    fn reversal(self) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0() } }
    }
}

impl Conjugation for AntiScalar {
    type Output = AntiScalar;

    fn conjugation(self) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0() * -1.0 } }
    }
}

impl GeometricProduct<Scalar> for AntiScalar {
    type Output = AntiScalar;

    fn geometric_product(self, other: Scalar) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0() * other.group0() } }
    }
}

impl OuterProduct<Scalar> for AntiScalar {
    type Output = AntiScalar;

    fn outer_product(self, other: Scalar) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0() * other.group0() } }
    }
}

impl InnerProduct<Scalar> for AntiScalar {
    type Output = AntiScalar;

    fn inner_product(self, other: Scalar) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0() * other.group0() } }
    }
}

impl RightContraction<Scalar> for AntiScalar {
    type Output = AntiScalar;

    fn right_contraction(self, other: Scalar) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0() * other.group0() } }
    }
}

impl Add<AntiScalar> for AntiScalar {
    type Output = AntiScalar;

    fn add(self, other: AntiScalar) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0() + other.group0() } }
    }
}

impl AddAssign<AntiScalar> for AntiScalar {
    fn add_assign(&mut self, other: AntiScalar) {
        *self = (*self).add(other);
    }
}

impl Sub<AntiScalar> for AntiScalar {
    type Output = AntiScalar;

    fn sub(self, other: AntiScalar) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0() - other.group0() } }
    }
}

impl SubAssign<AntiScalar> for AntiScalar {
    fn sub_assign(&mut self, other: AntiScalar) {
        *self = (*self).sub(other);
    }
}

impl Mul<AntiScalar> for AntiScalar {
    type Output = AntiScalar;

    fn mul(self, other: AntiScalar) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0() * other.group0() } }
    }
}

impl MulAssign<AntiScalar> for AntiScalar {
    fn mul_assign(&mut self, other: AntiScalar) {
        *self = (*self).mul(other);
    }
}

impl Div<AntiScalar> for AntiScalar {
    type Output = AntiScalar;

    fn div(self, other: AntiScalar) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0() * 1.0 / other.group0() * 1.0 } }
    }
}

impl DivAssign<AntiScalar> for AntiScalar {
    fn div_assign(&mut self, other: AntiScalar) {
        *self = (*self).div(other);
    }
}

impl Add<Motor> for AntiScalar {
    type Output = Motor;

    fn add(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + other.group0(), g1: other.group1() } }
    }
}

impl Sub<Motor> for AntiScalar {
    type Output = Motor;

    fn sub(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) - other.group0(), g1: Simd32x4::from(0.0) - other.group1() } }
    }
}

impl Add<Rotor> for AntiScalar {
    type Output = Rotor;

    fn add(self, other: Rotor) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group0()) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + other.group0() } }
    }
}

impl Sub<Rotor> for AntiScalar {
    type Output = Rotor;

    fn sub(self, other: Rotor) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group0()) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) - other.group0() } }
    }
}

impl Add<Translator> for AntiScalar {
    type Output = Translator;

    fn add(self, other: Translator) -> Translator {
        Translator { groups: TranslatorGroups { g0: Simd32x4::from(self.group0()) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + other.group0() } }
    }
}

impl Sub<Translator> for AntiScalar {
    type Output = Translator;

    fn sub(self, other: Translator) -> Translator {
        Translator { groups: TranslatorGroups { g0: Simd32x4::from(self.group0()) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) - other.group0() } }
    }
}

impl Add<MultiVector> for AntiScalar {
    type Output = MultiVector;

    fn add(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x3::from(self.group0()) * Simd32x3::from([0.0, 0.0, 1.0]) + other.group0(), g1: other.group1(), g2: other.group2(), g3: other.group3(), g4: other.group4(), g5: other.group5(), g6: other.group6(), g7: other.group7(), g8: other.group8(), g9: other.group9() } }
    }
}

impl Sub<MultiVector> for AntiScalar {
    type Output = MultiVector;

    fn sub(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x3::from(self.group0()) * Simd32x3::from([0.0, 0.0, 1.0]) - other.group0(), g1: Simd32x3::from(0.0) - other.group1(), g2: Simd32x2::from(0.0) - other.group2(), g3: Simd32x4::from(0.0) - other.group3(), g4: Simd32x3::from(0.0) - other.group4(), g5: Simd32x3::from(0.0) - other.group5(), g6: Simd32x3::from(0.0) - other.group6(), g7: Simd32x3::from(0.0) - other.group7(), g8: Simd32x4::from(0.0) - other.group8(), g9: Simd32x4::from(0.0) - other.group9() } }
    }
}

impl OuterProduct<MultiVector> for AntiScalar {
    type Output = AntiScalar;

    fn outer_product(self, other: MultiVector) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0() * other.group0()[0] } }
    }
}

impl Scale for AntiScalar {
    type Output = AntiScalar;

    fn scale(self, other: f32) -> AntiScalar {
        self.geometric_product(Scalar { groups: ScalarGroups { g0: other } })
    }
}

impl Zero for RadialPoint {
    fn zero() -> Self {
        RadialPoint { groups: RadialPointGroups { g0: Simd32x3::from(0.0), g1: Simd32x2::from(0.0) } }
    }
}

impl One for RadialPoint {
    fn one() -> Self {
        RadialPoint { groups: RadialPointGroups { g0: Simd32x3::from(0.0), g1: Simd32x2::from(0.0) } }
    }
}

impl Neg for RadialPoint {
    type Output = RadialPoint;

    fn neg(self) -> RadialPoint {
        RadialPoint { groups: RadialPointGroups { g0: self.group0() * Simd32x3::from(-1.0), g1: self.group1() * Simd32x2::from(-1.0) } }
    }
}

impl Automorphism for RadialPoint {
    type Output = RadialPoint;

    fn automorphism(self) -> RadialPoint {
        RadialPoint { groups: RadialPointGroups { g0: self.group0() * Simd32x3::from(-1.0), g1: self.group1() * Simd32x2::from(-1.0) } }
    }
}

impl Reversal for RadialPoint {
    type Output = RadialPoint;

    fn reversal(self) -> RadialPoint {
        RadialPoint { groups: RadialPointGroups { g0: self.group0(), g1: self.group1() } }
    }
}

impl Conjugation for RadialPoint {
    type Output = RadialPoint;

    fn conjugation(self) -> RadialPoint {
        RadialPoint { groups: RadialPointGroups { g0: self.group0() * Simd32x3::from(-1.0), g1: self.group1() * Simd32x2::from(-1.0) } }
    }
}

impl GeometricProduct<Scalar> for RadialPoint {
    type Output = RadialPoint;

    fn geometric_product(self, other: Scalar) -> RadialPoint {
        RadialPoint { groups: RadialPointGroups { g0: self.group0() * Simd32x3::from(other.group0()), g1: self.group1() * Simd32x2::from(other.group0()) } }
    }
}

impl OuterProduct<Scalar> for RadialPoint {
    type Output = RadialPoint;

    fn outer_product(self, other: Scalar) -> RadialPoint {
        RadialPoint { groups: RadialPointGroups { g0: self.group0() * Simd32x3::from(other.group0()), g1: self.group1() * Simd32x2::from(other.group0()) } }
    }
}

impl InnerProduct<Scalar> for RadialPoint {
    type Output = RadialPoint;

    fn inner_product(self, other: Scalar) -> RadialPoint {
        RadialPoint { groups: RadialPointGroups { g0: self.group0() * Simd32x3::from(other.group0()), g1: self.group1() * Simd32x2::from(other.group0()) } }
    }
}

impl RightContraction<Scalar> for RadialPoint {
    type Output = RadialPoint;

    fn right_contraction(self, other: Scalar) -> RadialPoint {
        RadialPoint { groups: RadialPointGroups { g0: self.group0() * Simd32x3::from(other.group0()), g1: self.group1() * Simd32x2::from(other.group0()) } }
    }
}

impl Add<RadialPoint> for RadialPoint {
    type Output = RadialPoint;

    fn add(self, other: RadialPoint) -> RadialPoint {
        RadialPoint { groups: RadialPointGroups { g0: self.group0() + other.group0(), g1: self.group1() + other.group1() } }
    }
}

impl AddAssign<RadialPoint> for RadialPoint {
    fn add_assign(&mut self, other: RadialPoint) {
        *self = (*self).add(other);
    }
}

impl Sub<RadialPoint> for RadialPoint {
    type Output = RadialPoint;

    fn sub(self, other: RadialPoint) -> RadialPoint {
        RadialPoint { groups: RadialPointGroups { g0: self.group0() - other.group0(), g1: self.group1() - other.group1() } }
    }
}

impl SubAssign<RadialPoint> for RadialPoint {
    fn sub_assign(&mut self, other: RadialPoint) {
        *self = (*self).sub(other);
    }
}

impl Mul<RadialPoint> for RadialPoint {
    type Output = RadialPoint;

    fn mul(self, other: RadialPoint) -> RadialPoint {
        RadialPoint { groups: RadialPointGroups { g0: self.group0() * other.group0(), g1: self.group1() * other.group1() } }
    }
}

impl MulAssign<RadialPoint> for RadialPoint {
    fn mul_assign(&mut self, other: RadialPoint) {
        *self = (*self).mul(other);
    }
}

impl Div<RadialPoint> for RadialPoint {
    type Output = RadialPoint;

    fn div(self, other: RadialPoint) -> RadialPoint {
        RadialPoint { groups: RadialPointGroups { g0: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from([1.0, 1.0, 1.0]) / Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from([1.0, 1.0, 1.0]), g1: Simd32x2::from([self.group1()[0], self.group1()[1]]) * Simd32x2::from([1.0, 1.0]) / Simd32x2::from([other.group1()[0], other.group1()[1]]) * Simd32x2::from([1.0, 1.0]) } }
    }
}

impl DivAssign<RadialPoint> for RadialPoint {
    fn div_assign(&mut self, other: RadialPoint) {
        *self = (*self).div(other);
    }
}

impl OuterProduct<RadialPoint> for RadialPoint {
    type Output = Dipole;

    fn outer_product(self, other: RadialPoint) -> Dipole {
        Dipole { groups: DipoleGroups { g0: Simd32x3::from(self.group1()[0]) * other.group0() + self.group0() * Simd32x3::from(other.group1()[0]) * Simd32x3::from(-1.0), g1: Simd32x3::from(self.group0()[1]) * swizzle!(other.group0(), 2, 2, 0) * Simd32x3::from([1.0, 0.0, -1.0]) + Simd32x3::from(self.group0()[2]) * swizzle!(other.group0(), 1, 0, 1) * Simd32x3::from([-1.0, 1.0, 0.0]) + Simd32x3::from(self.group0()[0]) * swizzle!(other.group0(), 0, 2, 1) * Simd32x3::from([0.0, -1.0, 1.0]), g2: Simd32x4::from(self.group1()[0]) * Simd32x4::from(other.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) - Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[0]]) + Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group0()[0]]) * Simd32x4::from([other.group1()[1], other.group1()[1], other.group1()[1], other.group1()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) } }
    }
}

impl InnerProduct<RadialPoint> for RadialPoint {
    type Output = Scalar;

    fn inner_product(self, other: RadialPoint) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2] } }
    }
}

impl LeftContraction<RadialPoint> for RadialPoint {
    type Output = Scalar;

    fn left_contraction(self, other: RadialPoint) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2] } }
    }
}

impl RightContraction<RadialPoint> for RadialPoint {
    type Output = Scalar;

    fn right_contraction(self, other: RadialPoint) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2] } }
    }
}

impl ScalarProduct<RadialPoint> for RadialPoint {
    type Output = Scalar;

    fn scalar_product(self, other: RadialPoint) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2] } }
    }
}

impl OuterProduct<FlatPoint> for RadialPoint {
    type Output = Line;

    fn outer_product(self, other: FlatPoint) -> Line {
        Line { groups: LineGroups { g0: Simd32x3::from(self.group1()[0]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) + self.group0() * Simd32x3::from(other.group0()[3]) * Simd32x3::from(-1.0), g1: Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], other.group0()[2], other.group0()[0]]) * Simd32x3::from([1.0, 0.0, -1.0]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], other.group0()[0], other.group0()[1]]) * Simd32x3::from([-1.0, 1.0, 0.0]) + Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[0], other.group0()[2], other.group0()[1]]) * Simd32x3::from([0.0, -1.0, 1.0]) } }
    }
}

impl OuterProduct<Dipole> for RadialPoint {
    type Output = Circle;

    fn outer_product(self, other: Dipole) -> Circle {
        Circle { groups: CircleGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], other.group0()[2], other.group0()[0], other.group1()[1]]) * Simd32x4::from([-1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], other.group0()[0], other.group0()[1], other.group1()[2]]) * Simd32x4::from([1.0, -1.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group1()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[0], other.group0()[2], other.group0()[1], other.group1()[0]]) * Simd32x4::from([0.0, 1.0, -1.0, -1.0]), g1: Simd32x3::from(self.group1()[0]) * Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]) + Simd32x3::from(self.group1()[1]) * other.group0() + self.group0() * Simd32x3::from(other.group2()[3]) * Simd32x3::from(-1.0), g2: Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group2()[2], other.group2()[2], other.group2()[0]]) * Simd32x3::from([1.0, 0.0, -1.0]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group2()[1], other.group2()[0], other.group2()[1]]) * Simd32x3::from([-1.0, 1.0, 0.0]) + Simd32x3::from(self.group1()[1]) * other.group1() + Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group2()[0], other.group2()[2], other.group2()[1]]) * Simd32x3::from([0.0, -1.0, 1.0]) } }
    }
}

impl InnerProduct<Dipole> for RadialPoint {
    type Output = RadialPoint;

    fn inner_product(self, other: Dipole) -> RadialPoint {
        RadialPoint { groups: RadialPointGroups { g0: Simd32x3::from(self.group0()[1]) * swizzle!(other.group1(), 2, 2, 0) * Simd32x3::from([-1.0, 0.0, 1.0]) + Simd32x3::from(self.group0()[2]) * swizzle!(other.group1(), 1, 0, 1) * Simd32x3::from([1.0, -1.0, 0.0]) + Simd32x3::from(self.group0()[0]) * swizzle!(other.group1(), 0, 2, 1) * Simd32x3::from([0.0, 1.0, -1.0]), g1: Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group0()[0], other.group2()[0]]) * Simd32x2::from([-1.0, 1.0]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group0()[1], other.group2()[1]]) * Simd32x2::from([-1.0, 1.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group0()[2], other.group2()[2]]) * Simd32x2::from([-1.0, 1.0]) } }
    }
}

impl LeftContraction<Dipole> for RadialPoint {
    type Output = RadialPoint;

    fn left_contraction(self, other: Dipole) -> RadialPoint {
        RadialPoint { groups: RadialPointGroups { g0: Simd32x3::from(self.group0()[1]) * swizzle!(other.group1(), 2, 2, 0) * Simd32x3::from([-1.0, 0.0, 1.0]) + Simd32x3::from(self.group0()[2]) * swizzle!(other.group1(), 1, 0, 1) * Simd32x3::from([1.0, -1.0, 0.0]) + Simd32x3::from(self.group0()[0]) * swizzle!(other.group1(), 0, 2, 1) * Simd32x3::from([0.0, 1.0, -1.0]), g1: Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group0()[0], other.group2()[0]]) * Simd32x2::from([-1.0, 1.0]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group0()[1], other.group2()[1]]) * Simd32x2::from([-1.0, 1.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group0()[2], other.group2()[2]]) * Simd32x2::from([-1.0, 1.0]) } }
    }
}

impl GeometricProduct<Line> for RadialPoint {
    type Output = Flector;

    fn geometric_product(self, other: Line) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group1()[2], other.group1()[2], other.group1()[0], other.group0()[1]]) * Simd32x4::from([-1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[1], other.group1()[0], other.group1()[1], other.group0()[2]]) * Simd32x4::from([1.0, -1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group1()[0], other.group1()[2], other.group1()[1], other.group0()[0]]) * Simd32x4::from([0.0, 1.0, -1.0, -1.0]), g1: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], other.group0()[2], other.group0()[0], other.group1()[1]]) * Simd32x4::from([-1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], other.group0()[0], other.group0()[1], other.group1()[2]]) * Simd32x4::from([1.0, -1.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group1()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[0], other.group0()[2], other.group0()[1], other.group1()[0]]) * Simd32x4::from([0.0, 1.0, -1.0, -1.0]) } }
    }
}

impl OuterProduct<Line> for RadialPoint {
    type Output = Plane;

    fn outer_product(self, other: Line) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], other.group0()[2], other.group0()[0], other.group1()[1]]) * Simd32x4::from([-1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], other.group0()[0], other.group0()[1], other.group1()[2]]) * Simd32x4::from([1.0, -1.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group1()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[0], other.group0()[2], other.group0()[1], other.group1()[0]]) * Simd32x4::from([0.0, 1.0, -1.0, -1.0]) } }
    }
}

impl InnerProduct<Line> for RadialPoint {
    type Output = FlatPoint;

    fn inner_product(self, other: Line) -> FlatPoint {
        FlatPoint { groups: FlatPointGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group1()[2], other.group1()[2], other.group1()[0], other.group0()[1]]) * Simd32x4::from([-1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[1], other.group1()[0], other.group1()[1], other.group0()[2]]) * Simd32x4::from([1.0, -1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group1()[0], other.group1()[2], other.group1()[1], other.group0()[0]]) * Simd32x4::from([0.0, 1.0, -1.0, -1.0]) } }
    }
}

impl LeftContraction<Line> for RadialPoint {
    type Output = FlatPoint;

    fn left_contraction(self, other: Line) -> FlatPoint {
        FlatPoint { groups: FlatPointGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group1()[2], other.group1()[2], other.group1()[0], other.group0()[1]]) * Simd32x4::from([-1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[1], other.group1()[0], other.group1()[1], other.group0()[2]]) * Simd32x4::from([1.0, -1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group1()[0], other.group1()[2], other.group1()[1], other.group0()[0]]) * Simd32x4::from([0.0, 1.0, -1.0, -1.0]) } }
    }
}

impl OuterProduct<Circle> for RadialPoint {
    type Output = Sphere;

    fn outer_product(self, other: Circle) -> Sphere {
        Sphere { groups: SphereGroups { g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2] + self.group1()[0] * other.group0()[3], g1: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group1()[2], other.group1()[2], other.group1()[0], other.group2()[1]]) * Simd32x4::from([-1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[1], other.group1()[0], other.group1()[1], other.group2()[2]]) * Simd32x4::from([1.0, -1.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], other.group2()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) - Simd32x4::from(self.group1()[1]) * other.group0() + Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group1()[0], other.group1()[2], other.group1()[1], other.group2()[0]]) * Simd32x4::from([0.0, 1.0, -1.0, -1.0]) } }
    }
}

impl InnerProduct<Circle> for RadialPoint {
    type Output = Dipole;

    fn inner_product(self, other: Circle) -> Dipole {
        Dipole { groups: DipoleGroups { g0: Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], other.group0()[2], other.group0()[0]]) * Simd32x3::from([1.0, 0.0, -1.0]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], other.group0()[0], other.group0()[1]]) * Simd32x3::from([-1.0, 1.0, 0.0]) + Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[0], other.group0()[2], other.group0()[1]]) * Simd32x3::from([0.0, -1.0, 1.0]), g1: self.group0() * Simd32x3::from(other.group0()[3]) * Simd32x3::from(-1.0), g2: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group2()[2], other.group2()[2], other.group2()[0], other.group1()[1]]) * Simd32x4::from([-1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group2()[1], other.group2()[0], other.group2()[1], other.group1()[2]]) * Simd32x4::from([1.0, -1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group2()[0], other.group2()[2], other.group2()[1], other.group1()[0]]) * Simd32x4::from([0.0, 1.0, -1.0, -1.0]) } }
    }
}

impl LeftContraction<Circle> for RadialPoint {
    type Output = Dipole;

    fn left_contraction(self, other: Circle) -> Dipole {
        Dipole { groups: DipoleGroups { g0: Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], other.group0()[2], other.group0()[0]]) * Simd32x3::from([1.0, 0.0, -1.0]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], other.group0()[0], other.group0()[1]]) * Simd32x3::from([-1.0, 1.0, 0.0]) + Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[0], other.group0()[2], other.group0()[1]]) * Simd32x3::from([0.0, -1.0, 1.0]), g1: self.group0() * Simd32x3::from(other.group0()[3]) * Simd32x3::from(-1.0), g2: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group2()[2], other.group2()[2], other.group2()[0], other.group1()[1]]) * Simd32x4::from([-1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group2()[1], other.group2()[0], other.group2()[1], other.group1()[2]]) * Simd32x4::from([1.0, -1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group2()[0], other.group2()[2], other.group2()[1], other.group1()[0]]) * Simd32x4::from([0.0, 1.0, -1.0, -1.0]) } }
    }
}

impl OuterProduct<Plane> for RadialPoint {
    type Output = AntiScalar;

    fn outer_product(self, other: Plane) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2] + self.group1()[0] * other.group0()[3] } }
    }
}

impl InnerProduct<Plane> for RadialPoint {
    type Output = Line;

    fn inner_product(self, other: Plane) -> Line {
        Line { groups: LineGroups { g0: Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], other.group0()[2], other.group0()[0]]) * Simd32x3::from([1.0, 0.0, -1.0]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], other.group0()[0], other.group0()[1]]) * Simd32x3::from([-1.0, 1.0, 0.0]) + Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[0], other.group0()[2], other.group0()[1]]) * Simd32x3::from([0.0, -1.0, 1.0]), g1: self.group0() * Simd32x3::from(other.group0()[3]) * Simd32x3::from(-1.0) } }
    }
}

impl LeftContraction<Plane> for RadialPoint {
    type Output = Line;

    fn left_contraction(self, other: Plane) -> Line {
        Line { groups: LineGroups { g0: Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], other.group0()[2], other.group0()[0]]) * Simd32x3::from([1.0, 0.0, -1.0]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], other.group0()[0], other.group0()[1]]) * Simd32x3::from([-1.0, 1.0, 0.0]) + Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[0], other.group0()[2], other.group0()[1]]) * Simd32x3::from([0.0, -1.0, 1.0]), g1: self.group0() * Simd32x3::from(other.group0()[3]) * Simd32x3::from(-1.0) } }
    }
}

impl OuterProduct<Sphere> for RadialPoint {
    type Output = AntiScalar;

    fn outer_product(self, other: Sphere) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0()[0] * other.group1()[0] + self.group0()[1] * other.group1()[1] + self.group0()[2] * other.group1()[2] + self.group1()[0] * other.group1()[3] + self.group1()[1] * other.group0() } }
    }
}

impl GeometricProduct<Motor> for RadialPoint {
    type Output = Flector;

    fn geometric_product(self, other: Motor) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group1()[3], other.group1()[2], other.group1()[1], other.group0()[0]]) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group1()[2], other.group1()[3], other.group1()[0], other.group0()[1]]) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[1], other.group1()[0], other.group1()[3], other.group0()[2]]) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[0]) * swizzle!(other.group1(), 0, 0, 0, 3) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[3], other.group0()[2], other.group0()[1], other.group1()[0]]) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], other.group0()[3], other.group0()[0], other.group1()[1]]) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], other.group0()[0], other.group0()[3], other.group1()[2]]) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[0]) * swizzle!(other.group1(), 0, 1, 2, 0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) } }
    }
}

impl OuterProduct<Motor> for RadialPoint {
    type Output = Flector;

    fn outer_product(self, other: Motor) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group1()[0]) * Simd32x4::from(other.group1()[3]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group0()[0]]) * swizzle!(other.group1(), 3, 3, 3, 0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]), g1: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], other.group0()[2], other.group0()[0], other.group1()[1]]) * Simd32x4::from([-1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], other.group0()[0], other.group0()[1], other.group1()[2]]) * Simd32x4::from([1.0, -1.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[0]) * swizzle!(other.group1(), 0, 1, 2, 0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[0], other.group0()[2], other.group0()[1], other.group1()[0]]) * Simd32x4::from([0.0, 1.0, -1.0, -1.0]) } }
    }
}

impl OuterProduct<Translator> for RadialPoint {
    type Output = Plane;

    fn outer_product(self, other: Translator) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[0]) * swizzle!(other.group0(), 0, 1, 2, 0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) } }
    }
}

impl GeometricProduct<Flector> for RadialPoint {
    type Output = Motor;

    fn geometric_product(self, other: Flector) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[3], other.group1()[2], other.group1()[1], other.group1()[0]]) * Simd32x4::from([-1.0, -1.0, 1.0, 1.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group1()[2], other.group0()[3], other.group1()[0], other.group1()[1]]) * Simd32x4::from([1.0, -1.0, -1.0, 1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[1], other.group1()[0], other.group0()[3], other.group1()[2]]) * Simd32x4::from([-1.0, 1.0, -1.0, 1.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group1()[3], other.group0()[2], other.group0()[1], other.group0()[0]]) * Simd32x4::from([-1.0, -1.0, 1.0, 1.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], other.group1()[3], other.group0()[0], other.group0()[1]]) * Simd32x4::from([1.0, -1.0, -1.0, 1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], other.group0()[0], other.group1()[3], other.group0()[2]]) * Simd32x4::from([-1.0, 1.0, -1.0, 1.0]) } }
    }
}

impl Add<MultiVector> for RadialPoint {
    type Output = MultiVector;

    fn add(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: other.group0(), g1: self.group0() + other.group1(), g2: self.group1() + other.group2(), g3: other.group3(), g4: other.group4(), g5: other.group5(), g6: other.group6(), g7: other.group7(), g8: other.group8(), g9: other.group9() } }
    }
}

impl Sub<MultiVector> for RadialPoint {
    type Output = MultiVector;

    fn sub(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x3::from(0.0) - other.group0(), g1: self.group0() - other.group1(), g2: self.group1() - other.group2(), g3: Simd32x4::from(0.0) - other.group3(), g4: Simd32x3::from(0.0) - other.group4(), g5: Simd32x3::from(0.0) - other.group5(), g6: Simd32x3::from(0.0) - other.group6(), g7: Simd32x3::from(0.0) - other.group7(), g8: Simd32x4::from(0.0) - other.group8(), g9: Simd32x4::from(0.0) - other.group9() } }
    }
}

impl GeometricProduct<MultiVector> for RadialPoint {
    type Output = MultiVector;

    fn geometric_product(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group1()[0], other.group8()[0], other.group9()[0]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group1()[1], other.group8()[1], other.group9()[1]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group1()[2], other.group8()[2], other.group9()[2]]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from(other.group0()[1]) * Simd32x3::from([0.0, 0.0, 1.0]) + Simd32x3::from(self.group1()[0]) * Simd32x3::from([other.group8()[0], other.group8()[3], other.group9()[3]]) * Simd32x3::from([0.0, 1.0, 1.0]), g1: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[0], other.group5()[2], other.group5()[1]]) * Simd32x3::from([1.0, 1.0, -1.0]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group5()[2], other.group0()[0], other.group5()[0]]) * Simd32x3::from([-1.0, 1.0, 1.0]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group5()[1], other.group5()[0], other.group0()[0]]) * Simd32x3::from([1.0, -1.0, 1.0]), g2: Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group4()[0], other.group3()[0]]) * Simd32x2::from([-1.0, 1.0]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group4()[1], other.group3()[1]]) * Simd32x2::from([-1.0, 1.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group4()[2], other.group3()[2]]) * Simd32x2::from([-1.0, 1.0]) + self.group1() * Simd32x2::from(other.group0()[0]), g3: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group2()[1], other.group7()[2], other.group7()[1], other.group6()[0]]) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group7()[2], other.group2()[1], other.group7()[0], other.group6()[1]]) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group7()[1], other.group7()[0], other.group2()[1], other.group6()[2]]) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) - Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group2()[0]]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group2()[0], other.group2()[0], other.group2()[0], other.group2()[1]]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]), g4: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group2()[0], other.group8()[2], other.group8()[1]]) * Simd32x3::from([-1.0, -1.0, 1.0]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group8()[2], other.group2()[0], other.group8()[0]]) * Simd32x3::from([1.0, -1.0, -1.0]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group8()[1], other.group8()[0], other.group2()[0]]) * Simd32x3::from([-1.0, 1.0, -1.0]) + Simd32x3::from(self.group1()[0]) * other.group1(), g5: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group8()[3], other.group1()[2], other.group1()[1]]) * Simd32x3::from([-1.0, -1.0, 1.0]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group1()[2], other.group8()[3], other.group1()[0]]) * Simd32x3::from([1.0, -1.0, -1.0]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group1()[1], other.group1()[0], other.group8()[3]]) * Simd32x3::from([-1.0, 1.0, -1.0]), g6: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group3()[3], other.group9()[2], other.group9()[1]]) * Simd32x3::from([-1.0, -1.0, 1.0]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group9()[2], other.group3()[3], other.group9()[0]]) * Simd32x3::from([1.0, -1.0, -1.0]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group9()[1], other.group9()[0], other.group3()[3]]) * Simd32x3::from([-1.0, 1.0, -1.0]) + Simd32x3::from(self.group1()[0]) * Simd32x3::from([other.group3()[0], other.group3()[1], other.group3()[2]]) + Simd32x3::from(self.group1()[1]) * other.group4(), g7: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group9()[3], other.group3()[2], other.group3()[1]]) * Simd32x3::from([-1.0, -1.0, 1.0]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group3()[2], other.group9()[3], other.group3()[0]]) * Simd32x3::from([1.0, -1.0, -1.0]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group3()[1], other.group3()[0], other.group9()[3]]) * Simd32x3::from([-1.0, 1.0, -1.0]) + Simd32x3::from(self.group1()[1]) * other.group5(), g8: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[1], other.group4()[2], other.group4()[1], other.group5()[0]]) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group4()[2], other.group0()[1], other.group4()[0], other.group5()[1]]) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group4()[1], other.group4()[0], other.group0()[1], other.group5()[2]]) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group5()[0], other.group5()[1], other.group5()[2], other.group5()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]), g9: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[2], other.group6()[2], other.group6()[1], other.group7()[0]]) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group6()[2], other.group0()[2], other.group6()[0], other.group7()[1]]) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group6()[1], other.group6()[0], other.group0()[2], other.group7()[2]]) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) - Simd32x4::from(self.group1()[1]) * other.group8() + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group7()[0], other.group7()[1], other.group7()[2], other.group7()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) } }
    }
}

impl ScalarProduct<MultiVector> for RadialPoint {
    type Output = Scalar;

    fn scalar_product(self, other: MultiVector) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[0] * other.group1()[0] + self.group0()[1] * other.group1()[1] + self.group0()[2] * other.group1()[2] } }
    }
}

impl SquaredMagnitude for RadialPoint {
    type Output = Scalar;

    fn squared_magnitude(self) -> Scalar {
        self.scalar_product(self.reversal())
    }
}

impl Magnitude for RadialPoint {
    type Output = Scalar;

    fn magnitude(self) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.squared_magnitude().group0().sqrt() } }
    }
}

impl Scale for RadialPoint {
    type Output = RadialPoint;

    fn scale(self, other: f32) -> RadialPoint {
        self.geometric_product(Scalar { groups: ScalarGroups { g0: other } })
    }
}

impl Signum for RadialPoint {
    type Output = RadialPoint;

    fn signum(self) -> RadialPoint {
        self.geometric_product(Scalar { groups: ScalarGroups { g0: 1.0 / self.magnitude().group0() } })
    }
}

impl Inverse for RadialPoint {
    type Output = RadialPoint;

    fn inverse(self) -> RadialPoint {
        self.reversal().geometric_product(Scalar { groups: ScalarGroups { g0: 1.0 / self.squared_magnitude().group0() } })
    }
}

impl Zero for FlatPoint {
    fn zero() -> Self {
        FlatPoint { groups: FlatPointGroups { g0: Simd32x4::from(0.0) } }
    }
}

impl One for FlatPoint {
    fn one() -> Self {
        FlatPoint { groups: FlatPointGroups { g0: Simd32x4::from(0.0) } }
    }
}

impl Neg for FlatPoint {
    type Output = FlatPoint;

    fn neg(self) -> FlatPoint {
        FlatPoint { groups: FlatPointGroups { g0: self.group0() * Simd32x4::from(-1.0) } }
    }
}

impl Automorphism for FlatPoint {
    type Output = FlatPoint;

    fn automorphism(self) -> FlatPoint {
        FlatPoint { groups: FlatPointGroups { g0: self.group0() } }
    }
}

impl Reversal for FlatPoint {
    type Output = FlatPoint;

    fn reversal(self) -> FlatPoint {
        FlatPoint { groups: FlatPointGroups { g0: self.group0() * Simd32x4::from(-1.0) } }
    }
}

impl Conjugation for FlatPoint {
    type Output = FlatPoint;

    fn conjugation(self) -> FlatPoint {
        FlatPoint { groups: FlatPointGroups { g0: self.group0() * Simd32x4::from(-1.0) } }
    }
}

impl GeometricProduct<Scalar> for FlatPoint {
    type Output = FlatPoint;

    fn geometric_product(self, other: Scalar) -> FlatPoint {
        FlatPoint { groups: FlatPointGroups { g0: self.group0() * Simd32x4::from(other.group0()) } }
    }
}

impl OuterProduct<Scalar> for FlatPoint {
    type Output = FlatPoint;

    fn outer_product(self, other: Scalar) -> FlatPoint {
        FlatPoint { groups: FlatPointGroups { g0: self.group0() * Simd32x4::from(other.group0()) } }
    }
}

impl InnerProduct<Scalar> for FlatPoint {
    type Output = FlatPoint;

    fn inner_product(self, other: Scalar) -> FlatPoint {
        FlatPoint { groups: FlatPointGroups { g0: self.group0() * Simd32x4::from(other.group0()) } }
    }
}

impl RightContraction<Scalar> for FlatPoint {
    type Output = FlatPoint;

    fn right_contraction(self, other: Scalar) -> FlatPoint {
        FlatPoint { groups: FlatPointGroups { g0: self.group0() * Simd32x4::from(other.group0()) } }
    }
}

impl OuterProduct<RadialPoint> for FlatPoint {
    type Output = Line;

    fn outer_product(self, other: RadialPoint) -> Line {
        Line { groups: LineGroups { g0: Simd32x3::from(0.0) - Simd32x3::from(self.group0()[3]) * other.group0() + Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group1()[0]), g1: Simd32x3::from(self.group0()[1]) * swizzle!(other.group0(), 2, 2, 0) * Simd32x3::from([-1.0, 0.0, 1.0]) + Simd32x3::from(self.group0()[2]) * swizzle!(other.group0(), 1, 0, 1) * Simd32x3::from([1.0, -1.0, 0.0]) + Simd32x3::from(self.group0()[0]) * swizzle!(other.group0(), 0, 2, 1) * Simd32x3::from([0.0, 1.0, -1.0]) } }
    }
}

impl Add<FlatPoint> for FlatPoint {
    type Output = FlatPoint;

    fn add(self, other: FlatPoint) -> FlatPoint {
        FlatPoint { groups: FlatPointGroups { g0: self.group0() + other.group0() } }
    }
}

impl AddAssign<FlatPoint> for FlatPoint {
    fn add_assign(&mut self, other: FlatPoint) {
        *self = (*self).add(other);
    }
}

impl Sub<FlatPoint> for FlatPoint {
    type Output = FlatPoint;

    fn sub(self, other: FlatPoint) -> FlatPoint {
        FlatPoint { groups: FlatPointGroups { g0: self.group0() - other.group0() } }
    }
}

impl SubAssign<FlatPoint> for FlatPoint {
    fn sub_assign(&mut self, other: FlatPoint) {
        *self = (*self).sub(other);
    }
}

impl Mul<FlatPoint> for FlatPoint {
    type Output = FlatPoint;

    fn mul(self, other: FlatPoint) -> FlatPoint {
        FlatPoint { groups: FlatPointGroups { g0: self.group0() * other.group0() } }
    }
}

impl MulAssign<FlatPoint> for FlatPoint {
    fn mul_assign(&mut self, other: FlatPoint) {
        *self = (*self).mul(other);
    }
}

impl Div<FlatPoint> for FlatPoint {
    type Output = FlatPoint;

    fn div(self, other: FlatPoint) -> FlatPoint {
        FlatPoint { groups: FlatPointGroups { g0: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group0()[3]]) * Simd32x4::from([1.0, 1.0, 1.0, 1.0]) / Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group0()[3]]) * Simd32x4::from([1.0, 1.0, 1.0, 1.0]) } }
    }
}

impl DivAssign<FlatPoint> for FlatPoint {
    fn div_assign(&mut self, other: FlatPoint) {
        *self = (*self).div(other);
    }
}

impl Add<Dipole> for FlatPoint {
    type Output = Dipole;

    fn add(self, other: Dipole) -> Dipole {
        Dipole { groups: DipoleGroups { g0: other.group0(), g1: other.group1(), g2: self.group0() + other.group2() } }
    }
}

impl Sub<Dipole> for FlatPoint {
    type Output = Dipole;

    fn sub(self, other: Dipole) -> Dipole {
        Dipole { groups: DipoleGroups { g0: Simd32x3::from(0.0) - other.group0(), g1: Simd32x3::from(0.0) - other.group1(), g2: self.group0() - other.group2() } }
    }
}

impl GeometricProduct<Dipole> for FlatPoint {
    type Output = Flector;

    fn geometric_product(self, other: Dipole) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group1()[2], other.group1()[2], other.group1()[0], other.group0()[1]]) * Simd32x4::from([-1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[1], other.group1()[0], other.group1()[1], other.group0()[2]]) * Simd32x4::from([1.0, -1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group1()[0], other.group1()[2], other.group1()[1], other.group0()[0]]) * Simd32x4::from([0.0, 1.0, -1.0, -1.0]), g1: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], other.group0()[2], other.group0()[0], other.group1()[1]]) * Simd32x4::from([-1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], other.group0()[0], other.group0()[1], other.group1()[2]]) * Simd32x4::from([1.0, -1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group1()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[0], other.group0()[2], other.group0()[1], other.group1()[0]]) * Simd32x4::from([0.0, 1.0, -1.0, -1.0]) } }
    }
}

impl OuterProduct<Dipole> for FlatPoint {
    type Output = Plane;

    fn outer_product(self, other: Dipole) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], other.group0()[2], other.group0()[0], other.group1()[1]]) * Simd32x4::from([-1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], other.group0()[0], other.group0()[1], other.group1()[2]]) * Simd32x4::from([1.0, -1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group1()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[0], other.group0()[2], other.group0()[1], other.group1()[0]]) * Simd32x4::from([0.0, 1.0, -1.0, -1.0]) } }
    }
}

impl OuterProduct<Circle> for FlatPoint {
    type Output = AntiScalar;

    fn outer_product(self, other: Circle) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2] - self.group0()[3] * other.group0()[3] } }
    }
}

impl Add<Plane> for FlatPoint {
    type Output = Flector;

    fn add(self, other: Plane) -> Flector {
        Flector { groups: FlectorGroups { g0: self.group0(), g1: other.group0() } }
    }
}

impl Sub<Plane> for FlatPoint {
    type Output = Flector;

    fn sub(self, other: Plane) -> Flector {
        Flector { groups: FlectorGroups { g0: self.group0(), g1: Simd32x4::from(0.0) - other.group0() } }
    }
}

impl Add<Flector> for FlatPoint {
    type Output = Flector;

    fn add(self, other: Flector) -> Flector {
        Flector { groups: FlectorGroups { g0: self.group0() + other.group0(), g1: other.group1() } }
    }
}

impl Sub<Flector> for FlatPoint {
    type Output = Flector;

    fn sub(self, other: Flector) -> Flector {
        Flector { groups: FlectorGroups { g0: self.group0() - other.group0(), g1: Simd32x4::from(0.0) - other.group1() } }
    }
}

impl Add<MultiVector> for FlatPoint {
    type Output = MultiVector;

    fn add(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: other.group0(), g1: other.group1(), g2: other.group2(), g3: self.group0() + other.group3(), g4: other.group4(), g5: other.group5(), g6: other.group6(), g7: other.group7(), g8: other.group8(), g9: other.group9() } }
    }
}

impl Sub<MultiVector> for FlatPoint {
    type Output = MultiVector;

    fn sub(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x3::from(0.0) - other.group0(), g1: Simd32x3::from(0.0) - other.group1(), g2: Simd32x2::from(0.0) - other.group2(), g3: self.group0() - other.group3(), g4: Simd32x3::from(0.0) - other.group4(), g5: Simd32x3::from(0.0) - other.group5(), g6: Simd32x3::from(0.0) - other.group6(), g7: Simd32x3::from(0.0) - other.group7(), g8: Simd32x4::from(0.0) - other.group8(), g9: Simd32x4::from(0.0) - other.group9() } }
    }
}

impl Scale for FlatPoint {
    type Output = FlatPoint;

    fn scale(self, other: f32) -> FlatPoint {
        self.geometric_product(Scalar { groups: ScalarGroups { g0: other } })
    }
}

impl Zero for Dipole {
    fn zero() -> Self {
        Dipole { groups: DipoleGroups { g0: Simd32x3::from(0.0), g1: Simd32x3::from(0.0), g2: Simd32x4::from(0.0) } }
    }
}

impl One for Dipole {
    fn one() -> Self {
        Dipole { groups: DipoleGroups { g0: Simd32x3::from(0.0), g1: Simd32x3::from(0.0), g2: Simd32x4::from(0.0) } }
    }
}

impl Neg for Dipole {
    type Output = Dipole;

    fn neg(self) -> Dipole {
        Dipole { groups: DipoleGroups { g0: self.group0() * Simd32x3::from(-1.0), g1: self.group1() * Simd32x3::from(-1.0), g2: self.group2() * Simd32x4::from(-1.0) } }
    }
}

impl Automorphism for Dipole {
    type Output = Dipole;

    fn automorphism(self) -> Dipole {
        Dipole { groups: DipoleGroups { g0: self.group0(), g1: self.group1(), g2: self.group2() } }
    }
}

impl Reversal for Dipole {
    type Output = Dipole;

    fn reversal(self) -> Dipole {
        Dipole { groups: DipoleGroups { g0: self.group0() * Simd32x3::from(-1.0), g1: self.group1() * Simd32x3::from(-1.0), g2: self.group2() * Simd32x4::from(-1.0) } }
    }
}

impl Conjugation for Dipole {
    type Output = Dipole;

    fn conjugation(self) -> Dipole {
        Dipole { groups: DipoleGroups { g0: self.group0() * Simd32x3::from(-1.0), g1: self.group1() * Simd32x3::from(-1.0), g2: self.group2() * Simd32x4::from(-1.0) } }
    }
}

impl GeometricProduct<Scalar> for Dipole {
    type Output = Dipole;

    fn geometric_product(self, other: Scalar) -> Dipole {
        Dipole { groups: DipoleGroups { g0: self.group0() * Simd32x3::from(other.group0()), g1: self.group1() * Simd32x3::from(other.group0()), g2: self.group2() * Simd32x4::from(other.group0()) } }
    }
}

impl OuterProduct<Scalar> for Dipole {
    type Output = Dipole;

    fn outer_product(self, other: Scalar) -> Dipole {
        Dipole { groups: DipoleGroups { g0: self.group0() * Simd32x3::from(other.group0()), g1: self.group1() * Simd32x3::from(other.group0()), g2: self.group2() * Simd32x4::from(other.group0()) } }
    }
}

impl InnerProduct<Scalar> for Dipole {
    type Output = Dipole;

    fn inner_product(self, other: Scalar) -> Dipole {
        Dipole { groups: DipoleGroups { g0: self.group0() * Simd32x3::from(other.group0()), g1: self.group1() * Simd32x3::from(other.group0()), g2: self.group2() * Simd32x4::from(other.group0()) } }
    }
}

impl RightContraction<Scalar> for Dipole {
    type Output = Dipole;

    fn right_contraction(self, other: Scalar) -> Dipole {
        Dipole { groups: DipoleGroups { g0: self.group0() * Simd32x3::from(other.group0()), g1: self.group1() * Simd32x3::from(other.group0()), g2: self.group2() * Simd32x4::from(other.group0()) } }
    }
}

impl OuterProduct<RadialPoint> for Dipole {
    type Output = Circle;

    fn outer_product(self, other: RadialPoint) -> Circle {
        Circle { groups: CircleGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], other.group0()[2], other.group0()[0], other.group0()[2]]) * Simd32x4::from([1.0, 0.0, -1.0, 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], other.group0()[0], other.group0()[1], other.group0()[1]]) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group1()[0], other.group1()[0], other.group1()[0], other.group0()[0]]) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group1()[0], other.group1()[0], other.group1()[0], other.group0()[1]]) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group1()[0], other.group1()[0], other.group1()[0], other.group0()[2]]) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[0], other.group0()[2], other.group0()[1], other.group0()[0]]) * Simd32x4::from([0.0, -1.0, 1.0, 0.0]), g1: Simd32x3::from(self.group2()[0]) * Simd32x3::from(other.group1()[0]) * Simd32x3::from([1.0, 0.0, 0.0]) + Simd32x3::from(self.group2()[1]) * Simd32x3::from(other.group1()[0]) * Simd32x3::from([0.0, 1.0, 0.0]) + Simd32x3::from(self.group2()[2]) * Simd32x3::from(other.group1()[0]) * Simd32x3::from([0.0, 0.0, 1.0]) - Simd32x3::from(self.group2()[3]) * other.group0() + self.group0() * Simd32x3::from(other.group1()[1]), g2: Simd32x3::from(self.group2()[0]) * swizzle!(other.group0(), 2, 2, 1) * Simd32x3::from([0.0, 1.0, -1.0]) + Simd32x3::from(self.group2()[1]) * swizzle!(other.group0(), 2, 2, 0) * Simd32x3::from([-1.0, 0.0, 1.0]) + Simd32x3::from(self.group2()[2]) * swizzle!(other.group0(), 1, 0, 1) * Simd32x3::from([1.0, -1.0, 0.0]) + self.group1() * Simd32x3::from(other.group1()[1]) } }
    }
}

impl InnerProduct<RadialPoint> for Dipole {
    type Output = RadialPoint;

    fn inner_product(self, other: RadialPoint) -> RadialPoint {
        RadialPoint { groups: RadialPointGroups { g0: Simd32x3::from(self.group1()[1]) * swizzle!(other.group0(), 2, 2, 0) * Simd32x3::from([-1.0, 0.0, 1.0]) + Simd32x3::from(self.group1()[2]) * swizzle!(other.group0(), 1, 0, 1) * Simd32x3::from([1.0, -1.0, 0.0]) + Simd32x3::from(self.group1()[0]) * swizzle!(other.group0(), 0, 2, 1) * Simd32x3::from([0.0, 1.0, -1.0]), g1: Simd32x2::from(self.group0()[1]) * Simd32x2::from(other.group0()[1]) * Simd32x2::from([1.0, 0.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from(other.group0()[2]) * Simd32x2::from([1.0, 0.0]) + Simd32x2::from(self.group2()[0]) * Simd32x2::from(other.group0()[0]) * Simd32x2::from([0.0, -1.0]) + Simd32x2::from(self.group2()[1]) * Simd32x2::from(other.group0()[1]) * Simd32x2::from([0.0, -1.0]) + Simd32x2::from(self.group2()[2]) * Simd32x2::from(other.group0()[2]) * Simd32x2::from([0.0, -1.0]) + Simd32x2::from(self.group0()[0]) * Simd32x2::from(other.group0()[0]) * Simd32x2::from([1.0, 0.0]) } }
    }
}

impl RightContraction<RadialPoint> for Dipole {
    type Output = RadialPoint;

    fn right_contraction(self, other: RadialPoint) -> RadialPoint {
        RadialPoint { groups: RadialPointGroups { g0: Simd32x3::from(self.group1()[1]) * swizzle!(other.group0(), 2, 2, 0) * Simd32x3::from([-1.0, 0.0, 1.0]) + Simd32x3::from(self.group1()[2]) * swizzle!(other.group0(), 1, 0, 1) * Simd32x3::from([1.0, -1.0, 0.0]) + Simd32x3::from(self.group1()[0]) * swizzle!(other.group0(), 0, 2, 1) * Simd32x3::from([0.0, 1.0, -1.0]), g1: Simd32x2::from(self.group0()[1]) * Simd32x2::from(other.group0()[1]) * Simd32x2::from([1.0, 0.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from(other.group0()[2]) * Simd32x2::from([1.0, 0.0]) + Simd32x2::from(self.group2()[0]) * Simd32x2::from(other.group0()[0]) * Simd32x2::from([0.0, -1.0]) + Simd32x2::from(self.group2()[1]) * Simd32x2::from(other.group0()[1]) * Simd32x2::from([0.0, -1.0]) + Simd32x2::from(self.group2()[2]) * Simd32x2::from(other.group0()[2]) * Simd32x2::from([0.0, -1.0]) + Simd32x2::from(self.group0()[0]) * Simd32x2::from(other.group0()[0]) * Simd32x2::from([1.0, 0.0]) } }
    }
}

impl Into<FlatPoint> for Dipole {
    fn into(self) -> FlatPoint {
        FlatPoint { groups: FlatPointGroups { g0: self.group2() } }
    }
}

impl Add<FlatPoint> for Dipole {
    type Output = Dipole;

    fn add(self, other: FlatPoint) -> Dipole {
        Dipole { groups: DipoleGroups { g0: self.group0(), g1: self.group1(), g2: self.group2() + other.group0() } }
    }
}

impl AddAssign<FlatPoint> for Dipole {
    fn add_assign(&mut self, other: FlatPoint) {
        *self = (*self).add(other);
    }
}

impl Sub<FlatPoint> for Dipole {
    type Output = Dipole;

    fn sub(self, other: FlatPoint) -> Dipole {
        Dipole { groups: DipoleGroups { g0: self.group0(), g1: self.group1(), g2: self.group2() - other.group0() } }
    }
}

impl SubAssign<FlatPoint> for Dipole {
    fn sub_assign(&mut self, other: FlatPoint) {
        *self = (*self).sub(other);
    }
}

impl GeometricProduct<FlatPoint> for Dipole {
    type Output = Flector;

    fn geometric_product(self, other: FlatPoint) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group1()[1]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([-1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group0(), 1, 0, 1, 1) * Simd32x4::from([1.0, -1.0, 0.0, 0.0]) + Simd32x4::from([self.group0()[0], self.group1()[0], self.group1()[0], self.group0()[0]]) * swizzle!(other.group0(), 0, 2, 1, 0) * Simd32x4::from([0.0, 1.0, -1.0, 1.0]), g1: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, -1.0, 0.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 1, 0, 1, 1) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[1]) * swizzle!(other.group0(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group0(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from([self.group1()[0], self.group0()[0], self.group0()[0], self.group1()[0]]) * swizzle!(other.group0(), 3, 2, 1, 0) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) } }
    }
}

impl OuterProduct<FlatPoint> for Dipole {
    type Output = Plane;

    fn outer_product(self, other: FlatPoint) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, -1.0, 0.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 1, 0, 1, 1) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[1]) * swizzle!(other.group0(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group0(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from([self.group1()[0], self.group0()[0], self.group0()[0], self.group1()[0]]) * swizzle!(other.group0(), 3, 2, 1, 0) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) } }
    }
}

impl Add<Dipole> for Dipole {
    type Output = Dipole;

    fn add(self, other: Dipole) -> Dipole {
        Dipole { groups: DipoleGroups { g0: self.group0() + other.group0(), g1: self.group1() + other.group1(), g2: self.group2() + other.group2() } }
    }
}

impl AddAssign<Dipole> for Dipole {
    fn add_assign(&mut self, other: Dipole) {
        *self = (*self).add(other);
    }
}

impl Sub<Dipole> for Dipole {
    type Output = Dipole;

    fn sub(self, other: Dipole) -> Dipole {
        Dipole { groups: DipoleGroups { g0: self.group0() - other.group0(), g1: self.group1() - other.group1(), g2: self.group2() - other.group2() } }
    }
}

impl SubAssign<Dipole> for Dipole {
    fn sub_assign(&mut self, other: Dipole) {
        *self = (*self).sub(other);
    }
}

impl Mul<Dipole> for Dipole {
    type Output = Dipole;

    fn mul(self, other: Dipole) -> Dipole {
        Dipole { groups: DipoleGroups { g0: self.group0() * other.group0(), g1: self.group1() * other.group1(), g2: self.group2() * other.group2() } }
    }
}

impl MulAssign<Dipole> for Dipole {
    fn mul_assign(&mut self, other: Dipole) {
        *self = (*self).mul(other);
    }
}

impl Div<Dipole> for Dipole {
    type Output = Dipole;

    fn div(self, other: Dipole) -> Dipole {
        Dipole { groups: DipoleGroups { g0: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from([1.0, 1.0, 1.0]) / Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from([1.0, 1.0, 1.0]), g1: Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]) * Simd32x3::from([1.0, 1.0, 1.0]) / Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x3::from([1.0, 1.0, 1.0]), g2: Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group2()[3]]) * Simd32x4::from([1.0, 1.0, 1.0, 1.0]) / Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], other.group2()[3]]) * Simd32x4::from([1.0, 1.0, 1.0, 1.0]) } }
    }
}

impl DivAssign<Dipole> for Dipole {
    fn div_assign(&mut self, other: Dipole) {
        *self = (*self).div(other);
    }
}

impl OuterProduct<Dipole> for Dipole {
    type Output = Sphere;

    fn outer_product(self, other: Dipole) -> Sphere {
        Sphere { groups: SphereGroups { g0: 0.0 - self.group0()[0] * other.group1()[0] - self.group0()[1] * other.group1()[1] - self.group0()[2] * other.group1()[2] - self.group1()[0] * other.group0()[0] - self.group1()[1] * other.group0()[1] - self.group1()[2] * other.group0()[2], g1: Simd32x4::from(self.group0()[1]) * swizzle!(other.group2(), 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, -1.0, 0.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group2(), 1, 0, 1, 1) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[1]) * swizzle!(other.group2(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group2(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group2()[0]) * Simd32x4::from([other.group0()[2], other.group0()[2], other.group0()[1], other.group1()[0]]) * Simd32x4::from([0.0, 1.0, -1.0, -1.0]) + Simd32x4::from(self.group2()[1]) * Simd32x4::from([other.group0()[2], other.group0()[2], other.group0()[0], other.group1()[1]]) * Simd32x4::from([-1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group2()[2]) * Simd32x4::from([other.group0()[1], other.group0()[0], other.group0()[1], other.group1()[2]]) * Simd32x4::from([1.0, -1.0, 0.0, -1.0]) + Simd32x4::from(self.group2()[3]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group1()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) + Simd32x4::from([self.group1()[0], self.group0()[0], self.group0()[0], self.group1()[0]]) * swizzle!(other.group2(), 3, 2, 1, 0) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) } }
    }
}

impl InnerProduct<Dipole> for Dipole {
    type Output = Scalar;

    fn inner_product(self, other: Dipole) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group1()[0] * other.group1()[0] - self.group1()[1] * other.group1()[1] - self.group1()[2] * other.group1()[2] } }
    }
}

impl LeftContraction<Dipole> for Dipole {
    type Output = Scalar;

    fn left_contraction(self, other: Dipole) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group1()[0] * other.group1()[0] - self.group1()[1] * other.group1()[1] - self.group1()[2] * other.group1()[2] } }
    }
}

impl RightContraction<Dipole> for Dipole {
    type Output = Scalar;

    fn right_contraction(self, other: Dipole) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group1()[0] * other.group1()[0] - self.group1()[1] * other.group1()[1] - self.group1()[2] * other.group1()[2] } }
    }
}

impl ScalarProduct<Dipole> for Dipole {
    type Output = Scalar;

    fn scalar_product(self, other: Dipole) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group1()[0] * other.group1()[0] - self.group1()[1] * other.group1()[1] - self.group1()[2] * other.group1()[2] } }
    }
}

impl GeometricProduct<Line> for Dipole {
    type Output = Motor;

    fn geometric_product(self, other: Line) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group1()[2], other.group1()[2], other.group1()[0], other.group1()[1]]) * Simd32x4::from([-1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[1], other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x4::from([1.0, -1.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group0()[2], other.group0()[2], other.group0()[1], other.group0()[0]]) * Simd32x4::from([0.0, 1.0, -1.0, -1.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group0()[2], other.group0()[2], other.group0()[0], other.group0()[1]]) * Simd32x4::from([-1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group0()[1], other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x4::from([1.0, -1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group1()[0], other.group1()[2], other.group1()[1], other.group1()[0]]) * Simd32x4::from([0.0, 1.0, -1.0, -1.0]), g1: Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group1()[2], other.group1()[2], other.group1()[0], other.group1()[1]]) * Simd32x4::from([-1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group1()[1], other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x4::from([1.0, -1.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group1()[0], other.group1()[2], other.group1()[1], other.group1()[0]]) * Simd32x4::from([0.0, 1.0, -1.0, -1.0]) } }
    }
}

impl OuterProduct<Line> for Dipole {
    type Output = AntiScalar;

    fn outer_product(self, other: Line) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: 0.0 - self.group0()[0] * other.group1()[0] - self.group0()[1] * other.group1()[1] - self.group0()[2] * other.group1()[2] - self.group1()[0] * other.group0()[0] - self.group1()[1] * other.group0()[1] - self.group1()[2] * other.group0()[2] } }
    }
}

impl OuterProduct<Circle> for Dipole {
    type Output = AntiScalar;

    fn outer_product(self, other: Circle) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: 0.0 - self.group0()[0] * other.group2()[0] - self.group0()[1] * other.group2()[1] - self.group0()[2] * other.group2()[2] - self.group1()[0] * other.group1()[0] - self.group1()[1] * other.group1()[1] - self.group1()[2] * other.group1()[2] - self.group2()[0] * other.group0()[0] - self.group2()[1] * other.group0()[1] - self.group2()[2] * other.group0()[2] - self.group2()[3] * other.group0()[3] } }
    }
}

impl InnerProduct<Circle> for Dipole {
    type Output = RadialPoint;

    fn inner_product(self, other: Circle) -> RadialPoint {
        RadialPoint { groups: RadialPointGroups { g0: self.group1() * Simd32x3::from(other.group0()[3]), g1: Simd32x2::from(0.0) - Simd32x2::from(self.group1()[0]) * Simd32x2::from([other.group0()[0], other.group2()[0]]) - Simd32x2::from(self.group1()[1]) * Simd32x2::from([other.group0()[1], other.group2()[1]]) - Simd32x2::from(self.group1()[2]) * Simd32x2::from([other.group0()[2], other.group2()[2]]) } }
    }
}

impl LeftContraction<Circle> for Dipole {
    type Output = RadialPoint;

    fn left_contraction(self, other: Circle) -> RadialPoint {
        RadialPoint { groups: RadialPointGroups { g0: self.group1() * Simd32x3::from(other.group0()[3]), g1: Simd32x2::from(0.0) - Simd32x2::from(self.group1()[0]) * Simd32x2::from([other.group0()[0], other.group2()[0]]) - Simd32x2::from(self.group1()[1]) * Simd32x2::from([other.group0()[1], other.group2()[1]]) - Simd32x2::from(self.group1()[2]) * Simd32x2::from([other.group0()[2], other.group2()[2]]) } }
    }
}

impl InnerProduct<Plane> for Dipole {
    type Output = FlatPoint;

    fn inner_product(self, other: Plane) -> FlatPoint {
        FlatPoint { groups: FlatPointGroups { g0: Simd32x4::from(self.group1()[1]) * swizzle!(other.group0(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group0(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[0]) * swizzle!(other.group0(), 3, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]) } }
    }
}

impl LeftContraction<Plane> for Dipole {
    type Output = FlatPoint;

    fn left_contraction(self, other: Plane) -> FlatPoint {
        FlatPoint { groups: FlatPointGroups { g0: Simd32x4::from(self.group1()[1]) * swizzle!(other.group0(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group0(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[0]) * swizzle!(other.group0(), 3, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]) } }
    }
}

impl GeometricProduct<Motor> for Dipole {
    type Output = Motor;

    fn geometric_product(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[0]) * swizzle!(other.group1(), 3, 2, 1, 0) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]) + Simd32x4::from(self.group0()[1]) * swizzle!(other.group1(), 2, 3, 0, 1) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group1(), 1, 0, 3, 2) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[0]) * swizzle!(other.group0(), 3, 2, 1, 0) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]) + Simd32x4::from(self.group1()[1]) * swizzle!(other.group0(), 2, 3, 0, 1) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group0(), 1, 0, 3, 2) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]), g1: Simd32x4::from(self.group1()[0]) * swizzle!(other.group1(), 3, 2, 1, 0) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]) + Simd32x4::from(self.group1()[1]) * swizzle!(other.group1(), 2, 3, 0, 1) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group1(), 1, 0, 3, 2) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) } }
    }
}

impl GeometricProduct<Rotor> for Dipole {
    type Output = Rotor;

    fn geometric_product(self, other: Rotor) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group1()[0]) * swizzle!(other.group0(), 3, 2, 1, 0) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]) + Simd32x4::from(self.group1()[1]) * swizzle!(other.group0(), 2, 3, 0, 1) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group0(), 1, 0, 3, 2) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) } }
    }
}

impl OuterProduct<Rotor> for Dipole {
    type Output = AntiScalar;

    fn outer_product(self, other: Rotor) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: 0.0 - self.group1()[0] * other.group0()[0] - self.group1()[1] * other.group0()[1] - self.group1()[2] * other.group0()[2] } }
    }
}

impl GeometricProduct<Translator> for Dipole {
    type Output = Motor;

    fn geometric_product(self, other: Translator) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 2, 2, 0, 1) * Simd32x4::from([-1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 1, 0, 1, 2) * Simd32x4::from([1.0, -1.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from(other.group0()[3]) * Simd32x4::from([0.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from(other.group0()[3]) * Simd32x4::from([0.0, 0.0, 1.0, 0.0]) + Simd32x4::from([self.group1()[0], self.group0()[0], self.group0()[0], self.group0()[0]]) * swizzle!(other.group0(), 3, 2, 1, 0) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]), g1: Simd32x4::from(self.group1()[1]) * swizzle!(other.group0(), 2, 2, 0, 1) * Simd32x4::from([-1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group0(), 1, 0, 1, 2) * Simd32x4::from([1.0, -1.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[0]) * swizzle!(other.group0(), 0, 2, 1, 0) * Simd32x4::from([0.0, 1.0, -1.0, -1.0]) } }
    }
}

impl OuterProduct<Translator> for Dipole {
    type Output = AntiScalar;

    fn outer_product(self, other: Translator) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2] } }
    }
}

impl GeometricProduct<Flector> for Dipole {
    type Output = Flector;

    fn geometric_product(self, other: Flector) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group1()[3], other.group0()[2], other.group0()[1], other.group1()[0]]) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group0()[2], other.group1()[3], other.group0()[0], other.group1()[1]]) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group0()[1], other.group0()[0], other.group1()[3], other.group1()[2]]) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]), g1: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], other.group1()[3], other.group0()[0], other.group0()[2]]) * Simd32x4::from([1.0, -1.0, -1.0, 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], other.group0()[0], other.group1()[3], other.group0()[1]]) * Simd32x4::from([-1.0, 1.0, -1.0, 0.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group0()[3], other.group1()[2], other.group1()[1], other.group0()[0]]) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group1()[2], other.group0()[3], other.group1()[0], other.group0()[1]]) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group1()[1], other.group1()[0], other.group0()[3], other.group0()[2]]) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group1()[3], other.group0()[2], other.group0()[1], other.group1()[0]]) * Simd32x4::from([-1.0, -1.0, 1.0, 0.0]) } }
    }
}

impl OuterProduct<Flector> for Dipole {
    type Output = Plane;

    fn outer_product(self, other: Flector) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, -1.0, 0.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 1, 0, 1, 1) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[1]) * swizzle!(other.group0(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group0(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from([self.group1()[0], self.group0()[0], self.group0()[0], self.group1()[0]]) * swizzle!(other.group0(), 3, 2, 1, 0) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) } }
    }
}

impl InnerProduct<Flector> for Dipole {
    type Output = FlatPoint;

    fn inner_product(self, other: Flector) -> FlatPoint {
        FlatPoint { groups: FlatPointGroups { g0: Simd32x4::from(self.group1()[1]) * swizzle!(other.group1(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group1(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[0]) * swizzle!(other.group1(), 3, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]) } }
    }
}

impl LeftContraction<Flector> for Dipole {
    type Output = FlatPoint;

    fn left_contraction(self, other: Flector) -> FlatPoint {
        FlatPoint { groups: FlatPointGroups { g0: Simd32x4::from(self.group1()[1]) * swizzle!(other.group1(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group1(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[0]) * swizzle!(other.group1(), 3, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]) } }
    }
}

impl Add<MultiVector> for Dipole {
    type Output = MultiVector;

    fn add(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: other.group0(), g1: other.group1(), g2: other.group2(), g3: self.group2() + other.group3(), g4: self.group0() + other.group4(), g5: self.group1() + other.group5(), g6: other.group6(), g7: other.group7(), g8: other.group8(), g9: other.group9() } }
    }
}

impl Sub<MultiVector> for Dipole {
    type Output = MultiVector;

    fn sub(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x3::from(0.0) - other.group0(), g1: Simd32x3::from(0.0) - other.group1(), g2: Simd32x2::from(0.0) - other.group2(), g3: self.group2() - other.group3(), g4: self.group0() - other.group4(), g5: self.group1() - other.group5(), g6: Simd32x3::from(0.0) - other.group6(), g7: Simd32x3::from(0.0) - other.group7(), g8: Simd32x4::from(0.0) - other.group8(), g9: Simd32x4::from(0.0) - other.group9() } }
    }
}

impl GeometricProduct<MultiVector> for Dipole {
    type Output = MultiVector;

    fn geometric_product(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group5()[1], other.group5()[1], other.group7()[1]]) * Simd32x3::from([0.0, -1.0, -1.0]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group5()[2], other.group5()[2], other.group7()[2]]) * Simd32x3::from([0.0, -1.0, -1.0]) - Simd32x3::from(self.group1()[0]) * Simd32x3::from([other.group5()[0], other.group4()[0], other.group6()[0]]) - Simd32x3::from(self.group1()[1]) * Simd32x3::from([other.group5()[1], other.group4()[1], other.group6()[1]]) - Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group5()[2], other.group4()[2], other.group6()[2]]) + Simd32x3::from(self.group2()[0]) * Simd32x3::from(other.group8()[0]) * Simd32x3::from([0.0, 0.0, -1.0]) + Simd32x3::from(self.group2()[1]) * Simd32x3::from(other.group8()[1]) * Simd32x3::from([0.0, 0.0, -1.0]) + Simd32x3::from(self.group2()[2]) * Simd32x3::from(other.group8()[2]) * Simd32x3::from([0.0, 0.0, -1.0]) + Simd32x3::from(self.group2()[3]) * Simd32x3::from(other.group8()[3]) * Simd32x3::from([0.0, 0.0, -1.0]) + Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group5()[0], other.group5()[0], other.group7()[0]]) * Simd32x3::from([0.0, -1.0, -1.0]), g1: Simd32x3::from(self.group1()[0]) * Simd32x3::from([other.group8()[3], other.group1()[2], other.group1()[1]]) * Simd32x3::from([1.0, 1.0, -1.0]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from([other.group1()[2], other.group8()[3], other.group1()[0]]) * Simd32x3::from([-1.0, 1.0, 1.0]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group1()[1], other.group1()[0], other.group8()[3]]) * Simd32x3::from([1.0, -1.0, 1.0]), g2: Simd32x2::from(self.group0()[1]) * Simd32x2::from(other.group1()[1]) * Simd32x2::from([1.0, 0.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from(other.group1()[2]) * Simd32x2::from([1.0, 0.0]) - Simd32x2::from(self.group1()[0]) * Simd32x2::from([other.group8()[0], other.group7()[0]]) - Simd32x2::from(self.group1()[1]) * Simd32x2::from([other.group8()[1], other.group7()[1]]) - Simd32x2::from(self.group1()[2]) * Simd32x2::from([other.group8()[2], other.group7()[2]]) + Simd32x2::from(self.group2()[0]) * Simd32x2::from(other.group1()[0]) * Simd32x2::from([0.0, -1.0]) + Simd32x2::from(self.group2()[1]) * Simd32x2::from(other.group1()[1]) * Simd32x2::from([0.0, -1.0]) + Simd32x2::from(self.group2()[2]) * Simd32x2::from(other.group1()[2]) * Simd32x2::from([0.0, -1.0]) + Simd32x2::from(self.group0()[0]) * Simd32x2::from(other.group1()[0]) * Simd32x2::from([1.0, 0.0]), g3: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group3()[1]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group3()[2]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group9()[3], other.group3()[2], other.group3()[1], other.group9()[0]]) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group3()[2], other.group9()[3], other.group3()[0], other.group9()[1]]) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group3()[1], other.group3()[0], other.group9()[3], other.group9()[2]]) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) + Simd32x4::from(self.group2()[0]) * Simd32x4::from([other.group0()[0], other.group5()[2], other.group5()[1], other.group4()[0]]) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]) + Simd32x4::from(self.group2()[1]) * Simd32x4::from([other.group5()[2], other.group0()[0], other.group5()[0], other.group4()[1]]) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group2()[2]) * Simd32x4::from([other.group5()[1], other.group5()[0], other.group0()[0], other.group4()[2]]) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) + Simd32x4::from(self.group2()[3]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from(other.group3()[0]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]), g4: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[0], other.group5()[2], other.group5()[1]]) * Simd32x3::from([1.0, 1.0, -1.0]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group5()[2], other.group0()[0], other.group5()[0]]) * Simd32x3::from([-1.0, 1.0, 1.0]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group5()[1], other.group5()[0], other.group0()[0]]) * Simd32x3::from([1.0, -1.0, 1.0]) + Simd32x3::from(self.group1()[0]) * Simd32x3::from([other.group0()[1], other.group4()[2], other.group4()[1]]) * Simd32x3::from([1.0, 1.0, -1.0]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from([other.group4()[2], other.group0()[1], other.group4()[0]]) * Simd32x3::from([-1.0, 1.0, 1.0]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group4()[1], other.group4()[0], other.group0()[1]]) * Simd32x3::from([1.0, -1.0, 1.0]), g5: Simd32x3::from(self.group1()[0]) * Simd32x3::from([other.group0()[0], other.group5()[2], other.group5()[1]]) * Simd32x3::from([1.0, 1.0, -1.0]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from([other.group5()[2], other.group0()[0], other.group5()[0]]) * Simd32x3::from([-1.0, 1.0, 1.0]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group5()[1], other.group5()[0], other.group0()[0]]) * Simd32x3::from([1.0, -1.0, 1.0]), g6: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group2()[1], other.group7()[2], other.group7()[1]]) * Simd32x3::from([1.0, 1.0, -1.0]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group7()[2], other.group2()[1], other.group7()[0]]) * Simd32x3::from([-1.0, 1.0, 1.0]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group7()[1], other.group7()[0], other.group2()[1]]) * Simd32x3::from([1.0, -1.0, 1.0]) + Simd32x3::from(self.group1()[0]) * Simd32x3::from([other.group0()[2], other.group6()[2], other.group6()[1]]) * Simd32x3::from([1.0, 1.0, -1.0]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from([other.group6()[2], other.group0()[2], other.group6()[0]]) * Simd32x3::from([-1.0, 1.0, 1.0]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group6()[1], other.group6()[0], other.group0()[2]]) * Simd32x3::from([1.0, -1.0, 1.0]) + Simd32x3::from(self.group2()[0]) * Simd32x3::from([other.group2()[0], other.group8()[2], other.group8()[1]]) * Simd32x3::from([1.0, 1.0, -1.0]) + Simd32x3::from(self.group2()[1]) * Simd32x3::from([other.group8()[2], other.group2()[0], other.group8()[0]]) * Simd32x3::from([-1.0, 1.0, 1.0]) + Simd32x3::from(self.group2()[2]) * Simd32x3::from([other.group8()[1], other.group8()[0], other.group2()[0]]) * Simd32x3::from([1.0, -1.0, 1.0]) - Simd32x3::from(self.group2()[3]) * other.group1(), g7: Simd32x3::from(self.group1()[0]) * Simd32x3::from([other.group2()[1], other.group7()[2], other.group7()[1]]) * Simd32x3::from([1.0, 1.0, -1.0]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from([other.group7()[2], other.group2()[1], other.group7()[0]]) * Simd32x3::from([-1.0, 1.0, 1.0]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group7()[1], other.group7()[0], other.group2()[1]]) * Simd32x3::from([1.0, -1.0, 1.0]) + Simd32x3::from(self.group2()[0]) * Simd32x3::from([other.group8()[3], other.group1()[2], other.group1()[1]]) * Simd32x3::from([1.0, 1.0, -1.0]) + Simd32x3::from(self.group2()[1]) * Simd32x3::from([other.group1()[2], other.group8()[3], other.group1()[0]]) * Simd32x3::from([-1.0, 1.0, 1.0]) + Simd32x3::from(self.group2()[2]) * Simd32x3::from([other.group1()[1], other.group1()[0], other.group8()[3]]) * Simd32x3::from([1.0, -1.0, 1.0]), g8: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group1()[2], other.group8()[3], other.group1()[0], other.group1()[2]]) * Simd32x4::from([1.0, -1.0, -1.0, 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[1], other.group1()[0], other.group8()[3], other.group1()[1]]) * Simd32x4::from([-1.0, 1.0, -1.0, 0.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group2()[0], other.group8()[2], other.group8()[1], other.group1()[0]]) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group8()[2], other.group2()[0], other.group8()[0], other.group1()[1]]) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group8()[1], other.group8()[0], other.group2()[0], other.group1()[2]]) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group8()[3], other.group1()[2], other.group1()[1], other.group8()[0]]) * Simd32x4::from([-1.0, -1.0, 1.0, 0.0]), g9: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group3()[2], other.group9()[3], other.group3()[0], other.group3()[2]]) * Simd32x4::from([1.0, -1.0, -1.0, 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group3()[1], other.group3()[0], other.group9()[3], other.group3()[1]]) * Simd32x4::from([-1.0, 1.0, -1.0, 0.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group3()[3], other.group9()[2], other.group9()[1], other.group3()[0]]) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group9()[2], other.group3()[3], other.group9()[0], other.group3()[1]]) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group9()[1], other.group9()[0], other.group3()[3], other.group3()[2]]) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) + Simd32x4::from(self.group2()[0]) * Simd32x4::from([other.group0()[1], other.group4()[2], other.group4()[1], other.group5()[0]]) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]) + Simd32x4::from(self.group2()[1]) * Simd32x4::from([other.group4()[2], other.group0()[1], other.group4()[0], other.group5()[1]]) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group2()[2]) * Simd32x4::from([other.group4()[1], other.group4()[0], other.group0()[1], other.group5()[2]]) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) + Simd32x4::from(self.group2()[3]) * Simd32x4::from([other.group5()[0], other.group5()[1], other.group5()[2], other.group5()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group9()[3], other.group3()[2], other.group3()[1], other.group9()[0]]) * Simd32x4::from([-1.0, -1.0, 1.0, 0.0]) } }
    }
}

impl ScalarProduct<MultiVector> for Dipole {
    type Output = Scalar;

    fn scalar_product(self, other: MultiVector) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group1()[0] * other.group5()[0] - self.group1()[1] * other.group5()[1] - self.group1()[2] * other.group5()[2] } }
    }
}

impl SquaredMagnitude for Dipole {
    type Output = Scalar;

    fn squared_magnitude(self) -> Scalar {
        self.scalar_product(self.reversal())
    }
}

impl Magnitude for Dipole {
    type Output = Scalar;

    fn magnitude(self) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.squared_magnitude().group0().sqrt() } }
    }
}

impl Scale for Dipole {
    type Output = Dipole;

    fn scale(self, other: f32) -> Dipole {
        self.geometric_product(Scalar { groups: ScalarGroups { g0: other } })
    }
}

impl Signum for Dipole {
    type Output = Dipole;

    fn signum(self) -> Dipole {
        self.geometric_product(Scalar { groups: ScalarGroups { g0: 1.0 / self.magnitude().group0() } })
    }
}

impl Inverse for Dipole {
    type Output = Dipole;

    fn inverse(self) -> Dipole {
        self.reversal().geometric_product(Scalar { groups: ScalarGroups { g0: 1.0 / self.squared_magnitude().group0() } })
    }
}

impl Zero for Line {
    fn zero() -> Self {
        Line { groups: LineGroups { g0: Simd32x3::from(0.0), g1: Simd32x3::from(0.0) } }
    }
}

impl One for Line {
    fn one() -> Self {
        Line { groups: LineGroups { g0: Simd32x3::from(0.0), g1: Simd32x3::from(0.0) } }
    }
}

impl Neg for Line {
    type Output = Line;

    fn neg(self) -> Line {
        Line { groups: LineGroups { g0: self.group0() * Simd32x3::from(-1.0), g1: self.group1() * Simd32x3::from(-1.0) } }
    }
}

impl Automorphism for Line {
    type Output = Line;

    fn automorphism(self) -> Line {
        Line { groups: LineGroups { g0: self.group0() * Simd32x3::from(-1.0), g1: self.group1() * Simd32x3::from(-1.0) } }
    }
}

impl Reversal for Line {
    type Output = Line;

    fn reversal(self) -> Line {
        Line { groups: LineGroups { g0: self.group0() * Simd32x3::from(-1.0), g1: self.group1() * Simd32x3::from(-1.0) } }
    }
}

impl Conjugation for Line {
    type Output = Line;

    fn conjugation(self) -> Line {
        Line { groups: LineGroups { g0: self.group0(), g1: self.group1() } }
    }
}

impl GeometricProduct<Scalar> for Line {
    type Output = Line;

    fn geometric_product(self, other: Scalar) -> Line {
        Line { groups: LineGroups { g0: self.group0() * Simd32x3::from(other.group0()), g1: self.group1() * Simd32x3::from(other.group0()) } }
    }
}

impl OuterProduct<Scalar> for Line {
    type Output = Line;

    fn outer_product(self, other: Scalar) -> Line {
        Line { groups: LineGroups { g0: self.group0() * Simd32x3::from(other.group0()), g1: self.group1() * Simd32x3::from(other.group0()) } }
    }
}

impl InnerProduct<Scalar> for Line {
    type Output = Line;

    fn inner_product(self, other: Scalar) -> Line {
        Line { groups: LineGroups { g0: self.group0() * Simd32x3::from(other.group0()), g1: self.group1() * Simd32x3::from(other.group0()) } }
    }
}

impl RightContraction<Scalar> for Line {
    type Output = Line;

    fn right_contraction(self, other: Scalar) -> Line {
        Line { groups: LineGroups { g0: self.group0() * Simd32x3::from(other.group0()), g1: self.group1() * Simd32x3::from(other.group0()) } }
    }
}

impl GeometricProduct<RadialPoint> for Line {
    type Output = Flector;

    fn geometric_product(self, other: RadialPoint) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group0()[2], other.group0()[2], other.group0()[0], other.group0()[2]]) * Simd32x4::from([1.0, 0.0, -1.0, 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group0()[1], other.group0()[0], other.group0()[1], other.group0()[1]]) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) + Simd32x4::from([self.group0()[0], self.group1()[0], self.group1()[0], self.group0()[0]]) * Simd32x4::from([other.group0()[0], other.group0()[2], other.group0()[1], other.group0()[0]]) * Simd32x4::from([0.0, -1.0, 1.0, -1.0]), g1: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], other.group0()[2], other.group0()[0], other.group0()[2]]) * Simd32x4::from([-1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], other.group0()[0], other.group0()[1], other.group0()[1]]) * Simd32x4::from([1.0, -1.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group1()[0], other.group1()[0], other.group1()[0], other.group0()[0]]) * Simd32x4::from([-1.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group1()[0], other.group1()[0], other.group1()[0], other.group0()[1]]) * Simd32x4::from([0.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group1()[0], other.group1()[0], other.group1()[0], other.group0()[2]]) * Simd32x4::from([0.0, 0.0, -1.0, 1.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[0], other.group0()[2], other.group0()[1], other.group0()[0]]) * Simd32x4::from([0.0, 1.0, -1.0, 0.0]) } }
    }
}

impl OuterProduct<RadialPoint> for Line {
    type Output = Plane;

    fn outer_product(self, other: RadialPoint) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], other.group0()[2], other.group0()[0], other.group0()[2]]) * Simd32x4::from([-1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], other.group0()[0], other.group0()[1], other.group0()[1]]) * Simd32x4::from([1.0, -1.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group1()[0], other.group1()[0], other.group1()[0], other.group0()[0]]) * Simd32x4::from([-1.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group1()[0], other.group1()[0], other.group1()[0], other.group0()[1]]) * Simd32x4::from([0.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group1()[0], other.group1()[0], other.group1()[0], other.group0()[2]]) * Simd32x4::from([0.0, 0.0, -1.0, 1.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[0], other.group0()[2], other.group0()[1], other.group0()[0]]) * Simd32x4::from([0.0, 1.0, -1.0, 0.0]) } }
    }
}

impl InnerProduct<RadialPoint> for Line {
    type Output = FlatPoint;

    fn inner_product(self, other: RadialPoint) -> FlatPoint {
        FlatPoint { groups: FlatPointGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group0()[2], other.group0()[2], other.group0()[0], other.group0()[2]]) * Simd32x4::from([1.0, 0.0, -1.0, 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group0()[1], other.group0()[0], other.group0()[1], other.group0()[1]]) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) + Simd32x4::from([self.group0()[0], self.group1()[0], self.group1()[0], self.group0()[0]]) * Simd32x4::from([other.group0()[0], other.group0()[2], other.group0()[1], other.group0()[0]]) * Simd32x4::from([0.0, -1.0, 1.0, -1.0]) } }
    }
}

impl RightContraction<RadialPoint> for Line {
    type Output = FlatPoint;

    fn right_contraction(self, other: RadialPoint) -> FlatPoint {
        FlatPoint { groups: FlatPointGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group0()[2], other.group0()[2], other.group0()[0], other.group0()[2]]) * Simd32x4::from([1.0, 0.0, -1.0, 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group0()[1], other.group0()[0], other.group0()[1], other.group0()[1]]) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) + Simd32x4::from([self.group0()[0], self.group1()[0], self.group1()[0], self.group0()[0]]) * Simd32x4::from([other.group0()[0], other.group0()[2], other.group0()[1], other.group0()[0]]) * Simd32x4::from([0.0, -1.0, 1.0, -1.0]) } }
    }
}

impl GeometricProduct<Dipole> for Line {
    type Output = Motor;

    fn geometric_product(self, other: Dipole) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group1()[2], other.group1()[2], other.group1()[0], other.group1()[1]]) * Simd32x4::from([-1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[1], other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x4::from([1.0, -1.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group0()[2], other.group0()[2], other.group0()[1], other.group0()[0]]) * Simd32x4::from([0.0, 1.0, -1.0, -1.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group0()[2], other.group0()[2], other.group0()[0], other.group0()[1]]) * Simd32x4::from([-1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group0()[1], other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x4::from([1.0, -1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group1()[0], other.group1()[2], other.group1()[1], other.group1()[0]]) * Simd32x4::from([0.0, 1.0, -1.0, -1.0]), g1: Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group1()[2], other.group1()[2], other.group1()[0], other.group1()[1]]) * Simd32x4::from([-1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group1()[1], other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x4::from([1.0, -1.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group1()[0], other.group1()[2], other.group1()[1], other.group1()[0]]) * Simd32x4::from([0.0, 1.0, -1.0, -1.0]) } }
    }
}

impl OuterProduct<Dipole> for Line {
    type Output = AntiScalar;

    fn outer_product(self, other: Dipole) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: 0.0 - self.group0()[0] * other.group1()[0] - self.group0()[1] * other.group1()[1] - self.group0()[2] * other.group1()[2] - self.group1()[0] * other.group0()[0] - self.group1()[1] * other.group0()[1] - self.group1()[2] * other.group0()[2] } }
    }
}

impl Add<Line> for Line {
    type Output = Line;

    fn add(self, other: Line) -> Line {
        Line { groups: LineGroups { g0: self.group0() + other.group0(), g1: self.group1() + other.group1() } }
    }
}

impl AddAssign<Line> for Line {
    fn add_assign(&mut self, other: Line) {
        *self = (*self).add(other);
    }
}

impl Sub<Line> for Line {
    type Output = Line;

    fn sub(self, other: Line) -> Line {
        Line { groups: LineGroups { g0: self.group0() - other.group0(), g1: self.group1() - other.group1() } }
    }
}

impl SubAssign<Line> for Line {
    fn sub_assign(&mut self, other: Line) {
        *self = (*self).sub(other);
    }
}

impl Mul<Line> for Line {
    type Output = Line;

    fn mul(self, other: Line) -> Line {
        Line { groups: LineGroups { g0: self.group0() * other.group0(), g1: self.group1() * other.group1() } }
    }
}

impl MulAssign<Line> for Line {
    fn mul_assign(&mut self, other: Line) {
        *self = (*self).mul(other);
    }
}

impl Div<Line> for Line {
    type Output = Line;

    fn div(self, other: Line) -> Line {
        Line { groups: LineGroups { g0: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from([1.0, 1.0, 1.0]) / Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from([1.0, 1.0, 1.0]), g1: Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]) * Simd32x3::from([1.0, 1.0, 1.0]) / Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x3::from([1.0, 1.0, 1.0]) } }
    }
}

impl DivAssign<Line> for Line {
    fn div_assign(&mut self, other: Line) {
        *self = (*self).div(other);
    }
}

impl Add<Circle> for Line {
    type Output = Circle;

    fn add(self, other: Circle) -> Circle {
        Circle { groups: CircleGroups { g0: other.group0(), g1: self.group0() + other.group1(), g2: self.group1() + other.group2() } }
    }
}

impl Sub<Circle> for Line {
    type Output = Circle;

    fn sub(self, other: Circle) -> Circle {
        Circle { groups: CircleGroups { g0: Simd32x4::from(0.0) - other.group0(), g1: self.group0() - other.group1(), g2: self.group1() - other.group2() } }
    }
}

impl Add<Motor> for Line {
    type Output = Motor;

    fn add(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group0()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) + other.group0(), g1: Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group0()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) + other.group1() } }
    }
}

impl Sub<Motor> for Line {
    type Output = Motor;

    fn sub(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group0()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) - other.group0(), g1: Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group0()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) - other.group1() } }
    }
}

impl Add<MultiVector> for Line {
    type Output = MultiVector;

    fn add(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: other.group0(), g1: other.group1(), g2: other.group2(), g3: other.group3(), g4: other.group4(), g5: other.group5(), g6: self.group0() + other.group6(), g7: self.group1() + other.group7(), g8: other.group8(), g9: other.group9() } }
    }
}

impl Sub<MultiVector> for Line {
    type Output = MultiVector;

    fn sub(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x3::from(0.0) - other.group0(), g1: Simd32x3::from(0.0) - other.group1(), g2: Simd32x2::from(0.0) - other.group2(), g3: Simd32x4::from(0.0) - other.group3(), g4: Simd32x3::from(0.0) - other.group4(), g5: Simd32x3::from(0.0) - other.group5(), g6: self.group0() - other.group6(), g7: self.group1() - other.group7(), g8: Simd32x4::from(0.0) - other.group8(), g9: Simd32x4::from(0.0) - other.group9() } }
    }
}

impl Scale for Line {
    type Output = Line;

    fn scale(self, other: f32) -> Line {
        self.geometric_product(Scalar { groups: ScalarGroups { g0: other } })
    }
}

impl Zero for Circle {
    fn zero() -> Self {
        Circle { groups: CircleGroups { g0: Simd32x4::from(0.0), g1: Simd32x3::from(0.0), g2: Simd32x3::from(0.0) } }
    }
}

impl One for Circle {
    fn one() -> Self {
        Circle { groups: CircleGroups { g0: Simd32x4::from(0.0), g1: Simd32x3::from(0.0), g2: Simd32x3::from(0.0) } }
    }
}

impl Neg for Circle {
    type Output = Circle;

    fn neg(self) -> Circle {
        Circle { groups: CircleGroups { g0: self.group0() * Simd32x4::from(-1.0), g1: self.group1() * Simd32x3::from(-1.0), g2: self.group2() * Simd32x3::from(-1.0) } }
    }
}

impl Automorphism for Circle {
    type Output = Circle;

    fn automorphism(self) -> Circle {
        Circle { groups: CircleGroups { g0: self.group0() * Simd32x4::from(-1.0), g1: self.group1() * Simd32x3::from(-1.0), g2: self.group2() * Simd32x3::from(-1.0) } }
    }
}

impl Reversal for Circle {
    type Output = Circle;

    fn reversal(self) -> Circle {
        Circle { groups: CircleGroups { g0: self.group0() * Simd32x4::from(-1.0), g1: self.group1() * Simd32x3::from(-1.0), g2: self.group2() * Simd32x3::from(-1.0) } }
    }
}

impl Conjugation for Circle {
    type Output = Circle;

    fn conjugation(self) -> Circle {
        Circle { groups: CircleGroups { g0: self.group0(), g1: self.group1(), g2: self.group2() } }
    }
}

impl GeometricProduct<Scalar> for Circle {
    type Output = Circle;

    fn geometric_product(self, other: Scalar) -> Circle {
        Circle { groups: CircleGroups { g0: self.group0() * Simd32x4::from(other.group0()), g1: self.group1() * Simd32x3::from(other.group0()), g2: self.group2() * Simd32x3::from(other.group0()) } }
    }
}

impl OuterProduct<Scalar> for Circle {
    type Output = Circle;

    fn outer_product(self, other: Scalar) -> Circle {
        Circle { groups: CircleGroups { g0: self.group0() * Simd32x4::from(other.group0()), g1: self.group1() * Simd32x3::from(other.group0()), g2: self.group2() * Simd32x3::from(other.group0()) } }
    }
}

impl InnerProduct<Scalar> for Circle {
    type Output = Circle;

    fn inner_product(self, other: Scalar) -> Circle {
        Circle { groups: CircleGroups { g0: self.group0() * Simd32x4::from(other.group0()), g1: self.group1() * Simd32x3::from(other.group0()), g2: self.group2() * Simd32x3::from(other.group0()) } }
    }
}

impl RightContraction<Scalar> for Circle {
    type Output = Circle;

    fn right_contraction(self, other: Scalar) -> Circle {
        Circle { groups: CircleGroups { g0: self.group0() * Simd32x4::from(other.group0()), g1: self.group1() * Simd32x3::from(other.group0()), g2: self.group2() * Simd32x3::from(other.group0()) } }
    }
}

impl OuterProduct<RadialPoint> for Circle {
    type Output = Sphere;

    fn outer_product(self, other: RadialPoint) -> Sphere {
        Sphere { groups: SphereGroups { g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2] - self.group0()[3] * other.group1()[0], g1: Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group0()[2], other.group0()[2], other.group0()[1], other.group0()[2]]) * Simd32x4::from([0.0, 1.0, -1.0, 0.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group0()[2], other.group0()[2], other.group0()[0], other.group0()[2]]) * Simd32x4::from([-1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group0()[1], other.group0()[0], other.group0()[1], other.group0()[1]]) * Simd32x4::from([1.0, -1.0, 0.0, 0.0]) + Simd32x4::from(self.group2()[0]) * Simd32x4::from([other.group1()[0], other.group1()[0], other.group1()[0], other.group0()[0]]) * Simd32x4::from([-1.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group2()[1]) * Simd32x4::from([other.group1()[0], other.group1()[0], other.group1()[0], other.group0()[1]]) * Simd32x4::from([0.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.group2()[2]) * Simd32x4::from([other.group1()[0], other.group1()[0], other.group1()[0], other.group0()[2]]) * Simd32x4::from([0.0, 0.0, -1.0, 1.0]) + self.group0() * Simd32x4::from(other.group1()[1]) } }
    }
}

impl InnerProduct<RadialPoint> for Circle {
    type Output = Dipole;

    fn inner_product(self, other: RadialPoint) -> Dipole {
        Dipole { groups: DipoleGroups { g0: Simd32x3::from(self.group0()[1]) * swizzle!(other.group0(), 2, 2, 0) * Simd32x3::from([-1.0, 0.0, 1.0]) + Simd32x3::from(self.group0()[2]) * swizzle!(other.group0(), 1, 0, 1) * Simd32x3::from([1.0, -1.0, 0.0]) + Simd32x3::from(self.group0()[0]) * swizzle!(other.group0(), 0, 2, 1) * Simd32x3::from([0.0, 1.0, -1.0]), g1: Simd32x3::from(0.0) - Simd32x3::from(self.group0()[3]) * other.group0(), g2: Simd32x4::from(self.group1()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group2()[1]) * Simd32x4::from([other.group0()[2], other.group0()[2], other.group0()[0], other.group0()[2]]) * Simd32x4::from([1.0, 0.0, -1.0, 0.0]) + Simd32x4::from(self.group2()[2]) * Simd32x4::from([other.group0()[1], other.group0()[0], other.group0()[1], other.group0()[1]]) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) + Simd32x4::from([self.group1()[0], self.group2()[0], self.group2()[0], self.group1()[0]]) * Simd32x4::from([other.group0()[0], other.group0()[2], other.group0()[1], other.group0()[0]]) * Simd32x4::from([0.0, -1.0, 1.0, -1.0]) } }
    }
}

impl RightContraction<RadialPoint> for Circle {
    type Output = Dipole;

    fn right_contraction(self, other: RadialPoint) -> Dipole {
        Dipole { groups: DipoleGroups { g0: Simd32x3::from(self.group0()[1]) * swizzle!(other.group0(), 2, 2, 0) * Simd32x3::from([-1.0, 0.0, 1.0]) + Simd32x3::from(self.group0()[2]) * swizzle!(other.group0(), 1, 0, 1) * Simd32x3::from([1.0, -1.0, 0.0]) + Simd32x3::from(self.group0()[0]) * swizzle!(other.group0(), 0, 2, 1) * Simd32x3::from([0.0, 1.0, -1.0]), g1: Simd32x3::from(0.0) - Simd32x3::from(self.group0()[3]) * other.group0(), g2: Simd32x4::from(self.group1()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group2()[1]) * Simd32x4::from([other.group0()[2], other.group0()[2], other.group0()[0], other.group0()[2]]) * Simd32x4::from([1.0, 0.0, -1.0, 0.0]) + Simd32x4::from(self.group2()[2]) * Simd32x4::from([other.group0()[1], other.group0()[0], other.group0()[1], other.group0()[1]]) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) + Simd32x4::from([self.group1()[0], self.group2()[0], self.group2()[0], self.group1()[0]]) * Simd32x4::from([other.group0()[0], other.group0()[2], other.group0()[1], other.group0()[0]]) * Simd32x4::from([0.0, -1.0, 1.0, -1.0]) } }
    }
}

impl OuterProduct<FlatPoint> for Circle {
    type Output = AntiScalar;

    fn outer_product(self, other: FlatPoint) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2] - self.group0()[3] * other.group0()[3] } }
    }
}

impl OuterProduct<Dipole> for Circle {
    type Output = AntiScalar;

    fn outer_product(self, other: Dipole) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: 0.0 - self.group0()[0] * other.group2()[0] - self.group0()[1] * other.group2()[1] - self.group0()[2] * other.group2()[2] - self.group0()[3] * other.group2()[3] - self.group1()[0] * other.group1()[0] - self.group1()[1] * other.group1()[1] - self.group1()[2] * other.group1()[2] - self.group2()[0] * other.group0()[0] - self.group2()[1] * other.group0()[1] - self.group2()[2] * other.group0()[2] } }
    }
}

impl InnerProduct<Dipole> for Circle {
    type Output = RadialPoint;

    fn inner_product(self, other: Dipole) -> RadialPoint {
        RadialPoint { groups: RadialPointGroups { g0: Simd32x3::from(self.group0()[3]) * other.group1(), g1: Simd32x2::from(self.group0()[1]) * Simd32x2::from(other.group1()[1]) * Simd32x2::from([-1.0, 0.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from(other.group1()[2]) * Simd32x2::from([-1.0, 0.0]) + Simd32x2::from(self.group2()[0]) * Simd32x2::from(other.group1()[0]) * Simd32x2::from([0.0, -1.0]) + Simd32x2::from(self.group2()[1]) * Simd32x2::from(other.group1()[1]) * Simd32x2::from([0.0, -1.0]) + Simd32x2::from(self.group2()[2]) * Simd32x2::from(other.group1()[2]) * Simd32x2::from([0.0, -1.0]) + Simd32x2::from(self.group0()[0]) * Simd32x2::from(other.group1()[0]) * Simd32x2::from([-1.0, 0.0]) } }
    }
}

impl RightContraction<Dipole> for Circle {
    type Output = RadialPoint;

    fn right_contraction(self, other: Dipole) -> RadialPoint {
        RadialPoint { groups: RadialPointGroups { g0: Simd32x3::from(self.group0()[3]) * other.group1(), g1: Simd32x2::from(self.group0()[1]) * Simd32x2::from(other.group1()[1]) * Simd32x2::from([-1.0, 0.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from(other.group1()[2]) * Simd32x2::from([-1.0, 0.0]) + Simd32x2::from(self.group2()[0]) * Simd32x2::from(other.group1()[0]) * Simd32x2::from([0.0, -1.0]) + Simd32x2::from(self.group2()[1]) * Simd32x2::from(other.group1()[1]) * Simd32x2::from([0.0, -1.0]) + Simd32x2::from(self.group2()[2]) * Simd32x2::from(other.group1()[2]) * Simd32x2::from([0.0, -1.0]) + Simd32x2::from(self.group0()[0]) * Simd32x2::from(other.group1()[0]) * Simd32x2::from([-1.0, 0.0]) } }
    }
}

impl Into<Line> for Circle {
    fn into(self) -> Line {
        Line { groups: LineGroups { g0: self.group1(), g1: self.group2() } }
    }
}

impl Add<Line> for Circle {
    type Output = Circle;

    fn add(self, other: Line) -> Circle {
        Circle { groups: CircleGroups { g0: self.group0(), g1: self.group1() + other.group0(), g2: self.group2() + other.group1() } }
    }
}

impl AddAssign<Line> for Circle {
    fn add_assign(&mut self, other: Line) {
        *self = (*self).add(other);
    }
}

impl Sub<Line> for Circle {
    type Output = Circle;

    fn sub(self, other: Line) -> Circle {
        Circle { groups: CircleGroups { g0: self.group0(), g1: self.group1() - other.group0(), g2: self.group2() - other.group1() } }
    }
}

impl SubAssign<Line> for Circle {
    fn sub_assign(&mut self, other: Line) {
        *self = (*self).sub(other);
    }
}

impl Add<Circle> for Circle {
    type Output = Circle;

    fn add(self, other: Circle) -> Circle {
        Circle { groups: CircleGroups { g0: self.group0() + other.group0(), g1: self.group1() + other.group1(), g2: self.group2() + other.group2() } }
    }
}

impl AddAssign<Circle> for Circle {
    fn add_assign(&mut self, other: Circle) {
        *self = (*self).add(other);
    }
}

impl Sub<Circle> for Circle {
    type Output = Circle;

    fn sub(self, other: Circle) -> Circle {
        Circle { groups: CircleGroups { g0: self.group0() - other.group0(), g1: self.group1() - other.group1(), g2: self.group2() - other.group2() } }
    }
}

impl SubAssign<Circle> for Circle {
    fn sub_assign(&mut self, other: Circle) {
        *self = (*self).sub(other);
    }
}

impl Mul<Circle> for Circle {
    type Output = Circle;

    fn mul(self, other: Circle) -> Circle {
        Circle { groups: CircleGroups { g0: self.group0() * other.group0(), g1: self.group1() * other.group1(), g2: self.group2() * other.group2() } }
    }
}

impl MulAssign<Circle> for Circle {
    fn mul_assign(&mut self, other: Circle) {
        *self = (*self).mul(other);
    }
}

impl Div<Circle> for Circle {
    type Output = Circle;

    fn div(self, other: Circle) -> Circle {
        Circle { groups: CircleGroups { g0: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group0()[3]]) * Simd32x4::from([1.0, 1.0, 1.0, 1.0]) / Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group0()[3]]) * Simd32x4::from([1.0, 1.0, 1.0, 1.0]), g1: Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]) * Simd32x3::from([1.0, 1.0, 1.0]) / Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x3::from([1.0, 1.0, 1.0]), g2: Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]) * Simd32x3::from([1.0, 1.0, 1.0]) / Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]) * Simd32x3::from([1.0, 1.0, 1.0]) } }
    }
}

impl DivAssign<Circle> for Circle {
    fn div_assign(&mut self, other: Circle) {
        *self = (*self).div(other);
    }
}

impl InnerProduct<Circle> for Circle {
    type Output = Scalar;

    fn inner_product(self, other: Circle) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group0()[3] * other.group0()[3] } }
    }
}

impl LeftContraction<Circle> for Circle {
    type Output = Scalar;

    fn left_contraction(self, other: Circle) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group0()[3] * other.group0()[3] } }
    }
}

impl RightContraction<Circle> for Circle {
    type Output = Scalar;

    fn right_contraction(self, other: Circle) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group0()[3] * other.group0()[3] } }
    }
}

impl ScalarProduct<Circle> for Circle {
    type Output = Scalar;

    fn scalar_product(self, other: Circle) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group0()[3] * other.group0()[3] } }
    }
}

impl GeometricProduct<Motor> for Circle {
    type Output = Flector;

    fn geometric_product(self, other: Motor) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group1()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group0()[3]]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from(other.group1()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]), g1: Simd32x4::from(self.group0()[1]) * swizzle!(other.group1(), 2, 3, 0, 2) * Simd32x4::from([-1.0, 1.0, 1.0, 0.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group1(), 1, 0, 3, 1) * Simd32x4::from([1.0, -1.0, 1.0, 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]]) + Simd32x4::from(self.group0()[0]) * swizzle!(other.group1(), 3, 2, 1, 0) * Simd32x4::from([1.0, 1.0, -1.0, 0.0]) } }
    }
}

impl OuterProduct<Motor> for Circle {
    type Output = Plane;

    fn outer_product(self, other: Motor) -> Plane {
        Plane { groups: PlaneGroups { g0: self.group0() * Simd32x4::from(other.group1()[3]) } }
    }
}

impl GeometricProduct<Flector> for Circle {
    type Output = Motor;

    fn geometric_product(self, other: Flector) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group1()[3], other.group0()[2], other.group0()[1], other.group0()[0]]) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], other.group1()[3], other.group0()[0], other.group0()[1]]) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], other.group0()[0], other.group1()[3], other.group0()[2]]) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) - Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group0()[3]]), g1: Simd32x4::from(0.0) - Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]]) } }
    }
}

impl OuterProduct<Flector> for Circle {
    type Output = AntiScalar;

    fn outer_product(self, other: Flector) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2] - self.group0()[3] * other.group0()[3] } }
    }
}

impl Add<MultiVector> for Circle {
    type Output = MultiVector;

    fn add(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: other.group0(), g1: other.group1(), g2: other.group2(), g3: other.group3(), g4: other.group4(), g5: other.group5(), g6: self.group1() + other.group6(), g7: self.group2() + other.group7(), g8: self.group0() + other.group8(), g9: other.group9() } }
    }
}

impl Sub<MultiVector> for Circle {
    type Output = MultiVector;

    fn sub(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x3::from(0.0) - other.group0(), g1: Simd32x3::from(0.0) - other.group1(), g2: Simd32x2::from(0.0) - other.group2(), g3: Simd32x4::from(0.0) - other.group3(), g4: Simd32x3::from(0.0) - other.group4(), g5: Simd32x3::from(0.0) - other.group5(), g6: self.group1() - other.group6(), g7: self.group2() - other.group7(), g8: self.group0() - other.group8(), g9: Simd32x4::from(0.0) - other.group9() } }
    }
}

impl GeometricProduct<MultiVector> for Circle {
    type Output = MultiVector;

    fn geometric_product(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group1()[1], other.group1()[1], other.group3()[1]]) * Simd32x3::from([0.0, -1.0, -1.0]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group1()[2], other.group1()[2], other.group3()[2]]) * Simd32x3::from([0.0, -1.0, -1.0]) - Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group8()[3], other.group2()[0], other.group3()[3]]) + Simd32x3::from(self.group1()[0]) * Simd32x3::from(other.group5()[0]) * Simd32x3::from([0.0, 0.0, -1.0]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from(other.group5()[1]) * Simd32x3::from([0.0, 0.0, -1.0]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from(other.group5()[2]) * Simd32x3::from([0.0, 0.0, -1.0]) + Simd32x3::from(self.group2()[0]) * Simd32x3::from(other.group4()[0]) * Simd32x3::from([0.0, 0.0, -1.0]) + Simd32x3::from(self.group2()[1]) * Simd32x3::from(other.group4()[1]) * Simd32x3::from([0.0, 0.0, -1.0]) + Simd32x3::from(self.group2()[2]) * Simd32x3::from(other.group4()[2]) * Simd32x3::from([0.0, 0.0, -1.0]) + Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group1()[0], other.group1()[0], other.group3()[0]]) * Simd32x3::from([0.0, -1.0, -1.0]), g1: Simd32x3::from(self.group0()[3]) * other.group5(), g2: Simd32x2::from(self.group0()[1]) * Simd32x2::from(other.group5()[1]) * Simd32x2::from([-1.0, 0.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from(other.group5()[2]) * Simd32x2::from([-1.0, 0.0]) + Simd32x2::from(self.group0()[3]) * Simd32x2::from([other.group0()[1], other.group9()[3]]) * Simd32x2::from([1.0, -1.0]) + Simd32x2::from(self.group2()[0]) * Simd32x2::from(other.group5()[0]) * Simd32x2::from([0.0, -1.0]) + Simd32x2::from(self.group2()[1]) * Simd32x2::from(other.group5()[1]) * Simd32x2::from([0.0, -1.0]) + Simd32x2::from(self.group2()[2]) * Simd32x2::from(other.group5()[2]) * Simd32x2::from([0.0, -1.0]) + Simd32x2::from(self.group0()[0]) * Simd32x2::from(other.group5()[0]) * Simd32x2::from([-1.0, 0.0]), g3: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group7()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group7()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group7()[0], other.group7()[1], other.group7()[2], other.group0()[2]]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from(other.group1()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from(other.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from(other.group1()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group2()[0]) * Simd32x4::from([other.group8()[3], other.group1()[2], other.group1()[1], other.group8()[0]]) * Simd32x4::from([-1.0, -1.0, 1.0, 1.0]) + Simd32x4::from(self.group2()[1]) * Simd32x4::from([other.group1()[2], other.group8()[3], other.group1()[0], other.group8()[1]]) * Simd32x4::from([1.0, -1.0, -1.0, 1.0]) + Simd32x4::from(self.group2()[2]) * Simd32x4::from([other.group1()[1], other.group1()[0], other.group8()[3], other.group8()[2]]) * Simd32x4::from([-1.0, 1.0, -1.0, 1.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from(other.group7()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]), g4: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group8()[3], other.group1()[2], other.group1()[1]]) * Simd32x3::from([1.0, 1.0, -1.0]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group1()[2], other.group8()[3], other.group1()[0]]) * Simd32x3::from([-1.0, 1.0, 1.0]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group1()[1], other.group1()[0], other.group8()[3]]) * Simd32x3::from([1.0, -1.0, 1.0]) - Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group8()[0], other.group8()[1], other.group8()[2]]), g5: Simd32x3::from(0.0) - Simd32x3::from(self.group0()[3]) * other.group1(), g6: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group9()[3], other.group3()[2], other.group3()[1]]) * Simd32x3::from([1.0, 1.0, -1.0]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group3()[2], other.group9()[3], other.group3()[0]]) * Simd32x3::from([-1.0, 1.0, 1.0]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group3()[1], other.group3()[0], other.group9()[3]]) * Simd32x3::from([1.0, -1.0, 1.0]) - Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group9()[0], other.group9()[1], other.group9()[2]]) + Simd32x3::from(self.group1()[0]) * Simd32x3::from([other.group0()[0], other.group5()[2], other.group5()[1]]) * Simd32x3::from([1.0, 1.0, -1.0]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from([other.group5()[2], other.group0()[0], other.group5()[0]]) * Simd32x3::from([-1.0, 1.0, 1.0]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group5()[1], other.group5()[0], other.group0()[0]]) * Simd32x3::from([1.0, -1.0, 1.0]) + Simd32x3::from(self.group2()[0]) * Simd32x3::from([other.group0()[1], other.group4()[2], other.group4()[1]]) * Simd32x3::from([1.0, 1.0, -1.0]) + Simd32x3::from(self.group2()[1]) * Simd32x3::from([other.group4()[2], other.group0()[1], other.group4()[0]]) * Simd32x3::from([-1.0, 1.0, 1.0]) + Simd32x3::from(self.group2()[2]) * Simd32x3::from([other.group4()[1], other.group4()[0], other.group0()[1]]) * Simd32x3::from([1.0, -1.0, 1.0]), g7: Simd32x3::from(0.0) - Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group3()[0], other.group3()[1], other.group3()[2]]) + Simd32x3::from(self.group2()[0]) * Simd32x3::from([other.group0()[0], other.group5()[2], other.group5()[1]]) * Simd32x3::from([1.0, 1.0, -1.0]) + Simd32x3::from(self.group2()[1]) * Simd32x3::from([other.group5()[2], other.group0()[0], other.group5()[0]]) * Simd32x3::from([-1.0, 1.0, 1.0]) + Simd32x3::from(self.group2()[2]) * Simd32x3::from([other.group5()[1], other.group5()[0], other.group0()[0]]) * Simd32x3::from([1.0, -1.0, 1.0]), g8: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group5()[2], other.group0()[0], other.group5()[0], other.group5()[2]]) * Simd32x4::from([-1.0, 1.0, 1.0, 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group5()[1], other.group5()[0], other.group0()[0], other.group5()[1]]) * Simd32x4::from([1.0, -1.0, 1.0, 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group4()[0], other.group4()[1], other.group4()[2], other.group0()[0]]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[0], other.group5()[2], other.group5()[1], other.group0()[0]]) * Simd32x4::from([1.0, 1.0, -1.0, 0.0]), g9: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group7()[2], other.group2()[1], other.group7()[0], other.group7()[2]]) * Simd32x4::from([-1.0, 1.0, 1.0, 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group7()[1], other.group7()[0], other.group2()[1], other.group7()[1]]) * Simd32x4::from([1.0, -1.0, 1.0, 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group6()[0], other.group6()[1], other.group6()[2], other.group2()[1]]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group8()[3], other.group1()[2], other.group1()[1], other.group8()[3]]) * Simd32x4::from([1.0, 1.0, -1.0, 0.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group1()[2], other.group8()[3], other.group1()[0], other.group1()[2]]) * Simd32x4::from([-1.0, 1.0, 1.0, 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group1()[1], other.group1()[0], other.group8()[3], other.group1()[1]]) * Simd32x4::from([1.0, -1.0, 1.0, 0.0]) + Simd32x4::from(self.group2()[0]) * Simd32x4::from([other.group2()[0], other.group8()[2], other.group8()[1], other.group1()[0]]) * Simd32x4::from([-1.0, -1.0, 1.0, 1.0]) + Simd32x4::from(self.group2()[1]) * Simd32x4::from([other.group8()[2], other.group2()[0], other.group8()[0], other.group1()[1]]) * Simd32x4::from([1.0, -1.0, -1.0, 1.0]) + Simd32x4::from(self.group2()[2]) * Simd32x4::from([other.group8()[1], other.group8()[0], other.group2()[0], other.group1()[2]]) * Simd32x4::from([-1.0, 1.0, -1.0, 1.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group2()[1], other.group7()[2], other.group7()[1], other.group2()[0]]) * Simd32x4::from([1.0, 1.0, -1.0, 0.0]) } }
    }
}

impl ScalarProduct<MultiVector> for Circle {
    type Output = Scalar;

    fn scalar_product(self, other: MultiVector) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group0()[3] * other.group8()[3] } }
    }
}

impl SquaredMagnitude for Circle {
    type Output = Scalar;

    fn squared_magnitude(self) -> Scalar {
        self.scalar_product(self.reversal())
    }
}

impl Magnitude for Circle {
    type Output = Scalar;

    fn magnitude(self) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.squared_magnitude().group0().sqrt() } }
    }
}

impl Scale for Circle {
    type Output = Circle;

    fn scale(self, other: f32) -> Circle {
        self.geometric_product(Scalar { groups: ScalarGroups { g0: other } })
    }
}

impl Signum for Circle {
    type Output = Circle;

    fn signum(self) -> Circle {
        self.geometric_product(Scalar { groups: ScalarGroups { g0: 1.0 / self.magnitude().group0() } })
    }
}

impl Inverse for Circle {
    type Output = Circle;

    fn inverse(self) -> Circle {
        self.reversal().geometric_product(Scalar { groups: ScalarGroups { g0: 1.0 / self.squared_magnitude().group0() } })
    }
}

impl Zero for Plane {
    fn zero() -> Self {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(0.0) } }
    }
}

impl One for Plane {
    fn one() -> Self {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(0.0) } }
    }
}

impl Neg for Plane {
    type Output = Plane;

    fn neg(self) -> Plane {
        Plane { groups: PlaneGroups { g0: self.group0() * Simd32x4::from(-1.0) } }
    }
}

impl Automorphism for Plane {
    type Output = Plane;

    fn automorphism(self) -> Plane {
        Plane { groups: PlaneGroups { g0: self.group0() } }
    }
}

impl Reversal for Plane {
    type Output = Plane;

    fn reversal(self) -> Plane {
        Plane { groups: PlaneGroups { g0: self.group0() } }
    }
}

impl Conjugation for Plane {
    type Output = Plane;

    fn conjugation(self) -> Plane {
        Plane { groups: PlaneGroups { g0: self.group0() } }
    }
}

impl GeometricProduct<Scalar> for Plane {
    type Output = Plane;

    fn geometric_product(self, other: Scalar) -> Plane {
        Plane { groups: PlaneGroups { g0: self.group0() * Simd32x4::from(other.group0()) } }
    }
}

impl OuterProduct<Scalar> for Plane {
    type Output = Plane;

    fn outer_product(self, other: Scalar) -> Plane {
        Plane { groups: PlaneGroups { g0: self.group0() * Simd32x4::from(other.group0()) } }
    }
}

impl InnerProduct<Scalar> for Plane {
    type Output = Plane;

    fn inner_product(self, other: Scalar) -> Plane {
        Plane { groups: PlaneGroups { g0: self.group0() * Simd32x4::from(other.group0()) } }
    }
}

impl RightContraction<Scalar> for Plane {
    type Output = Plane;

    fn right_contraction(self, other: Scalar) -> Plane {
        Plane { groups: PlaneGroups { g0: self.group0() * Simd32x4::from(other.group0()) } }
    }
}

impl OuterProduct<RadialPoint> for Plane {
    type Output = AntiScalar;

    fn outer_product(self, other: RadialPoint) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2] + self.group0()[3] * other.group1()[0] } }
    }
}

impl InnerProduct<RadialPoint> for Plane {
    type Output = Line;

    fn inner_product(self, other: RadialPoint) -> Line {
        Line { groups: LineGroups { g0: Simd32x3::from(self.group0()[1]) * swizzle!(other.group0(), 2, 2, 0) * Simd32x3::from([1.0, 0.0, -1.0]) + Simd32x3::from(self.group0()[2]) * swizzle!(other.group0(), 1, 0, 1) * Simd32x3::from([-1.0, 1.0, 0.0]) + Simd32x3::from(self.group0()[0]) * swizzle!(other.group0(), 0, 2, 1) * Simd32x3::from([0.0, -1.0, 1.0]), g1: Simd32x3::from(self.group0()[3]) * other.group0() } }
    }
}

impl RightContraction<RadialPoint> for Plane {
    type Output = Line;

    fn right_contraction(self, other: RadialPoint) -> Line {
        Line { groups: LineGroups { g0: Simd32x3::from(self.group0()[1]) * swizzle!(other.group0(), 2, 2, 0) * Simd32x3::from([1.0, 0.0, -1.0]) + Simd32x3::from(self.group0()[2]) * swizzle!(other.group0(), 1, 0, 1) * Simd32x3::from([-1.0, 1.0, 0.0]) + Simd32x3::from(self.group0()[0]) * swizzle!(other.group0(), 0, 2, 1) * Simd32x3::from([0.0, -1.0, 1.0]), g1: Simd32x3::from(self.group0()[3]) * other.group0() } }
    }
}

impl Add<FlatPoint> for Plane {
    type Output = Flector;

    fn add(self, other: FlatPoint) -> Flector {
        Flector { groups: FlectorGroups { g0: other.group0(), g1: self.group0() } }
    }
}

impl Sub<FlatPoint> for Plane {
    type Output = Flector;

    fn sub(self, other: FlatPoint) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(0.0) - other.group0(), g1: self.group0() } }
    }
}

impl InnerProduct<Dipole> for Plane {
    type Output = FlatPoint;

    fn inner_product(self, other: Dipole) -> FlatPoint {
        FlatPoint { groups: FlatPointGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group1()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + swizzle!(self.group0(), 3, 3, 3, 0) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group1()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) } }
    }
}

impl RightContraction<Dipole> for Plane {
    type Output = FlatPoint;

    fn right_contraction(self, other: Dipole) -> FlatPoint {
        FlatPoint { groups: FlatPointGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group1()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + swizzle!(self.group0(), 3, 3, 3, 0) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group1()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) } }
    }
}

impl Add<Plane> for Plane {
    type Output = Plane;

    fn add(self, other: Plane) -> Plane {
        Plane { groups: PlaneGroups { g0: self.group0() + other.group0() } }
    }
}

impl AddAssign<Plane> for Plane {
    fn add_assign(&mut self, other: Plane) {
        *self = (*self).add(other);
    }
}

impl Sub<Plane> for Plane {
    type Output = Plane;

    fn sub(self, other: Plane) -> Plane {
        Plane { groups: PlaneGroups { g0: self.group0() - other.group0() } }
    }
}

impl SubAssign<Plane> for Plane {
    fn sub_assign(&mut self, other: Plane) {
        *self = (*self).sub(other);
    }
}

impl Mul<Plane> for Plane {
    type Output = Plane;

    fn mul(self, other: Plane) -> Plane {
        Plane { groups: PlaneGroups { g0: self.group0() * other.group0() } }
    }
}

impl MulAssign<Plane> for Plane {
    fn mul_assign(&mut self, other: Plane) {
        *self = (*self).mul(other);
    }
}

impl Div<Plane> for Plane {
    type Output = Plane;

    fn div(self, other: Plane) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group0()[3]]) * Simd32x4::from([1.0, 1.0, 1.0, 1.0]) / Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group0()[3]]) * Simd32x4::from([1.0, 1.0, 1.0, 1.0]) } }
    }
}

impl DivAssign<Plane> for Plane {
    fn div_assign(&mut self, other: Plane) {
        *self = (*self).div(other);
    }
}

impl Add<Sphere> for Plane {
    type Output = Sphere;

    fn add(self, other: Sphere) -> Sphere {
        Sphere { groups: SphereGroups { g0: other.group0(), g1: self.group0() + other.group1() } }
    }
}

impl Sub<Sphere> for Plane {
    type Output = Sphere;

    fn sub(self, other: Sphere) -> Sphere {
        Sphere { groups: SphereGroups { g0: 0.0 - other.group0(), g1: self.group0() - other.group1() } }
    }
}

impl Add<Flector> for Plane {
    type Output = Flector;

    fn add(self, other: Flector) -> Flector {
        Flector { groups: FlectorGroups { g0: other.group0(), g1: self.group0() + other.group1() } }
    }
}

impl Sub<Flector> for Plane {
    type Output = Flector;

    fn sub(self, other: Flector) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(0.0) - other.group0(), g1: self.group0() - other.group1() } }
    }
}

impl Add<MultiVector> for Plane {
    type Output = MultiVector;

    fn add(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: other.group0(), g1: other.group1(), g2: other.group2(), g3: other.group3(), g4: other.group4(), g5: other.group5(), g6: other.group6(), g7: other.group7(), g8: other.group8(), g9: self.group0() + other.group9() } }
    }
}

impl Sub<MultiVector> for Plane {
    type Output = MultiVector;

    fn sub(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x3::from(0.0) - other.group0(), g1: Simd32x3::from(0.0) - other.group1(), g2: Simd32x2::from(0.0) - other.group2(), g3: Simd32x4::from(0.0) - other.group3(), g4: Simd32x3::from(0.0) - other.group4(), g5: Simd32x3::from(0.0) - other.group5(), g6: Simd32x3::from(0.0) - other.group6(), g7: Simd32x3::from(0.0) - other.group7(), g8: Simd32x4::from(0.0) - other.group8(), g9: self.group0() - other.group9() } }
    }
}

impl Scale for Plane {
    type Output = Plane;

    fn scale(self, other: f32) -> Plane {
        self.geometric_product(Scalar { groups: ScalarGroups { g0: other } })
    }
}

impl Zero for Sphere {
    fn zero() -> Self {
        Sphere { groups: SphereGroups { g0: 0.0, g1: Simd32x4::from(0.0) } }
    }
}

impl One for Sphere {
    fn one() -> Self {
        Sphere { groups: SphereGroups { g0: 0.0, g1: Simd32x4::from(0.0) } }
    }
}

impl Neg for Sphere {
    type Output = Sphere;

    fn neg(self) -> Sphere {
        Sphere { groups: SphereGroups { g0: self.group0() * -1.0, g1: self.group1() * Simd32x4::from(-1.0) } }
    }
}

impl Automorphism for Sphere {
    type Output = Sphere;

    fn automorphism(self) -> Sphere {
        Sphere { groups: SphereGroups { g0: self.group0(), g1: self.group1() } }
    }
}

impl Reversal for Sphere {
    type Output = Sphere;

    fn reversal(self) -> Sphere {
        Sphere { groups: SphereGroups { g0: self.group0(), g1: self.group1() } }
    }
}

impl Conjugation for Sphere {
    type Output = Sphere;

    fn conjugation(self) -> Sphere {
        Sphere { groups: SphereGroups { g0: self.group0(), g1: self.group1() } }
    }
}

impl GeometricProduct<Scalar> for Sphere {
    type Output = Sphere;

    fn geometric_product(self, other: Scalar) -> Sphere {
        Sphere { groups: SphereGroups { g0: self.group0() * other.group0(), g1: self.group1() * Simd32x4::from(other.group0()) } }
    }
}

impl OuterProduct<Scalar> for Sphere {
    type Output = Sphere;

    fn outer_product(self, other: Scalar) -> Sphere {
        Sphere { groups: SphereGroups { g0: self.group0() * other.group0(), g1: self.group1() * Simd32x4::from(other.group0()) } }
    }
}

impl InnerProduct<Scalar> for Sphere {
    type Output = Sphere;

    fn inner_product(self, other: Scalar) -> Sphere {
        Sphere { groups: SphereGroups { g0: self.group0() * other.group0(), g1: self.group1() * Simd32x4::from(other.group0()) } }
    }
}

impl RightContraction<Scalar> for Sphere {
    type Output = Sphere;

    fn right_contraction(self, other: Scalar) -> Sphere {
        Sphere { groups: SphereGroups { g0: self.group0() * other.group0(), g1: self.group1() * Simd32x4::from(other.group0()) } }
    }
}

impl OuterProduct<RadialPoint> for Sphere {
    type Output = AntiScalar;

    fn outer_product(self, other: RadialPoint) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0() * other.group1()[1] + self.group1()[0] * other.group0()[0] + self.group1()[1] * other.group0()[1] + self.group1()[2] * other.group0()[2] + self.group1()[3] * other.group1()[0] } }
    }
}

impl Into<Plane> for Sphere {
    fn into(self) -> Plane {
        Plane { groups: PlaneGroups { g0: self.group1() } }
    }
}

impl Add<Plane> for Sphere {
    type Output = Sphere;

    fn add(self, other: Plane) -> Sphere {
        Sphere { groups: SphereGroups { g0: self.group0(), g1: self.group1() + other.group0() } }
    }
}

impl AddAssign<Plane> for Sphere {
    fn add_assign(&mut self, other: Plane) {
        *self = (*self).add(other);
    }
}

impl Sub<Plane> for Sphere {
    type Output = Sphere;

    fn sub(self, other: Plane) -> Sphere {
        Sphere { groups: SphereGroups { g0: self.group0(), g1: self.group1() - other.group0() } }
    }
}

impl SubAssign<Plane> for Sphere {
    fn sub_assign(&mut self, other: Plane) {
        *self = (*self).sub(other);
    }
}

impl Add<Sphere> for Sphere {
    type Output = Sphere;

    fn add(self, other: Sphere) -> Sphere {
        Sphere { groups: SphereGroups { g0: self.group0() + other.group0(), g1: self.group1() + other.group1() } }
    }
}

impl AddAssign<Sphere> for Sphere {
    fn add_assign(&mut self, other: Sphere) {
        *self = (*self).add(other);
    }
}

impl Sub<Sphere> for Sphere {
    type Output = Sphere;

    fn sub(self, other: Sphere) -> Sphere {
        Sphere { groups: SphereGroups { g0: self.group0() - other.group0(), g1: self.group1() - other.group1() } }
    }
}

impl SubAssign<Sphere> for Sphere {
    fn sub_assign(&mut self, other: Sphere) {
        *self = (*self).sub(other);
    }
}

impl Mul<Sphere> for Sphere {
    type Output = Sphere;

    fn mul(self, other: Sphere) -> Sphere {
        Sphere { groups: SphereGroups { g0: self.group0() * other.group0(), g1: self.group1() * other.group1() } }
    }
}

impl MulAssign<Sphere> for Sphere {
    fn mul_assign(&mut self, other: Sphere) {
        *self = (*self).mul(other);
    }
}

impl Div<Sphere> for Sphere {
    type Output = Sphere;

    fn div(self, other: Sphere) -> Sphere {
        Sphere { groups: SphereGroups { g0: self.group0() * 1.0 / other.group0() * 1.0, g1: Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group1()[3]]) * Simd32x4::from([1.0, 1.0, 1.0, 1.0]) / Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group1()[3]]) * Simd32x4::from([1.0, 1.0, 1.0, 1.0]) } }
    }
}

impl DivAssign<Sphere> for Sphere {
    fn div_assign(&mut self, other: Sphere) {
        *self = (*self).div(other);
    }
}

impl GeometricProduct<Motor> for Sphere {
    type Output = Rotor;

    fn geometric_product(self, other: Motor) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group0()) * other.group1() } }
    }
}

impl OuterProduct<Motor> for Sphere {
    type Output = AntiScalar;

    fn outer_product(self, other: Motor) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0() * other.group1()[3] } }
    }
}

impl Add<MultiVector> for Sphere {
    type Output = MultiVector;

    fn add(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x3::from(self.group0()) * Simd32x3::from([0.0, 1.0, 0.0]) + other.group0(), g1: other.group1(), g2: other.group2(), g3: other.group3(), g4: other.group4(), g5: other.group5(), g6: other.group6(), g7: other.group7(), g8: other.group8(), g9: self.group1() + other.group9() } }
    }
}

impl Sub<MultiVector> for Sphere {
    type Output = MultiVector;

    fn sub(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x3::from(self.group0()) * Simd32x3::from([0.0, 1.0, 0.0]) - other.group0(), g1: Simd32x3::from(0.0) - other.group1(), g2: Simd32x2::from(0.0) - other.group2(), g3: Simd32x4::from(0.0) - other.group3(), g4: Simd32x3::from(0.0) - other.group4(), g5: Simd32x3::from(0.0) - other.group5(), g6: Simd32x3::from(0.0) - other.group6(), g7: Simd32x3::from(0.0) - other.group7(), g8: Simd32x4::from(0.0) - other.group8(), g9: self.group1() - other.group9() } }
    }
}

impl Scale for Sphere {
    type Output = Sphere;

    fn scale(self, other: f32) -> Sphere {
        self.geometric_product(Scalar { groups: ScalarGroups { g0: other } })
    }
}

impl Zero for Motor {
    fn zero() -> Self {
        Motor { groups: MotorGroups { g0: Simd32x4::from(0.0), g1: Simd32x4::from(0.0) } }
    }
}

impl One for Motor {
    fn one() -> Self {
        Motor { groups: MotorGroups { g0: Simd32x4::from(0.0), g1: Simd32x4::from(0.0) } }
    }
}

impl Neg for Motor {
    type Output = Motor;

    fn neg(self) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() * Simd32x4::from(-1.0), g1: self.group1() * Simd32x4::from(-1.0) } }
    }
}

impl Automorphism for Motor {
    type Output = Motor;

    fn automorphism(self) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() * Simd32x4::from(-1.0), g1: self.group1() * Simd32x4::from(-1.0) } }
    }
}

impl Reversal for Motor {
    type Output = Motor;

    fn reversal(self) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]), g1: self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]) } }
    }
}

impl Conjugation for Motor {
    type Output = Motor;

    fn conjugation(self) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]), g1: self.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) } }
    }
}

impl GeometricProduct<Scalar> for Motor {
    type Output = Motor;

    fn geometric_product(self, other: Scalar) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() * Simd32x4::from(other.group0()), g1: self.group1() * Simd32x4::from(other.group0()) } }
    }
}

impl OuterProduct<Scalar> for Motor {
    type Output = Motor;

    fn outer_product(self, other: Scalar) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() * Simd32x4::from(other.group0()), g1: self.group1() * Simd32x4::from(other.group0()) } }
    }
}

impl InnerProduct<Scalar> for Motor {
    type Output = Motor;

    fn inner_product(self, other: Scalar) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() * Simd32x4::from(other.group0()), g1: self.group1() * Simd32x4::from(other.group0()) } }
    }
}

impl RightContraction<Scalar> for Motor {
    type Output = Motor;

    fn right_contraction(self, other: Scalar) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() * Simd32x4::from(other.group0()), g1: self.group1() * Simd32x4::from(other.group0()) } }
    }
}

impl Into<AntiScalar> for Motor {
    fn into(self) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0()[3] } }
    }
}

impl Add<AntiScalar> for Motor {
    type Output = Motor;

    fn add(self, other: AntiScalar) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() + Simd32x4::from(other.group0()) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]), g1: self.group1() } }
    }
}

impl AddAssign<AntiScalar> for Motor {
    fn add_assign(&mut self, other: AntiScalar) {
        *self = (*self).add(other);
    }
}

impl Sub<AntiScalar> for Motor {
    type Output = Motor;

    fn sub(self, other: AntiScalar) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() - Simd32x4::from(other.group0()) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]), g1: self.group1() } }
    }
}

impl SubAssign<AntiScalar> for Motor {
    fn sub_assign(&mut self, other: AntiScalar) {
        *self = (*self).sub(other);
    }
}

impl GeometricProduct<RadialPoint> for Motor {
    type Output = Flector;

    fn geometric_product(self, other: RadialPoint) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group0()[2], other.group0()[2], other.group0()[0], other.group0()[2]]) * Simd32x4::from([1.0, 0.0, -1.0, 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group0()[1], other.group0()[0], other.group0()[1], other.group0()[1]]) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) - Simd32x4::from(self.group1()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[0]]) + Simd32x4::from([self.group0()[0], self.group1()[0], self.group1()[0], self.group0()[0]]) * Simd32x4::from([other.group0()[0], other.group0()[2], other.group0()[1], other.group0()[0]]) * Simd32x4::from([0.0, -1.0, 1.0, -1.0]), g1: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], other.group0()[2], other.group0()[0], other.group0()[2]]) * Simd32x4::from([-1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], other.group0()[0], other.group0()[1], other.group0()[1]]) * Simd32x4::from([1.0, -1.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group0()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group1()[0], other.group1()[0], other.group1()[0], other.group0()[0]]) * Simd32x4::from([-1.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group1()[0], other.group1()[0], other.group1()[0], other.group0()[1]]) * Simd32x4::from([0.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group1()[0], other.group1()[0], other.group1()[0], other.group0()[2]]) * Simd32x4::from([0.0, 0.0, -1.0, 1.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[0], other.group0()[2], other.group0()[1], other.group0()[0]]) * Simd32x4::from([0.0, 1.0, -1.0, 0.0]) } }
    }
}

impl OuterProduct<RadialPoint> for Motor {
    type Output = Flector;

    fn outer_product(self, other: RadialPoint) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(0.0) - Simd32x4::from(self.group1()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[0]]), g1: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], other.group0()[2], other.group0()[0], other.group0()[2]]) * Simd32x4::from([-1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], other.group0()[0], other.group0()[1], other.group0()[1]]) * Simd32x4::from([1.0, -1.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group1()[0], other.group1()[0], other.group1()[0], other.group0()[0]]) * Simd32x4::from([-1.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group1()[0], other.group1()[0], other.group1()[0], other.group0()[1]]) * Simd32x4::from([0.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group1()[0], other.group1()[0], other.group1()[0], other.group0()[2]]) * Simd32x4::from([0.0, 0.0, -1.0, 1.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[0], other.group0()[2], other.group0()[1], other.group0()[0]]) * Simd32x4::from([0.0, 1.0, -1.0, 0.0]) } }
    }
}

impl GeometricProduct<Dipole> for Motor {
    type Output = Motor;

    fn geometric_product(self, other: Dipole) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group1()[2], other.group1()[2], other.group1()[0], other.group1()[1]]) * Simd32x4::from([-1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[1], other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x4::from([1.0, -1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group1()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group0()[2], other.group0()[2], other.group0()[1], other.group0()[0]]) * Simd32x4::from([0.0, 1.0, -1.0, -1.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group0()[2], other.group0()[2], other.group0()[0], other.group0()[1]]) * Simd32x4::from([-1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group0()[1], other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x4::from([1.0, -1.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group0()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group1()[0], other.group1()[2], other.group1()[1], other.group1()[0]]) * Simd32x4::from([0.0, 1.0, -1.0, -1.0]), g1: Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group1()[2], other.group1()[2], other.group1()[0], other.group1()[1]]) * Simd32x4::from([-1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group1()[1], other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x4::from([1.0, -1.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group1()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group1()[0], other.group1()[2], other.group1()[1], other.group1()[0]]) * Simd32x4::from([0.0, 1.0, -1.0, -1.0]) } }
    }
}

impl Into<Line> for Motor {
    fn into(self) -> Line {
        Line { groups: LineGroups { g0: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]), g1: Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]) } }
    }
}

impl Add<Line> for Motor {
    type Output = Motor;

    fn add(self, other: Line) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() + Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group0()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]), g1: self.group1() + Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group0()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) } }
    }
}

impl AddAssign<Line> for Motor {
    fn add_assign(&mut self, other: Line) {
        *self = (*self).add(other);
    }
}

impl Sub<Line> for Motor {
    type Output = Motor;

    fn sub(self, other: Line) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() - Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group0()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]), g1: self.group1() - Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group0()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) } }
    }
}

impl SubAssign<Line> for Motor {
    fn sub_assign(&mut self, other: Line) {
        *self = (*self).sub(other);
    }
}

impl GeometricProduct<Circle> for Motor {
    type Output = Flector;

    fn geometric_product(self, other: Circle) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group1()[0]) * swizzle!(other.group0(), 3, 3, 3, 0) * Simd32x4::from([-1.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group1()[1]) * swizzle!(other.group0(), 3, 3, 3, 1) * Simd32x4::from([0.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group0(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, -1.0, 1.0]) + swizzle!(self.group0(), 0, 0, 0, 3) * swizzle!(other.group0(), 0, 0, 0, 3) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]), g1: Simd32x4::from(self.group1()[0]) * swizzle!(other.group0(), 2, 2, 1, 2) * Simd32x4::from([0.0, -1.0, 1.0, 0.0]) + Simd32x4::from(self.group1()[1]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, -1.0, 0.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group0(), 1, 0, 1, 1) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) - Simd32x4::from(self.group1()[3]) * other.group0() + swizzle!(self.group0(), 0, 1, 2, 0) * swizzle!(other.group0(), 3, 3, 3, 0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) } }
    }
}

impl OuterProduct<Circle> for Motor {
    type Output = Plane;

    fn outer_product(self, other: Circle) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(0.0) - Simd32x4::from(self.group1()[3]) * other.group0() } }
    }
}

impl GeometricProduct<Sphere> for Motor {
    type Output = Rotor;

    fn geometric_product(self, other: Sphere) -> Rotor {
        Rotor { groups: RotorGroups { g0: self.group1() * Simd32x4::from(other.group0()) } }
    }
}

impl OuterProduct<Sphere> for Motor {
    type Output = AntiScalar;

    fn outer_product(self, other: Sphere) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group1()[3] * other.group0() } }
    }
}

impl Add<Motor> for Motor {
    type Output = Motor;

    fn add(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() + other.group0(), g1: self.group1() + other.group1() } }
    }
}

impl AddAssign<Motor> for Motor {
    fn add_assign(&mut self, other: Motor) {
        *self = (*self).add(other);
    }
}

impl Sub<Motor> for Motor {
    type Output = Motor;

    fn sub(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() - other.group0(), g1: self.group1() - other.group1() } }
    }
}

impl SubAssign<Motor> for Motor {
    fn sub_assign(&mut self, other: Motor) {
        *self = (*self).sub(other);
    }
}

impl Mul<Motor> for Motor {
    type Output = Motor;

    fn mul(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() * other.group0(), g1: self.group1() * other.group1() } }
    }
}

impl MulAssign<Motor> for Motor {
    fn mul_assign(&mut self, other: Motor) {
        *self = (*self).mul(other);
    }
}

impl Div<Motor> for Motor {
    type Output = Motor;

    fn div(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group0()[3]]) * Simd32x4::from([1.0, 1.0, 1.0, 1.0]) / Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group0()[3]]) * Simd32x4::from([1.0, 1.0, 1.0, 1.0]), g1: Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group1()[3]]) * Simd32x4::from([1.0, 1.0, 1.0, 1.0]) / Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group1()[3]]) * Simd32x4::from([1.0, 1.0, 1.0, 1.0]) } }
    }
}

impl DivAssign<Motor> for Motor {
    fn div_assign(&mut self, other: Motor) {
        *self = (*self).div(other);
    }
}

impl Into<Rotor> for Motor {
    fn into(self) -> Rotor {
        Rotor { groups: RotorGroups { g0: self.group0() } }
    }
}

impl Add<Rotor> for Motor {
    type Output = Motor;

    fn add(self, other: Rotor) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() + other.group0(), g1: self.group1() } }
    }
}

impl AddAssign<Rotor> for Motor {
    fn add_assign(&mut self, other: Rotor) {
        *self = (*self).add(other);
    }
}

impl Sub<Rotor> for Motor {
    type Output = Motor;

    fn sub(self, other: Rotor) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() - other.group0(), g1: self.group1() } }
    }
}

impl SubAssign<Rotor> for Motor {
    fn sub_assign(&mut self, other: Rotor) {
        *self = (*self).sub(other);
    }
}

impl Into<Translator> for Motor {
    fn into(self) -> Translator {
        Translator { groups: TranslatorGroups { g0: Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group0()[3]]) } }
    }
}

impl Add<Translator> for Motor {
    type Output = Motor;

    fn add(self, other: Translator) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() + swizzle!(other.group0(), 0, 0, 0, 3) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]), g1: self.group1() + swizzle!(other.group0(), 0, 1, 2, 0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) } }
    }
}

impl AddAssign<Translator> for Motor {
    fn add_assign(&mut self, other: Translator) {
        *self = (*self).add(other);
    }
}

impl Sub<Translator> for Motor {
    type Output = Motor;

    fn sub(self, other: Translator) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() - swizzle!(other.group0(), 0, 0, 0, 3) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]), g1: self.group1() - swizzle!(other.group0(), 0, 1, 2, 0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) } }
    }
}

impl SubAssign<Translator> for Motor {
    fn sub_assign(&mut self, other: Translator) {
        *self = (*self).sub(other);
    }
}

impl Add<MultiVector> for Motor {
    type Output = MultiVector;

    fn add(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x3::from([self.group0()[0], self.group0()[0], self.group0()[3]]) * Simd32x3::from([0.0, 0.0, 1.0]) + other.group0(), g1: other.group1(), g2: Simd32x2::from([self.group0()[0], self.group1()[3]]) * Simd32x2::from([0.0, 1.0]) + other.group2(), g3: other.group3(), g4: other.group4(), g5: other.group5(), g6: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) + other.group6(), g7: Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]) + other.group7(), g8: other.group8(), g9: other.group9() } }
    }
}

impl Sub<MultiVector> for Motor {
    type Output = MultiVector;

    fn sub(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x3::from([self.group0()[0], self.group0()[0], self.group0()[3]]) * Simd32x3::from([0.0, 0.0, 1.0]) - other.group0(), g1: Simd32x3::from(0.0) - other.group1(), g2: Simd32x2::from([self.group0()[0], self.group1()[3]]) * Simd32x2::from([0.0, 1.0]) - other.group2(), g3: Simd32x4::from(0.0) - other.group3(), g4: Simd32x3::from(0.0) - other.group4(), g5: Simd32x3::from(0.0) - other.group5(), g6: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) - other.group6(), g7: Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]) - other.group7(), g8: Simd32x4::from(0.0) - other.group8(), g9: Simd32x4::from(0.0) - other.group9() } }
    }
}

impl Scale for Motor {
    type Output = Motor;

    fn scale(self, other: f32) -> Motor {
        self.geometric_product(Scalar { groups: ScalarGroups { g0: other } })
    }
}

impl Zero for Rotor {
    fn zero() -> Self {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(0.0) } }
    }
}

impl One for Rotor {
    fn one() -> Self {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(0.0) } }
    }
}

impl Neg for Rotor {
    type Output = Rotor;

    fn neg(self) -> Rotor {
        Rotor { groups: RotorGroups { g0: self.group0() * Simd32x4::from(-1.0) } }
    }
}

impl Automorphism for Rotor {
    type Output = Rotor;

    fn automorphism(self) -> Rotor {
        Rotor { groups: RotorGroups { g0: self.group0() * Simd32x4::from(-1.0) } }
    }
}

impl Reversal for Rotor {
    type Output = Rotor;

    fn reversal(self) -> Rotor {
        Rotor { groups: RotorGroups { g0: self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]) } }
    }
}

impl Conjugation for Rotor {
    type Output = Rotor;

    fn conjugation(self) -> Rotor {
        Rotor { groups: RotorGroups { g0: self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) } }
    }
}

impl GeometricProduct<Scalar> for Rotor {
    type Output = Rotor;

    fn geometric_product(self, other: Scalar) -> Rotor {
        Rotor { groups: RotorGroups { g0: self.group0() * Simd32x4::from(other.group0()) } }
    }
}

impl OuterProduct<Scalar> for Rotor {
    type Output = Rotor;

    fn outer_product(self, other: Scalar) -> Rotor {
        Rotor { groups: RotorGroups { g0: self.group0() * Simd32x4::from(other.group0()) } }
    }
}

impl InnerProduct<Scalar> for Rotor {
    type Output = Rotor;

    fn inner_product(self, other: Scalar) -> Rotor {
        Rotor { groups: RotorGroups { g0: self.group0() * Simd32x4::from(other.group0()) } }
    }
}

impl RightContraction<Scalar> for Rotor {
    type Output = Rotor;

    fn right_contraction(self, other: Scalar) -> Rotor {
        Rotor { groups: RotorGroups { g0: self.group0() * Simd32x4::from(other.group0()) } }
    }
}

impl Into<AntiScalar> for Rotor {
    fn into(self) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0()[3] } }
    }
}

impl Add<AntiScalar> for Rotor {
    type Output = Rotor;

    fn add(self, other: AntiScalar) -> Rotor {
        Rotor { groups: RotorGroups { g0: self.group0() + Simd32x4::from(other.group0()) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) } }
    }
}

impl AddAssign<AntiScalar> for Rotor {
    fn add_assign(&mut self, other: AntiScalar) {
        *self = (*self).add(other);
    }
}

impl Sub<AntiScalar> for Rotor {
    type Output = Rotor;

    fn sub(self, other: AntiScalar) -> Rotor {
        Rotor { groups: RotorGroups { g0: self.group0() - Simd32x4::from(other.group0()) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) } }
    }
}

impl SubAssign<AntiScalar> for Rotor {
    fn sub_assign(&mut self, other: AntiScalar) {
        *self = (*self).sub(other);
    }
}

impl GeometricProduct<Dipole> for Rotor {
    type Output = Rotor;

    fn geometric_product(self, other: Dipole) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group1()[2], other.group1()[2], other.group1()[0], other.group1()[1]]) * Simd32x4::from([-1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[1], other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x4::from([1.0, -1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group1()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group1()[0], other.group1()[2], other.group1()[1], other.group1()[0]]) * Simd32x4::from([0.0, 1.0, -1.0, -1.0]) } }
    }
}

impl OuterProduct<Dipole> for Rotor {
    type Output = AntiScalar;

    fn outer_product(self, other: Dipole) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: 0.0 - self.group0()[0] * other.group1()[0] - self.group0()[1] * other.group1()[1] - self.group0()[2] * other.group1()[2] } }
    }
}

impl Add<Motor> for Rotor {
    type Output = Motor;

    fn add(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() + other.group0(), g1: other.group1() } }
    }
}

impl Sub<Motor> for Rotor {
    type Output = Motor;

    fn sub(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() - other.group0(), g1: Simd32x4::from(0.0) - other.group1() } }
    }
}

impl Add<Rotor> for Rotor {
    type Output = Rotor;

    fn add(self, other: Rotor) -> Rotor {
        Rotor { groups: RotorGroups { g0: self.group0() + other.group0() } }
    }
}

impl AddAssign<Rotor> for Rotor {
    fn add_assign(&mut self, other: Rotor) {
        *self = (*self).add(other);
    }
}

impl Sub<Rotor> for Rotor {
    type Output = Rotor;

    fn sub(self, other: Rotor) -> Rotor {
        Rotor { groups: RotorGroups { g0: self.group0() - other.group0() } }
    }
}

impl SubAssign<Rotor> for Rotor {
    fn sub_assign(&mut self, other: Rotor) {
        *self = (*self).sub(other);
    }
}

impl Mul<Rotor> for Rotor {
    type Output = Rotor;

    fn mul(self, other: Rotor) -> Rotor {
        Rotor { groups: RotorGroups { g0: self.group0() * other.group0() } }
    }
}

impl MulAssign<Rotor> for Rotor {
    fn mul_assign(&mut self, other: Rotor) {
        *self = (*self).mul(other);
    }
}

impl Div<Rotor> for Rotor {
    type Output = Rotor;

    fn div(self, other: Rotor) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group0()[3]]) * Simd32x4::from([1.0, 1.0, 1.0, 1.0]) / Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group0()[3]]) * Simd32x4::from([1.0, 1.0, 1.0, 1.0]) } }
    }
}

impl DivAssign<Rotor> for Rotor {
    fn div_assign(&mut self, other: Rotor) {
        *self = (*self).div(other);
    }
}

impl Add<MultiVector> for Rotor {
    type Output = MultiVector;

    fn add(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x3::from([self.group0()[0], self.group0()[0], self.group0()[3]]) * Simd32x3::from([0.0, 0.0, 1.0]) + other.group0(), g1: other.group1(), g2: other.group2(), g3: other.group3(), g4: other.group4(), g5: other.group5(), g6: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) + other.group6(), g7: other.group7(), g8: other.group8(), g9: other.group9() } }
    }
}

impl Sub<MultiVector> for Rotor {
    type Output = MultiVector;

    fn sub(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x3::from([self.group0()[0], self.group0()[0], self.group0()[3]]) * Simd32x3::from([0.0, 0.0, 1.0]) - other.group0(), g1: Simd32x3::from(0.0) - other.group1(), g2: Simd32x2::from(0.0) - other.group2(), g3: Simd32x4::from(0.0) - other.group3(), g4: Simd32x3::from(0.0) - other.group4(), g5: Simd32x3::from(0.0) - other.group5(), g6: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) - other.group6(), g7: Simd32x3::from(0.0) - other.group7(), g8: Simd32x4::from(0.0) - other.group8(), g9: Simd32x4::from(0.0) - other.group9() } }
    }
}

impl Scale for Rotor {
    type Output = Rotor;

    fn scale(self, other: f32) -> Rotor {
        self.geometric_product(Scalar { groups: ScalarGroups { g0: other } })
    }
}

impl Zero for Translator {
    fn zero() -> Self {
        Translator { groups: TranslatorGroups { g0: Simd32x4::from(0.0) } }
    }
}

impl One for Translator {
    fn one() -> Self {
        Translator { groups: TranslatorGroups { g0: Simd32x4::from(0.0) } }
    }
}

impl Neg for Translator {
    type Output = Translator;

    fn neg(self) -> Translator {
        Translator { groups: TranslatorGroups { g0: self.group0() * Simd32x4::from(-1.0) } }
    }
}

impl Automorphism for Translator {
    type Output = Translator;

    fn automorphism(self) -> Translator {
        Translator { groups: TranslatorGroups { g0: self.group0() * Simd32x4::from(-1.0) } }
    }
}

impl Reversal for Translator {
    type Output = Translator;

    fn reversal(self) -> Translator {
        Translator { groups: TranslatorGroups { g0: self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]) } }
    }
}

impl Conjugation for Translator {
    type Output = Translator;

    fn conjugation(self) -> Translator {
        Translator { groups: TranslatorGroups { g0: self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) } }
    }
}

impl GeometricProduct<Scalar> for Translator {
    type Output = Translator;

    fn geometric_product(self, other: Scalar) -> Translator {
        Translator { groups: TranslatorGroups { g0: self.group0() * Simd32x4::from(other.group0()) } }
    }
}

impl OuterProduct<Scalar> for Translator {
    type Output = Translator;

    fn outer_product(self, other: Scalar) -> Translator {
        Translator { groups: TranslatorGroups { g0: self.group0() * Simd32x4::from(other.group0()) } }
    }
}

impl InnerProduct<Scalar> for Translator {
    type Output = Translator;

    fn inner_product(self, other: Scalar) -> Translator {
        Translator { groups: TranslatorGroups { g0: self.group0() * Simd32x4::from(other.group0()) } }
    }
}

impl RightContraction<Scalar> for Translator {
    type Output = Translator;

    fn right_contraction(self, other: Scalar) -> Translator {
        Translator { groups: TranslatorGroups { g0: self.group0() * Simd32x4::from(other.group0()) } }
    }
}

impl Into<AntiScalar> for Translator {
    fn into(self) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0()[3] } }
    }
}

impl Add<AntiScalar> for Translator {
    type Output = Translator;

    fn add(self, other: AntiScalar) -> Translator {
        Translator { groups: TranslatorGroups { g0: self.group0() + Simd32x4::from(other.group0()) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) } }
    }
}

impl AddAssign<AntiScalar> for Translator {
    fn add_assign(&mut self, other: AntiScalar) {
        *self = (*self).add(other);
    }
}

impl Sub<AntiScalar> for Translator {
    type Output = Translator;

    fn sub(self, other: AntiScalar) -> Translator {
        Translator { groups: TranslatorGroups { g0: self.group0() - Simd32x4::from(other.group0()) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) } }
    }
}

impl SubAssign<AntiScalar> for Translator {
    fn sub_assign(&mut self, other: AntiScalar) {
        *self = (*self).sub(other);
    }
}

impl OuterProduct<RadialPoint> for Translator {
    type Output = Plane;

    fn outer_product(self, other: RadialPoint) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group1()[0], other.group1()[0], other.group1()[0], other.group0()[1]]) * Simd32x4::from([0.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[0], other.group1()[0], other.group1()[0], other.group0()[2]]) * Simd32x4::from([0.0, 0.0, -1.0, 1.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group1()[0], other.group1()[0], other.group1()[0], other.group0()[0]]) * Simd32x4::from([-1.0, 0.0, 0.0, 1.0]) } }
    }
}

impl GeometricProduct<Dipole> for Translator {
    type Output = Motor;

    fn geometric_product(self, other: Dipole) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], other.group0()[2], other.group0()[0], other.group0()[1]]) * Simd32x4::from([-1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x4::from([1.0, -1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group1()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[0], other.group0()[2], other.group0()[1], other.group0()[0]]) * Simd32x4::from([0.0, 1.0, -1.0, -1.0]), g1: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group1()[2], other.group1()[2], other.group1()[0], other.group1()[1]]) * Simd32x4::from([-1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[1], other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x4::from([1.0, -1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group1()[0], other.group1()[2], other.group1()[1], other.group1()[0]]) * Simd32x4::from([0.0, 1.0, -1.0, -1.0]) } }
    }
}

impl OuterProduct<Dipole> for Translator {
    type Output = AntiScalar;

    fn outer_product(self, other: Dipole) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2] } }
    }
}

impl Add<Motor> for Translator {
    type Output = Motor;

    fn add(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: swizzle!(self.group0(), 0, 0, 0, 3) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + other.group0(), g1: swizzle!(self.group0(), 0, 1, 2, 0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) + other.group1() } }
    }
}

impl Sub<Motor> for Translator {
    type Output = Motor;

    fn sub(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: swizzle!(self.group0(), 0, 0, 0, 3) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) - other.group0(), g1: swizzle!(self.group0(), 0, 1, 2, 0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) - other.group1() } }
    }
}

impl Add<Translator> for Translator {
    type Output = Translator;

    fn add(self, other: Translator) -> Translator {
        Translator { groups: TranslatorGroups { g0: self.group0() + other.group0() } }
    }
}

impl AddAssign<Translator> for Translator {
    fn add_assign(&mut self, other: Translator) {
        *self = (*self).add(other);
    }
}

impl Sub<Translator> for Translator {
    type Output = Translator;

    fn sub(self, other: Translator) -> Translator {
        Translator { groups: TranslatorGroups { g0: self.group0() - other.group0() } }
    }
}

impl SubAssign<Translator> for Translator {
    fn sub_assign(&mut self, other: Translator) {
        *self = (*self).sub(other);
    }
}

impl Mul<Translator> for Translator {
    type Output = Translator;

    fn mul(self, other: Translator) -> Translator {
        Translator { groups: TranslatorGroups { g0: self.group0() * other.group0() } }
    }
}

impl MulAssign<Translator> for Translator {
    fn mul_assign(&mut self, other: Translator) {
        *self = (*self).mul(other);
    }
}

impl Div<Translator> for Translator {
    type Output = Translator;

    fn div(self, other: Translator) -> Translator {
        Translator { groups: TranslatorGroups { g0: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group0()[3]]) * Simd32x4::from([1.0, 1.0, 1.0, 1.0]) / Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group0()[3]]) * Simd32x4::from([1.0, 1.0, 1.0, 1.0]) } }
    }
}

impl DivAssign<Translator> for Translator {
    fn div_assign(&mut self, other: Translator) {
        *self = (*self).div(other);
    }
}

impl Add<MultiVector> for Translator {
    type Output = MultiVector;

    fn add(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x3::from([self.group0()[0], self.group0()[0], self.group0()[3]]) * Simd32x3::from([0.0, 0.0, 1.0]) + other.group0(), g1: other.group1(), g2: other.group2(), g3: other.group3(), g4: other.group4(), g5: other.group5(), g6: other.group6(), g7: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) + other.group7(), g8: other.group8(), g9: other.group9() } }
    }
}

impl Sub<MultiVector> for Translator {
    type Output = MultiVector;

    fn sub(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x3::from([self.group0()[0], self.group0()[0], self.group0()[3]]) * Simd32x3::from([0.0, 0.0, 1.0]) - other.group0(), g1: Simd32x3::from(0.0) - other.group1(), g2: Simd32x2::from(0.0) - other.group2(), g3: Simd32x4::from(0.0) - other.group3(), g4: Simd32x3::from(0.0) - other.group4(), g5: Simd32x3::from(0.0) - other.group5(), g6: Simd32x3::from(0.0) - other.group6(), g7: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) - other.group7(), g8: Simd32x4::from(0.0) - other.group8(), g9: Simd32x4::from(0.0) - other.group9() } }
    }
}

impl Scale for Translator {
    type Output = Translator;

    fn scale(self, other: f32) -> Translator {
        self.geometric_product(Scalar { groups: ScalarGroups { g0: other } })
    }
}

impl Zero for Flector {
    fn zero() -> Self {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(0.0), g1: Simd32x4::from(0.0) } }
    }
}

impl One for Flector {
    fn one() -> Self {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(0.0), g1: Simd32x4::from(0.0) } }
    }
}

impl Neg for Flector {
    type Output = Flector;

    fn neg(self) -> Flector {
        Flector { groups: FlectorGroups { g0: self.group0() * Simd32x4::from(-1.0), g1: self.group1() * Simd32x4::from(-1.0) } }
    }
}

impl Automorphism for Flector {
    type Output = Flector;

    fn automorphism(self) -> Flector {
        Flector { groups: FlectorGroups { g0: self.group0(), g1: self.group1() } }
    }
}

impl Reversal for Flector {
    type Output = Flector;

    fn reversal(self) -> Flector {
        Flector { groups: FlectorGroups { g0: self.group0() * Simd32x4::from(-1.0), g1: self.group1() } }
    }
}

impl Conjugation for Flector {
    type Output = Flector;

    fn conjugation(self) -> Flector {
        Flector { groups: FlectorGroups { g0: self.group0() * Simd32x4::from(-1.0), g1: self.group1() } }
    }
}

impl GeometricProduct<Scalar> for Flector {
    type Output = Flector;

    fn geometric_product(self, other: Scalar) -> Flector {
        Flector { groups: FlectorGroups { g0: self.group0() * Simd32x4::from(other.group0()), g1: self.group1() * Simd32x4::from(other.group0()) } }
    }
}

impl OuterProduct<Scalar> for Flector {
    type Output = Flector;

    fn outer_product(self, other: Scalar) -> Flector {
        Flector { groups: FlectorGroups { g0: self.group0() * Simd32x4::from(other.group0()), g1: self.group1() * Simd32x4::from(other.group0()) } }
    }
}

impl InnerProduct<Scalar> for Flector {
    type Output = Flector;

    fn inner_product(self, other: Scalar) -> Flector {
        Flector { groups: FlectorGroups { g0: self.group0() * Simd32x4::from(other.group0()), g1: self.group1() * Simd32x4::from(other.group0()) } }
    }
}

impl RightContraction<Scalar> for Flector {
    type Output = Flector;

    fn right_contraction(self, other: Scalar) -> Flector {
        Flector { groups: FlectorGroups { g0: self.group0() * Simd32x4::from(other.group0()), g1: self.group1() * Simd32x4::from(other.group0()) } }
    }
}

impl GeometricProduct<RadialPoint> for Flector {
    type Output = Motor;

    fn geometric_product(self, other: RadialPoint) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group0()[0]]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group0()[2], other.group0()[2], other.group0()[1], other.group0()[0]]) * Simd32x4::from([0.0, -1.0, 1.0, 1.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group0()[2], other.group0()[2], other.group0()[0], other.group0()[1]]) * Simd32x4::from([1.0, 0.0, -1.0, 1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group0()[1], other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x4::from([-1.0, 1.0, 0.0, 1.0]) + Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]) * Simd32x4::from(other.group1()[0]), g1: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], other.group0()[2], other.group0()[0], other.group0()[1]]) * Simd32x4::from([-1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x4::from([1.0, -1.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group0()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[0], other.group0()[2], other.group0()[1], other.group0()[0]]) * Simd32x4::from([0.0, 1.0, -1.0, -1.0]) } }
    }
}

impl Into<FlatPoint> for Flector {
    fn into(self) -> FlatPoint {
        FlatPoint { groups: FlatPointGroups { g0: self.group0() } }
    }
}

impl Add<FlatPoint> for Flector {
    type Output = Flector;

    fn add(self, other: FlatPoint) -> Flector {
        Flector { groups: FlectorGroups { g0: self.group0() + other.group0(), g1: self.group1() } }
    }
}

impl AddAssign<FlatPoint> for Flector {
    fn add_assign(&mut self, other: FlatPoint) {
        *self = (*self).add(other);
    }
}

impl Sub<FlatPoint> for Flector {
    type Output = Flector;

    fn sub(self, other: FlatPoint) -> Flector {
        Flector { groups: FlectorGroups { g0: self.group0() - other.group0(), g1: self.group1() } }
    }
}

impl SubAssign<FlatPoint> for Flector {
    fn sub_assign(&mut self, other: FlatPoint) {
        *self = (*self).sub(other);
    }
}

impl GeometricProduct<Dipole> for Flector {
    type Output = Flector;

    fn geometric_product(self, other: Dipole) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group1()[2], other.group1()[2], other.group1()[0], other.group0()[1]]) * Simd32x4::from([-1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[1], other.group1()[0], other.group1()[1], other.group0()[2]]) * Simd32x4::from([1.0, -1.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from(other.group1()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from(other.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from(other.group1()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group1()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group1()[0], other.group1()[2], other.group1()[1], other.group0()[0]]) * Simd32x4::from([0.0, 1.0, -1.0, -1.0]), g1: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], other.group0()[2], other.group0()[0], other.group1()[1]]) * Simd32x4::from([-1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], other.group0()[0], other.group0()[1], other.group1()[2]]) * Simd32x4::from([1.0, -1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group1()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group1()[2], other.group1()[2], other.group1()[1], other.group1()[2]]) * Simd32x4::from([0.0, 1.0, -1.0, 0.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group1()[2], other.group1()[2], other.group1()[0], other.group1()[2]]) * Simd32x4::from([-1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group1()[1], other.group1()[0], other.group1()[1], other.group1()[1]]) * Simd32x4::from([1.0, -1.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group0()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[0], other.group0()[2], other.group0()[1], other.group1()[0]]) * Simd32x4::from([0.0, 1.0, -1.0, -1.0]) } }
    }
}

impl OuterProduct<Dipole> for Flector {
    type Output = Plane;

    fn outer_product(self, other: Dipole) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], other.group0()[2], other.group0()[0], other.group1()[1]]) * Simd32x4::from([-1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], other.group0()[0], other.group0()[1], other.group1()[2]]) * Simd32x4::from([1.0, -1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group1()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[0], other.group0()[2], other.group0()[1], other.group1()[0]]) * Simd32x4::from([0.0, 1.0, -1.0, -1.0]) } }
    }
}

impl InnerProduct<Dipole> for Flector {
    type Output = FlatPoint;

    fn inner_product(self, other: Dipole) -> FlatPoint {
        FlatPoint { groups: FlatPointGroups { g0: Simd32x4::from(self.group1()[1]) * Simd32x4::from(other.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from(other.group1()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + swizzle!(self.group1(), 3, 3, 3, 0) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group1()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) } }
    }
}

impl RightContraction<Dipole> for Flector {
    type Output = FlatPoint;

    fn right_contraction(self, other: Dipole) -> FlatPoint {
        FlatPoint { groups: FlatPointGroups { g0: Simd32x4::from(self.group1()[1]) * Simd32x4::from(other.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from(other.group1()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + swizzle!(self.group1(), 3, 3, 3, 0) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group1()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) } }
    }
}

impl GeometricProduct<Circle> for Flector {
    type Output = Motor;

    fn geometric_product(self, other: Circle) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 2, 2, 0, 1) * Simd32x4::from([-1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 1, 0, 1, 2) * Simd32x4::from([1.0, -1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from(other.group0()[3]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from(other.group0()[3]) * Simd32x4::from([0.0, -1.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from(other.group0()[3]) * Simd32x4::from([0.0, 0.0, -1.0, 0.0]) + Simd32x4::from(self.group1()[3]) * swizzle!(other.group0(), 0, 1, 2, 0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) + Simd32x4::from([self.group1()[0], self.group0()[0], self.group0()[0], self.group0()[0]]) * swizzle!(other.group0(), 3, 2, 1, 0) * Simd32x4::from([-1.0, 1.0, -1.0, -1.0]), g1: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]) * Simd32x4::from(other.group0()[3]) } }
    }
}

impl OuterProduct<Circle> for Flector {
    type Output = AntiScalar;

    fn outer_product(self, other: Circle) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2] - self.group0()[3] * other.group0()[3] } }
    }
}

impl Into<Plane> for Flector {
    fn into(self) -> Plane {
        Plane { groups: PlaneGroups { g0: self.group1() } }
    }
}

impl Add<Plane> for Flector {
    type Output = Flector;

    fn add(self, other: Plane) -> Flector {
        Flector { groups: FlectorGroups { g0: self.group0(), g1: self.group1() + other.group0() } }
    }
}

impl AddAssign<Plane> for Flector {
    fn add_assign(&mut self, other: Plane) {
        *self = (*self).add(other);
    }
}

impl Sub<Plane> for Flector {
    type Output = Flector;

    fn sub(self, other: Plane) -> Flector {
        Flector { groups: FlectorGroups { g0: self.group0(), g1: self.group1() - other.group0() } }
    }
}

impl SubAssign<Plane> for Flector {
    fn sub_assign(&mut self, other: Plane) {
        *self = (*self).sub(other);
    }
}

impl Add<Flector> for Flector {
    type Output = Flector;

    fn add(self, other: Flector) -> Flector {
        Flector { groups: FlectorGroups { g0: self.group0() + other.group0(), g1: self.group1() + other.group1() } }
    }
}

impl AddAssign<Flector> for Flector {
    fn add_assign(&mut self, other: Flector) {
        *self = (*self).add(other);
    }
}

impl Sub<Flector> for Flector {
    type Output = Flector;

    fn sub(self, other: Flector) -> Flector {
        Flector { groups: FlectorGroups { g0: self.group0() - other.group0(), g1: self.group1() - other.group1() } }
    }
}

impl SubAssign<Flector> for Flector {
    fn sub_assign(&mut self, other: Flector) {
        *self = (*self).sub(other);
    }
}

impl Mul<Flector> for Flector {
    type Output = Flector;

    fn mul(self, other: Flector) -> Flector {
        Flector { groups: FlectorGroups { g0: self.group0() * other.group0(), g1: self.group1() * other.group1() } }
    }
}

impl MulAssign<Flector> for Flector {
    fn mul_assign(&mut self, other: Flector) {
        *self = (*self).mul(other);
    }
}

impl Div<Flector> for Flector {
    type Output = Flector;

    fn div(self, other: Flector) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group0()[3]]) * Simd32x4::from([1.0, 1.0, 1.0, 1.0]) / Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group0()[3]]) * Simd32x4::from([1.0, 1.0, 1.0, 1.0]), g1: Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group1()[3]]) * Simd32x4::from([1.0, 1.0, 1.0, 1.0]) / Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group1()[3]]) * Simd32x4::from([1.0, 1.0, 1.0, 1.0]) } }
    }
}

impl DivAssign<Flector> for Flector {
    fn div_assign(&mut self, other: Flector) {
        *self = (*self).div(other);
    }
}

impl Add<MultiVector> for Flector {
    type Output = MultiVector;

    fn add(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: other.group0(), g1: other.group1(), g2: other.group2(), g3: self.group0() + other.group3(), g4: other.group4(), g5: other.group5(), g6: other.group6(), g7: other.group7(), g8: other.group8(), g9: self.group1() + other.group9() } }
    }
}

impl Sub<MultiVector> for Flector {
    type Output = MultiVector;

    fn sub(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x3::from(0.0) - other.group0(), g1: Simd32x3::from(0.0) - other.group1(), g2: Simd32x2::from(0.0) - other.group2(), g3: self.group0() - other.group3(), g4: Simd32x3::from(0.0) - other.group4(), g5: Simd32x3::from(0.0) - other.group5(), g6: Simd32x3::from(0.0) - other.group6(), g7: Simd32x3::from(0.0) - other.group7(), g8: Simd32x4::from(0.0) - other.group8(), g9: self.group1() - other.group9() } }
    }
}

impl Scale for Flector {
    type Output = Flector;

    fn scale(self, other: f32) -> Flector {
        self.geometric_product(Scalar { groups: ScalarGroups { g0: other } })
    }
}

impl Zero for MultiVector {
    fn zero() -> Self {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x3::from(0.0), g1: Simd32x3::from(0.0), g2: Simd32x2::from(0.0), g3: Simd32x4::from(0.0), g4: Simd32x3::from(0.0), g5: Simd32x3::from(0.0), g6: Simd32x3::from(0.0), g7: Simd32x3::from(0.0), g8: Simd32x4::from(0.0), g9: Simd32x4::from(0.0) } }
    }
}

impl One for MultiVector {
    fn one() -> Self {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x3::from([1.0, 0.0, 0.0]), g1: Simd32x3::from(0.0), g2: Simd32x2::from(0.0), g3: Simd32x4::from(0.0), g4: Simd32x3::from(0.0), g5: Simd32x3::from(0.0), g6: Simd32x3::from(0.0), g7: Simd32x3::from(0.0), g8: Simd32x4::from(0.0), g9: Simd32x4::from(0.0) } }
    }
}

impl Neg for MultiVector {
    type Output = MultiVector;

    fn neg(self) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() * Simd32x3::from(-1.0), g1: self.group1() * Simd32x3::from(-1.0), g2: self.group2() * Simd32x2::from(-1.0), g3: self.group3() * Simd32x4::from(-1.0), g4: self.group4() * Simd32x3::from(-1.0), g5: self.group5() * Simd32x3::from(-1.0), g6: self.group6() * Simd32x3::from(-1.0), g7: self.group7() * Simd32x3::from(-1.0), g8: self.group8() * Simd32x4::from(-1.0), g9: self.group9() * Simd32x4::from(-1.0) } }
    }
}

impl Automorphism for MultiVector {
    type Output = MultiVector;

    fn automorphism(self) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() * Simd32x3::from([1.0, 1.0, -1.0]), g1: self.group1() * Simd32x3::from(-1.0), g2: self.group2() * Simd32x2::from(-1.0), g3: self.group3(), g4: self.group4(), g5: self.group5(), g6: self.group6() * Simd32x3::from(-1.0), g7: self.group7() * Simd32x3::from(-1.0), g8: self.group8() * Simd32x4::from(-1.0), g9: self.group9() } }
    }
}

impl Reversal for MultiVector {
    type Output = MultiVector;

    fn reversal(self) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0(), g1: self.group1(), g2: self.group2(), g3: self.group3() * Simd32x4::from(-1.0), g4: self.group4() * Simd32x3::from(-1.0), g5: self.group5() * Simd32x3::from(-1.0), g6: self.group6() * Simd32x3::from(-1.0), g7: self.group7() * Simd32x3::from(-1.0), g8: self.group8() * Simd32x4::from(-1.0), g9: self.group9() } }
    }
}

impl Conjugation for MultiVector {
    type Output = MultiVector;

    fn conjugation(self) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() * Simd32x3::from([1.0, 1.0, -1.0]), g1: self.group1() * Simd32x3::from(-1.0), g2: self.group2() * Simd32x2::from(-1.0), g3: self.group3() * Simd32x4::from(-1.0), g4: self.group4() * Simd32x3::from(-1.0), g5: self.group5() * Simd32x3::from(-1.0), g6: self.group6(), g7: self.group7(), g8: self.group8(), g9: self.group9() } }
    }
}

impl Into<Scalar> for MultiVector {
    fn into(self) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[0] } }
    }
}

impl Add<Scalar> for MultiVector {
    type Output = MultiVector;

    fn add(self, other: Scalar) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() + Simd32x3::from(other.group0()) * Simd32x3::from([1.0, 0.0, 0.0]), g1: self.group1(), g2: self.group2(), g3: self.group3(), g4: self.group4(), g5: self.group5(), g6: self.group6(), g7: self.group7(), g8: self.group8(), g9: self.group9() } }
    }
}

impl AddAssign<Scalar> for MultiVector {
    fn add_assign(&mut self, other: Scalar) {
        *self = (*self).add(other);
    }
}

impl Sub<Scalar> for MultiVector {
    type Output = MultiVector;

    fn sub(self, other: Scalar) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() - Simd32x3::from(other.group0()) * Simd32x3::from([1.0, 0.0, 0.0]), g1: self.group1(), g2: self.group2(), g3: self.group3(), g4: self.group4(), g5: self.group5(), g6: self.group6(), g7: self.group7(), g8: self.group8(), g9: self.group9() } }
    }
}

impl SubAssign<Scalar> for MultiVector {
    fn sub_assign(&mut self, other: Scalar) {
        *self = (*self).sub(other);
    }
}

impl GeometricProduct<Scalar> for MultiVector {
    type Output = MultiVector;

    fn geometric_product(self, other: Scalar) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() * Simd32x3::from(other.group0()), g1: self.group1() * Simd32x3::from(other.group0()), g2: self.group2() * Simd32x2::from(other.group0()), g3: self.group3() * Simd32x4::from(other.group0()), g4: self.group4() * Simd32x3::from(other.group0()), g5: self.group5() * Simd32x3::from(other.group0()), g6: self.group6() * Simd32x3::from(other.group0()), g7: self.group7() * Simd32x3::from(other.group0()), g8: self.group8() * Simd32x4::from(other.group0()), g9: self.group9() * Simd32x4::from(other.group0()) } }
    }
}

impl OuterProduct<Scalar> for MultiVector {
    type Output = MultiVector;

    fn outer_product(self, other: Scalar) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() * Simd32x3::from(other.group0()), g1: self.group1() * Simd32x3::from(other.group0()), g2: self.group2() * Simd32x2::from(other.group0()), g3: self.group3() * Simd32x4::from(other.group0()), g4: self.group4() * Simd32x3::from(other.group0()), g5: self.group5() * Simd32x3::from(other.group0()), g6: self.group6() * Simd32x3::from(other.group0()), g7: self.group7() * Simd32x3::from(other.group0()), g8: self.group8() * Simd32x4::from(other.group0()), g9: self.group9() * Simd32x4::from(other.group0()) } }
    }
}

impl InnerProduct<Scalar> for MultiVector {
    type Output = MultiVector;

    fn inner_product(self, other: Scalar) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() * Simd32x3::from(other.group0()), g1: self.group1() * Simd32x3::from(other.group0()), g2: self.group2() * Simd32x2::from(other.group0()), g3: self.group3() * Simd32x4::from(other.group0()), g4: self.group4() * Simd32x3::from(other.group0()), g5: self.group5() * Simd32x3::from(other.group0()), g6: self.group6() * Simd32x3::from(other.group0()), g7: self.group7() * Simd32x3::from(other.group0()), g8: self.group8() * Simd32x4::from(other.group0()), g9: self.group9() * Simd32x4::from(other.group0()) } }
    }
}

impl LeftContraction<Scalar> for MultiVector {
    type Output = Scalar;

    fn left_contraction(self, other: Scalar) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[0] * other.group0() } }
    }
}

impl RightContraction<Scalar> for MultiVector {
    type Output = MultiVector;

    fn right_contraction(self, other: Scalar) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() * Simd32x3::from(other.group0()), g1: self.group1() * Simd32x3::from(other.group0()), g2: self.group2() * Simd32x2::from(other.group0()), g3: self.group3() * Simd32x4::from(other.group0()), g4: self.group4() * Simd32x3::from(other.group0()), g5: self.group5() * Simd32x3::from(other.group0()), g6: self.group6() * Simd32x3::from(other.group0()), g7: self.group7() * Simd32x3::from(other.group0()), g8: self.group8() * Simd32x4::from(other.group0()), g9: self.group9() * Simd32x4::from(other.group0()) } }
    }
}

impl ScalarProduct<Scalar> for MultiVector {
    type Output = Scalar;

    fn scalar_product(self, other: Scalar) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[0] * other.group0() } }
    }
}

impl Into<AntiScalar> for MultiVector {
    fn into(self) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0()[2] } }
    }
}

impl Add<AntiScalar> for MultiVector {
    type Output = MultiVector;

    fn add(self, other: AntiScalar) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() + Simd32x3::from(other.group0()) * Simd32x3::from([0.0, 0.0, 1.0]), g1: self.group1(), g2: self.group2(), g3: self.group3(), g4: self.group4(), g5: self.group5(), g6: self.group6(), g7: self.group7(), g8: self.group8(), g9: self.group9() } }
    }
}

impl AddAssign<AntiScalar> for MultiVector {
    fn add_assign(&mut self, other: AntiScalar) {
        *self = (*self).add(other);
    }
}

impl Sub<AntiScalar> for MultiVector {
    type Output = MultiVector;

    fn sub(self, other: AntiScalar) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() - Simd32x3::from(other.group0()) * Simd32x3::from([0.0, 0.0, 1.0]), g1: self.group1(), g2: self.group2(), g3: self.group3(), g4: self.group4(), g5: self.group5(), g6: self.group6(), g7: self.group7(), g8: self.group8(), g9: self.group9() } }
    }
}

impl SubAssign<AntiScalar> for MultiVector {
    fn sub_assign(&mut self, other: AntiScalar) {
        *self = (*self).sub(other);
    }
}

impl OuterProduct<AntiScalar> for MultiVector {
    type Output = AntiScalar;

    fn outer_product(self, other: AntiScalar) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0()[0] * other.group0() } }
    }
}

impl Into<RadialPoint> for MultiVector {
    fn into(self) -> RadialPoint {
        RadialPoint { groups: RadialPointGroups { g0: self.group1(), g1: self.group2() } }
    }
}

impl Add<RadialPoint> for MultiVector {
    type Output = MultiVector;

    fn add(self, other: RadialPoint) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0(), g1: self.group1() + other.group0(), g2: self.group2() + other.group1(), g3: self.group3(), g4: self.group4(), g5: self.group5(), g6: self.group6(), g7: self.group7(), g8: self.group8(), g9: self.group9() } }
    }
}

impl AddAssign<RadialPoint> for MultiVector {
    fn add_assign(&mut self, other: RadialPoint) {
        *self = (*self).add(other);
    }
}

impl Sub<RadialPoint> for MultiVector {
    type Output = MultiVector;

    fn sub(self, other: RadialPoint) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0(), g1: self.group1() - other.group0(), g2: self.group2() - other.group1(), g3: self.group3(), g4: self.group4(), g5: self.group5(), g6: self.group6(), g7: self.group7(), g8: self.group8(), g9: self.group9() } }
    }
}

impl SubAssign<RadialPoint> for MultiVector {
    fn sub_assign(&mut self, other: RadialPoint) {
        *self = (*self).sub(other);
    }
}

impl GeometricProduct<RadialPoint> for MultiVector {
    type Output = MultiVector;

    fn geometric_product(self, other: RadialPoint) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x3::from(self.group1()[0]) * Simd32x3::from(other.group0()[0]) * Simd32x3::from([1.0, 0.0, 0.0]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from(other.group0()[1]) * Simd32x3::from([1.0, 0.0, 0.0]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from(other.group0()[2]) * Simd32x3::from([1.0, 0.0, 0.0]) + Simd32x3::from(self.group8()[0]) * Simd32x3::from(other.group0()[0]) * Simd32x3::from([0.0, -1.0, 0.0]) + Simd32x3::from(self.group8()[1]) * Simd32x3::from(other.group0()[1]) * Simd32x3::from([0.0, -1.0, 0.0]) + Simd32x3::from(self.group8()[2]) * Simd32x3::from(other.group0()[2]) * Simd32x3::from([0.0, -1.0, 0.0]) + Simd32x3::from(self.group8()[3]) * Simd32x3::from(other.group1()[0]) * Simd32x3::from([0.0, -1.0, 0.0]) + Simd32x3::from(self.group9()[0]) * Simd32x3::from(other.group0()[0]) * Simd32x3::from([0.0, 0.0, 1.0]) + Simd32x3::from(self.group9()[1]) * Simd32x3::from(other.group0()[1]) * Simd32x3::from([0.0, 0.0, 1.0]) + Simd32x3::from(self.group9()[2]) * Simd32x3::from(other.group0()[2]) * Simd32x3::from([0.0, 0.0, 1.0]) + Simd32x3::from(self.group9()[3]) * Simd32x3::from(other.group1()[0]) * Simd32x3::from([0.0, 0.0, 1.0]) + swizzle!(self.group0(), 0, 0, 1) * Simd32x3::from([other.group1()[0], other.group1()[0], other.group1()[1]]) * Simd32x3::from([0.0, 0.0, 1.0]), g1: Simd32x3::from(self.group0()[0]) * other.group0() + Simd32x3::from(self.group5()[1]) * swizzle!(other.group0(), 2, 2, 0) * Simd32x3::from([-1.0, 0.0, 1.0]) + Simd32x3::from(self.group5()[2]) * swizzle!(other.group0(), 1, 0, 1) * Simd32x3::from([1.0, -1.0, 0.0]) + Simd32x3::from(self.group5()[0]) * swizzle!(other.group0(), 0, 2, 1) * Simd32x3::from([0.0, 1.0, -1.0]), g2: Simd32x2::from(self.group0()[0]) * other.group1() + Simd32x2::from(self.group3()[1]) * Simd32x2::from(other.group0()[1]) * Simd32x2::from([0.0, -1.0]) + Simd32x2::from(self.group3()[2]) * Simd32x2::from(other.group0()[2]) * Simd32x2::from([0.0, -1.0]) + Simd32x2::from(self.group4()[0]) * Simd32x2::from(other.group0()[0]) * Simd32x2::from([1.0, 0.0]) + Simd32x2::from(self.group4()[1]) * Simd32x2::from(other.group0()[1]) * Simd32x2::from([1.0, 0.0]) + Simd32x2::from(self.group4()[2]) * Simd32x2::from(other.group0()[2]) * Simd32x2::from([1.0, 0.0]) + Simd32x2::from(self.group3()[0]) * Simd32x2::from(other.group0()[0]) * Simd32x2::from([0.0, -1.0]), g3: Simd32x4::from(self.group2()[0]) * Simd32x4::from(other.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) - Simd32x4::from(self.group2()[1]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[0]]) + Simd32x4::from(self.group6()[0]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group6()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group6()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group7()[0]) * Simd32x4::from([other.group0()[2], other.group0()[2], other.group0()[1], other.group0()[2]]) * Simd32x4::from([0.0, -1.0, 1.0, 0.0]) + Simd32x4::from(self.group7()[1]) * Simd32x4::from([other.group0()[2], other.group0()[2], other.group0()[0], other.group0()[2]]) * Simd32x4::from([1.0, 0.0, -1.0, 0.0]) + Simd32x4::from(self.group7()[2]) * Simd32x4::from([other.group0()[1], other.group0()[0], other.group0()[1], other.group0()[1]]) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) + Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group1()[0]]) * Simd32x4::from([other.group1()[1], other.group1()[1], other.group1()[1], other.group1()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]), g4: Simd32x3::from(self.group2()[0]) * other.group0() + Simd32x3::from(self.group8()[0]) * swizzle!(other.group0(), 2, 2, 1) * Simd32x3::from([0.0, 1.0, -1.0]) + Simd32x3::from(self.group8()[1]) * swizzle!(other.group0(), 2, 2, 0) * Simd32x3::from([-1.0, 0.0, 1.0]) + Simd32x3::from(self.group8()[2]) * swizzle!(other.group0(), 1, 0, 1) * Simd32x3::from([1.0, -1.0, 0.0]) + self.group1() * Simd32x3::from(other.group1()[0]) * Simd32x3::from(-1.0), g5: Simd32x3::from(self.group1()[1]) * swizzle!(other.group0(), 2, 2, 0) * Simd32x3::from([1.0, 0.0, -1.0]) + Simd32x3::from(self.group1()[2]) * swizzle!(other.group0(), 1, 0, 1) * Simd32x3::from([-1.0, 1.0, 0.0]) - Simd32x3::from(self.group8()[3]) * other.group0() + Simd32x3::from(self.group1()[0]) * swizzle!(other.group0(), 0, 2, 1) * Simd32x3::from([0.0, -1.0, 1.0]), g6: Simd32x3::from(0.0) - Simd32x3::from(self.group3()[3]) * other.group0() + Simd32x3::from(self.group4()[0]) * Simd32x3::from(other.group1()[1]) * Simd32x3::from([1.0, 0.0, 0.0]) + Simd32x3::from(self.group4()[1]) * Simd32x3::from(other.group1()[1]) * Simd32x3::from([0.0, 1.0, 0.0]) + Simd32x3::from(self.group4()[2]) * Simd32x3::from(other.group1()[1]) * Simd32x3::from([0.0, 0.0, 1.0]) + Simd32x3::from(self.group9()[0]) * swizzle!(other.group0(), 2, 2, 1) * Simd32x3::from([0.0, -1.0, 1.0]) + Simd32x3::from(self.group9()[1]) * swizzle!(other.group0(), 2, 2, 0) * Simd32x3::from([1.0, 0.0, -1.0]) + Simd32x3::from(self.group9()[2]) * swizzle!(other.group0(), 1, 0, 1) * Simd32x3::from([-1.0, 1.0, 0.0]) + Simd32x3::from([self.group3()[0], self.group3()[1], self.group3()[2]]) * Simd32x3::from(other.group1()[0]), g7: Simd32x3::from(self.group3()[1]) * swizzle!(other.group0(), 2, 2, 0) * Simd32x3::from([-1.0, 0.0, 1.0]) + Simd32x3::from(self.group3()[2]) * swizzle!(other.group0(), 1, 0, 1) * Simd32x3::from([1.0, -1.0, 0.0]) + Simd32x3::from(self.group5()[0]) * Simd32x3::from(other.group1()[1]) * Simd32x3::from([1.0, 0.0, 0.0]) + Simd32x3::from(self.group5()[1]) * Simd32x3::from(other.group1()[1]) * Simd32x3::from([0.0, 1.0, 0.0]) + Simd32x3::from(self.group5()[2]) * Simd32x3::from(other.group1()[1]) * Simd32x3::from([0.0, 0.0, 1.0]) + Simd32x3::from(self.group9()[3]) * other.group0() + Simd32x3::from(self.group3()[0]) * swizzle!(other.group0(), 0, 2, 1) * Simd32x3::from([0.0, 1.0, -1.0]), g8: Simd32x4::from(self.group4()[0]) * Simd32x4::from([other.group0()[2], other.group0()[2], other.group0()[1], other.group0()[2]]) * Simd32x4::from([0.0, -1.0, 1.0, 0.0]) + Simd32x4::from(self.group4()[1]) * Simd32x4::from([other.group0()[2], other.group0()[2], other.group0()[0], other.group0()[2]]) * Simd32x4::from([1.0, 0.0, -1.0, 0.0]) + Simd32x4::from(self.group4()[2]) * Simd32x4::from([other.group0()[1], other.group0()[0], other.group0()[1], other.group0()[1]]) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group5()[0]) * Simd32x4::from([other.group1()[0], other.group1()[0], other.group1()[0], other.group0()[0]]) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group5()[1]) * Simd32x4::from([other.group1()[0], other.group1()[0], other.group1()[0], other.group0()[1]]) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group5()[2]) * Simd32x4::from([other.group1()[0], other.group1()[0], other.group1()[0], other.group0()[2]]) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from([self.group0()[1], self.group0()[1], self.group0()[1], self.group0()[0]]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group0()[0]]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]), g9: Simd32x4::from(self.group6()[0]) * Simd32x4::from([other.group0()[2], other.group0()[2], other.group0()[1], other.group0()[2]]) * Simd32x4::from([0.0, 1.0, -1.0, 0.0]) + Simd32x4::from(self.group6()[1]) * Simd32x4::from([other.group0()[2], other.group0()[2], other.group0()[0], other.group0()[2]]) * Simd32x4::from([-1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group6()[2]) * Simd32x4::from([other.group0()[1], other.group0()[0], other.group0()[1], other.group0()[1]]) * Simd32x4::from([1.0, -1.0, 0.0, 0.0]) + Simd32x4::from(self.group7()[0]) * Simd32x4::from([other.group1()[0], other.group1()[0], other.group1()[0], other.group0()[0]]) * Simd32x4::from([-1.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group7()[1]) * Simd32x4::from([other.group1()[0], other.group1()[0], other.group1()[0], other.group0()[1]]) * Simd32x4::from([0.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.group7()[2]) * Simd32x4::from([other.group1()[0], other.group1()[0], other.group1()[0], other.group0()[2]]) * Simd32x4::from([0.0, 0.0, -1.0, 1.0]) + Simd32x4::from(self.group8()[0]) * Simd32x4::from(other.group1()[1]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group8()[1]) * Simd32x4::from(other.group1()[1]) * Simd32x4::from([0.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group8()[2]) * Simd32x4::from(other.group1()[1]) * Simd32x4::from([0.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group8()[3]) * Simd32x4::from(other.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from([self.group0()[2], self.group0()[2], self.group0()[2], self.group0()[0]]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group0()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) } }
    }
}

impl ScalarProduct<RadialPoint> for MultiVector {
    type Output = Scalar;

    fn scalar_product(self, other: RadialPoint) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group1()[0] * other.group0()[0] + self.group1()[1] * other.group0()[1] + self.group1()[2] * other.group0()[2] } }
    }
}

impl Into<FlatPoint> for MultiVector {
    fn into(self) -> FlatPoint {
        FlatPoint { groups: FlatPointGroups { g0: self.group3() } }
    }
}

impl Add<FlatPoint> for MultiVector {
    type Output = MultiVector;

    fn add(self, other: FlatPoint) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0(), g1: self.group1(), g2: self.group2(), g3: self.group3() + other.group0(), g4: self.group4(), g5: self.group5(), g6: self.group6(), g7: self.group7(), g8: self.group8(), g9: self.group9() } }
    }
}

impl AddAssign<FlatPoint> for MultiVector {
    fn add_assign(&mut self, other: FlatPoint) {
        *self = (*self).add(other);
    }
}

impl Sub<FlatPoint> for MultiVector {
    type Output = MultiVector;

    fn sub(self, other: FlatPoint) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0(), g1: self.group1(), g2: self.group2(), g3: self.group3() - other.group0(), g4: self.group4(), g5: self.group5(), g6: self.group6(), g7: self.group7(), g8: self.group8(), g9: self.group9() } }
    }
}

impl SubAssign<FlatPoint> for MultiVector {
    fn sub_assign(&mut self, other: FlatPoint) {
        *self = (*self).sub(other);
    }
}

impl Into<Dipole> for MultiVector {
    fn into(self) -> Dipole {
        Dipole { groups: DipoleGroups { g0: self.group4(), g1: self.group5(), g2: self.group3() } }
    }
}

impl Add<Dipole> for MultiVector {
    type Output = MultiVector;

    fn add(self, other: Dipole) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0(), g1: self.group1(), g2: self.group2(), g3: self.group3() + other.group2(), g4: self.group4() + other.group0(), g5: self.group5() + other.group1(), g6: self.group6(), g7: self.group7(), g8: self.group8(), g9: self.group9() } }
    }
}

impl AddAssign<Dipole> for MultiVector {
    fn add_assign(&mut self, other: Dipole) {
        *self = (*self).add(other);
    }
}

impl Sub<Dipole> for MultiVector {
    type Output = MultiVector;

    fn sub(self, other: Dipole) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0(), g1: self.group1(), g2: self.group2(), g3: self.group3() - other.group2(), g4: self.group4() - other.group0(), g5: self.group5() - other.group1(), g6: self.group6(), g7: self.group7(), g8: self.group8(), g9: self.group9() } }
    }
}

impl SubAssign<Dipole> for MultiVector {
    fn sub_assign(&mut self, other: Dipole) {
        *self = (*self).sub(other);
    }
}

impl GeometricProduct<Dipole> for MultiVector {
    type Output = MultiVector;

    fn geometric_product(self, other: Dipole) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x3::from(self.group4()[1]) * Simd32x3::from(other.group1()[1]) * Simd32x3::from([0.0, -1.0, 0.0]) + Simd32x3::from(self.group4()[2]) * Simd32x3::from(other.group1()[2]) * Simd32x3::from([0.0, -1.0, 0.0]) + Simd32x3::from(self.group5()[0]) * Simd32x3::from([other.group1()[0], other.group0()[0], other.group1()[0]]) * Simd32x3::from([-1.0, -1.0, 0.0]) + Simd32x3::from(self.group5()[1]) * Simd32x3::from([other.group1()[1], other.group0()[1], other.group1()[1]]) * Simd32x3::from([-1.0, -1.0, 0.0]) + Simd32x3::from(self.group5()[2]) * Simd32x3::from([other.group1()[2], other.group0()[2], other.group1()[2]]) * Simd32x3::from([-1.0, -1.0, 0.0]) + Simd32x3::from(self.group6()[1]) * Simd32x3::from(other.group1()[1]) * Simd32x3::from([0.0, 0.0, -1.0]) + Simd32x3::from(self.group6()[2]) * Simd32x3::from(other.group1()[2]) * Simd32x3::from([0.0, 0.0, -1.0]) + Simd32x3::from(self.group7()[0]) * Simd32x3::from(other.group0()[0]) * Simd32x3::from([0.0, 0.0, -1.0]) + Simd32x3::from(self.group7()[1]) * Simd32x3::from(other.group0()[1]) * Simd32x3::from([0.0, 0.0, -1.0]) + Simd32x3::from(self.group7()[2]) * Simd32x3::from(other.group0()[2]) * Simd32x3::from([0.0, 0.0, -1.0]) + Simd32x3::from(self.group8()[0]) * Simd32x3::from(other.group2()[0]) * Simd32x3::from([0.0, 0.0, -1.0]) + Simd32x3::from(self.group8()[1]) * Simd32x3::from(other.group2()[1]) * Simd32x3::from([0.0, 0.0, -1.0]) + Simd32x3::from(self.group8()[2]) * Simd32x3::from(other.group2()[2]) * Simd32x3::from([0.0, 0.0, -1.0]) + Simd32x3::from(self.group8()[3]) * Simd32x3::from(other.group2()[3]) * Simd32x3::from([0.0, 0.0, -1.0]) + Simd32x3::from([self.group4()[0], self.group4()[0], self.group6()[0]]) * Simd32x3::from(other.group1()[0]) * Simd32x3::from([0.0, -1.0, -1.0]), g1: Simd32x3::from(self.group1()[1]) * swizzle!(other.group1(), 2, 2, 0) * Simd32x3::from([-1.0, 0.0, 1.0]) + Simd32x3::from(self.group1()[2]) * swizzle!(other.group1(), 1, 0, 1) * Simd32x3::from([1.0, -1.0, 0.0]) + Simd32x3::from(self.group8()[3]) * other.group1() + Simd32x3::from(self.group1()[0]) * swizzle!(other.group1(), 0, 2, 1) * Simd32x3::from([0.0, 1.0, -1.0]), g2: Simd32x2::from(self.group1()[0]) * Simd32x2::from([other.group0()[0], other.group2()[0]]) * Simd32x2::from([-1.0, 1.0]) + Simd32x2::from(self.group1()[1]) * Simd32x2::from([other.group0()[1], other.group2()[1]]) * Simd32x2::from([-1.0, 1.0]) + Simd32x2::from(self.group1()[2]) * Simd32x2::from([other.group0()[2], other.group2()[2]]) * Simd32x2::from([-1.0, 1.0]) + Simd32x2::from(self.group7()[1]) * Simd32x2::from(other.group1()[1]) * Simd32x2::from([0.0, -1.0]) + Simd32x2::from(self.group7()[2]) * Simd32x2::from(other.group1()[2]) * Simd32x2::from([0.0, -1.0]) + Simd32x2::from(self.group8()[0]) * Simd32x2::from(other.group1()[0]) * Simd32x2::from([-1.0, 0.0]) + Simd32x2::from(self.group8()[1]) * Simd32x2::from(other.group1()[1]) * Simd32x2::from([-1.0, 0.0]) + Simd32x2::from(self.group8()[2]) * Simd32x2::from(other.group1()[2]) * Simd32x2::from([-1.0, 0.0]) + Simd32x2::from(self.group7()[0]) * Simd32x2::from(other.group1()[0]) * Simd32x2::from([0.0, -1.0]), g3: Simd32x4::from(self.group0()[0]) * other.group2() + Simd32x4::from(self.group3()[1]) * Simd32x4::from([other.group1()[2], other.group1()[2], other.group1()[0], other.group0()[1]]) * Simd32x4::from([-1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group3()[2]) * Simd32x4::from([other.group1()[1], other.group1()[0], other.group1()[1], other.group0()[2]]) * Simd32x4::from([1.0, -1.0, 0.0, -1.0]) + Simd32x4::from(self.group4()[0]) * Simd32x4::from(other.group2()[0]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group4()[1]) * Simd32x4::from(other.group2()[1]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group4()[2]) * Simd32x4::from(other.group2()[2]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group5()[0]) * swizzle!(other.group2(), 2, 2, 1, 2) * Simd32x4::from([0.0, 1.0, -1.0, 0.0]) + Simd32x4::from(self.group5()[1]) * swizzle!(other.group2(), 2, 2, 0, 2) * Simd32x4::from([-1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group5()[2]) * swizzle!(other.group2(), 1, 0, 1, 1) * Simd32x4::from([1.0, -1.0, 0.0, 0.0]) + Simd32x4::from(self.group9()[0]) * Simd32x4::from(other.group1()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group9()[1]) * Simd32x4::from(other.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group9()[2]) * Simd32x4::from(other.group1()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group9()[3]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group1()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) + Simd32x4::from(self.group3()[0]) * Simd32x4::from([other.group1()[0], other.group1()[2], other.group1()[1], other.group0()[0]]) * Simd32x4::from([0.0, 1.0, -1.0, -1.0]), g4: Simd32x3::from(self.group0()[0]) * other.group0() + Simd32x3::from(self.group0()[1]) * other.group1() + Simd32x3::from(self.group4()[1]) * swizzle!(other.group1(), 2, 2, 0) * Simd32x3::from([-1.0, 0.0, 1.0]) + Simd32x3::from(self.group4()[2]) * swizzle!(other.group1(), 1, 0, 1) * Simd32x3::from([1.0, -1.0, 0.0]) + Simd32x3::from(self.group5()[0]) * swizzle!(other.group0(), 2, 2, 1) * Simd32x3::from([0.0, 1.0, -1.0]) + Simd32x3::from(self.group5()[1]) * swizzle!(other.group0(), 2, 2, 0) * Simd32x3::from([-1.0, 0.0, 1.0]) + Simd32x3::from(self.group5()[2]) * swizzle!(other.group0(), 1, 0, 1) * Simd32x3::from([1.0, -1.0, 0.0]) + Simd32x3::from(self.group4()[0]) * swizzle!(other.group1(), 0, 2, 1) * Simd32x3::from([0.0, 1.0, -1.0]), g5: Simd32x3::from(self.group0()[0]) * other.group1() + Simd32x3::from(self.group5()[1]) * swizzle!(other.group1(), 2, 2, 0) * Simd32x3::from([-1.0, 0.0, 1.0]) + Simd32x3::from(self.group5()[2]) * swizzle!(other.group1(), 1, 0, 1) * Simd32x3::from([1.0, -1.0, 0.0]) + Simd32x3::from(self.group5()[0]) * swizzle!(other.group1(), 0, 2, 1) * Simd32x3::from([0.0, 1.0, -1.0]), g6: Simd32x3::from(self.group0()[2]) * other.group1() + Simd32x3::from(self.group2()[0]) * Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]) + Simd32x3::from(self.group2()[1]) * other.group0() + Simd32x3::from(self.group6()[0]) * swizzle!(other.group1(), 2, 2, 1) * Simd32x3::from([0.0, 1.0, -1.0]) + Simd32x3::from(self.group6()[1]) * swizzle!(other.group1(), 2, 2, 0) * Simd32x3::from([-1.0, 0.0, 1.0]) + Simd32x3::from(self.group6()[2]) * swizzle!(other.group1(), 1, 0, 1) * Simd32x3::from([1.0, -1.0, 0.0]) + Simd32x3::from(self.group7()[0]) * swizzle!(other.group0(), 2, 2, 1) * Simd32x3::from([0.0, 1.0, -1.0]) + Simd32x3::from(self.group7()[1]) * swizzle!(other.group0(), 2, 2, 0) * Simd32x3::from([-1.0, 0.0, 1.0]) + Simd32x3::from(self.group7()[2]) * swizzle!(other.group0(), 1, 0, 1) * Simd32x3::from([1.0, -1.0, 0.0]) + Simd32x3::from(self.group8()[0]) * Simd32x3::from([other.group2()[2], other.group2()[2], other.group2()[1]]) * Simd32x3::from([0.0, 1.0, -1.0]) + Simd32x3::from(self.group8()[1]) * Simd32x3::from([other.group2()[2], other.group2()[2], other.group2()[0]]) * Simd32x3::from([-1.0, 0.0, 1.0]) + Simd32x3::from(self.group8()[2]) * Simd32x3::from([other.group2()[1], other.group2()[0], other.group2()[1]]) * Simd32x3::from([1.0, -1.0, 0.0]) + self.group1() * Simd32x3::from(other.group2()[3]) * Simd32x3::from(-1.0), g7: Simd32x3::from(self.group1()[1]) * Simd32x3::from([other.group2()[2], other.group2()[2], other.group2()[0]]) * Simd32x3::from([1.0, 0.0, -1.0]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group2()[1], other.group2()[0], other.group2()[1]]) * Simd32x3::from([-1.0, 1.0, 0.0]) + Simd32x3::from(self.group2()[1]) * other.group1() + Simd32x3::from(self.group7()[0]) * swizzle!(other.group1(), 2, 2, 1) * Simd32x3::from([0.0, 1.0, -1.0]) + Simd32x3::from(self.group7()[1]) * swizzle!(other.group1(), 2, 2, 0) * Simd32x3::from([-1.0, 0.0, 1.0]) + Simd32x3::from(self.group7()[2]) * swizzle!(other.group1(), 1, 0, 1) * Simd32x3::from([1.0, -1.0, 0.0]) - Simd32x3::from(self.group8()[3]) * Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]) + Simd32x3::from(self.group1()[0]) * Simd32x3::from([other.group2()[0], other.group2()[2], other.group2()[1]]) * Simd32x3::from([0.0, -1.0, 1.0]), g8: Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group0()[2], other.group0()[2], other.group0()[0], other.group1()[1]]) * Simd32x4::from([-1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group0()[1], other.group0()[0], other.group0()[1], other.group1()[2]]) * Simd32x4::from([1.0, -1.0, 0.0, -1.0]) + Simd32x4::from(self.group2()[0]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group1()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) + Simd32x4::from(self.group8()[0]) * Simd32x4::from([other.group1()[2], other.group1()[2], other.group1()[1], other.group1()[2]]) * Simd32x4::from([0.0, 1.0, -1.0, 0.0]) + Simd32x4::from(self.group8()[1]) * Simd32x4::from([other.group1()[2], other.group1()[2], other.group1()[0], other.group1()[2]]) * Simd32x4::from([-1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group8()[2]) * Simd32x4::from([other.group1()[1], other.group1()[0], other.group1()[1], other.group1()[1]]) * Simd32x4::from([1.0, -1.0, 0.0, 0.0]) + Simd32x4::from(self.group8()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group0()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group0()[0], other.group0()[2], other.group0()[1], other.group1()[0]]) * Simd32x4::from([0.0, 1.0, -1.0, -1.0]), g9: Simd32x4::from(self.group3()[0]) * Simd32x4::from([other.group0()[2], other.group0()[2], other.group0()[1], other.group1()[0]]) * Simd32x4::from([0.0, 1.0, -1.0, -1.0]) + Simd32x4::from(self.group3()[1]) * Simd32x4::from([other.group0()[2], other.group0()[2], other.group0()[0], other.group1()[1]]) * Simd32x4::from([-1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group3()[2]) * Simd32x4::from([other.group0()[1], other.group0()[0], other.group0()[1], other.group1()[2]]) * Simd32x4::from([1.0, -1.0, 0.0, -1.0]) + Simd32x4::from(self.group3()[3]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group1()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) + Simd32x4::from(self.group4()[0]) * swizzle!(other.group2(), 2, 2, 1, 2) * Simd32x4::from([0.0, -1.0, 1.0, 0.0]) + Simd32x4::from(self.group4()[1]) * swizzle!(other.group2(), 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, -1.0, 0.0]) + Simd32x4::from(self.group4()[2]) * swizzle!(other.group2(), 1, 0, 1, 1) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group5()[0]) * swizzle!(other.group2(), 3, 3, 3, 0) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group5()[1]) * swizzle!(other.group2(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group5()[2]) * swizzle!(other.group2(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group9()[0]) * Simd32x4::from([other.group1()[2], other.group1()[2], other.group1()[1], other.group1()[2]]) * Simd32x4::from([0.0, 1.0, -1.0, 0.0]) + Simd32x4::from(self.group9()[1]) * Simd32x4::from([other.group1()[2], other.group1()[2], other.group1()[0], other.group1()[2]]) * Simd32x4::from([-1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group9()[2]) * Simd32x4::from([other.group1()[1], other.group1()[0], other.group1()[1], other.group1()[1]]) * Simd32x4::from([1.0, -1.0, 0.0, 0.0]) + Simd32x4::from(self.group9()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group0()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) + Simd32x4::from([self.group0()[1], self.group0()[1], self.group0()[1], self.group0()[0]]) * swizzle!(other.group2(), 0, 1, 2, 0) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]) } }
    }
}

impl ScalarProduct<Dipole> for MultiVector {
    type Output = Scalar;

    fn scalar_product(self, other: Dipole) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group5()[0] * other.group1()[0] - self.group5()[1] * other.group1()[1] - self.group5()[2] * other.group1()[2] } }
    }
}

impl Into<Line> for MultiVector {
    fn into(self) -> Line {
        Line { groups: LineGroups { g0: self.group6(), g1: self.group7() } }
    }
}

impl Add<Line> for MultiVector {
    type Output = MultiVector;

    fn add(self, other: Line) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0(), g1: self.group1(), g2: self.group2(), g3: self.group3(), g4: self.group4(), g5: self.group5(), g6: self.group6() + other.group0(), g7: self.group7() + other.group1(), g8: self.group8(), g9: self.group9() } }
    }
}

impl AddAssign<Line> for MultiVector {
    fn add_assign(&mut self, other: Line) {
        *self = (*self).add(other);
    }
}

impl Sub<Line> for MultiVector {
    type Output = MultiVector;

    fn sub(self, other: Line) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0(), g1: self.group1(), g2: self.group2(), g3: self.group3(), g4: self.group4(), g5: self.group5(), g6: self.group6() - other.group0(), g7: self.group7() - other.group1(), g8: self.group8(), g9: self.group9() } }
    }
}

impl SubAssign<Line> for MultiVector {
    fn sub_assign(&mut self, other: Line) {
        *self = (*self).sub(other);
    }
}

impl Into<Circle> for MultiVector {
    fn into(self) -> Circle {
        Circle { groups: CircleGroups { g0: self.group8(), g1: self.group6(), g2: self.group7() } }
    }
}

impl Add<Circle> for MultiVector {
    type Output = MultiVector;

    fn add(self, other: Circle) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0(), g1: self.group1(), g2: self.group2(), g3: self.group3(), g4: self.group4(), g5: self.group5(), g6: self.group6() + other.group1(), g7: self.group7() + other.group2(), g8: self.group8() + other.group0(), g9: self.group9() } }
    }
}

impl AddAssign<Circle> for MultiVector {
    fn add_assign(&mut self, other: Circle) {
        *self = (*self).add(other);
    }
}

impl Sub<Circle> for MultiVector {
    type Output = MultiVector;

    fn sub(self, other: Circle) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0(), g1: self.group1(), g2: self.group2(), g3: self.group3(), g4: self.group4(), g5: self.group5(), g6: self.group6() - other.group1(), g7: self.group7() - other.group2(), g8: self.group8() - other.group0(), g9: self.group9() } }
    }
}

impl SubAssign<Circle> for MultiVector {
    fn sub_assign(&mut self, other: Circle) {
        *self = (*self).sub(other);
    }
}

impl GeometricProduct<Circle> for MultiVector {
    type Output = MultiVector;

    fn geometric_product(self, other: Circle) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x3::from(self.group1()[1]) * Simd32x3::from(other.group0()[1]) * Simd32x3::from([0.0, 1.0, 0.0]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from(other.group0()[2]) * Simd32x3::from([0.0, 1.0, 0.0]) + Simd32x3::from(self.group2()[0]) * Simd32x3::from(other.group0()[3]) * Simd32x3::from([0.0, 1.0, 0.0]) + Simd32x3::from(self.group3()[0]) * Simd32x3::from(other.group0()[0]) * Simd32x3::from([0.0, 0.0, -1.0]) + Simd32x3::from(self.group3()[1]) * Simd32x3::from(other.group0()[1]) * Simd32x3::from([0.0, 0.0, -1.0]) + Simd32x3::from(self.group3()[2]) * Simd32x3::from(other.group0()[2]) * Simd32x3::from([0.0, 0.0, -1.0]) + Simd32x3::from(self.group3()[3]) * Simd32x3::from(other.group0()[3]) * Simd32x3::from([0.0, 0.0, -1.0]) + Simd32x3::from(self.group4()[0]) * Simd32x3::from(other.group2()[0]) * Simd32x3::from([0.0, 0.0, -1.0]) + Simd32x3::from(self.group4()[1]) * Simd32x3::from(other.group2()[1]) * Simd32x3::from([0.0, 0.0, -1.0]) + Simd32x3::from(self.group4()[2]) * Simd32x3::from(other.group2()[2]) * Simd32x3::from([0.0, 0.0, -1.0]) + Simd32x3::from(self.group5()[0]) * Simd32x3::from(other.group1()[0]) * Simd32x3::from([0.0, 0.0, -1.0]) + Simd32x3::from(self.group5()[1]) * Simd32x3::from(other.group1()[1]) * Simd32x3::from([0.0, 0.0, -1.0]) + Simd32x3::from(self.group5()[2]) * Simd32x3::from(other.group1()[2]) * Simd32x3::from([0.0, 0.0, -1.0]) + Simd32x3::from(self.group8()[3]) * Simd32x3::from(other.group0()[3]) * Simd32x3::from([-1.0, 0.0, 0.0]) + Simd32x3::from(self.group1()[0]) * Simd32x3::from(other.group0()[0]) * Simd32x3::from([0.0, 1.0, 0.0]), g1: self.group5() * Simd32x3::from(other.group0()[3]), g2: Simd32x2::from(0.0) - Simd32x2::from(self.group5()[0]) * Simd32x2::from([other.group0()[0], other.group2()[0]]) - Simd32x2::from(self.group5()[1]) * Simd32x2::from([other.group0()[1], other.group2()[1]]) - Simd32x2::from(self.group5()[2]) * Simd32x2::from([other.group0()[2], other.group2()[2]]) + Simd32x2::from(self.group9()[3]) * Simd32x2::from(other.group0()[3]) * Simd32x2::from([0.0, 1.0]) + Simd32x2::from([self.group0()[1], self.group0()[0]]) * Simd32x2::from([other.group0()[3], other.group0()[0]]) * Simd32x2::from([-1.0, 0.0]), g3: Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group2()[2], other.group2()[2], other.group2()[1], other.group1()[0]]) * Simd32x4::from([0.0, 1.0, -1.0, -1.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group2()[2], other.group2()[2], other.group2()[0], other.group1()[1]]) * Simd32x4::from([-1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group2()[1], other.group2()[0], other.group2()[1], other.group1()[2]]) * Simd32x4::from([1.0, -1.0, 0.0, -1.0]) + Simd32x4::from(self.group7()[0]) * swizzle!(other.group0(), 3, 3, 3, 0) * Simd32x4::from([-1.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group7()[1]) * swizzle!(other.group0(), 3, 3, 3, 1) * Simd32x4::from([0.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.group7()[2]) * swizzle!(other.group0(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, -1.0, 1.0]) + Simd32x4::from(self.group8()[0]) * Simd32x4::from(other.group2()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group8()[1]) * Simd32x4::from(other.group2()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group8()[2]) * Simd32x4::from(other.group2()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group8()[3]) * Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], other.group2()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) + Simd32x4::from([self.group0()[0], self.group0()[0], self.group0()[0], self.group0()[2]]) * swizzle!(other.group0(), 0, 0, 0, 3) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]), g4: Simd32x3::from(self.group1()[1]) * Simd32x3::from([other.group0()[2], other.group0()[2], other.group0()[0]]) * Simd32x3::from([1.0, 0.0, -1.0]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group0()[1], other.group0()[0], other.group0()[1]]) * Simd32x3::from([-1.0, 1.0, 0.0]) + Simd32x3::from(self.group8()[0]) * Simd32x3::from(other.group0()[3]) * Simd32x3::from([1.0, 0.0, 0.0]) + Simd32x3::from(self.group8()[1]) * Simd32x3::from(other.group0()[3]) * Simd32x3::from([0.0, 1.0, 0.0]) + Simd32x3::from(self.group8()[2]) * Simd32x3::from(other.group0()[3]) * Simd32x3::from([0.0, 0.0, 1.0]) - Simd32x3::from(self.group8()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) + Simd32x3::from(self.group1()[0]) * Simd32x3::from([other.group0()[0], other.group0()[2], other.group0()[1]]) * Simd32x3::from([0.0, -1.0, 1.0]), g5: self.group1() * Simd32x3::from(other.group0()[3]) * Simd32x3::from(-1.0), g6: Simd32x3::from(self.group0()[0]) * other.group1() + Simd32x3::from(self.group0()[1]) * other.group2() + Simd32x3::from(self.group3()[1]) * Simd32x3::from([other.group0()[2], other.group0()[2], other.group0()[0]]) * Simd32x3::from([-1.0, 0.0, 1.0]) + Simd32x3::from(self.group3()[2]) * Simd32x3::from([other.group0()[1], other.group0()[0], other.group0()[1]]) * Simd32x3::from([1.0, -1.0, 0.0]) + Simd32x3::from(self.group4()[0]) * swizzle!(other.group2(), 2, 2, 1) * Simd32x3::from([0.0, 1.0, -1.0]) + Simd32x3::from(self.group4()[1]) * swizzle!(other.group2(), 2, 2, 0) * Simd32x3::from([-1.0, 0.0, 1.0]) + Simd32x3::from(self.group4()[2]) * swizzle!(other.group2(), 1, 0, 1) * Simd32x3::from([1.0, -1.0, 0.0]) + Simd32x3::from(self.group5()[0]) * swizzle!(other.group1(), 2, 2, 1) * Simd32x3::from([0.0, 1.0, -1.0]) + Simd32x3::from(self.group5()[1]) * swizzle!(other.group1(), 2, 2, 0) * Simd32x3::from([-1.0, 0.0, 1.0]) + Simd32x3::from(self.group5()[2]) * swizzle!(other.group1(), 1, 0, 1) * Simd32x3::from([1.0, -1.0, 0.0]) + Simd32x3::from(self.group9()[1]) * Simd32x3::from(other.group0()[3]) * Simd32x3::from([0.0, -1.0, 0.0]) + Simd32x3::from(self.group9()[2]) * Simd32x3::from(other.group0()[3]) * Simd32x3::from([0.0, 0.0, -1.0]) + Simd32x3::from(self.group9()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) + Simd32x3::from([self.group9()[0], self.group3()[0], self.group3()[0]]) * Simd32x3::from([other.group0()[3], other.group0()[2], other.group0()[1]]) * Simd32x3::from([-1.0, 1.0, -1.0]), g7: Simd32x3::from(self.group0()[0]) * other.group2() + Simd32x3::from(self.group5()[0]) * swizzle!(other.group2(), 2, 2, 1) * Simd32x3::from([0.0, 1.0, -1.0]) + Simd32x3::from(self.group5()[1]) * swizzle!(other.group2(), 2, 2, 0) * Simd32x3::from([-1.0, 0.0, 1.0]) + Simd32x3::from(self.group5()[2]) * swizzle!(other.group2(), 1, 0, 1) * Simd32x3::from([1.0, -1.0, 0.0]) + Simd32x3::from([self.group3()[0], self.group3()[1], self.group3()[2]]) * Simd32x3::from(other.group0()[3]), g8: Simd32x4::from(self.group0()[0]) * other.group0() + Simd32x4::from(self.group5()[0]) * swizzle!(other.group0(), 2, 2, 1, 2) * Simd32x4::from([0.0, 1.0, -1.0, 0.0]) + Simd32x4::from(self.group5()[1]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([-1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group5()[2]) * swizzle!(other.group0(), 1, 0, 1, 1) * Simd32x4::from([1.0, -1.0, 0.0, 0.0]) + Simd32x4::from([self.group4()[0], self.group4()[1], self.group4()[2], self.group4()[0]]) * swizzle!(other.group0(), 3, 3, 3, 0) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]), g9: Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group1()[2], other.group1()[2], other.group1()[0], other.group2()[1]]) * Simd32x4::from([-1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group1()[1], other.group1()[0], other.group1()[1], other.group2()[2]]) * Simd32x4::from([1.0, -1.0, 0.0, -1.0]) + Simd32x4::from(self.group2()[0]) * Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], other.group2()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) - Simd32x4::from(self.group2()[1]) * other.group0() + Simd32x4::from(self.group6()[0]) * Simd32x4::from(other.group0()[3]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group6()[1]) * Simd32x4::from(other.group0()[3]) * Simd32x4::from([0.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group6()[2]) * Simd32x4::from(other.group0()[3]) * Simd32x4::from([0.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group7()[0]) * swizzle!(other.group0(), 2, 2, 1, 2) * Simd32x4::from([0.0, -1.0, 1.0, 0.0]) + Simd32x4::from(self.group7()[1]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, -1.0, 0.0]) + Simd32x4::from(self.group7()[2]) * swizzle!(other.group0(), 1, 0, 1, 1) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group8()[0]) * Simd32x4::from([other.group2()[2], other.group2()[2], other.group2()[1], other.group2()[2]]) * Simd32x4::from([0.0, 1.0, -1.0, 0.0]) + Simd32x4::from(self.group8()[1]) * Simd32x4::from([other.group2()[2], other.group2()[2], other.group2()[0], other.group2()[2]]) * Simd32x4::from([-1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group8()[2]) * Simd32x4::from([other.group2()[1], other.group2()[0], other.group2()[1], other.group2()[1]]) * Simd32x4::from([1.0, -1.0, 0.0, 0.0]) + Simd32x4::from(self.group8()[3]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group1()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group1()[0], other.group1()[2], other.group1()[1], other.group2()[0]]) * Simd32x4::from([0.0, 1.0, -1.0, -1.0]) } }
    }
}

impl ScalarProduct<Circle> for MultiVector {
    type Output = Scalar;

    fn scalar_product(self, other: Circle) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group8()[3] * other.group0()[3] } }
    }
}

impl Into<Plane> for MultiVector {
    fn into(self) -> Plane {
        Plane { groups: PlaneGroups { g0: self.group9() } }
    }
}

impl Add<Plane> for MultiVector {
    type Output = MultiVector;

    fn add(self, other: Plane) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0(), g1: self.group1(), g2: self.group2(), g3: self.group3(), g4: self.group4(), g5: self.group5(), g6: self.group6(), g7: self.group7(), g8: self.group8(), g9: self.group9() + other.group0() } }
    }
}

impl AddAssign<Plane> for MultiVector {
    fn add_assign(&mut self, other: Plane) {
        *self = (*self).add(other);
    }
}

impl Sub<Plane> for MultiVector {
    type Output = MultiVector;

    fn sub(self, other: Plane) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0(), g1: self.group1(), g2: self.group2(), g3: self.group3(), g4: self.group4(), g5: self.group5(), g6: self.group6(), g7: self.group7(), g8: self.group8(), g9: self.group9() - other.group0() } }
    }
}

impl SubAssign<Plane> for MultiVector {
    fn sub_assign(&mut self, other: Plane) {
        *self = (*self).sub(other);
    }
}

impl Into<Sphere> for MultiVector {
    fn into(self) -> Sphere {
        Sphere { groups: SphereGroups { g0: self.group0()[1], g1: self.group9() } }
    }
}

impl Add<Sphere> for MultiVector {
    type Output = MultiVector;

    fn add(self, other: Sphere) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() + Simd32x3::from(other.group0()) * Simd32x3::from([0.0, 1.0, 0.0]), g1: self.group1(), g2: self.group2(), g3: self.group3(), g4: self.group4(), g5: self.group5(), g6: self.group6(), g7: self.group7(), g8: self.group8(), g9: self.group9() + other.group1() } }
    }
}

impl AddAssign<Sphere> for MultiVector {
    fn add_assign(&mut self, other: Sphere) {
        *self = (*self).add(other);
    }
}

impl Sub<Sphere> for MultiVector {
    type Output = MultiVector;

    fn sub(self, other: Sphere) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() - Simd32x3::from(other.group0()) * Simd32x3::from([0.0, 1.0, 0.0]), g1: self.group1(), g2: self.group2(), g3: self.group3(), g4: self.group4(), g5: self.group5(), g6: self.group6(), g7: self.group7(), g8: self.group8(), g9: self.group9() - other.group1() } }
    }
}

impl SubAssign<Sphere> for MultiVector {
    fn sub_assign(&mut self, other: Sphere) {
        *self = (*self).sub(other);
    }
}

impl Into<Motor> for MultiVector {
    fn into(self) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from([self.group6()[0], self.group6()[1], self.group6()[2], self.group0()[2]]), g1: Simd32x4::from([self.group7()[0], self.group7()[1], self.group7()[2], self.group2()[1]]) } }
    }
}

impl Add<Motor> for MultiVector {
    type Output = MultiVector;

    fn add(self, other: Motor) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() + Simd32x3::from([other.group0()[0], other.group0()[0], other.group0()[3]]) * Simd32x3::from([0.0, 0.0, 1.0]), g1: self.group1(), g2: self.group2() + Simd32x2::from([other.group0()[0], other.group1()[3]]) * Simd32x2::from([0.0, 1.0]), g3: self.group3(), g4: self.group4(), g5: self.group5(), g6: self.group6() + Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]), g7: self.group7() + Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]), g8: self.group8(), g9: self.group9() } }
    }
}

impl AddAssign<Motor> for MultiVector {
    fn add_assign(&mut self, other: Motor) {
        *self = (*self).add(other);
    }
}

impl Sub<Motor> for MultiVector {
    type Output = MultiVector;

    fn sub(self, other: Motor) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() - Simd32x3::from([other.group0()[0], other.group0()[0], other.group0()[3]]) * Simd32x3::from([0.0, 0.0, 1.0]), g1: self.group1(), g2: self.group2() - Simd32x2::from([other.group0()[0], other.group1()[3]]) * Simd32x2::from([0.0, 1.0]), g3: self.group3(), g4: self.group4(), g5: self.group5(), g6: self.group6() - Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]), g7: self.group7() - Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]), g8: self.group8(), g9: self.group9() } }
    }
}

impl SubAssign<Motor> for MultiVector {
    fn sub_assign(&mut self, other: Motor) {
        *self = (*self).sub(other);
    }
}

impl Into<Rotor> for MultiVector {
    fn into(self) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from([self.group6()[0], self.group6()[1], self.group6()[2], self.group0()[2]]) } }
    }
}

impl Add<Rotor> for MultiVector {
    type Output = MultiVector;

    fn add(self, other: Rotor) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() + Simd32x3::from([other.group0()[0], other.group0()[0], other.group0()[3]]) * Simd32x3::from([0.0, 0.0, 1.0]), g1: self.group1(), g2: self.group2(), g3: self.group3(), g4: self.group4(), g5: self.group5(), g6: self.group6() + Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]), g7: self.group7(), g8: self.group8(), g9: self.group9() } }
    }
}

impl AddAssign<Rotor> for MultiVector {
    fn add_assign(&mut self, other: Rotor) {
        *self = (*self).add(other);
    }
}

impl Sub<Rotor> for MultiVector {
    type Output = MultiVector;

    fn sub(self, other: Rotor) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() - Simd32x3::from([other.group0()[0], other.group0()[0], other.group0()[3]]) * Simd32x3::from([0.0, 0.0, 1.0]), g1: self.group1(), g2: self.group2(), g3: self.group3(), g4: self.group4(), g5: self.group5(), g6: self.group6() - Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]), g7: self.group7(), g8: self.group8(), g9: self.group9() } }
    }
}

impl SubAssign<Rotor> for MultiVector {
    fn sub_assign(&mut self, other: Rotor) {
        *self = (*self).sub(other);
    }
}

impl Into<Translator> for MultiVector {
    fn into(self) -> Translator {
        Translator { groups: TranslatorGroups { g0: Simd32x4::from([self.group7()[0], self.group7()[1], self.group7()[2], self.group0()[2]]) } }
    }
}

impl Add<Translator> for MultiVector {
    type Output = MultiVector;

    fn add(self, other: Translator) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() + Simd32x3::from([other.group0()[0], other.group0()[0], other.group0()[3]]) * Simd32x3::from([0.0, 0.0, 1.0]), g1: self.group1(), g2: self.group2(), g3: self.group3(), g4: self.group4(), g5: self.group5(), g6: self.group6(), g7: self.group7() + Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]), g8: self.group8(), g9: self.group9() } }
    }
}

impl AddAssign<Translator> for MultiVector {
    fn add_assign(&mut self, other: Translator) {
        *self = (*self).add(other);
    }
}

impl Sub<Translator> for MultiVector {
    type Output = MultiVector;

    fn sub(self, other: Translator) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() - Simd32x3::from([other.group0()[0], other.group0()[0], other.group0()[3]]) * Simd32x3::from([0.0, 0.0, 1.0]), g1: self.group1(), g2: self.group2(), g3: self.group3(), g4: self.group4(), g5: self.group5(), g6: self.group6(), g7: self.group7() - Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]), g8: self.group8(), g9: self.group9() } }
    }
}

impl SubAssign<Translator> for MultiVector {
    fn sub_assign(&mut self, other: Translator) {
        *self = (*self).sub(other);
    }
}

impl Into<Flector> for MultiVector {
    fn into(self) -> Flector {
        Flector { groups: FlectorGroups { g0: self.group3(), g1: self.group9() } }
    }
}

impl Add<Flector> for MultiVector {
    type Output = MultiVector;

    fn add(self, other: Flector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0(), g1: self.group1(), g2: self.group2(), g3: self.group3() + other.group0(), g4: self.group4(), g5: self.group5(), g6: self.group6(), g7: self.group7(), g8: self.group8(), g9: self.group9() + other.group1() } }
    }
}

impl AddAssign<Flector> for MultiVector {
    fn add_assign(&mut self, other: Flector) {
        *self = (*self).add(other);
    }
}

impl Sub<Flector> for MultiVector {
    type Output = MultiVector;

    fn sub(self, other: Flector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0(), g1: self.group1(), g2: self.group2(), g3: self.group3() - other.group0(), g4: self.group4(), g5: self.group5(), g6: self.group6(), g7: self.group7(), g8: self.group8(), g9: self.group9() - other.group1() } }
    }
}

impl SubAssign<Flector> for MultiVector {
    fn sub_assign(&mut self, other: Flector) {
        *self = (*self).sub(other);
    }
}

impl Add<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn add(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() + other.group0(), g1: self.group1() + other.group1(), g2: self.group2() + other.group2(), g3: self.group3() + other.group3(), g4: self.group4() + other.group4(), g5: self.group5() + other.group5(), g6: self.group6() + other.group6(), g7: self.group7() + other.group7(), g8: self.group8() + other.group8(), g9: self.group9() + other.group9() } }
    }
}

impl AddAssign<MultiVector> for MultiVector {
    fn add_assign(&mut self, other: MultiVector) {
        *self = (*self).add(other);
    }
}

impl Sub<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn sub(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() - other.group0(), g1: self.group1() - other.group1(), g2: self.group2() - other.group2(), g3: self.group3() - other.group3(), g4: self.group4() - other.group4(), g5: self.group5() - other.group5(), g6: self.group6() - other.group6(), g7: self.group7() - other.group7(), g8: self.group8() - other.group8(), g9: self.group9() - other.group9() } }
    }
}

impl SubAssign<MultiVector> for MultiVector {
    fn sub_assign(&mut self, other: MultiVector) {
        *self = (*self).sub(other);
    }
}

impl Mul<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn mul(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() * other.group0(), g1: self.group1() * other.group1(), g2: self.group2() * other.group2(), g3: self.group3() * other.group3(), g4: self.group4() * other.group4(), g5: self.group5() * other.group5(), g6: self.group6() * other.group6(), g7: self.group7() * other.group7(), g8: self.group8() * other.group8(), g9: self.group9() * other.group9() } }
    }
}

impl MulAssign<MultiVector> for MultiVector {
    fn mul_assign(&mut self, other: MultiVector) {
        *self = (*self).mul(other);
    }
}

impl Div<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn div(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from([1.0, 1.0, 1.0]) / Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from([1.0, 1.0, 1.0]), g1: Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]) * Simd32x3::from([1.0, 1.0, 1.0]) / Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x3::from([1.0, 1.0, 1.0]), g2: Simd32x2::from([self.group2()[0], self.group2()[1]]) * Simd32x2::from([1.0, 1.0]) / Simd32x2::from([other.group2()[0], other.group2()[1]]) * Simd32x2::from([1.0, 1.0]), g3: Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group3()[3]]) * Simd32x4::from([1.0, 1.0, 1.0, 1.0]) / Simd32x4::from([other.group3()[0], other.group3()[1], other.group3()[2], other.group3()[3]]) * Simd32x4::from([1.0, 1.0, 1.0, 1.0]), g4: Simd32x3::from([self.group4()[0], self.group4()[1], self.group4()[2]]) * Simd32x3::from([1.0, 1.0, 1.0]) / Simd32x3::from([other.group4()[0], other.group4()[1], other.group4()[2]]) * Simd32x3::from([1.0, 1.0, 1.0]), g5: Simd32x3::from([self.group5()[0], self.group5()[1], self.group5()[2]]) * Simd32x3::from([1.0, 1.0, 1.0]) / Simd32x3::from([other.group5()[0], other.group5()[1], other.group5()[2]]) * Simd32x3::from([1.0, 1.0, 1.0]), g6: Simd32x3::from([self.group6()[0], self.group6()[1], self.group6()[2]]) * Simd32x3::from([1.0, 1.0, 1.0]) / Simd32x3::from([other.group6()[0], other.group6()[1], other.group6()[2]]) * Simd32x3::from([1.0, 1.0, 1.0]), g7: Simd32x3::from([self.group7()[0], self.group7()[1], self.group7()[2]]) * Simd32x3::from([1.0, 1.0, 1.0]) / Simd32x3::from([other.group7()[0], other.group7()[1], other.group7()[2]]) * Simd32x3::from([1.0, 1.0, 1.0]), g8: Simd32x4::from([self.group8()[0], self.group8()[1], self.group8()[2], self.group8()[3]]) * Simd32x4::from([1.0, 1.0, 1.0, 1.0]) / Simd32x4::from([other.group8()[0], other.group8()[1], other.group8()[2], other.group8()[3]]) * Simd32x4::from([1.0, 1.0, 1.0, 1.0]), g9: Simd32x4::from([self.group9()[0], self.group9()[1], self.group9()[2], self.group9()[3]]) * Simd32x4::from([1.0, 1.0, 1.0, 1.0]) / Simd32x4::from([other.group9()[0], other.group9()[1], other.group9()[2], other.group9()[3]]) * Simd32x4::from([1.0, 1.0, 1.0, 1.0]) } }
    }
}

impl DivAssign<MultiVector> for MultiVector {
    fn div_assign(&mut self, other: MultiVector) {
        *self = (*self).div(other);
    }
}

impl GeometricProduct<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn geometric_product(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x3::from(self.group0()[0]) * other.group0() + Simd32x3::from(self.group0()[2]) * Simd32x3::from(other.group0()[0]) * Simd32x3::from([0.0, 0.0, 1.0]) + Simd32x3::from(self.group1()[0]) * Simd32x3::from([other.group1()[0], other.group8()[0], other.group9()[0]]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from([other.group1()[1], other.group8()[1], other.group9()[1]]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group1()[2], other.group8()[2], other.group9()[2]]) + Simd32x3::from(self.group2()[0]) * Simd32x3::from([other.group8()[3], other.group8()[3], other.group9()[3]]) * Simd32x3::from([0.0, 1.0, 1.0]) + Simd32x3::from(self.group2()[1]) * Simd32x3::from(other.group0()[1]) * Simd32x3::from([0.0, 0.0, 1.0]) + Simd32x3::from(self.group3()[0]) * Simd32x3::from(other.group8()[0]) * Simd32x3::from([0.0, 0.0, -1.0]) + Simd32x3::from(self.group3()[1]) * Simd32x3::from(other.group8()[1]) * Simd32x3::from([0.0, 0.0, -1.0]) + Simd32x3::from(self.group3()[2]) * Simd32x3::from(other.group8()[2]) * Simd32x3::from([0.0, 0.0, -1.0]) + Simd32x3::from(self.group3()[3]) * Simd32x3::from(other.group8()[3]) * Simd32x3::from([0.0, 0.0, -1.0]) + Simd32x3::from(self.group4()[0]) * Simd32x3::from([other.group5()[0], other.group5()[0], other.group7()[0]]) * Simd32x3::from([0.0, -1.0, -1.0]) + Simd32x3::from(self.group4()[1]) * Simd32x3::from([other.group5()[1], other.group5()[1], other.group7()[1]]) * Simd32x3::from([0.0, -1.0, -1.0]) + Simd32x3::from(self.group4()[2]) * Simd32x3::from([other.group5()[2], other.group5()[2], other.group7()[2]]) * Simd32x3::from([0.0, -1.0, -1.0]) - Simd32x3::from(self.group5()[0]) * Simd32x3::from([other.group5()[0], other.group4()[0], other.group6()[0]]) - Simd32x3::from(self.group5()[1]) * Simd32x3::from([other.group5()[1], other.group4()[1], other.group6()[1]]) - Simd32x3::from(self.group5()[2]) * Simd32x3::from([other.group5()[2], other.group4()[2], other.group6()[2]]) + Simd32x3::from(self.group6()[0]) * Simd32x3::from(other.group5()[0]) * Simd32x3::from([0.0, 0.0, -1.0]) + Simd32x3::from(self.group6()[1]) * Simd32x3::from(other.group5()[1]) * Simd32x3::from([0.0, 0.0, -1.0]) + Simd32x3::from(self.group6()[2]) * Simd32x3::from(other.group5()[2]) * Simd32x3::from([0.0, 0.0, -1.0]) + Simd32x3::from(self.group7()[0]) * Simd32x3::from(other.group4()[0]) * Simd32x3::from([0.0, 0.0, -1.0]) + Simd32x3::from(self.group7()[1]) * Simd32x3::from(other.group4()[1]) * Simd32x3::from([0.0, 0.0, -1.0]) + Simd32x3::from(self.group7()[2]) * Simd32x3::from(other.group4()[2]) * Simd32x3::from([0.0, 0.0, -1.0]) + Simd32x3::from(self.group8()[0]) * Simd32x3::from([other.group1()[0], other.group1()[0], other.group3()[0]]) * Simd32x3::from([0.0, -1.0, -1.0]) + Simd32x3::from(self.group8()[1]) * Simd32x3::from([other.group1()[1], other.group1()[1], other.group3()[1]]) * Simd32x3::from([0.0, -1.0, -1.0]) + Simd32x3::from(self.group8()[2]) * Simd32x3::from([other.group1()[2], other.group1()[2], other.group3()[2]]) * Simd32x3::from([0.0, -1.0, -1.0]) - Simd32x3::from(self.group8()[3]) * Simd32x3::from([other.group8()[3], other.group2()[0], other.group3()[3]]) + Simd32x3::from(self.group9()[0]) * Simd32x3::from(other.group1()[0]) * Simd32x3::from([0.0, 0.0, 1.0]) + Simd32x3::from(self.group9()[1]) * Simd32x3::from(other.group1()[1]) * Simd32x3::from([0.0, 0.0, 1.0]) + Simd32x3::from(self.group9()[2]) * Simd32x3::from(other.group1()[2]) * Simd32x3::from([0.0, 0.0, 1.0]) + Simd32x3::from(self.group9()[3]) * Simd32x3::from(other.group2()[0]) * Simd32x3::from([0.0, 0.0, 1.0]) + swizzle!(self.group0(), 0, 1, 1) * Simd32x3::from([other.group0()[0], other.group0()[0], other.group2()[1]]) * Simd32x3::from([0.0, 1.0, 1.0]), g1: Simd32x3::from(self.group0()[0]) * other.group1() + Simd32x3::from(self.group1()[0]) * Simd32x3::from([other.group0()[0], other.group5()[2], other.group5()[1]]) * Simd32x3::from([1.0, 1.0, -1.0]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from([other.group5()[2], other.group0()[0], other.group5()[0]]) * Simd32x3::from([-1.0, 1.0, 1.0]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group5()[1], other.group5()[0], other.group0()[0]]) * Simd32x3::from([1.0, -1.0, 1.0]) + Simd32x3::from(self.group5()[0]) * Simd32x3::from([other.group8()[3], other.group1()[2], other.group1()[1]]) * Simd32x3::from([1.0, 1.0, -1.0]) + Simd32x3::from(self.group5()[1]) * Simd32x3::from([other.group1()[2], other.group8()[3], other.group1()[0]]) * Simd32x3::from([-1.0, 1.0, 1.0]) + Simd32x3::from(self.group5()[2]) * Simd32x3::from([other.group1()[1], other.group1()[0], other.group8()[3]]) * Simd32x3::from([1.0, -1.0, 1.0]) + Simd32x3::from(self.group8()[3]) * other.group5(), g2: Simd32x2::from(self.group0()[0]) * other.group2() + Simd32x2::from(self.group1()[0]) * Simd32x2::from([other.group4()[0], other.group3()[0]]) * Simd32x2::from([-1.0, 1.0]) + Simd32x2::from(self.group1()[1]) * Simd32x2::from([other.group4()[1], other.group3()[1]]) * Simd32x2::from([-1.0, 1.0]) + Simd32x2::from(self.group1()[2]) * Simd32x2::from([other.group4()[2], other.group3()[2]]) * Simd32x2::from([-1.0, 1.0]) + Simd32x2::from(self.group2()[0]) * Simd32x2::from(other.group0()[0]) * Simd32x2::from([1.0, 0.0]) + Simd32x2::from(self.group2()[1]) * Simd32x2::from(other.group0()[0]) * Simd32x2::from([0.0, 1.0]) + Simd32x2::from(self.group3()[0]) * Simd32x2::from(other.group1()[0]) * Simd32x2::from([0.0, -1.0]) + Simd32x2::from(self.group3()[1]) * Simd32x2::from(other.group1()[1]) * Simd32x2::from([0.0, -1.0]) + Simd32x2::from(self.group3()[2]) * Simd32x2::from(other.group1()[2]) * Simd32x2::from([0.0, -1.0]) + Simd32x2::from(self.group4()[0]) * Simd32x2::from(other.group1()[0]) * Simd32x2::from([1.0, 0.0]) + Simd32x2::from(self.group4()[1]) * Simd32x2::from(other.group1()[1]) * Simd32x2::from([1.0, 0.0]) + Simd32x2::from(self.group4()[2]) * Simd32x2::from(other.group1()[2]) * Simd32x2::from([1.0, 0.0]) - Simd32x2::from(self.group5()[0]) * Simd32x2::from([other.group8()[0], other.group7()[0]]) - Simd32x2::from(self.group5()[1]) * Simd32x2::from([other.group8()[1], other.group7()[1]]) - Simd32x2::from(self.group5()[2]) * Simd32x2::from([other.group8()[2], other.group7()[2]]) + Simd32x2::from(self.group7()[0]) * Simd32x2::from(other.group5()[0]) * Simd32x2::from([0.0, -1.0]) + Simd32x2::from(self.group7()[1]) * Simd32x2::from(other.group5()[1]) * Simd32x2::from([0.0, -1.0]) + Simd32x2::from(self.group7()[2]) * Simd32x2::from(other.group5()[2]) * Simd32x2::from([0.0, -1.0]) + Simd32x2::from(self.group8()[0]) * Simd32x2::from(other.group5()[0]) * Simd32x2::from([-1.0, 0.0]) + Simd32x2::from(self.group8()[1]) * Simd32x2::from(other.group5()[1]) * Simd32x2::from([-1.0, 0.0]) + Simd32x2::from(self.group8()[2]) * Simd32x2::from(other.group5()[2]) * Simd32x2::from([-1.0, 0.0]) + Simd32x2::from(self.group8()[3]) * Simd32x2::from([other.group0()[1], other.group9()[3]]) * Simd32x2::from([1.0, -1.0]) + Simd32x2::from(self.group9()[3]) * Simd32x2::from(other.group8()[3]) * Simd32x2::from([0.0, 1.0]) + Simd32x2::from([self.group0()[1], self.group0()[0]]) * Simd32x2::from([other.group8()[3], other.group8()[0]]) * Simd32x2::from([-1.0, 0.0]), g3: Simd32x4::from(self.group0()[0]) * other.group3() + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group8()[3]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group2()[1], other.group7()[2], other.group7()[1], other.group6()[0]]) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group7()[2], other.group2()[1], other.group7()[0], other.group6()[1]]) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group7()[1], other.group7()[0], other.group2()[1], other.group6()[2]]) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) + Simd32x4::from(self.group2()[0]) * Simd32x4::from(other.group2()[1]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) - Simd32x4::from(self.group2()[1]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group2()[0]]) + Simd32x4::from(self.group3()[0]) * Simd32x4::from([other.group0()[0], other.group5()[2], other.group5()[1], other.group4()[0]]) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]) + Simd32x4::from(self.group3()[1]) * Simd32x4::from([other.group5()[2], other.group0()[0], other.group5()[0], other.group4()[1]]) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group3()[2]) * Simd32x4::from([other.group5()[1], other.group5()[0], other.group0()[0], other.group4()[2]]) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) + Simd32x4::from(self.group3()[3]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group4()[0]) * Simd32x4::from(other.group3()[0]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group4()[1]) * Simd32x4::from(other.group3()[1]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group4()[2]) * Simd32x4::from(other.group3()[2]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group5()[0]) * Simd32x4::from([other.group9()[3], other.group3()[2], other.group3()[1], other.group9()[0]]) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]) + Simd32x4::from(self.group5()[1]) * Simd32x4::from([other.group3()[2], other.group9()[3], other.group3()[0], other.group9()[1]]) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group5()[2]) * Simd32x4::from([other.group3()[1], other.group3()[0], other.group9()[3], other.group9()[2]]) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) + Simd32x4::from(self.group6()[0]) * Simd32x4::from(other.group1()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group6()[1]) * Simd32x4::from(other.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group6()[2]) * Simd32x4::from(other.group1()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group7()[0]) * Simd32x4::from([other.group8()[3], other.group1()[2], other.group1()[1], other.group8()[0]]) * Simd32x4::from([-1.0, -1.0, 1.0, 1.0]) + Simd32x4::from(self.group7()[1]) * Simd32x4::from([other.group1()[2], other.group8()[3], other.group1()[0], other.group8()[1]]) * Simd32x4::from([1.0, -1.0, -1.0, 1.0]) + Simd32x4::from(self.group7()[2]) * Simd32x4::from([other.group1()[1], other.group1()[0], other.group8()[3], other.group8()[2]]) * Simd32x4::from([-1.0, 1.0, -1.0, 1.0]) + Simd32x4::from(self.group8()[0]) * Simd32x4::from(other.group7()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group8()[1]) * Simd32x4::from(other.group7()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group8()[2]) * Simd32x4::from(other.group7()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group8()[3]) * Simd32x4::from([other.group7()[0], other.group7()[1], other.group7()[2], other.group0()[2]]) + Simd32x4::from(self.group9()[0]) * Simd32x4::from(other.group5()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group9()[1]) * Simd32x4::from(other.group5()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group9()[2]) * Simd32x4::from(other.group5()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group9()[3]) * Simd32x4::from([other.group5()[0], other.group5()[1], other.group5()[2], other.group0()[1]]) + Simd32x4::from([self.group0()[0], self.group0()[0], self.group0()[0], self.group0()[1]]) * swizzle!(other.group9(), 0, 0, 0, 3) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]), g4: Simd32x3::from(self.group0()[0]) * other.group4() + Simd32x3::from(self.group0()[1]) * other.group5() + Simd32x3::from(self.group1()[0]) * Simd32x3::from([other.group2()[0], other.group8()[2], other.group8()[1]]) * Simd32x3::from([-1.0, -1.0, 1.0]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from([other.group8()[2], other.group2()[0], other.group8()[0]]) * Simd32x3::from([1.0, -1.0, -1.0]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group8()[1], other.group8()[0], other.group2()[0]]) * Simd32x3::from([-1.0, 1.0, -1.0]) + Simd32x3::from(self.group2()[0]) * other.group1() + Simd32x3::from(self.group4()[0]) * Simd32x3::from([other.group0()[0], other.group5()[2], other.group5()[1]]) * Simd32x3::from([1.0, 1.0, -1.0]) + Simd32x3::from(self.group4()[1]) * Simd32x3::from([other.group5()[2], other.group0()[0], other.group5()[0]]) * Simd32x3::from([-1.0, 1.0, 1.0]) + Simd32x3::from(self.group4()[2]) * Simd32x3::from([other.group5()[1], other.group5()[0], other.group0()[0]]) * Simd32x3::from([1.0, -1.0, 1.0]) + Simd32x3::from(self.group5()[0]) * Simd32x3::from([other.group0()[1], other.group4()[2], other.group4()[1]]) * Simd32x3::from([1.0, 1.0, -1.0]) + Simd32x3::from(self.group5()[1]) * Simd32x3::from([other.group4()[2], other.group0()[1], other.group4()[0]]) * Simd32x3::from([-1.0, 1.0, 1.0]) + Simd32x3::from(self.group5()[2]) * Simd32x3::from([other.group4()[1], other.group4()[0], other.group0()[1]]) * Simd32x3::from([1.0, -1.0, 1.0]) + Simd32x3::from(self.group8()[0]) * Simd32x3::from([other.group8()[3], other.group1()[2], other.group1()[1]]) * Simd32x3::from([1.0, 1.0, -1.0]) + Simd32x3::from(self.group8()[1]) * Simd32x3::from([other.group1()[2], other.group8()[3], other.group1()[0]]) * Simd32x3::from([-1.0, 1.0, 1.0]) + Simd32x3::from(self.group8()[2]) * Simd32x3::from([other.group1()[1], other.group1()[0], other.group8()[3]]) * Simd32x3::from([1.0, -1.0, 1.0]) - Simd32x3::from(self.group8()[3]) * Simd32x3::from([other.group8()[0], other.group8()[1], other.group8()[2]]), g5: Simd32x3::from(self.group0()[0]) * other.group5() + Simd32x3::from(self.group1()[0]) * Simd32x3::from([other.group8()[3], other.group1()[2], other.group1()[1]]) * Simd32x3::from([-1.0, -1.0, 1.0]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from([other.group1()[2], other.group8()[3], other.group1()[0]]) * Simd32x3::from([1.0, -1.0, -1.0]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group1()[1], other.group1()[0], other.group8()[3]]) * Simd32x3::from([-1.0, 1.0, -1.0]) + Simd32x3::from(self.group5()[0]) * Simd32x3::from([other.group0()[0], other.group5()[2], other.group5()[1]]) * Simd32x3::from([1.0, 1.0, -1.0]) + Simd32x3::from(self.group5()[1]) * Simd32x3::from([other.group5()[2], other.group0()[0], other.group5()[0]]) * Simd32x3::from([-1.0, 1.0, 1.0]) + Simd32x3::from(self.group5()[2]) * Simd32x3::from([other.group5()[1], other.group5()[0], other.group0()[0]]) * Simd32x3::from([1.0, -1.0, 1.0]) - Simd32x3::from(self.group8()[3]) * other.group1(), g6: Simd32x3::from(self.group0()[0]) * other.group6() + Simd32x3::from(self.group0()[1]) * other.group7() + Simd32x3::from(self.group0()[2]) * other.group5() + Simd32x3::from(self.group1()[0]) * Simd32x3::from([other.group3()[3], other.group9()[2], other.group9()[1]]) * Simd32x3::from([-1.0, -1.0, 1.0]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from([other.group9()[2], other.group3()[3], other.group9()[0]]) * Simd32x3::from([1.0, -1.0, -1.0]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group9()[1], other.group9()[0], other.group3()[3]]) * Simd32x3::from([-1.0, 1.0, -1.0]) + Simd32x3::from(self.group2()[0]) * Simd32x3::from([other.group3()[0], other.group3()[1], other.group3()[2]]) + Simd32x3::from(self.group2()[1]) * other.group4() + Simd32x3::from(self.group3()[0]) * Simd32x3::from([other.group2()[0], other.group8()[2], other.group8()[1]]) * Simd32x3::from([1.0, 1.0, -1.0]) + Simd32x3::from(self.group3()[1]) * Simd32x3::from([other.group8()[2], other.group2()[0], other.group8()[0]]) * Simd32x3::from([-1.0, 1.0, 1.0]) + Simd32x3::from(self.group3()[2]) * Simd32x3::from([other.group8()[1], other.group8()[0], other.group2()[0]]) * Simd32x3::from([1.0, -1.0, 1.0]) - Simd32x3::from(self.group3()[3]) * other.group1() + Simd32x3::from(self.group4()[0]) * Simd32x3::from([other.group2()[1], other.group7()[2], other.group7()[1]]) * Simd32x3::from([1.0, 1.0, -1.0]) + Simd32x3::from(self.group4()[1]) * Simd32x3::from([other.group7()[2], other.group2()[1], other.group7()[0]]) * Simd32x3::from([-1.0, 1.0, 1.0]) + Simd32x3::from(self.group4()[2]) * Simd32x3::from([other.group7()[1], other.group7()[0], other.group2()[1]]) * Simd32x3::from([1.0, -1.0, 1.0]) + Simd32x3::from(self.group5()[0]) * Simd32x3::from([other.group0()[2], other.group6()[2], other.group6()[1]]) * Simd32x3::from([1.0, 1.0, -1.0]) + Simd32x3::from(self.group5()[1]) * Simd32x3::from([other.group6()[2], other.group0()[2], other.group6()[0]]) * Simd32x3::from([-1.0, 1.0, 1.0]) + Simd32x3::from(self.group5()[2]) * Simd32x3::from([other.group6()[1], other.group6()[0], other.group0()[2]]) * Simd32x3::from([1.0, -1.0, 1.0]) + Simd32x3::from(self.group6()[0]) * Simd32x3::from([other.group0()[0], other.group5()[2], other.group5()[1]]) * Simd32x3::from([1.0, 1.0, -1.0]) + Simd32x3::from(self.group6()[1]) * Simd32x3::from([other.group5()[2], other.group0()[0], other.group5()[0]]) * Simd32x3::from([-1.0, 1.0, 1.0]) + Simd32x3::from(self.group6()[2]) * Simd32x3::from([other.group5()[1], other.group5()[0], other.group0()[0]]) * Simd32x3::from([1.0, -1.0, 1.0]) + Simd32x3::from(self.group7()[0]) * Simd32x3::from([other.group0()[1], other.group4()[2], other.group4()[1]]) * Simd32x3::from([1.0, 1.0, -1.0]) + Simd32x3::from(self.group7()[1]) * Simd32x3::from([other.group4()[2], other.group0()[1], other.group4()[0]]) * Simd32x3::from([-1.0, 1.0, 1.0]) + Simd32x3::from(self.group7()[2]) * Simd32x3::from([other.group4()[1], other.group4()[0], other.group0()[1]]) * Simd32x3::from([1.0, -1.0, 1.0]) + Simd32x3::from(self.group8()[0]) * Simd32x3::from([other.group9()[3], other.group3()[2], other.group3()[1]]) * Simd32x3::from([1.0, 1.0, -1.0]) + Simd32x3::from(self.group8()[1]) * Simd32x3::from([other.group3()[2], other.group9()[3], other.group3()[0]]) * Simd32x3::from([-1.0, 1.0, 1.0]) + Simd32x3::from(self.group8()[2]) * Simd32x3::from([other.group3()[1], other.group3()[0], other.group9()[3]]) * Simd32x3::from([1.0, -1.0, 1.0]) - Simd32x3::from(self.group8()[3]) * Simd32x3::from([other.group9()[0], other.group9()[1], other.group9()[2]]) + Simd32x3::from(self.group9()[0]) * Simd32x3::from([other.group8()[3], other.group1()[2], other.group1()[1]]) * Simd32x3::from([-1.0, -1.0, 1.0]) + Simd32x3::from(self.group9()[1]) * Simd32x3::from([other.group1()[2], other.group8()[3], other.group1()[0]]) * Simd32x3::from([1.0, -1.0, -1.0]) + Simd32x3::from(self.group9()[2]) * Simd32x3::from([other.group1()[1], other.group1()[0], other.group8()[3]]) * Simd32x3::from([-1.0, 1.0, -1.0]) + Simd32x3::from(self.group9()[3]) * Simd32x3::from([other.group8()[0], other.group8()[1], other.group8()[2]]), g7: Simd32x3::from(self.group0()[0]) * other.group7() + Simd32x3::from(self.group1()[0]) * Simd32x3::from([other.group9()[3], other.group3()[2], other.group3()[1]]) * Simd32x3::from([-1.0, -1.0, 1.0]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from([other.group3()[2], other.group9()[3], other.group3()[0]]) * Simd32x3::from([1.0, -1.0, -1.0]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group3()[1], other.group3()[0], other.group9()[3]]) * Simd32x3::from([-1.0, 1.0, -1.0]) + Simd32x3::from(self.group2()[1]) * other.group5() + Simd32x3::from(self.group3()[0]) * Simd32x3::from([other.group8()[3], other.group1()[2], other.group1()[1]]) * Simd32x3::from([1.0, 1.0, -1.0]) + Simd32x3::from(self.group3()[1]) * Simd32x3::from([other.group1()[2], other.group8()[3], other.group1()[0]]) * Simd32x3::from([-1.0, 1.0, 1.0]) + Simd32x3::from(self.group3()[2]) * Simd32x3::from([other.group1()[1], other.group1()[0], other.group8()[3]]) * Simd32x3::from([1.0, -1.0, 1.0]) + Simd32x3::from(self.group5()[0]) * Simd32x3::from([other.group2()[1], other.group7()[2], other.group7()[1]]) * Simd32x3::from([1.0, 1.0, -1.0]) + Simd32x3::from(self.group5()[1]) * Simd32x3::from([other.group7()[2], other.group2()[1], other.group7()[0]]) * Simd32x3::from([-1.0, 1.0, 1.0]) + Simd32x3::from(self.group5()[2]) * Simd32x3::from([other.group7()[1], other.group7()[0], other.group2()[1]]) * Simd32x3::from([1.0, -1.0, 1.0]) + Simd32x3::from(self.group7()[0]) * Simd32x3::from([other.group0()[0], other.group5()[2], other.group5()[1]]) * Simd32x3::from([1.0, 1.0, -1.0]) + Simd32x3::from(self.group7()[1]) * Simd32x3::from([other.group5()[2], other.group0()[0], other.group5()[0]]) * Simd32x3::from([-1.0, 1.0, 1.0]) + Simd32x3::from(self.group7()[2]) * Simd32x3::from([other.group5()[1], other.group5()[0], other.group0()[0]]) * Simd32x3::from([1.0, -1.0, 1.0]) - Simd32x3::from(self.group8()[3]) * Simd32x3::from([other.group3()[0], other.group3()[1], other.group3()[2]]) + Simd32x3::from(self.group9()[3]) * other.group1(), g8: Simd32x4::from(self.group0()[0]) * other.group8() + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group0()[1], other.group4()[2], other.group4()[1], other.group5()[0]]) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group4()[2], other.group0()[1], other.group4()[0], other.group5()[1]]) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group4()[1], other.group4()[0], other.group0()[1], other.group5()[2]]) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) + Simd32x4::from(self.group2()[0]) * Simd32x4::from([other.group5()[0], other.group5()[1], other.group5()[2], other.group5()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) + Simd32x4::from(self.group4()[0]) * Simd32x4::from([other.group8()[3], other.group1()[2], other.group1()[1], other.group8()[3]]) * Simd32x4::from([-1.0, -1.0, 1.0, 0.0]) + Simd32x4::from(self.group4()[1]) * Simd32x4::from([other.group1()[2], other.group8()[3], other.group1()[0], other.group1()[2]]) * Simd32x4::from([1.0, -1.0, -1.0, 0.0]) + Simd32x4::from(self.group4()[2]) * Simd32x4::from([other.group1()[1], other.group1()[0], other.group8()[3], other.group1()[1]]) * Simd32x4::from([-1.0, 1.0, -1.0, 0.0]) + Simd32x4::from(self.group5()[0]) * Simd32x4::from([other.group2()[0], other.group8()[2], other.group8()[1], other.group1()[0]]) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]) + Simd32x4::from(self.group5()[1]) * Simd32x4::from([other.group8()[2], other.group2()[0], other.group8()[0], other.group1()[1]]) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group5()[2]) * Simd32x4::from([other.group8()[1], other.group8()[0], other.group2()[0], other.group1()[2]]) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) + Simd32x4::from(self.group8()[0]) * Simd32x4::from([other.group0()[0], other.group5()[2], other.group5()[1], other.group0()[0]]) * Simd32x4::from([1.0, 1.0, -1.0, 0.0]) + Simd32x4::from(self.group8()[1]) * Simd32x4::from([other.group5()[2], other.group0()[0], other.group5()[0], other.group5()[2]]) * Simd32x4::from([-1.0, 1.0, 1.0, 0.0]) + Simd32x4::from(self.group8()[2]) * Simd32x4::from([other.group5()[1], other.group5()[0], other.group0()[0], other.group5()[1]]) * Simd32x4::from([1.0, -1.0, 1.0, 0.0]) + Simd32x4::from(self.group8()[3]) * Simd32x4::from([other.group4()[0], other.group4()[1], other.group4()[2], other.group0()[0]]) + Simd32x4::from([self.group0()[1], self.group0()[1], self.group0()[1], self.group0()[0]]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group1()[0]]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]), g9: Simd32x4::from(self.group0()[0]) * other.group9() + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group1()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group0()[2], other.group6()[2], other.group6()[1], other.group7()[0]]) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group6()[2], other.group0()[2], other.group6()[0], other.group7()[1]]) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group6()[1], other.group6()[0], other.group0()[2], other.group7()[2]]) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) + Simd32x4::from(self.group2()[0]) * Simd32x4::from([other.group7()[0], other.group7()[1], other.group7()[2], other.group7()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) - Simd32x4::from(self.group2()[1]) * other.group8() + Simd32x4::from(self.group3()[0]) * Simd32x4::from([other.group0()[1], other.group4()[2], other.group4()[1], other.group5()[0]]) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]) + Simd32x4::from(self.group3()[1]) * Simd32x4::from([other.group4()[2], other.group0()[1], other.group4()[0], other.group5()[1]]) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group3()[2]) * Simd32x4::from([other.group4()[1], other.group4()[0], other.group0()[1], other.group5()[2]]) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) + Simd32x4::from(self.group3()[3]) * Simd32x4::from([other.group5()[0], other.group5()[1], other.group5()[2], other.group5()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) + Simd32x4::from(self.group4()[0]) * Simd32x4::from([other.group9()[3], other.group3()[2], other.group3()[1], other.group9()[3]]) * Simd32x4::from([-1.0, -1.0, 1.0, 0.0]) + Simd32x4::from(self.group4()[1]) * Simd32x4::from([other.group3()[2], other.group9()[3], other.group3()[0], other.group3()[2]]) * Simd32x4::from([1.0, -1.0, -1.0, 0.0]) + Simd32x4::from(self.group4()[2]) * Simd32x4::from([other.group3()[1], other.group3()[0], other.group9()[3], other.group3()[1]]) * Simd32x4::from([-1.0, 1.0, -1.0, 0.0]) + Simd32x4::from(self.group5()[0]) * Simd32x4::from([other.group3()[3], other.group9()[2], other.group9()[1], other.group3()[0]]) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]) + Simd32x4::from(self.group5()[1]) * Simd32x4::from([other.group9()[2], other.group3()[3], other.group9()[0], other.group3()[1]]) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group5()[2]) * Simd32x4::from([other.group9()[1], other.group9()[0], other.group3()[3], other.group3()[2]]) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) + Simd32x4::from(self.group6()[0]) * Simd32x4::from([other.group8()[3], other.group1()[2], other.group1()[1], other.group8()[3]]) * Simd32x4::from([1.0, 1.0, -1.0, 0.0]) + Simd32x4::from(self.group6()[1]) * Simd32x4::from([other.group1()[2], other.group8()[3], other.group1()[0], other.group1()[2]]) * Simd32x4::from([-1.0, 1.0, 1.0, 0.0]) + Simd32x4::from(self.group6()[2]) * Simd32x4::from([other.group1()[1], other.group1()[0], other.group8()[3], other.group1()[1]]) * Simd32x4::from([1.0, -1.0, 1.0, 0.0]) + Simd32x4::from(self.group7()[0]) * Simd32x4::from([other.group2()[0], other.group8()[2], other.group8()[1], other.group1()[0]]) * Simd32x4::from([-1.0, -1.0, 1.0, 1.0]) + Simd32x4::from(self.group7()[1]) * Simd32x4::from([other.group8()[2], other.group2()[0], other.group8()[0], other.group1()[1]]) * Simd32x4::from([1.0, -1.0, -1.0, 1.0]) + Simd32x4::from(self.group7()[2]) * Simd32x4::from([other.group8()[1], other.group8()[0], other.group2()[0], other.group1()[2]]) * Simd32x4::from([-1.0, 1.0, -1.0, 1.0]) + Simd32x4::from(self.group8()[0]) * Simd32x4::from([other.group2()[1], other.group7()[2], other.group7()[1], other.group2()[1]]) * Simd32x4::from([1.0, 1.0, -1.0, 0.0]) + Simd32x4::from(self.group8()[1]) * Simd32x4::from([other.group7()[2], other.group2()[1], other.group7()[0], other.group7()[2]]) * Simd32x4::from([-1.0, 1.0, 1.0, 0.0]) + Simd32x4::from(self.group8()[2]) * Simd32x4::from([other.group7()[1], other.group7()[0], other.group2()[1], other.group7()[1]]) * Simd32x4::from([1.0, -1.0, 1.0, 0.0]) + Simd32x4::from(self.group8()[3]) * Simd32x4::from([other.group6()[0], other.group6()[1], other.group6()[2], other.group2()[1]]) + Simd32x4::from(self.group9()[0]) * Simd32x4::from([other.group0()[0], other.group5()[2], other.group5()[1], other.group0()[0]]) * Simd32x4::from([1.0, 1.0, -1.0, 0.0]) + Simd32x4::from(self.group9()[1]) * Simd32x4::from([other.group5()[2], other.group0()[0], other.group5()[0], other.group5()[2]]) * Simd32x4::from([-1.0, 1.0, 1.0, 0.0]) + Simd32x4::from(self.group9()[2]) * Simd32x4::from([other.group5()[1], other.group5()[0], other.group0()[0], other.group5()[1]]) * Simd32x4::from([1.0, -1.0, 1.0, 0.0]) + Simd32x4::from(self.group9()[3]) * Simd32x4::from([other.group4()[0], other.group4()[1], other.group4()[2], other.group0()[0]]) + Simd32x4::from([self.group0()[1], self.group0()[1], self.group0()[1], self.group0()[0]]) * swizzle!(other.group3(), 0, 1, 2, 0) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]) } }
    }
}

impl OuterProduct<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn outer_product(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x3::from(self.group0()[0]) * other.group0() + Simd32x3::from(self.group0()[2]) * Simd32x3::from(other.group0()[0]) * Simd32x3::from([0.0, 0.0, 1.0]) + Simd32x3::from(self.group1()[0]) * Simd32x3::from([other.group8()[0], other.group8()[0], other.group9()[0]]) * Simd32x3::from([0.0, 1.0, 1.0]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from([other.group8()[1], other.group8()[1], other.group9()[1]]) * Simd32x3::from([0.0, 1.0, 1.0]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group8()[2], other.group8()[2], other.group9()[2]]) * Simd32x3::from([0.0, 1.0, 1.0]) + Simd32x3::from(self.group2()[0]) * Simd32x3::from([other.group8()[3], other.group8()[3], other.group9()[3]]) * Simd32x3::from([0.0, 1.0, 1.0]) + Simd32x3::from(self.group2()[1]) * Simd32x3::from(other.group0()[1]) * Simd32x3::from([0.0, 0.0, 1.0]) + Simd32x3::from(self.group3()[0]) * Simd32x3::from(other.group8()[0]) * Simd32x3::from([0.0, 0.0, -1.0]) + Simd32x3::from(self.group3()[1]) * Simd32x3::from(other.group8()[1]) * Simd32x3::from([0.0, 0.0, -1.0]) + Simd32x3::from(self.group3()[2]) * Simd32x3::from(other.group8()[2]) * Simd32x3::from([0.0, 0.0, -1.0]) + Simd32x3::from(self.group3()[3]) * Simd32x3::from(other.group8()[3]) * Simd32x3::from([0.0, 0.0, -1.0]) + Simd32x3::from(self.group4()[0]) * Simd32x3::from([other.group5()[0], other.group5()[0], other.group7()[0]]) * Simd32x3::from([0.0, -1.0, -1.0]) + Simd32x3::from(self.group4()[1]) * Simd32x3::from([other.group5()[1], other.group5()[1], other.group7()[1]]) * Simd32x3::from([0.0, -1.0, -1.0]) + Simd32x3::from(self.group4()[2]) * Simd32x3::from([other.group5()[2], other.group5()[2], other.group7()[2]]) * Simd32x3::from([0.0, -1.0, -1.0]) + Simd32x3::from(self.group5()[0]) * Simd32x3::from([other.group4()[0], other.group4()[0], other.group6()[0]]) * Simd32x3::from([0.0, -1.0, -1.0]) + Simd32x3::from(self.group5()[1]) * Simd32x3::from([other.group4()[1], other.group4()[1], other.group6()[1]]) * Simd32x3::from([0.0, -1.0, -1.0]) + Simd32x3::from(self.group5()[2]) * Simd32x3::from([other.group4()[2], other.group4()[2], other.group6()[2]]) * Simd32x3::from([0.0, -1.0, -1.0]) + Simd32x3::from(self.group6()[0]) * Simd32x3::from(other.group5()[0]) * Simd32x3::from([0.0, 0.0, -1.0]) + Simd32x3::from(self.group6()[1]) * Simd32x3::from(other.group5()[1]) * Simd32x3::from([0.0, 0.0, -1.0]) + Simd32x3::from(self.group6()[2]) * Simd32x3::from(other.group5()[2]) * Simd32x3::from([0.0, 0.0, -1.0]) + Simd32x3::from(self.group7()[0]) * Simd32x3::from(other.group4()[0]) * Simd32x3::from([0.0, 0.0, -1.0]) + Simd32x3::from(self.group7()[1]) * Simd32x3::from(other.group4()[1]) * Simd32x3::from([0.0, 0.0, -1.0]) + Simd32x3::from(self.group7()[2]) * Simd32x3::from(other.group4()[2]) * Simd32x3::from([0.0, 0.0, -1.0]) + Simd32x3::from(self.group8()[0]) * Simd32x3::from([other.group1()[0], other.group1()[0], other.group3()[0]]) * Simd32x3::from([0.0, -1.0, -1.0]) + Simd32x3::from(self.group8()[1]) * Simd32x3::from([other.group1()[1], other.group1()[1], other.group3()[1]]) * Simd32x3::from([0.0, -1.0, -1.0]) + Simd32x3::from(self.group8()[2]) * Simd32x3::from([other.group1()[2], other.group1()[2], other.group3()[2]]) * Simd32x3::from([0.0, -1.0, -1.0]) + Simd32x3::from(self.group8()[3]) * Simd32x3::from([other.group2()[0], other.group2()[0], other.group3()[3]]) * Simd32x3::from([0.0, -1.0, -1.0]) + Simd32x3::from(self.group9()[0]) * Simd32x3::from(other.group1()[0]) * Simd32x3::from([0.0, 0.0, 1.0]) + Simd32x3::from(self.group9()[1]) * Simd32x3::from(other.group1()[1]) * Simd32x3::from([0.0, 0.0, 1.0]) + Simd32x3::from(self.group9()[2]) * Simd32x3::from(other.group1()[2]) * Simd32x3::from([0.0, 0.0, 1.0]) + Simd32x3::from(self.group9()[3]) * Simd32x3::from(other.group2()[0]) * Simd32x3::from([0.0, 0.0, 1.0]) + swizzle!(self.group0(), 0, 1, 1) * Simd32x3::from([other.group0()[0], other.group0()[0], other.group2()[1]]) * Simd32x3::from([0.0, 1.0, 1.0]), g1: Simd32x3::from(self.group0()[0]) * other.group1() + self.group1() * Simd32x3::from(other.group0()[0]), g2: Simd32x2::from(self.group0()[0]) * other.group2() + self.group2() * Simd32x2::from(other.group0()[0]), g3: Simd32x4::from(self.group0()[0]) * other.group3() + Simd32x4::from(self.group2()[0]) * Simd32x4::from(other.group2()[1]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) - Simd32x4::from(self.group2()[1]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group2()[0]]) + Simd32x4::from(self.group3()[0]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group3()[1]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group3()[2]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group3()[3]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group1()[0]]) * Simd32x4::from([other.group2()[1], other.group2()[1], other.group2()[1], other.group2()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]), g4: Simd32x3::from(self.group0()[0]) * other.group4() + Simd32x3::from(self.group2()[0]) * other.group1() + Simd32x3::from(self.group4()[0]) * Simd32x3::from(other.group0()[0]) * Simd32x3::from([1.0, 0.0, 0.0]) + Simd32x3::from(self.group4()[1]) * Simd32x3::from(other.group0()[0]) * Simd32x3::from([0.0, 1.0, 0.0]) + Simd32x3::from(self.group4()[2]) * Simd32x3::from(other.group0()[0]) * Simd32x3::from([0.0, 0.0, 1.0]) + self.group1() * Simd32x3::from(other.group2()[0]) * Simd32x3::from(-1.0), g5: Simd32x3::from(self.group0()[0]) * other.group5() + Simd32x3::from(self.group1()[1]) * swizzle!(other.group1(), 2, 2, 0) * Simd32x3::from([1.0, 0.0, -1.0]) + Simd32x3::from(self.group1()[2]) * swizzle!(other.group1(), 1, 0, 1) * Simd32x3::from([-1.0, 1.0, 0.0]) + Simd32x3::from(self.group5()[1]) * Simd32x3::from(other.group0()[0]) * Simd32x3::from([0.0, 1.0, 0.0]) + Simd32x3::from(self.group5()[2]) * Simd32x3::from(other.group0()[0]) * Simd32x3::from([0.0, 0.0, 1.0]) + Simd32x3::from([self.group5()[0], self.group1()[0], self.group1()[0]]) * Simd32x3::from([other.group0()[0], other.group1()[2], other.group1()[1]]) * Simd32x3::from([1.0, -1.0, 1.0]), g6: Simd32x3::from(self.group0()[0]) * other.group6() + Simd32x3::from(self.group2()[0]) * Simd32x3::from([other.group3()[0], other.group3()[1], other.group3()[2]]) + Simd32x3::from(self.group2()[1]) * other.group4() + Simd32x3::from(self.group3()[0]) * Simd32x3::from(other.group2()[0]) * Simd32x3::from([1.0, 0.0, 0.0]) + Simd32x3::from(self.group3()[1]) * Simd32x3::from(other.group2()[0]) * Simd32x3::from([0.0, 1.0, 0.0]) + Simd32x3::from(self.group3()[2]) * Simd32x3::from(other.group2()[0]) * Simd32x3::from([0.0, 0.0, 1.0]) - Simd32x3::from(self.group3()[3]) * other.group1() + Simd32x3::from(self.group4()[0]) * Simd32x3::from(other.group2()[1]) * Simd32x3::from([1.0, 0.0, 0.0]) + Simd32x3::from(self.group4()[1]) * Simd32x3::from(other.group2()[1]) * Simd32x3::from([0.0, 1.0, 0.0]) + Simd32x3::from(self.group4()[2]) * Simd32x3::from(other.group2()[1]) * Simd32x3::from([0.0, 0.0, 1.0]) + Simd32x3::from(self.group6()[0]) * Simd32x3::from(other.group0()[0]) * Simd32x3::from([1.0, 0.0, 0.0]) + Simd32x3::from(self.group6()[1]) * Simd32x3::from(other.group0()[0]) * Simd32x3::from([0.0, 1.0, 0.0]) + Simd32x3::from(self.group6()[2]) * Simd32x3::from(other.group0()[0]) * Simd32x3::from([0.0, 0.0, 1.0]) + self.group1() * Simd32x3::from(other.group3()[3]) * Simd32x3::from(-1.0), g7: Simd32x3::from(self.group0()[0]) * other.group7() + Simd32x3::from(self.group1()[1]) * Simd32x3::from([other.group3()[2], other.group3()[2], other.group3()[0]]) * Simd32x3::from([1.0, 0.0, -1.0]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group3()[1], other.group3()[0], other.group3()[1]]) * Simd32x3::from([-1.0, 1.0, 0.0]) + Simd32x3::from(self.group2()[1]) * other.group5() + Simd32x3::from(self.group3()[0]) * swizzle!(other.group1(), 2, 2, 1) * Simd32x3::from([0.0, 1.0, -1.0]) + Simd32x3::from(self.group3()[1]) * swizzle!(other.group1(), 2, 2, 0) * Simd32x3::from([-1.0, 0.0, 1.0]) + Simd32x3::from(self.group3()[2]) * swizzle!(other.group1(), 1, 0, 1) * Simd32x3::from([1.0, -1.0, 0.0]) + Simd32x3::from(self.group5()[0]) * Simd32x3::from(other.group2()[1]) * Simd32x3::from([1.0, 0.0, 0.0]) + Simd32x3::from(self.group5()[1]) * Simd32x3::from(other.group2()[1]) * Simd32x3::from([0.0, 1.0, 0.0]) + Simd32x3::from(self.group5()[2]) * Simd32x3::from(other.group2()[1]) * Simd32x3::from([0.0, 0.0, 1.0]) + Simd32x3::from(self.group7()[0]) * Simd32x3::from(other.group0()[0]) * Simd32x3::from([1.0, 0.0, 0.0]) + Simd32x3::from(self.group7()[1]) * Simd32x3::from(other.group0()[0]) * Simd32x3::from([0.0, 1.0, 0.0]) + Simd32x3::from(self.group7()[2]) * Simd32x3::from(other.group0()[0]) * Simd32x3::from([0.0, 0.0, 1.0]) + Simd32x3::from(self.group1()[0]) * Simd32x3::from([other.group3()[0], other.group3()[2], other.group3()[1]]) * Simd32x3::from([0.0, -1.0, 1.0]), g8: Simd32x4::from(self.group0()[0]) * other.group8() + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group4()[2], other.group4()[2], other.group4()[0], other.group5()[1]]) * Simd32x4::from([-1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group4()[1], other.group4()[0], other.group4()[1], other.group5()[2]]) * Simd32x4::from([1.0, -1.0, 0.0, -1.0]) + Simd32x4::from(self.group2()[0]) * Simd32x4::from([other.group5()[0], other.group5()[1], other.group5()[2], other.group5()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) + Simd32x4::from(self.group4()[0]) * Simd32x4::from([other.group1()[2], other.group1()[2], other.group1()[1], other.group1()[2]]) * Simd32x4::from([0.0, -1.0, 1.0, 0.0]) + Simd32x4::from(self.group4()[1]) * Simd32x4::from([other.group1()[2], other.group1()[2], other.group1()[0], other.group1()[2]]) * Simd32x4::from([1.0, 0.0, -1.0, 0.0]) + Simd32x4::from(self.group4()[2]) * Simd32x4::from([other.group1()[1], other.group1()[0], other.group1()[1], other.group1()[1]]) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group5()[0]) * Simd32x4::from([other.group2()[0], other.group2()[0], other.group2()[0], other.group1()[0]]) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group5()[1]) * Simd32x4::from([other.group2()[0], other.group2()[0], other.group2()[0], other.group1()[1]]) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group5()[2]) * Simd32x4::from([other.group2()[0], other.group2()[0], other.group2()[0], other.group1()[2]]) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group8()[0]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group8()[1]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group8()[2]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group8()[3]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group4()[0], other.group4()[2], other.group4()[1], other.group5()[0]]) * Simd32x4::from([0.0, 1.0, -1.0, -1.0]), g9: Simd32x4::from(self.group0()[0]) * other.group9() + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group6()[2], other.group6()[2], other.group6()[0], other.group7()[1]]) * Simd32x4::from([-1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group6()[1], other.group6()[0], other.group6()[1], other.group7()[2]]) * Simd32x4::from([1.0, -1.0, 0.0, -1.0]) + Simd32x4::from(self.group2()[0]) * Simd32x4::from([other.group7()[0], other.group7()[1], other.group7()[2], other.group7()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) - Simd32x4::from(self.group2()[1]) * other.group8() + Simd32x4::from(self.group3()[0]) * Simd32x4::from([other.group4()[2], other.group4()[2], other.group4()[1], other.group5()[0]]) * Simd32x4::from([0.0, 1.0, -1.0, -1.0]) + Simd32x4::from(self.group3()[1]) * Simd32x4::from([other.group4()[2], other.group4()[2], other.group4()[0], other.group5()[1]]) * Simd32x4::from([-1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group3()[2]) * Simd32x4::from([other.group4()[1], other.group4()[0], other.group4()[1], other.group5()[2]]) * Simd32x4::from([1.0, -1.0, 0.0, -1.0]) + Simd32x4::from(self.group3()[3]) * Simd32x4::from([other.group5()[0], other.group5()[1], other.group5()[2], other.group5()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) + Simd32x4::from(self.group4()[0]) * swizzle!(other.group3(), 2, 2, 1, 2) * Simd32x4::from([0.0, -1.0, 1.0, 0.0]) + Simd32x4::from(self.group4()[1]) * swizzle!(other.group3(), 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, -1.0, 0.0]) + Simd32x4::from(self.group4()[2]) * swizzle!(other.group3(), 1, 0, 1, 1) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group5()[0]) * swizzle!(other.group3(), 3, 3, 3, 0) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group5()[1]) * swizzle!(other.group3(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group5()[2]) * swizzle!(other.group3(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group6()[0]) * Simd32x4::from([other.group1()[2], other.group1()[2], other.group1()[1], other.group1()[2]]) * Simd32x4::from([0.0, 1.0, -1.0, 0.0]) + Simd32x4::from(self.group6()[1]) * Simd32x4::from([other.group1()[2], other.group1()[2], other.group1()[0], other.group1()[2]]) * Simd32x4::from([-1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group6()[2]) * Simd32x4::from([other.group1()[1], other.group1()[0], other.group1()[1], other.group1()[1]]) * Simd32x4::from([1.0, -1.0, 0.0, 0.0]) + Simd32x4::from(self.group7()[0]) * Simd32x4::from([other.group2()[0], other.group2()[0], other.group2()[0], other.group1()[0]]) * Simd32x4::from([-1.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group7()[1]) * Simd32x4::from([other.group2()[0], other.group2()[0], other.group2()[0], other.group1()[1]]) * Simd32x4::from([0.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.group7()[2]) * Simd32x4::from([other.group2()[0], other.group2()[0], other.group2()[0], other.group1()[2]]) * Simd32x4::from([0.0, 0.0, -1.0, 1.0]) + Simd32x4::from(self.group8()[0]) * Simd32x4::from(other.group2()[1]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group8()[1]) * Simd32x4::from(other.group2()[1]) * Simd32x4::from([0.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group8()[2]) * Simd32x4::from(other.group2()[1]) * Simd32x4::from([0.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group8()[3]) * Simd32x4::from(other.group2()[1]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group9()[0]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group9()[1]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group9()[2]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group9()[3]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group6()[0], other.group6()[2], other.group6()[1], other.group7()[0]]) * Simd32x4::from([0.0, 1.0, -1.0, -1.0]) } }
    }
}

impl InnerProduct<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn inner_product(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x3::from(self.group0()[0]) * other.group0() + Simd32x3::from(self.group1()[1]) * Simd32x3::from(other.group1()[1]) * Simd32x3::from([1.0, 0.0, 0.0]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from(other.group1()[2]) * Simd32x3::from([1.0, 0.0, 0.0]) + Simd32x3::from(self.group5()[0]) * Simd32x3::from(other.group5()[0]) * Simd32x3::from([-1.0, 0.0, 0.0]) + Simd32x3::from(self.group5()[1]) * Simd32x3::from(other.group5()[1]) * Simd32x3::from([-1.0, 0.0, 0.0]) + Simd32x3::from(self.group5()[2]) * Simd32x3::from(other.group5()[2]) * Simd32x3::from([-1.0, 0.0, 0.0]) + Simd32x3::from(self.group8()[3]) * Simd32x3::from(other.group8()[3]) * Simd32x3::from([-1.0, 0.0, 0.0]) + Simd32x3::from([self.group1()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from([other.group1()[0], other.group0()[0], other.group0()[0]]), g1: Simd32x3::from(self.group0()[0]) * other.group1() + Simd32x3::from(self.group1()[0]) * Simd32x3::from([other.group0()[0], other.group5()[2], other.group5()[1]]) * Simd32x3::from([1.0, 1.0, -1.0]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from([other.group5()[2], other.group0()[0], other.group5()[0]]) * Simd32x3::from([-1.0, 1.0, 1.0]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group5()[1], other.group5()[0], other.group0()[0]]) * Simd32x3::from([1.0, -1.0, 1.0]) + Simd32x3::from(self.group5()[0]) * Simd32x3::from([other.group8()[3], other.group1()[2], other.group1()[1]]) * Simd32x3::from([1.0, 1.0, -1.0]) + Simd32x3::from(self.group5()[1]) * Simd32x3::from([other.group1()[2], other.group8()[3], other.group1()[0]]) * Simd32x3::from([-1.0, 1.0, 1.0]) + Simd32x3::from(self.group5()[2]) * Simd32x3::from([other.group1()[1], other.group1()[0], other.group8()[3]]) * Simd32x3::from([1.0, -1.0, 1.0]) + Simd32x3::from(self.group8()[3]) * other.group5(), g2: Simd32x2::from(self.group0()[0]) * other.group2() + Simd32x2::from(self.group1()[0]) * Simd32x2::from([other.group4()[0], other.group3()[0]]) * Simd32x2::from([-1.0, 1.0]) + Simd32x2::from(self.group1()[1]) * Simd32x2::from([other.group4()[1], other.group3()[1]]) * Simd32x2::from([-1.0, 1.0]) + Simd32x2::from(self.group1()[2]) * Simd32x2::from([other.group4()[2], other.group3()[2]]) * Simd32x2::from([-1.0, 1.0]) + Simd32x2::from(self.group2()[0]) * Simd32x2::from(other.group0()[0]) * Simd32x2::from([1.0, 0.0]) + Simd32x2::from(self.group2()[1]) * Simd32x2::from(other.group0()[0]) * Simd32x2::from([0.0, 1.0]) + Simd32x2::from(self.group3()[0]) * Simd32x2::from(other.group1()[0]) * Simd32x2::from([0.0, -1.0]) + Simd32x2::from(self.group3()[1]) * Simd32x2::from(other.group1()[1]) * Simd32x2::from([0.0, -1.0]) + Simd32x2::from(self.group3()[2]) * Simd32x2::from(other.group1()[2]) * Simd32x2::from([0.0, -1.0]) + Simd32x2::from(self.group4()[0]) * Simd32x2::from(other.group1()[0]) * Simd32x2::from([1.0, 0.0]) + Simd32x2::from(self.group4()[1]) * Simd32x2::from(other.group1()[1]) * Simd32x2::from([1.0, 0.0]) + Simd32x2::from(self.group4()[2]) * Simd32x2::from(other.group1()[2]) * Simd32x2::from([1.0, 0.0]) - Simd32x2::from(self.group5()[0]) * Simd32x2::from([other.group8()[0], other.group7()[0]]) - Simd32x2::from(self.group5()[1]) * Simd32x2::from([other.group8()[1], other.group7()[1]]) - Simd32x2::from(self.group5()[2]) * Simd32x2::from([other.group8()[2], other.group7()[2]]) + Simd32x2::from(self.group7()[0]) * Simd32x2::from(other.group5()[0]) * Simd32x2::from([0.0, -1.0]) + Simd32x2::from(self.group7()[1]) * Simd32x2::from(other.group5()[1]) * Simd32x2::from([0.0, -1.0]) + Simd32x2::from(self.group7()[2]) * Simd32x2::from(other.group5()[2]) * Simd32x2::from([0.0, -1.0]) + Simd32x2::from(self.group8()[0]) * Simd32x2::from(other.group5()[0]) * Simd32x2::from([-1.0, 0.0]) + Simd32x2::from(self.group8()[1]) * Simd32x2::from(other.group5()[1]) * Simd32x2::from([-1.0, 0.0]) + Simd32x2::from(self.group8()[2]) * Simd32x2::from(other.group5()[2]) * Simd32x2::from([-1.0, 0.0]) + Simd32x2::from(self.group8()[3]) * Simd32x2::from([other.group0()[1], other.group9()[3]]) * Simd32x2::from([1.0, -1.0]) + Simd32x2::from(self.group9()[3]) * Simd32x2::from(other.group8()[3]) * Simd32x2::from([0.0, 1.0]) + Simd32x2::from([self.group0()[1], self.group0()[0]]) * Simd32x2::from([other.group8()[3], other.group8()[0]]) * Simd32x2::from([-1.0, 0.0]), g3: Simd32x4::from(self.group0()[0]) * other.group3() + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group7()[2], other.group7()[2], other.group7()[1], other.group6()[0]]) * Simd32x4::from([0.0, 1.0, -1.0, -1.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group7()[2], other.group7()[2], other.group7()[0], other.group6()[1]]) * Simd32x4::from([-1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group7()[1], other.group7()[0], other.group7()[1], other.group6()[2]]) * Simd32x4::from([1.0, -1.0, 0.0, -1.0]) + Simd32x4::from(self.group3()[0]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group3()[1]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group3()[2]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group3()[3]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group5()[0]) * swizzle!(other.group9(), 3, 3, 3, 0) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group5()[1]) * swizzle!(other.group9(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group5()[2]) * swizzle!(other.group9(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group6()[0]) * Simd32x4::from(other.group1()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group6()[1]) * Simd32x4::from(other.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group6()[2]) * Simd32x4::from(other.group1()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group7()[0]) * Simd32x4::from([other.group1()[2], other.group1()[2], other.group1()[1], other.group1()[2]]) * Simd32x4::from([0.0, -1.0, 1.0, 0.0]) + Simd32x4::from(self.group7()[1]) * Simd32x4::from([other.group1()[2], other.group1()[2], other.group1()[0], other.group1()[2]]) * Simd32x4::from([1.0, 0.0, -1.0, 0.0]) + Simd32x4::from(self.group7()[2]) * Simd32x4::from([other.group1()[1], other.group1()[0], other.group1()[1], other.group1()[1]]) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group8()[3]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group9()[0]) * Simd32x4::from(other.group5()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group9()[1]) * Simd32x4::from(other.group5()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group9()[2]) * Simd32x4::from(other.group5()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group9()[3]) * Simd32x4::from([other.group5()[0], other.group5()[1], other.group5()[2], other.group5()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) + Simd32x4::from([self.group0()[0], self.group0()[0], self.group0()[0], self.group0()[2]]) * swizzle!(other.group8(), 0, 0, 0, 3) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]), g4: Simd32x3::from(self.group0()[0]) * other.group4() + Simd32x3::from(self.group0()[1]) * other.group5() + Simd32x3::from(self.group1()[1]) * Simd32x3::from([other.group8()[2], other.group8()[2], other.group8()[0]]) * Simd32x3::from([1.0, 0.0, -1.0]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group8()[1], other.group8()[0], other.group8()[1]]) * Simd32x3::from([-1.0, 1.0, 0.0]) + Simd32x3::from(self.group4()[0]) * Simd32x3::from(other.group0()[0]) * Simd32x3::from([1.0, 0.0, 0.0]) + Simd32x3::from(self.group4()[1]) * Simd32x3::from(other.group0()[0]) * Simd32x3::from([0.0, 1.0, 0.0]) + Simd32x3::from(self.group4()[2]) * Simd32x3::from(other.group0()[0]) * Simd32x3::from([0.0, 0.0, 1.0]) + Simd32x3::from(self.group5()[0]) * Simd32x3::from(other.group0()[1]) * Simd32x3::from([1.0, 0.0, 0.0]) + Simd32x3::from(self.group5()[1]) * Simd32x3::from(other.group0()[1]) * Simd32x3::from([0.0, 1.0, 0.0]) + Simd32x3::from(self.group5()[2]) * Simd32x3::from(other.group0()[1]) * Simd32x3::from([0.0, 0.0, 1.0]) + Simd32x3::from(self.group8()[0]) * swizzle!(other.group1(), 2, 2, 1) * Simd32x3::from([0.0, 1.0, -1.0]) + Simd32x3::from(self.group8()[1]) * swizzle!(other.group1(), 2, 2, 0) * Simd32x3::from([-1.0, 0.0, 1.0]) + Simd32x3::from(self.group8()[2]) * swizzle!(other.group1(), 1, 0, 1) * Simd32x3::from([1.0, -1.0, 0.0]) + Simd32x3::from(self.group1()[0]) * Simd32x3::from([other.group8()[0], other.group8()[2], other.group8()[1]]) * Simd32x3::from([0.0, -1.0, 1.0]), g5: Simd32x3::from(self.group0()[0]) * other.group5() + Simd32x3::from(self.group5()[0]) * Simd32x3::from(other.group0()[0]) * Simd32x3::from([1.0, 0.0, 0.0]) + Simd32x3::from(self.group5()[1]) * Simd32x3::from(other.group0()[0]) * Simd32x3::from([0.0, 1.0, 0.0]) + Simd32x3::from(self.group5()[2]) * Simd32x3::from(other.group0()[0]) * Simd32x3::from([0.0, 0.0, 1.0]) - Simd32x3::from(self.group8()[3]) * other.group1() + self.group1() * Simd32x3::from(other.group8()[3]) * Simd32x3::from(-1.0), g6: Simd32x3::from(self.group0()[0]) * other.group6() + Simd32x3::from(self.group0()[2]) * other.group5() + Simd32x3::from(self.group1()[1]) * Simd32x3::from([other.group9()[2], other.group9()[2], other.group9()[0]]) * Simd32x3::from([1.0, 0.0, -1.0]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group9()[1], other.group9()[0], other.group9()[1]]) * Simd32x3::from([-1.0, 1.0, 0.0]) + Simd32x3::from(self.group5()[0]) * Simd32x3::from(other.group0()[2]) * Simd32x3::from([1.0, 0.0, 0.0]) + Simd32x3::from(self.group5()[1]) * Simd32x3::from(other.group0()[2]) * Simd32x3::from([0.0, 1.0, 0.0]) + Simd32x3::from(self.group5()[2]) * Simd32x3::from(other.group0()[2]) * Simd32x3::from([0.0, 0.0, 1.0]) + Simd32x3::from(self.group6()[0]) * Simd32x3::from(other.group0()[0]) * Simd32x3::from([1.0, 0.0, 0.0]) + Simd32x3::from(self.group6()[1]) * Simd32x3::from(other.group0()[0]) * Simd32x3::from([0.0, 1.0, 0.0]) + Simd32x3::from(self.group6()[2]) * Simd32x3::from(other.group0()[0]) * Simd32x3::from([0.0, 0.0, 1.0]) + Simd32x3::from(self.group9()[0]) * swizzle!(other.group1(), 2, 2, 1) * Simd32x3::from([0.0, -1.0, 1.0]) + Simd32x3::from(self.group9()[1]) * swizzle!(other.group1(), 2, 2, 0) * Simd32x3::from([1.0, 0.0, -1.0]) + Simd32x3::from(self.group9()[2]) * swizzle!(other.group1(), 1, 0, 1) * Simd32x3::from([-1.0, 1.0, 0.0]) + Simd32x3::from(self.group1()[0]) * Simd32x3::from([other.group9()[0], other.group9()[2], other.group9()[1]]) * Simd32x3::from([0.0, -1.0, 1.0]), g7: Simd32x3::from(self.group0()[0]) * other.group7() + Simd32x3::from(self.group7()[0]) * Simd32x3::from(other.group0()[0]) * Simd32x3::from([1.0, 0.0, 0.0]) + Simd32x3::from(self.group7()[1]) * Simd32x3::from(other.group0()[0]) * Simd32x3::from([0.0, 1.0, 0.0]) + Simd32x3::from(self.group7()[2]) * Simd32x3::from(other.group0()[0]) * Simd32x3::from([0.0, 0.0, 1.0]) + Simd32x3::from(self.group9()[3]) * other.group1() + self.group1() * Simd32x3::from(other.group9()[3]) * Simd32x3::from(-1.0), g8: Simd32x4::from(self.group0()[0]) * other.group8() + Simd32x4::from(self.group1()[0]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group8()[0]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group8()[1]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group8()[2]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group8()[3]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from([self.group0()[1], self.group0()[1], self.group0()[1], self.group0()[0]]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group1()[0]]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]), g9: Simd32x4::from(self.group0()[0]) * other.group9() + Simd32x4::from(self.group1()[0]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group9()[0]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group9()[1]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group9()[2]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group9()[3]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from([self.group0()[2], self.group0()[2], self.group0()[2], self.group0()[0]]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group1()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) } }
    }
}

impl LeftContraction<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn left_contraction(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x3::from(self.group0()[0]) * other.group0() + Simd32x3::from(self.group1()[1]) * Simd32x3::from(other.group1()[1]) * Simd32x3::from([1.0, 0.0, 0.0]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from(other.group1()[2]) * Simd32x3::from([1.0, 0.0, 0.0]) + Simd32x3::from(self.group5()[0]) * Simd32x3::from(other.group5()[0]) * Simd32x3::from([-1.0, 0.0, 0.0]) + Simd32x3::from(self.group5()[1]) * Simd32x3::from(other.group5()[1]) * Simd32x3::from([-1.0, 0.0, 0.0]) + Simd32x3::from(self.group5()[2]) * Simd32x3::from(other.group5()[2]) * Simd32x3::from([-1.0, 0.0, 0.0]) + Simd32x3::from(self.group8()[3]) * Simd32x3::from(other.group8()[3]) * Simd32x3::from([-1.0, 0.0, 0.0]) + Simd32x3::from(self.group1()[0]) * Simd32x3::from(other.group1()[0]) * Simd32x3::from([1.0, 0.0, 0.0]), g1: Simd32x3::from(self.group0()[0]) * other.group1() + Simd32x3::from(self.group1()[1]) * swizzle!(other.group5(), 2, 2, 0) * Simd32x3::from([-1.0, 0.0, 1.0]) + Simd32x3::from(self.group1()[2]) * swizzle!(other.group5(), 1, 0, 1) * Simd32x3::from([1.0, -1.0, 0.0]) + Simd32x3::from(self.group5()[0]) * Simd32x3::from(other.group8()[3]) * Simd32x3::from([1.0, 0.0, 0.0]) + Simd32x3::from(self.group5()[1]) * Simd32x3::from(other.group8()[3]) * Simd32x3::from([0.0, 1.0, 0.0]) + Simd32x3::from(self.group5()[2]) * Simd32x3::from(other.group8()[3]) * Simd32x3::from([0.0, 0.0, 1.0]) + Simd32x3::from(self.group1()[0]) * swizzle!(other.group5(), 0, 2, 1) * Simd32x3::from([0.0, 1.0, -1.0]), g2: Simd32x2::from(self.group0()[0]) * other.group2() + Simd32x2::from(self.group1()[0]) * Simd32x2::from([other.group4()[0], other.group3()[0]]) * Simd32x2::from([-1.0, 1.0]) + Simd32x2::from(self.group1()[1]) * Simd32x2::from([other.group4()[1], other.group3()[1]]) * Simd32x2::from([-1.0, 1.0]) + Simd32x2::from(self.group1()[2]) * Simd32x2::from([other.group4()[2], other.group3()[2]]) * Simd32x2::from([-1.0, 1.0]) - Simd32x2::from(self.group5()[0]) * Simd32x2::from([other.group8()[0], other.group7()[0]]) - Simd32x2::from(self.group5()[1]) * Simd32x2::from([other.group8()[1], other.group7()[1]]) - Simd32x2::from(self.group5()[2]) * Simd32x2::from([other.group8()[2], other.group7()[2]]) + Simd32x2::from(self.group8()[3]) * Simd32x2::from([other.group0()[1], other.group9()[3]]) * Simd32x2::from([1.0, -1.0]), g3: Simd32x4::from(self.group0()[0]) * other.group3() + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group7()[2], other.group7()[2], other.group7()[0], other.group6()[1]]) * Simd32x4::from([-1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group7()[1], other.group7()[0], other.group7()[1], other.group6()[2]]) * Simd32x4::from([1.0, -1.0, 0.0, -1.0]) + Simd32x4::from(self.group5()[0]) * swizzle!(other.group9(), 3, 3, 3, 0) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group5()[1]) * swizzle!(other.group9(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group5()[2]) * swizzle!(other.group9(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group8()[3]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group7()[0], other.group7()[2], other.group7()[1], other.group6()[0]]) * Simd32x4::from([0.0, 1.0, -1.0, -1.0]), g4: Simd32x3::from(self.group0()[0]) * other.group4() + Simd32x3::from(self.group1()[1]) * Simd32x3::from([other.group8()[2], other.group8()[2], other.group8()[0]]) * Simd32x3::from([1.0, 0.0, -1.0]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group8()[1], other.group8()[0], other.group8()[1]]) * Simd32x3::from([-1.0, 1.0, 0.0]) + Simd32x3::from(self.group5()[0]) * Simd32x3::from(other.group0()[1]) * Simd32x3::from([1.0, 0.0, 0.0]) + Simd32x3::from(self.group5()[1]) * Simd32x3::from(other.group0()[1]) * Simd32x3::from([0.0, 1.0, 0.0]) + Simd32x3::from(self.group5()[2]) * Simd32x3::from(other.group0()[1]) * Simd32x3::from([0.0, 0.0, 1.0]) + Simd32x3::from(self.group1()[0]) * Simd32x3::from([other.group8()[0], other.group8()[2], other.group8()[1]]) * Simd32x3::from([0.0, -1.0, 1.0]), g5: Simd32x3::from(self.group0()[0]) * other.group5() + self.group1() * Simd32x3::from(other.group8()[3]) * Simd32x3::from(-1.0), g6: Simd32x3::from(self.group0()[0]) * other.group6() + Simd32x3::from(self.group1()[1]) * Simd32x3::from([other.group9()[2], other.group9()[2], other.group9()[0]]) * Simd32x3::from([1.0, 0.0, -1.0]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group9()[1], other.group9()[0], other.group9()[1]]) * Simd32x3::from([-1.0, 1.0, 0.0]) + Simd32x3::from(self.group5()[0]) * Simd32x3::from(other.group0()[2]) * Simd32x3::from([1.0, 0.0, 0.0]) + Simd32x3::from(self.group5()[1]) * Simd32x3::from(other.group0()[2]) * Simd32x3::from([0.0, 1.0, 0.0]) + Simd32x3::from(self.group5()[2]) * Simd32x3::from(other.group0()[2]) * Simd32x3::from([0.0, 0.0, 1.0]) + Simd32x3::from(self.group1()[0]) * Simd32x3::from([other.group9()[0], other.group9()[2], other.group9()[1]]) * Simd32x3::from([0.0, -1.0, 1.0]), g7: Simd32x3::from(self.group0()[0]) * other.group7() + self.group1() * Simd32x3::from(other.group9()[3]) * Simd32x3::from(-1.0), g8: Simd32x4::from(self.group0()[0]) * other.group8() + Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group1()[0]]) * Simd32x4::from([other.group0()[1], other.group0()[1], other.group0()[1], other.group0()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]), g9: Simd32x4::from(self.group0()[0]) * other.group9() + Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group1()[0]]) * Simd32x4::from([other.group0()[2], other.group0()[2], other.group0()[2], other.group0()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) } }
    }
}

impl RightContraction<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn right_contraction(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x3::from(self.group1()[0]) * Simd32x3::from(other.group1()[0]) * Simd32x3::from([1.0, 0.0, 0.0]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from(other.group1()[1]) * Simd32x3::from([1.0, 0.0, 0.0]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from(other.group1()[2]) * Simd32x3::from([1.0, 0.0, 0.0]) + Simd32x3::from(self.group5()[0]) * Simd32x3::from(other.group5()[0]) * Simd32x3::from([-1.0, 0.0, 0.0]) + Simd32x3::from(self.group5()[1]) * Simd32x3::from(other.group5()[1]) * Simd32x3::from([-1.0, 0.0, 0.0]) + Simd32x3::from(self.group5()[2]) * Simd32x3::from(other.group5()[2]) * Simd32x3::from([-1.0, 0.0, 0.0]) + Simd32x3::from(self.group8()[3]) * Simd32x3::from(other.group8()[3]) * Simd32x3::from([-1.0, 0.0, 0.0]) + self.group0() * Simd32x3::from(other.group0()[0]), g1: Simd32x3::from(self.group5()[0]) * swizzle!(other.group1(), 2, 2, 1) * Simd32x3::from([0.0, 1.0, -1.0]) + Simd32x3::from(self.group5()[1]) * swizzle!(other.group1(), 2, 2, 0) * Simd32x3::from([-1.0, 0.0, 1.0]) + Simd32x3::from(self.group5()[2]) * swizzle!(other.group1(), 1, 0, 1) * Simd32x3::from([1.0, -1.0, 0.0]) + Simd32x3::from(self.group8()[3]) * other.group5() + self.group1() * Simd32x3::from(other.group0()[0]), g2: Simd32x2::from(self.group2()[0]) * Simd32x2::from(other.group0()[0]) * Simd32x2::from([1.0, 0.0]) + Simd32x2::from(self.group2()[1]) * Simd32x2::from(other.group0()[0]) * Simd32x2::from([0.0, 1.0]) + Simd32x2::from(self.group3()[0]) * Simd32x2::from(other.group1()[0]) * Simd32x2::from([0.0, -1.0]) + Simd32x2::from(self.group3()[1]) * Simd32x2::from(other.group1()[1]) * Simd32x2::from([0.0, -1.0]) + Simd32x2::from(self.group3()[2]) * Simd32x2::from(other.group1()[2]) * Simd32x2::from([0.0, -1.0]) + Simd32x2::from(self.group4()[0]) * Simd32x2::from(other.group1()[0]) * Simd32x2::from([1.0, 0.0]) + Simd32x2::from(self.group4()[1]) * Simd32x2::from(other.group1()[1]) * Simd32x2::from([1.0, 0.0]) + Simd32x2::from(self.group4()[2]) * Simd32x2::from(other.group1()[2]) * Simd32x2::from([1.0, 0.0]) + Simd32x2::from(self.group7()[0]) * Simd32x2::from(other.group5()[0]) * Simd32x2::from([0.0, -1.0]) + Simd32x2::from(self.group7()[1]) * Simd32x2::from(other.group5()[1]) * Simd32x2::from([0.0, -1.0]) + Simd32x2::from(self.group7()[2]) * Simd32x2::from(other.group5()[2]) * Simd32x2::from([0.0, -1.0]) + Simd32x2::from(self.group8()[0]) * Simd32x2::from(other.group5()[0]) * Simd32x2::from([-1.0, 0.0]) + Simd32x2::from(self.group8()[1]) * Simd32x2::from(other.group5()[1]) * Simd32x2::from([-1.0, 0.0]) + Simd32x2::from(self.group8()[2]) * Simd32x2::from(other.group5()[2]) * Simd32x2::from([-1.0, 0.0]) + Simd32x2::from(self.group9()[3]) * Simd32x2::from(other.group8()[3]) * Simd32x2::from([0.0, 1.0]) + Simd32x2::from([self.group0()[1], self.group0()[0]]) * Simd32x2::from([other.group8()[3], other.group8()[0]]) * Simd32x2::from([-1.0, 0.0]), g3: Simd32x4::from(self.group3()[0]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group3()[1]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group3()[2]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group3()[3]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group6()[0]) * Simd32x4::from(other.group1()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group6()[1]) * Simd32x4::from(other.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group6()[2]) * Simd32x4::from(other.group1()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group7()[0]) * Simd32x4::from([other.group1()[2], other.group1()[2], other.group1()[1], other.group1()[2]]) * Simd32x4::from([0.0, -1.0, 1.0, 0.0]) + Simd32x4::from(self.group7()[1]) * Simd32x4::from([other.group1()[2], other.group1()[2], other.group1()[0], other.group1()[2]]) * Simd32x4::from([1.0, 0.0, -1.0, 0.0]) + Simd32x4::from(self.group7()[2]) * Simd32x4::from([other.group1()[1], other.group1()[0], other.group1()[1], other.group1()[1]]) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group9()[0]) * Simd32x4::from(other.group5()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group9()[1]) * Simd32x4::from(other.group5()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group9()[2]) * Simd32x4::from(other.group5()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group9()[3]) * Simd32x4::from([other.group5()[0], other.group5()[1], other.group5()[2], other.group5()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) + Simd32x4::from([self.group0()[0], self.group0()[0], self.group0()[0], self.group0()[2]]) * swizzle!(other.group8(), 0, 0, 0, 3) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]), g4: Simd32x3::from(self.group0()[1]) * other.group5() + Simd32x3::from(self.group8()[0]) * swizzle!(other.group1(), 2, 2, 1) * Simd32x3::from([0.0, 1.0, -1.0]) + Simd32x3::from(self.group8()[1]) * swizzle!(other.group1(), 2, 2, 0) * Simd32x3::from([-1.0, 0.0, 1.0]) + Simd32x3::from(self.group8()[2]) * swizzle!(other.group1(), 1, 0, 1) * Simd32x3::from([1.0, -1.0, 0.0]) + self.group4() * Simd32x3::from(other.group0()[0]), g5: Simd32x3::from(0.0) - Simd32x3::from(self.group8()[3]) * other.group1() + self.group5() * Simd32x3::from(other.group0()[0]), g6: Simd32x3::from(self.group0()[2]) * other.group5() + Simd32x3::from(self.group9()[0]) * swizzle!(other.group1(), 2, 2, 1) * Simd32x3::from([0.0, -1.0, 1.0]) + Simd32x3::from(self.group9()[1]) * swizzle!(other.group1(), 2, 2, 0) * Simd32x3::from([1.0, 0.0, -1.0]) + Simd32x3::from(self.group9()[2]) * swizzle!(other.group1(), 1, 0, 1) * Simd32x3::from([-1.0, 1.0, 0.0]) + self.group6() * Simd32x3::from(other.group0()[0]), g7: Simd32x3::from(self.group9()[3]) * other.group1() + self.group7() * Simd32x3::from(other.group0()[0]), g8: Simd32x4::from(self.group8()[0]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group8()[1]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group8()[2]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group8()[3]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from([self.group0()[1], self.group0()[1], self.group0()[1], self.group0()[0]]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group1()[0]]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]), g9: Simd32x4::from(self.group9()[0]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group9()[1]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group9()[2]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group9()[3]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from([self.group0()[2], self.group0()[2], self.group0()[2], self.group0()[0]]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group1()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) } }
    }
}

impl ScalarProduct<MultiVector> for MultiVector {
    type Output = Scalar;

    fn scalar_product(self, other: MultiVector) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[0] * other.group0()[0] + self.group1()[0] * other.group1()[0] + self.group1()[1] * other.group1()[1] + self.group1()[2] * other.group1()[2] - self.group5()[0] * other.group5()[0] - self.group5()[1] * other.group5()[1] - self.group5()[2] * other.group5()[2] - self.group8()[3] * other.group8()[3] } }
    }
}

impl SquaredMagnitude for MultiVector {
    type Output = Scalar;

    fn squared_magnitude(self) -> Scalar {
        self.scalar_product(self.reversal())
    }
}

impl Magnitude for MultiVector {
    type Output = Scalar;

    fn magnitude(self) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.squared_magnitude().group0().sqrt() } }
    }
}

impl Scale for MultiVector {
    type Output = MultiVector;

    fn scale(self, other: f32) -> MultiVector {
        self.geometric_product(Scalar { groups: ScalarGroups { g0: other } })
    }
}

impl Signum for MultiVector {
    type Output = MultiVector;

    fn signum(self) -> MultiVector {
        self.geometric_product(Scalar { groups: ScalarGroups { g0: 1.0 / self.magnitude().group0() } })
    }
}

impl Inverse for MultiVector {
    type Output = MultiVector;

    fn inverse(self) -> MultiVector {
        self.reversal().geometric_product(Scalar { groups: ScalarGroups { g0: 1.0 / self.squared_magnitude().group0() } })
    }
}

impl GeometricQuotient<Scalar> for AntiScalar {
    type Output = AntiScalar;

    fn geometric_quotient(self, other: Scalar) -> AntiScalar {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Flector> for Circle {
    type Output = Flector;

    fn transformation(self, other: Flector) -> Flector {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl Transformation<Motor> for Circle {
    type Output = Motor;

    fn transformation(self, other: Motor) -> Motor {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl GeometricQuotient<MultiVector> for Circle {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<MultiVector> for Circle {
    type Output = MultiVector;

    fn transformation(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl GeometricQuotient<Scalar> for Circle {
    type Output = Circle;

    fn geometric_quotient(self, other: Scalar) -> Circle {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<FlatPoint> for Dipole {
    type Output = FlatPoint;

    fn transformation(self, other: FlatPoint) -> FlatPoint {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl Transformation<Flector> for Dipole {
    type Output = Flector;

    fn transformation(self, other: Flector) -> Flector {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl Transformation<Line> for Dipole {
    type Output = Line;

    fn transformation(self, other: Line) -> Line {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl Transformation<Motor> for Dipole {
    type Output = Motor;

    fn transformation(self, other: Motor) -> Motor {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl GeometricQuotient<MultiVector> for Dipole {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<MultiVector> for Dipole {
    type Output = MultiVector;

    fn transformation(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl Transformation<Rotor> for Dipole {
    type Output = Rotor;

    fn transformation(self, other: Rotor) -> Rotor {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl GeometricQuotient<Scalar> for Dipole {
    type Output = Dipole;

    fn geometric_quotient(self, other: Scalar) -> Dipole {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Translator> for Dipole {
    type Output = Translator;

    fn transformation(self, other: Translator) -> Translator {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl GeometricQuotient<Dipole> for FlatPoint {
    type Output = Flector;

    fn geometric_quotient(self, other: Dipole) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for FlatPoint {
    type Output = FlatPoint;

    fn geometric_quotient(self, other: Scalar) -> FlatPoint {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Circle> for Flector {
    type Output = Motor;

    fn geometric_quotient(self, other: Circle) -> Motor {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for Flector {
    type Output = Flector;

    fn geometric_quotient(self, other: Dipole) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RadialPoint> for Flector {
    type Output = Motor;

    fn geometric_quotient(self, other: RadialPoint) -> Motor {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for Flector {
    type Output = Flector;

    fn geometric_quotient(self, other: Scalar) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for Line {
    type Output = Motor;

    fn geometric_quotient(self, other: Dipole) -> Motor {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RadialPoint> for Line {
    type Output = Flector;

    fn geometric_quotient(self, other: RadialPoint) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for Line {
    type Output = Line;

    fn geometric_quotient(self, other: Scalar) -> Line {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Circle> for Motor {
    type Output = Flector;

    fn geometric_quotient(self, other: Circle) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for Motor {
    type Output = Motor;

    fn geometric_quotient(self, other: Dipole) -> Motor {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RadialPoint> for Motor {
    type Output = Flector;

    fn geometric_quotient(self, other: RadialPoint) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for Motor {
    type Output = Motor;

    fn geometric_quotient(self, other: Scalar) -> Motor {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Circle> for MultiVector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Circle) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Circle> for MultiVector {
    type Output = Circle;

    fn transformation(self, other: Circle) -> Circle {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl GeometricQuotient<Dipole> for MultiVector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Dipole> for MultiVector {
    type Output = Dipole;

    fn transformation(self, other: Dipole) -> Dipole {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl Powi for MultiVector {
    type Output = MultiVector;

    fn powi(self, exponent: isize) -> MultiVector {
        if exponent == 0 {
            return MultiVector::one();
        }
        let mut x: MultiVector = if exponent < 0 { self.inverse() } else { self };
        let mut y: MultiVector = MultiVector::one();
        let mut n: isize = exponent.abs();
        while 1 < n {
            if n & 1 == 1 {
                y = x.geometric_product(y);
            }
            x = x.geometric_product(x);
            n = n >> 1;
        }
        x.geometric_product(y)
    }
}

impl GeometricQuotient<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn transformation(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl GeometricQuotient<RadialPoint> for MultiVector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RadialPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<RadialPoint> for MultiVector {
    type Output = RadialPoint;

    fn transformation(self, other: RadialPoint) -> RadialPoint {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl GeometricQuotient<Scalar> for MultiVector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Scalar) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Scalar> for MultiVector {
    type Output = Scalar;

    fn transformation(self, other: Scalar) -> Scalar {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl GeometricQuotient<Scalar> for Plane {
    type Output = Plane;

    fn geometric_quotient(self, other: Scalar) -> Plane {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Flector> for RadialPoint {
    type Output = Flector;

    fn transformation(self, other: Flector) -> Flector {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl Transformation<Line> for RadialPoint {
    type Output = Line;

    fn transformation(self, other: Line) -> Line {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl Transformation<Motor> for RadialPoint {
    type Output = Motor;

    fn transformation(self, other: Motor) -> Motor {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl GeometricQuotient<MultiVector> for RadialPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<MultiVector> for RadialPoint {
    type Output = MultiVector;

    fn transformation(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl GeometricQuotient<Scalar> for RadialPoint {
    type Output = RadialPoint;

    fn geometric_quotient(self, other: Scalar) -> RadialPoint {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for Rotor {
    type Output = Rotor;

    fn geometric_quotient(self, other: Dipole) -> Rotor {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for Rotor {
    type Output = Rotor;

    fn geometric_quotient(self, other: Scalar) -> Rotor {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<AntiScalar> for Scalar {
    type Output = AntiScalar;

    fn transformation(self, other: AntiScalar) -> AntiScalar {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl GeometricQuotient<Circle> for Scalar {
    type Output = Circle;

    fn geometric_quotient(self, other: Circle) -> Circle {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Circle> for Scalar {
    type Output = Circle;

    fn transformation(self, other: Circle) -> Circle {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl GeometricQuotient<Dipole> for Scalar {
    type Output = Dipole;

    fn geometric_quotient(self, other: Dipole) -> Dipole {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Dipole> for Scalar {
    type Output = Dipole;

    fn transformation(self, other: Dipole) -> Dipole {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl Transformation<FlatPoint> for Scalar {
    type Output = FlatPoint;

    fn transformation(self, other: FlatPoint) -> FlatPoint {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl Transformation<Flector> for Scalar {
    type Output = Flector;

    fn transformation(self, other: Flector) -> Flector {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl Transformation<Line> for Scalar {
    type Output = Line;

    fn transformation(self, other: Line) -> Line {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl Transformation<Motor> for Scalar {
    type Output = Motor;

    fn transformation(self, other: Motor) -> Motor {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl GeometricQuotient<MultiVector> for Scalar {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<MultiVector> for Scalar {
    type Output = MultiVector;

    fn transformation(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl Transformation<Plane> for Scalar {
    type Output = Plane;

    fn transformation(self, other: Plane) -> Plane {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl GeometricQuotient<RadialPoint> for Scalar {
    type Output = RadialPoint;

    fn geometric_quotient(self, other: RadialPoint) -> RadialPoint {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<RadialPoint> for Scalar {
    type Output = RadialPoint;

    fn transformation(self, other: RadialPoint) -> RadialPoint {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl Transformation<Rotor> for Scalar {
    type Output = Rotor;

    fn transformation(self, other: Rotor) -> Rotor {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl Powi for Scalar {
    type Output = Scalar;

    fn powi(self, exponent: isize) -> Scalar {
        if exponent == 0 {
            return Scalar::one();
        }
        let mut x: Scalar = if exponent < 0 { self.inverse() } else { self };
        let mut y: Scalar = Scalar::one();
        let mut n: isize = exponent.abs();
        while 1 < n {
            if n & 1 == 1 {
                y = x.geometric_product(y);
            }
            x = x.geometric_product(x);
            n = n >> 1;
        }
        x.geometric_product(y)
    }
}

impl GeometricQuotient<Scalar> for Scalar {
    type Output = Scalar;

    fn geometric_quotient(self, other: Scalar) -> Scalar {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Scalar> for Scalar {
    type Output = Scalar;

    fn transformation(self, other: Scalar) -> Scalar {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl Transformation<Sphere> for Scalar {
    type Output = Sphere;

    fn transformation(self, other: Sphere) -> Sphere {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl Transformation<Translator> for Scalar {
    type Output = Translator;

    fn transformation(self, other: Translator) -> Translator {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl GeometricQuotient<Scalar> for Sphere {
    type Output = Sphere;

    fn geometric_quotient(self, other: Scalar) -> Sphere {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for Translator {
    type Output = Motor;

    fn geometric_quotient(self, other: Dipole) -> Motor {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for Translator {
    type Output = Translator;

    fn geometric_quotient(self, other: Scalar) -> Translator {
        self.geometric_product(other.inverse())
    }
}

