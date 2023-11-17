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
    /// e0123
    g0: f32,
}

#[derive(Clone, Copy)]
pub union AntiScalar {
    groups: AntiScalarGroups,
    /// e0123
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
            .field("e0123", &self[0])
            .finish()
    }
}

#[derive(Clone, Copy)]
struct HomogeneousMagnitudeGroups {
    /// 1, e0123
    g0: Simd32x2,
}

#[derive(Clone, Copy)]
pub union HomogeneousMagnitude {
    groups: HomogeneousMagnitudeGroups,
    /// 1, e0123, 0, 0
    elements: [f32; 4],
}

impl HomogeneousMagnitude {
    #[allow(clippy::too_many_arguments)]
    pub const fn new(element0: f32, element1: f32) -> Self {
        Self { elements: [element0, element1, 0.0, 0.0] }
    }
    pub const fn from_groups(g0: Simd32x2) -> Self {
        Self { groups: HomogeneousMagnitudeGroups { g0 } }
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

const HOMOGENEOUSMAGNITUDE_INDEX_REMAP: [usize; 2] = [0, 1];

impl std::ops::Index<usize> for HomogeneousMagnitude {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[HOMOGENEOUSMAGNITUDE_INDEX_REMAP[index]] }
    }
}

impl std::ops::IndexMut<usize> for HomogeneousMagnitude {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[HOMOGENEOUSMAGNITUDE_INDEX_REMAP[index]] }
    }
}

impl std::convert::From<HomogeneousMagnitude> for [f32; 2] {
    fn from(vector: HomogeneousMagnitude) -> Self {
        unsafe { [vector.elements[0], vector.elements[1]] }
    }
}

impl std::convert::From<[f32; 2]> for HomogeneousMagnitude {
    fn from(array: [f32; 2]) -> Self {
        Self { elements: [array[0], array[1], 0.0, 0.0] }
    }
}

impl std::fmt::Debug for HomogeneousMagnitude {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .debug_struct("HomogeneousMagnitude")
            .field("1", &self[0])
            .field("e0123", &self[1])
            .finish()
    }
}

#[derive(Clone, Copy)]
struct PointGroups {
    /// e0, e1, e2, e3
    g0: Simd32x4,
}

#[derive(Clone, Copy)]
pub union Point {
    groups: PointGroups,
    /// e0, e1, e2, e3
    elements: [f32; 4],
}

impl Point {
    #[allow(clippy::too_many_arguments)]
    pub const fn new(element0: f32, element1: f32, element2: f32, element3: f32) -> Self {
        Self { elements: [element0, element1, element2, element3] }
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

impl std::convert::From<Point> for [f32; 4] {
    fn from(vector: Point) -> Self {
        unsafe { [vector.elements[0], vector.elements[1], vector.elements[2], vector.elements[3]] }
    }
}

impl std::convert::From<[f32; 4]> for Point {
    fn from(array: [f32; 4]) -> Self {
        Self { elements: [array[0], array[1], array[2], array[3]] }
    }
}

impl std::fmt::Debug for Point {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .debug_struct("Point")
            .field("e0", &self[0])
            .field("e1", &self[1])
            .field("e2", &self[2])
            .field("e3", &self[3])
            .finish()
    }
}

#[derive(Clone, Copy)]
struct LineGroups {
    /// -e03, -e13, -e23
    g0: Simd32x3,
    /// e12, -e02, e01
    g1: Simd32x3,
}

#[derive(Clone, Copy)]
pub union Line {
    groups: LineGroups,
    /// -e03, -e13, -e23, 0, e12, -e02, e01, 0
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
            .field("-e03", &self[0])
            .field("-e13", &self[1])
            .field("-e23", &self[2])
            .field("e12", &self[3])
            .field("-e02", &self[4])
            .field("e01", &self[5])
            .finish()
    }
}

#[derive(Clone, Copy)]
struct PlaneGroups {
    /// e123, -e023, e013, -e012
    g0: Simd32x4,
}

#[derive(Clone, Copy)]
pub union Plane {
    groups: PlaneGroups,
    /// e123, -e023, e013, -e012
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
            .field("e123", &self[0])
            .field("-e023", &self[1])
            .field("e013", &self[2])
            .field("-e012", &self[3])
            .finish()
    }
}

#[derive(Clone, Copy)]
struct MotorGroups {
    /// -e03, -e13, -e23, e0123
    g0: Simd32x4,
    /// e12, -e02, e01
    g1: Simd32x3,
}

#[derive(Clone, Copy)]
pub union Motor {
    groups: MotorGroups,
    /// -e03, -e13, -e23, e0123, e12, -e02, e01, 0
    elements: [f32; 8],
}

impl Motor {
    #[allow(clippy::too_many_arguments)]
    pub const fn new(element0: f32, element1: f32, element2: f32, element3: f32, element4: f32, element5: f32, element6: f32) -> Self {
        Self { elements: [element0, element1, element2, element3, element4, element5, element6, 0.0] }
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
        unsafe { [vector.elements[0], vector.elements[1], vector.elements[2], vector.elements[3], vector.elements[4], vector.elements[5], vector.elements[6]] }
    }
}

impl std::convert::From<[f32; 7]> for Motor {
    fn from(array: [f32; 7]) -> Self {
        Self { elements: [array[0], array[1], array[2], array[3], array[4], array[5], array[6], 0.0] }
    }
}

impl std::fmt::Debug for Motor {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .debug_struct("Motor")
            .field("-e03", &self[0])
            .field("-e13", &self[1])
            .field("-e23", &self[2])
            .field("e0123", &self[3])
            .field("e12", &self[4])
            .field("-e02", &self[5])
            .field("e01", &self[6])
            .finish()
    }
}

#[derive(Clone, Copy)]
struct RotorGroups {
    /// -e03, -e13, -e23, e0123
    g0: Simd32x4,
}

#[derive(Clone, Copy)]
pub union Rotor {
    groups: RotorGroups,
    /// -e03, -e13, -e23, e0123
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
            .field("-e03", &self[0])
            .field("-e13", &self[1])
            .field("-e23", &self[2])
            .field("e0123", &self[3])
            .finish()
    }
}

#[derive(Clone, Copy)]
struct TranslatorGroups {
    /// e12, -e02, e01, e0123
    g0: Simd32x4,
}

#[derive(Clone, Copy)]
pub union Translator {
    groups: TranslatorGroups,
    /// e12, -e02, e01, e0123
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
            .field("e12", &self[0])
            .field("-e02", &self[1])
            .field("e01", &self[2])
            .field("e0123", &self[3])
            .finish()
    }
}

#[derive(Clone, Copy)]
struct FlectorGroups {
    /// e0, e1, e2, e3
    g0: Simd32x4,
    /// e123, -e023, e013, -e012
    g1: Simd32x4,
}

#[derive(Clone, Copy)]
pub union Flector {
    groups: FlectorGroups,
    /// e0, e1, e2, e3, e123, -e023, e013, -e012
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
            .field("e0", &self[0])
            .field("e1", &self[1])
            .field("e2", &self[2])
            .field("e3", &self[3])
            .field("e123", &self[4])
            .field("-e023", &self[5])
            .field("e013", &self[6])
            .field("-e012", &self[7])
            .finish()
    }
}

#[derive(Clone, Copy)]
struct MultiVectorGroups {
    /// 1, e0123, e0123
    g0: Simd32x3,
    /// e0, e1, e2, e3
    g1: Simd32x4,
    /// -e03, -e13, -e23
    g2: Simd32x3,
    /// e12, -e02, e01
    g3: Simd32x3,
    /// -e03, -e13, -e23
    g4: Simd32x3,
    /// e12, -e02, e01
    g5: Simd32x3,
    /// e123, -e023, e013, -e012
    g6: Simd32x4,
    /// e123, -e023, e013, -e012
    g7: Simd32x4,
}

#[derive(Clone, Copy)]
pub union MultiVector {
    groups: MultiVectorGroups,
    /// 1, e0123, e0123, 0, e0, e1, e2, e3, -e03, -e13, -e23, 0, e12, -e02, e01, 0, -e03, -e13, -e23, 0, e12, -e02, e01, 0, e123, -e023, e013, -e012, e123, -e023, e013, -e012
    elements: [f32; 32],
}

impl MultiVector {
    #[allow(clippy::too_many_arguments)]
    pub const fn new(element0: f32, element1: f32, element2: f32, element3: f32, element4: f32, element5: f32, element6: f32, element7: f32, element8: f32, element9: f32, element10: f32, element11: f32, element12: f32, element13: f32, element14: f32, element15: f32, element16: f32, element17: f32, element18: f32, element19: f32, element20: f32, element21: f32, element22: f32, element23: f32, element24: f32, element25: f32, element26: f32) -> Self {
        Self { elements: [element0, element1, element2, 0.0, element3, element4, element5, element6, element7, element8, element9, 0.0, element10, element11, element12, 0.0, element13, element14, element15, 0.0, element16, element17, element18, 0.0, element19, element20, element21, element22, element23, element24, element25, element26] }
    }
    pub const fn from_groups(g0: Simd32x3, g1: Simd32x4, g2: Simd32x3, g3: Simd32x3, g4: Simd32x3, g5: Simd32x3, g6: Simd32x4, g7: Simd32x4) -> Self {
        Self { groups: MultiVectorGroups { g0, g1, g2, g3, g4, g5, g6, g7 } }
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
    pub fn group1(&self) -> Simd32x4 {
        unsafe { self.groups.g1 }
    }
    #[inline(always)]
    pub fn group1_mut(&mut self) -> &mut Simd32x4 {
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
    pub fn group5(&self) -> Simd32x3 {
        unsafe { self.groups.g5 }
    }
    #[inline(always)]
    pub fn group5_mut(&mut self) -> &mut Simd32x3 {
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
    pub fn group7(&self) -> Simd32x4 {
        unsafe { self.groups.g7 }
    }
    #[inline(always)]
    pub fn group7_mut(&mut self) -> &mut Simd32x4 {
        unsafe { &mut self.groups.g7 }
    }
}

const MULTIVECTOR_INDEX_REMAP: [usize; 27] = [0, 1, 2, 4, 5, 6, 7, 8, 9, 10, 12, 13, 14, 16, 17, 18, 20, 21, 22, 24, 25, 26, 27, 28, 29, 30, 31];

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

impl std::convert::From<MultiVector> for [f32; 27] {
    fn from(vector: MultiVector) -> Self {
        unsafe { [vector.elements[0], vector.elements[1], vector.elements[2], vector.elements[4], vector.elements[5], vector.elements[6], vector.elements[7], vector.elements[8], vector.elements[9], vector.elements[10], vector.elements[12], vector.elements[13], vector.elements[14], vector.elements[16], vector.elements[17], vector.elements[18], vector.elements[20], vector.elements[21], vector.elements[22], vector.elements[24], vector.elements[25], vector.elements[26], vector.elements[27], vector.elements[28], vector.elements[29], vector.elements[30], vector.elements[31]] }
    }
}

impl std::convert::From<[f32; 27]> for MultiVector {
    fn from(array: [f32; 27]) -> Self {
        Self { elements: [array[0], array[1], array[2], 0.0, array[3], array[4], array[5], array[6], array[7], array[8], array[9], 0.0, array[10], array[11], array[12], 0.0, array[13], array[14], array[15], 0.0, array[16], array[17], array[18], 0.0, array[19], array[20], array[21], array[22], array[23], array[24], array[25], array[26]] }
    }
}

impl std::fmt::Debug for MultiVector {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .debug_struct("MultiVector")
            .field("1", &self[0])
            .field("e0123", &self[1])
            .field("e0123", &self[2])
            .field("e0", &self[3])
            .field("e1", &self[4])
            .field("e2", &self[5])
            .field("e3", &self[6])
            .field("-e03", &self[7])
            .field("-e13", &self[8])
            .field("-e23", &self[9])
            .field("e12", &self[10])
            .field("-e02", &self[11])
            .field("e01", &self[12])
            .field("-e03", &self[13])
            .field("-e13", &self[14])
            .field("-e23", &self[15])
            .field("e12", &self[16])
            .field("-e02", &self[17])
            .field("e01", &self[18])
            .field("e123", &self[19])
            .field("-e023", &self[20])
            .field("e013", &self[21])
            .field("-e012", &self[22])
            .field("e123", &self[23])
            .field("-e023", &self[24])
            .field("e013", &self[25])
            .field("-e012", &self[26])
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

impl Grade for Scalar {
    type Output = isize;

    fn grade(self) -> isize {
        0
    }
}

impl AntiGrade for Scalar {
    type Output = isize;

    fn anti_grade(self) -> isize {
        4
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

impl Dual for Scalar {
    type Output = AntiScalar;

    fn dual(self) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0() } }
    }
}

impl AntiReversal for Scalar {
    type Output = Scalar;

    fn anti_reversal(self) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0() } }
    }
}

impl RightComplement for Scalar {
    type Output = AntiScalar;

    fn right_complement(self) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0() } }
    }
}

impl LeftComplement for Scalar {
    type Output = AntiScalar;

    fn left_complement(self) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0() } }
    }
}

impl DoubleComplement for Scalar {
    type Output = Scalar;

    fn double_complement(self) -> Scalar {
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

impl Add<AntiScalar> for Scalar {
    type Output = HomogeneousMagnitude;

    fn add(self, other: AntiScalar) -> HomogeneousMagnitude {
        HomogeneousMagnitude { groups: HomogeneousMagnitudeGroups { g0: Simd32x2::from(self.group0()) * Simd32x2::from([1.0, 0.0]) + Simd32x2::from(other.group0()) * Simd32x2::from([0.0, 1.0]) } }
    }
}

impl Sub<AntiScalar> for Scalar {
    type Output = HomogeneousMagnitude;

    fn sub(self, other: AntiScalar) -> HomogeneousMagnitude {
        HomogeneousMagnitude { groups: HomogeneousMagnitudeGroups { g0: Simd32x2::from(self.group0()) * Simd32x2::from([1.0, 0.0]) - Simd32x2::from(other.group0()) * Simd32x2::from([0.0, 1.0]) } }
    }
}

impl GeometricProduct<AntiScalar> for Scalar {
    type Output = AntiScalar;

    fn geometric_product(self, other: AntiScalar) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0() * other.group0() } }
    }
}

impl RegressiveProduct<AntiScalar> for Scalar {
    type Output = Scalar;

    fn regressive_product(self, other: AntiScalar) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0() * other.group0() } }
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

impl GeometricAntiProduct<AntiScalar> for Scalar {
    type Output = Scalar;

    fn geometric_anti_product(self, other: AntiScalar) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0() * other.group0() } }
    }
}

impl InnerAntiProduct<AntiScalar> for Scalar {
    type Output = Scalar;

    fn inner_anti_product(self, other: AntiScalar) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0() * other.group0() } }
    }
}

impl LeftContraction<AntiScalar> for Scalar {
    type Output = AntiScalar;

    fn left_contraction(self, other: AntiScalar) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0() * other.group0() } }
    }
}

impl RightAntiContraction<AntiScalar> for Scalar {
    type Output = Scalar;

    fn right_anti_contraction(self, other: AntiScalar) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0() * other.group0() } }
    }
}

impl Add<HomogeneousMagnitude> for Scalar {
    type Output = HomogeneousMagnitude;

    fn add(self, other: HomogeneousMagnitude) -> HomogeneousMagnitude {
        HomogeneousMagnitude { groups: HomogeneousMagnitudeGroups { g0: Simd32x2::from(self.group0()) * Simd32x2::from([1.0, 0.0]) + other.group0() } }
    }
}

impl Sub<HomogeneousMagnitude> for Scalar {
    type Output = HomogeneousMagnitude;

    fn sub(self, other: HomogeneousMagnitude) -> HomogeneousMagnitude {
        HomogeneousMagnitude { groups: HomogeneousMagnitudeGroups { g0: Simd32x2::from(self.group0()) * Simd32x2::from([1.0, 0.0]) - other.group0() } }
    }
}

impl GeometricProduct<HomogeneousMagnitude> for Scalar {
    type Output = HomogeneousMagnitude;

    fn geometric_product(self, other: HomogeneousMagnitude) -> HomogeneousMagnitude {
        HomogeneousMagnitude { groups: HomogeneousMagnitudeGroups { g0: Simd32x2::from(self.group0()) * other.group0() } }
    }
}

impl RegressiveProduct<HomogeneousMagnitude> for Scalar {
    type Output = Scalar;

    fn regressive_product(self, other: HomogeneousMagnitude) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0() * other.group0()[1] } }
    }
}

impl OuterProduct<HomogeneousMagnitude> for Scalar {
    type Output = HomogeneousMagnitude;

    fn outer_product(self, other: HomogeneousMagnitude) -> HomogeneousMagnitude {
        HomogeneousMagnitude { groups: HomogeneousMagnitudeGroups { g0: Simd32x2::from(self.group0()) * other.group0() } }
    }
}

impl InnerProduct<HomogeneousMagnitude> for Scalar {
    type Output = HomogeneousMagnitude;

    fn inner_product(self, other: HomogeneousMagnitude) -> HomogeneousMagnitude {
        HomogeneousMagnitude { groups: HomogeneousMagnitudeGroups { g0: Simd32x2::from(self.group0()) * other.group0() } }
    }
}

impl GeometricAntiProduct<HomogeneousMagnitude> for Scalar {
    type Output = Scalar;

    fn geometric_anti_product(self, other: HomogeneousMagnitude) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0() * other.group0()[1] } }
    }
}

impl InnerAntiProduct<HomogeneousMagnitude> for Scalar {
    type Output = Scalar;

    fn inner_anti_product(self, other: HomogeneousMagnitude) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0() * other.group0()[1] } }
    }
}

impl LeftContraction<HomogeneousMagnitude> for Scalar {
    type Output = HomogeneousMagnitude;

    fn left_contraction(self, other: HomogeneousMagnitude) -> HomogeneousMagnitude {
        HomogeneousMagnitude { groups: HomogeneousMagnitudeGroups { g0: Simd32x2::from(self.group0()) * other.group0() } }
    }
}

impl RightContraction<HomogeneousMagnitude> for Scalar {
    type Output = Scalar;

    fn right_contraction(self, other: HomogeneousMagnitude) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0() * other.group0()[0] } }
    }
}

impl RightAntiContraction<HomogeneousMagnitude> for Scalar {
    type Output = Scalar;

    fn right_anti_contraction(self, other: HomogeneousMagnitude) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0() * other.group0()[1] } }
    }
}

impl ScalarProduct<HomogeneousMagnitude> for Scalar {
    type Output = Scalar;

    fn scalar_product(self, other: HomogeneousMagnitude) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0() * other.group0()[0] } }
    }
}

impl GeometricProduct<Point> for Scalar {
    type Output = Point;

    fn geometric_product(self, other: Point) -> Point {
        Point { groups: PointGroups { g0: Simd32x4::from(self.group0()) * other.group0() } }
    }
}

impl OuterProduct<Point> for Scalar {
    type Output = Point;

    fn outer_product(self, other: Point) -> Point {
        Point { groups: PointGroups { g0: Simd32x4::from(self.group0()) * other.group0() } }
    }
}

impl InnerProduct<Point> for Scalar {
    type Output = Point;

    fn inner_product(self, other: Point) -> Point {
        Point { groups: PointGroups { g0: Simd32x4::from(self.group0()) * other.group0() } }
    }
}

impl LeftContraction<Point> for Scalar {
    type Output = Point;

    fn left_contraction(self, other: Point) -> Point {
        Point { groups: PointGroups { g0: Simd32x4::from(self.group0()) * other.group0() } }
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

impl GeometricProduct<Motor> for Scalar {
    type Output = Motor;

    fn geometric_product(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()) * other.group0(), g1: Simd32x3::from(self.group0()) * other.group1() } }
    }
}

impl RegressiveProduct<Motor> for Scalar {
    type Output = Scalar;

    fn regressive_product(self, other: Motor) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0() * other.group0()[3] } }
    }
}

impl OuterProduct<Motor> for Scalar {
    type Output = Motor;

    fn outer_product(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()) * other.group0(), g1: Simd32x3::from(self.group0()) * other.group1() } }
    }
}

impl InnerProduct<Motor> for Scalar {
    type Output = Motor;

    fn inner_product(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()) * other.group0(), g1: Simd32x3::from(self.group0()) * other.group1() } }
    }
}

impl LeftContraction<Motor> for Scalar {
    type Output = Motor;

    fn left_contraction(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()) * other.group0(), g1: Simd32x3::from(self.group0()) * other.group1() } }
    }
}

impl GeometricProduct<Rotor> for Scalar {
    type Output = Rotor;

    fn geometric_product(self, other: Rotor) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group0()) * other.group0() } }
    }
}

impl RegressiveProduct<Rotor> for Scalar {
    type Output = Scalar;

    fn regressive_product(self, other: Rotor) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0() * other.group0()[3] } }
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

impl RegressiveProduct<Translator> for Scalar {
    type Output = Scalar;

    fn regressive_product(self, other: Translator) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0() * other.group0()[3] } }
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

impl GeometricAntiProduct<Translator> for Scalar {
    type Output = Scalar;

    fn geometric_anti_product(self, other: Translator) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0() * other.group0()[3] } }
    }
}

impl InnerAntiProduct<Translator> for Scalar {
    type Output = Scalar;

    fn inner_anti_product(self, other: Translator) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0() * other.group0()[3] } }
    }
}

impl LeftContraction<Translator> for Scalar {
    type Output = Translator;

    fn left_contraction(self, other: Translator) -> Translator {
        Translator { groups: TranslatorGroups { g0: Simd32x4::from(self.group0()) * other.group0() } }
    }
}

impl RightAntiContraction<Translator> for Scalar {
    type Output = Scalar;

    fn right_anti_contraction(self, other: Translator) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0() * other.group0()[3] } }
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

impl RegressiveProduct<MultiVector> for Scalar {
    type Output = Scalar;

    fn regressive_product(self, other: MultiVector) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0() * other.group0()[1] } }
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

impl Grade for AntiScalar {
    type Output = isize;

    fn grade(self) -> isize {
        4
    }
}

impl AntiGrade for AntiScalar {
    type Output = isize;

    fn anti_grade(self) -> isize {
        0
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
        AntiScalar { groups: AntiScalarGroups { g0: self.group0() } }
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
        AntiScalar { groups: AntiScalarGroups { g0: self.group0() } }
    }
}

impl Dual for AntiScalar {
    type Output = Scalar;

    fn dual(self) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0() } }
    }
}

impl AntiReversal for AntiScalar {
    type Output = AntiScalar;

    fn anti_reversal(self) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0() } }
    }
}

impl RightComplement for AntiScalar {
    type Output = Scalar;

    fn right_complement(self) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0() } }
    }
}

impl LeftComplement for AntiScalar {
    type Output = Scalar;

    fn left_complement(self) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0() } }
    }
}

impl DoubleComplement for AntiScalar {
    type Output = AntiScalar;

    fn double_complement(self) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0() } }
    }
}

impl Add<Scalar> for AntiScalar {
    type Output = HomogeneousMagnitude;

    fn add(self, other: Scalar) -> HomogeneousMagnitude {
        HomogeneousMagnitude { groups: HomogeneousMagnitudeGroups { g0: Simd32x2::from(self.group0()) * Simd32x2::from([0.0, 1.0]) + Simd32x2::from(other.group0()) * Simd32x2::from([1.0, 0.0]) } }
    }
}

impl Sub<Scalar> for AntiScalar {
    type Output = HomogeneousMagnitude;

    fn sub(self, other: Scalar) -> HomogeneousMagnitude {
        HomogeneousMagnitude { groups: HomogeneousMagnitudeGroups { g0: Simd32x2::from(self.group0()) * Simd32x2::from([0.0, 1.0]) - Simd32x2::from(other.group0()) * Simd32x2::from([1.0, 0.0]) } }
    }
}

impl GeometricProduct<Scalar> for AntiScalar {
    type Output = AntiScalar;

    fn geometric_product(self, other: Scalar) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0() * other.group0() } }
    }
}

impl RegressiveProduct<Scalar> for AntiScalar {
    type Output = Scalar;

    fn regressive_product(self, other: Scalar) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0() * other.group0() } }
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

impl GeometricAntiProduct<Scalar> for AntiScalar {
    type Output = Scalar;

    fn geometric_anti_product(self, other: Scalar) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0() * other.group0() } }
    }
}

impl InnerAntiProduct<Scalar> for AntiScalar {
    type Output = Scalar;

    fn inner_anti_product(self, other: Scalar) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0() * other.group0() } }
    }
}

impl RightContraction<Scalar> for AntiScalar {
    type Output = AntiScalar;

    fn right_contraction(self, other: Scalar) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0() * other.group0() } }
    }
}

impl LeftAntiContraction<Scalar> for AntiScalar {
    type Output = Scalar;

    fn left_anti_contraction(self, other: Scalar) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0() * other.group0() } }
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

impl RegressiveProduct<AntiScalar> for AntiScalar {
    type Output = AntiScalar;

    fn regressive_product(self, other: AntiScalar) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0() * other.group0() } }
    }
}

impl GeometricAntiProduct<AntiScalar> for AntiScalar {
    type Output = AntiScalar;

    fn geometric_anti_product(self, other: AntiScalar) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0() * other.group0() } }
    }
}

impl InnerAntiProduct<AntiScalar> for AntiScalar {
    type Output = AntiScalar;

    fn inner_anti_product(self, other: AntiScalar) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0() * other.group0() } }
    }
}

impl LeftAntiContraction<AntiScalar> for AntiScalar {
    type Output = AntiScalar;

    fn left_anti_contraction(self, other: AntiScalar) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0() * other.group0() } }
    }
}

impl RightAntiContraction<AntiScalar> for AntiScalar {
    type Output = AntiScalar;

    fn right_anti_contraction(self, other: AntiScalar) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0() * other.group0() } }
    }
}

impl AntiScalarProduct<AntiScalar> for AntiScalar {
    type Output = AntiScalar;

    fn anti_scalar_product(self, other: AntiScalar) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0() * other.group0() } }
    }
}

impl Add<HomogeneousMagnitude> for AntiScalar {
    type Output = HomogeneousMagnitude;

    fn add(self, other: HomogeneousMagnitude) -> HomogeneousMagnitude {
        HomogeneousMagnitude { groups: HomogeneousMagnitudeGroups { g0: Simd32x2::from(self.group0()) * Simd32x2::from([0.0, 1.0]) + other.group0() } }
    }
}

impl Sub<HomogeneousMagnitude> for AntiScalar {
    type Output = HomogeneousMagnitude;

    fn sub(self, other: HomogeneousMagnitude) -> HomogeneousMagnitude {
        HomogeneousMagnitude { groups: HomogeneousMagnitudeGroups { g0: Simd32x2::from(self.group0()) * Simd32x2::from([0.0, 1.0]) - other.group0() } }
    }
}

impl GeometricProduct<HomogeneousMagnitude> for AntiScalar {
    type Output = AntiScalar;

    fn geometric_product(self, other: HomogeneousMagnitude) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0() * other.group0()[0] } }
    }
}

impl RegressiveProduct<HomogeneousMagnitude> for AntiScalar {
    type Output = HomogeneousMagnitude;

    fn regressive_product(self, other: HomogeneousMagnitude) -> HomogeneousMagnitude {
        HomogeneousMagnitude { groups: HomogeneousMagnitudeGroups { g0: Simd32x2::from(self.group0()) * other.group0() } }
    }
}

impl OuterProduct<HomogeneousMagnitude> for AntiScalar {
    type Output = AntiScalar;

    fn outer_product(self, other: HomogeneousMagnitude) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0() * other.group0()[0] } }
    }
}

impl InnerProduct<HomogeneousMagnitude> for AntiScalar {
    type Output = AntiScalar;

    fn inner_product(self, other: HomogeneousMagnitude) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0() * other.group0()[0] } }
    }
}

impl GeometricAntiProduct<HomogeneousMagnitude> for AntiScalar {
    type Output = HomogeneousMagnitude;

    fn geometric_anti_product(self, other: HomogeneousMagnitude) -> HomogeneousMagnitude {
        HomogeneousMagnitude { groups: HomogeneousMagnitudeGroups { g0: Simd32x2::from(self.group0()) * other.group0() } }
    }
}

impl InnerAntiProduct<HomogeneousMagnitude> for AntiScalar {
    type Output = HomogeneousMagnitude;

    fn inner_anti_product(self, other: HomogeneousMagnitude) -> HomogeneousMagnitude {
        HomogeneousMagnitude { groups: HomogeneousMagnitudeGroups { g0: Simd32x2::from(self.group0()) * other.group0() } }
    }
}

impl RightContraction<HomogeneousMagnitude> for AntiScalar {
    type Output = AntiScalar;

    fn right_contraction(self, other: HomogeneousMagnitude) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0() * other.group0()[0] } }
    }
}

impl LeftAntiContraction<HomogeneousMagnitude> for AntiScalar {
    type Output = HomogeneousMagnitude;

    fn left_anti_contraction(self, other: HomogeneousMagnitude) -> HomogeneousMagnitude {
        HomogeneousMagnitude { groups: HomogeneousMagnitudeGroups { g0: Simd32x2::from(self.group0()) * other.group0() } }
    }
}

impl RightAntiContraction<HomogeneousMagnitude> for AntiScalar {
    type Output = AntiScalar;

    fn right_anti_contraction(self, other: HomogeneousMagnitude) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0() * other.group0()[1] } }
    }
}

impl AntiScalarProduct<HomogeneousMagnitude> for AntiScalar {
    type Output = AntiScalar;

    fn anti_scalar_product(self, other: HomogeneousMagnitude) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0() * other.group0()[1] } }
    }
}

impl RegressiveProduct<Point> for AntiScalar {
    type Output = Point;

    fn regressive_product(self, other: Point) -> Point {
        Point { groups: PointGroups { g0: Simd32x4::from(self.group0()) * other.group0() } }
    }
}

impl GeometricAntiProduct<Point> for AntiScalar {
    type Output = Point;

    fn geometric_anti_product(self, other: Point) -> Point {
        Point { groups: PointGroups { g0: Simd32x4::from(self.group0()) * other.group0() } }
    }
}

impl InnerAntiProduct<Point> for AntiScalar {
    type Output = Point;

    fn inner_anti_product(self, other: Point) -> Point {
        Point { groups: PointGroups { g0: Simd32x4::from(self.group0()) * other.group0() } }
    }
}

impl LeftAntiContraction<Point> for AntiScalar {
    type Output = Point;

    fn left_anti_contraction(self, other: Point) -> Point {
        Point { groups: PointGroups { g0: Simd32x4::from(self.group0()) * other.group0() } }
    }
}

impl Add<Line> for AntiScalar {
    type Output = Motor;

    fn add(self, other: Line) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group0()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]), g1: other.group1() } }
    }
}

impl Sub<Line> for AntiScalar {
    type Output = Motor;

    fn sub(self, other: Line) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) - Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group0()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]), g1: Simd32x3::from(0.0) - other.group1() } }
    }
}

impl RegressiveProduct<Line> for AntiScalar {
    type Output = Line;

    fn regressive_product(self, other: Line) -> Line {
        Line { groups: LineGroups { g0: Simd32x3::from(self.group0()) * other.group0(), g1: Simd32x3::from(self.group0()) * other.group1() } }
    }
}

impl GeometricAntiProduct<Line> for AntiScalar {
    type Output = Line;

    fn geometric_anti_product(self, other: Line) -> Line {
        Line { groups: LineGroups { g0: Simd32x3::from(self.group0()) * other.group0(), g1: Simd32x3::from(self.group0()) * other.group1() } }
    }
}

impl InnerAntiProduct<Line> for AntiScalar {
    type Output = Line;

    fn inner_anti_product(self, other: Line) -> Line {
        Line { groups: LineGroups { g0: Simd32x3::from(self.group0()) * other.group0(), g1: Simd32x3::from(self.group0()) * other.group1() } }
    }
}

impl LeftAntiContraction<Line> for AntiScalar {
    type Output = Line;

    fn left_anti_contraction(self, other: Line) -> Line {
        Line { groups: LineGroups { g0: Simd32x3::from(self.group0()) * other.group0(), g1: Simd32x3::from(self.group0()) * other.group1() } }
    }
}

impl RegressiveProduct<Plane> for AntiScalar {
    type Output = Plane;

    fn regressive_product(self, other: Plane) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()) * other.group0() } }
    }
}

impl GeometricAntiProduct<Plane> for AntiScalar {
    type Output = Plane;

    fn geometric_anti_product(self, other: Plane) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()) * other.group0() } }
    }
}

impl InnerAntiProduct<Plane> for AntiScalar {
    type Output = Plane;

    fn inner_anti_product(self, other: Plane) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()) * other.group0() } }
    }
}

impl LeftAntiContraction<Plane> for AntiScalar {
    type Output = Plane;

    fn left_anti_contraction(self, other: Plane) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()) * other.group0() } }
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
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) - other.group0(), g1: Simd32x3::from(0.0) - other.group1() } }
    }
}

impl RegressiveProduct<Motor> for AntiScalar {
    type Output = Motor;

    fn regressive_product(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()) * other.group0(), g1: Simd32x3::from(self.group0()) * other.group1() } }
    }
}

impl GeometricAntiProduct<Motor> for AntiScalar {
    type Output = Motor;

    fn geometric_anti_product(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()) * other.group0(), g1: Simd32x3::from(self.group0()) * other.group1() } }
    }
}

impl InnerAntiProduct<Motor> for AntiScalar {
    type Output = Motor;

    fn inner_anti_product(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()) * other.group0(), g1: Simd32x3::from(self.group0()) * other.group1() } }
    }
}

impl LeftAntiContraction<Motor> for AntiScalar {
    type Output = Motor;

    fn left_anti_contraction(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()) * other.group0(), g1: Simd32x3::from(self.group0()) * other.group1() } }
    }
}

impl RightAntiContraction<Motor> for AntiScalar {
    type Output = AntiScalar;

    fn right_anti_contraction(self, other: Motor) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0() * other.group0()[3] } }
    }
}

impl AntiScalarProduct<Motor> for AntiScalar {
    type Output = AntiScalar;

    fn anti_scalar_product(self, other: Motor) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0() * other.group0()[3] } }
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

impl RegressiveProduct<Rotor> for AntiScalar {
    type Output = Rotor;

    fn regressive_product(self, other: Rotor) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group0()) * other.group0() } }
    }
}

impl GeometricAntiProduct<Rotor> for AntiScalar {
    type Output = Rotor;

    fn geometric_anti_product(self, other: Rotor) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group0()) * other.group0() } }
    }
}

impl InnerAntiProduct<Rotor> for AntiScalar {
    type Output = Rotor;

    fn inner_anti_product(self, other: Rotor) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group0()) * other.group0() } }
    }
}

impl LeftAntiContraction<Rotor> for AntiScalar {
    type Output = Rotor;

    fn left_anti_contraction(self, other: Rotor) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group0()) * other.group0() } }
    }
}

impl RightAntiContraction<Rotor> for AntiScalar {
    type Output = AntiScalar;

    fn right_anti_contraction(self, other: Rotor) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0() * other.group0()[3] } }
    }
}

impl AntiScalarProduct<Rotor> for AntiScalar {
    type Output = AntiScalar;

    fn anti_scalar_product(self, other: Rotor) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0() * other.group0()[3] } }
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

impl RegressiveProduct<Translator> for AntiScalar {
    type Output = Translator;

    fn regressive_product(self, other: Translator) -> Translator {
        Translator { groups: TranslatorGroups { g0: Simd32x4::from(self.group0()) * other.group0() } }
    }
}

impl GeometricAntiProduct<Translator> for AntiScalar {
    type Output = Translator;

    fn geometric_anti_product(self, other: Translator) -> Translator {
        Translator { groups: TranslatorGroups { g0: Simd32x4::from(self.group0()) * other.group0() } }
    }
}

impl InnerAntiProduct<Translator> for AntiScalar {
    type Output = Translator;

    fn inner_anti_product(self, other: Translator) -> Translator {
        Translator { groups: TranslatorGroups { g0: Simd32x4::from(self.group0()) * other.group0() } }
    }
}

impl LeftAntiContraction<Translator> for AntiScalar {
    type Output = Translator;

    fn left_anti_contraction(self, other: Translator) -> Translator {
        Translator { groups: TranslatorGroups { g0: Simd32x4::from(self.group0()) * other.group0() } }
    }
}

impl RightAntiContraction<Translator> for AntiScalar {
    type Output = AntiScalar;

    fn right_anti_contraction(self, other: Translator) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0() * other.group0()[3] } }
    }
}

impl AntiScalarProduct<Translator> for AntiScalar {
    type Output = AntiScalar;

    fn anti_scalar_product(self, other: Translator) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0() * other.group0()[3] } }
    }
}

impl RegressiveProduct<Flector> for AntiScalar {
    type Output = Flector;

    fn regressive_product(self, other: Flector) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()) * other.group0(), g1: Simd32x4::from(self.group0()) * other.group1() } }
    }
}

impl GeometricAntiProduct<Flector> for AntiScalar {
    type Output = Flector;

    fn geometric_anti_product(self, other: Flector) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()) * other.group0(), g1: Simd32x4::from(self.group0()) * other.group1() } }
    }
}

impl InnerAntiProduct<Flector> for AntiScalar {
    type Output = Flector;

    fn inner_anti_product(self, other: Flector) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()) * other.group0(), g1: Simd32x4::from(self.group0()) * other.group1() } }
    }
}

impl LeftAntiContraction<Flector> for AntiScalar {
    type Output = Flector;

    fn left_anti_contraction(self, other: Flector) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()) * other.group0(), g1: Simd32x4::from(self.group0()) * other.group1() } }
    }
}

impl OuterProduct<MultiVector> for AntiScalar {
    type Output = AntiScalar;

    fn outer_product(self, other: MultiVector) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0() * other.group0()[0] } }
    }
}

impl RightAntiContraction<MultiVector> for AntiScalar {
    type Output = AntiScalar;

    fn right_anti_contraction(self, other: MultiVector) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0() * other.group0()[1] } }
    }
}

impl AntiScalarProduct<MultiVector> for AntiScalar {
    type Output = AntiScalar;

    fn anti_scalar_product(self, other: MultiVector) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0() * other.group0()[1] } }
    }
}

impl Scale for AntiScalar {
    type Output = AntiScalar;

    fn scale(self, other: f32) -> AntiScalar {
        self.geometric_product(Scalar { groups: ScalarGroups { g0: other } })
    }
}

impl Zero for HomogeneousMagnitude {
    fn zero() -> Self {
        HomogeneousMagnitude { groups: HomogeneousMagnitudeGroups { g0: Simd32x2::from(0.0) } }
    }
}

impl One for HomogeneousMagnitude {
    fn one() -> Self {
        HomogeneousMagnitude { groups: HomogeneousMagnitudeGroups { g0: Simd32x2::from([1.0, 0.0]) } }
    }
}

impl Neg for HomogeneousMagnitude {
    type Output = HomogeneousMagnitude;

    fn neg(self) -> HomogeneousMagnitude {
        HomogeneousMagnitude { groups: HomogeneousMagnitudeGroups { g0: self.group0() * Simd32x2::from(-1.0) } }
    }
}

impl Automorphism for HomogeneousMagnitude {
    type Output = HomogeneousMagnitude;

    fn automorphism(self) -> HomogeneousMagnitude {
        HomogeneousMagnitude { groups: HomogeneousMagnitudeGroups { g0: self.group0() } }
    }
}

impl Reversal for HomogeneousMagnitude {
    type Output = HomogeneousMagnitude;

    fn reversal(self) -> HomogeneousMagnitude {
        HomogeneousMagnitude { groups: HomogeneousMagnitudeGroups { g0: self.group0() } }
    }
}

impl Conjugation for HomogeneousMagnitude {
    type Output = HomogeneousMagnitude;

    fn conjugation(self) -> HomogeneousMagnitude {
        HomogeneousMagnitude { groups: HomogeneousMagnitudeGroups { g0: self.group0() } }
    }
}

impl Dual for HomogeneousMagnitude {
    type Output = HomogeneousMagnitude;

    fn dual(self) -> HomogeneousMagnitude {
        HomogeneousMagnitude { groups: HomogeneousMagnitudeGroups { g0: swizzle!(self.group0(), 1, 0) } }
    }
}

impl AntiReversal for HomogeneousMagnitude {
    type Output = HomogeneousMagnitude;

    fn anti_reversal(self) -> HomogeneousMagnitude {
        HomogeneousMagnitude { groups: HomogeneousMagnitudeGroups { g0: self.group0() } }
    }
}

impl RightComplement for HomogeneousMagnitude {
    type Output = HomogeneousMagnitude;

    fn right_complement(self) -> HomogeneousMagnitude {
        HomogeneousMagnitude { groups: HomogeneousMagnitudeGroups { g0: swizzle!(self.group0(), 1, 0) } }
    }
}

impl LeftComplement for HomogeneousMagnitude {
    type Output = HomogeneousMagnitude;

    fn left_complement(self) -> HomogeneousMagnitude {
        HomogeneousMagnitude { groups: HomogeneousMagnitudeGroups { g0: swizzle!(self.group0(), 1, 0) } }
    }
}

impl DoubleComplement for HomogeneousMagnitude {
    type Output = HomogeneousMagnitude;

    fn double_complement(self) -> HomogeneousMagnitude {
        HomogeneousMagnitude { groups: HomogeneousMagnitudeGroups { g0: self.group0() } }
    }
}

impl Into<Scalar> for HomogeneousMagnitude {
    fn into(self) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[0] } }
    }
}

impl Add<Scalar> for HomogeneousMagnitude {
    type Output = HomogeneousMagnitude;

    fn add(self, other: Scalar) -> HomogeneousMagnitude {
        HomogeneousMagnitude { groups: HomogeneousMagnitudeGroups { g0: self.group0() + Simd32x2::from(other.group0()) * Simd32x2::from([1.0, 0.0]) } }
    }
}

impl AddAssign<Scalar> for HomogeneousMagnitude {
    fn add_assign(&mut self, other: Scalar) {
        *self = (*self).add(other);
    }
}

impl Sub<Scalar> for HomogeneousMagnitude {
    type Output = HomogeneousMagnitude;

    fn sub(self, other: Scalar) -> HomogeneousMagnitude {
        HomogeneousMagnitude { groups: HomogeneousMagnitudeGroups { g0: self.group0() - Simd32x2::from(other.group0()) * Simd32x2::from([1.0, 0.0]) } }
    }
}

impl SubAssign<Scalar> for HomogeneousMagnitude {
    fn sub_assign(&mut self, other: Scalar) {
        *self = (*self).sub(other);
    }
}

impl GeometricProduct<Scalar> for HomogeneousMagnitude {
    type Output = HomogeneousMagnitude;

    fn geometric_product(self, other: Scalar) -> HomogeneousMagnitude {
        HomogeneousMagnitude { groups: HomogeneousMagnitudeGroups { g0: self.group0() * Simd32x2::from(other.group0()) } }
    }
}

impl RegressiveProduct<Scalar> for HomogeneousMagnitude {
    type Output = Scalar;

    fn regressive_product(self, other: Scalar) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[1] * other.group0() } }
    }
}

impl OuterProduct<Scalar> for HomogeneousMagnitude {
    type Output = HomogeneousMagnitude;

    fn outer_product(self, other: Scalar) -> HomogeneousMagnitude {
        HomogeneousMagnitude { groups: HomogeneousMagnitudeGroups { g0: self.group0() * Simd32x2::from(other.group0()) } }
    }
}

impl InnerProduct<Scalar> for HomogeneousMagnitude {
    type Output = HomogeneousMagnitude;

    fn inner_product(self, other: Scalar) -> HomogeneousMagnitude {
        HomogeneousMagnitude { groups: HomogeneousMagnitudeGroups { g0: self.group0() * Simd32x2::from(other.group0()) } }
    }
}

impl GeometricAntiProduct<Scalar> for HomogeneousMagnitude {
    type Output = Scalar;

    fn geometric_anti_product(self, other: Scalar) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[1] * other.group0() } }
    }
}

impl InnerAntiProduct<Scalar> for HomogeneousMagnitude {
    type Output = Scalar;

    fn inner_anti_product(self, other: Scalar) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[1] * other.group0() } }
    }
}

impl LeftContraction<Scalar> for HomogeneousMagnitude {
    type Output = Scalar;

    fn left_contraction(self, other: Scalar) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[0] * other.group0() } }
    }
}

impl RightContraction<Scalar> for HomogeneousMagnitude {
    type Output = HomogeneousMagnitude;

    fn right_contraction(self, other: Scalar) -> HomogeneousMagnitude {
        HomogeneousMagnitude { groups: HomogeneousMagnitudeGroups { g0: self.group0() * Simd32x2::from(other.group0()) } }
    }
}

impl LeftAntiContraction<Scalar> for HomogeneousMagnitude {
    type Output = Scalar;

    fn left_anti_contraction(self, other: Scalar) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[1] * other.group0() } }
    }
}

impl ScalarProduct<Scalar> for HomogeneousMagnitude {
    type Output = Scalar;

    fn scalar_product(self, other: Scalar) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[0] * other.group0() } }
    }
}

impl Into<AntiScalar> for HomogeneousMagnitude {
    fn into(self) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0()[1] } }
    }
}

impl Add<AntiScalar> for HomogeneousMagnitude {
    type Output = HomogeneousMagnitude;

    fn add(self, other: AntiScalar) -> HomogeneousMagnitude {
        HomogeneousMagnitude { groups: HomogeneousMagnitudeGroups { g0: self.group0() + Simd32x2::from(other.group0()) * Simd32x2::from([0.0, 1.0]) } }
    }
}

impl AddAssign<AntiScalar> for HomogeneousMagnitude {
    fn add_assign(&mut self, other: AntiScalar) {
        *self = (*self).add(other);
    }
}

impl Sub<AntiScalar> for HomogeneousMagnitude {
    type Output = HomogeneousMagnitude;

    fn sub(self, other: AntiScalar) -> HomogeneousMagnitude {
        HomogeneousMagnitude { groups: HomogeneousMagnitudeGroups { g0: self.group0() - Simd32x2::from(other.group0()) * Simd32x2::from([0.0, 1.0]) } }
    }
}

impl SubAssign<AntiScalar> for HomogeneousMagnitude {
    fn sub_assign(&mut self, other: AntiScalar) {
        *self = (*self).sub(other);
    }
}

impl GeometricProduct<AntiScalar> for HomogeneousMagnitude {
    type Output = AntiScalar;

    fn geometric_product(self, other: AntiScalar) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0()[0] * other.group0() } }
    }
}

impl RegressiveProduct<AntiScalar> for HomogeneousMagnitude {
    type Output = HomogeneousMagnitude;

    fn regressive_product(self, other: AntiScalar) -> HomogeneousMagnitude {
        HomogeneousMagnitude { groups: HomogeneousMagnitudeGroups { g0: self.group0() * Simd32x2::from(other.group0()) } }
    }
}

impl OuterProduct<AntiScalar> for HomogeneousMagnitude {
    type Output = AntiScalar;

    fn outer_product(self, other: AntiScalar) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0()[0] * other.group0() } }
    }
}

impl InnerProduct<AntiScalar> for HomogeneousMagnitude {
    type Output = AntiScalar;

    fn inner_product(self, other: AntiScalar) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0()[0] * other.group0() } }
    }
}

impl GeometricAntiProduct<AntiScalar> for HomogeneousMagnitude {
    type Output = HomogeneousMagnitude;

    fn geometric_anti_product(self, other: AntiScalar) -> HomogeneousMagnitude {
        HomogeneousMagnitude { groups: HomogeneousMagnitudeGroups { g0: self.group0() * Simd32x2::from(other.group0()) } }
    }
}

impl InnerAntiProduct<AntiScalar> for HomogeneousMagnitude {
    type Output = HomogeneousMagnitude;

    fn inner_anti_product(self, other: AntiScalar) -> HomogeneousMagnitude {
        HomogeneousMagnitude { groups: HomogeneousMagnitudeGroups { g0: self.group0() * Simd32x2::from(other.group0()) } }
    }
}

impl LeftContraction<AntiScalar> for HomogeneousMagnitude {
    type Output = AntiScalar;

    fn left_contraction(self, other: AntiScalar) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0()[0] * other.group0() } }
    }
}

impl LeftAntiContraction<AntiScalar> for HomogeneousMagnitude {
    type Output = AntiScalar;

    fn left_anti_contraction(self, other: AntiScalar) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0()[1] * other.group0() } }
    }
}

impl RightAntiContraction<AntiScalar> for HomogeneousMagnitude {
    type Output = HomogeneousMagnitude;

    fn right_anti_contraction(self, other: AntiScalar) -> HomogeneousMagnitude {
        HomogeneousMagnitude { groups: HomogeneousMagnitudeGroups { g0: self.group0() * Simd32x2::from(other.group0()) } }
    }
}

impl AntiScalarProduct<AntiScalar> for HomogeneousMagnitude {
    type Output = AntiScalar;

    fn anti_scalar_product(self, other: AntiScalar) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0()[1] * other.group0() } }
    }
}

impl Add<HomogeneousMagnitude> for HomogeneousMagnitude {
    type Output = HomogeneousMagnitude;

    fn add(self, other: HomogeneousMagnitude) -> HomogeneousMagnitude {
        HomogeneousMagnitude { groups: HomogeneousMagnitudeGroups { g0: self.group0() + other.group0() } }
    }
}

impl AddAssign<HomogeneousMagnitude> for HomogeneousMagnitude {
    fn add_assign(&mut self, other: HomogeneousMagnitude) {
        *self = (*self).add(other);
    }
}

impl Sub<HomogeneousMagnitude> for HomogeneousMagnitude {
    type Output = HomogeneousMagnitude;

    fn sub(self, other: HomogeneousMagnitude) -> HomogeneousMagnitude {
        HomogeneousMagnitude { groups: HomogeneousMagnitudeGroups { g0: self.group0() - other.group0() } }
    }
}

impl SubAssign<HomogeneousMagnitude> for HomogeneousMagnitude {
    fn sub_assign(&mut self, other: HomogeneousMagnitude) {
        *self = (*self).sub(other);
    }
}

impl Mul<HomogeneousMagnitude> for HomogeneousMagnitude {
    type Output = HomogeneousMagnitude;

    fn mul(self, other: HomogeneousMagnitude) -> HomogeneousMagnitude {
        HomogeneousMagnitude { groups: HomogeneousMagnitudeGroups { g0: self.group0() * other.group0() } }
    }
}

impl MulAssign<HomogeneousMagnitude> for HomogeneousMagnitude {
    fn mul_assign(&mut self, other: HomogeneousMagnitude) {
        *self = (*self).mul(other);
    }
}

impl Div<HomogeneousMagnitude> for HomogeneousMagnitude {
    type Output = HomogeneousMagnitude;

    fn div(self, other: HomogeneousMagnitude) -> HomogeneousMagnitude {
        HomogeneousMagnitude { groups: HomogeneousMagnitudeGroups { g0: Simd32x2::from([self.group0()[0], self.group0()[1]]) * Simd32x2::from([1.0, 1.0]) / Simd32x2::from([other.group0()[0], other.group0()[1]]) * Simd32x2::from([1.0, 1.0]) } }
    }
}

impl DivAssign<HomogeneousMagnitude> for HomogeneousMagnitude {
    fn div_assign(&mut self, other: HomogeneousMagnitude) {
        *self = (*self).div(other);
    }
}

impl GeometricProduct<HomogeneousMagnitude> for HomogeneousMagnitude {
    type Output = HomogeneousMagnitude;

    fn geometric_product(self, other: HomogeneousMagnitude) -> HomogeneousMagnitude {
        HomogeneousMagnitude { groups: HomogeneousMagnitudeGroups { g0: Simd32x2::from(self.group0()[0]) * other.group0() + self.group0() * Simd32x2::from(other.group0()[0]) * Simd32x2::from([0.0, 1.0]) } }
    }
}

impl RegressiveProduct<HomogeneousMagnitude> for HomogeneousMagnitude {
    type Output = HomogeneousMagnitude;

    fn regressive_product(self, other: HomogeneousMagnitude) -> HomogeneousMagnitude {
        HomogeneousMagnitude { groups: HomogeneousMagnitudeGroups { g0: Simd32x2::from(self.group0()[1]) * other.group0() + Simd32x2::from(self.group0()[0]) * swizzle!(other.group0(), 1, 0) * Simd32x2::from([1.0, 0.0]) } }
    }
}

impl OuterProduct<HomogeneousMagnitude> for HomogeneousMagnitude {
    type Output = HomogeneousMagnitude;

    fn outer_product(self, other: HomogeneousMagnitude) -> HomogeneousMagnitude {
        HomogeneousMagnitude { groups: HomogeneousMagnitudeGroups { g0: Simd32x2::from(self.group0()[0]) * other.group0() + self.group0() * Simd32x2::from(other.group0()[0]) * Simd32x2::from([0.0, 1.0]) } }
    }
}

impl InnerProduct<HomogeneousMagnitude> for HomogeneousMagnitude {
    type Output = HomogeneousMagnitude;

    fn inner_product(self, other: HomogeneousMagnitude) -> HomogeneousMagnitude {
        HomogeneousMagnitude { groups: HomogeneousMagnitudeGroups { g0: Simd32x2::from(self.group0()[0]) * other.group0() + self.group0() * Simd32x2::from(other.group0()[0]) * Simd32x2::from([0.0, 1.0]) } }
    }
}

impl GeometricAntiProduct<HomogeneousMagnitude> for HomogeneousMagnitude {
    type Output = HomogeneousMagnitude;

    fn geometric_anti_product(self, other: HomogeneousMagnitude) -> HomogeneousMagnitude {
        HomogeneousMagnitude { groups: HomogeneousMagnitudeGroups { g0: Simd32x2::from(self.group0()[1]) * other.group0() + Simd32x2::from(self.group0()[0]) * swizzle!(other.group0(), 1, 0) * Simd32x2::from([1.0, 0.0]) } }
    }
}

impl InnerAntiProduct<HomogeneousMagnitude> for HomogeneousMagnitude {
    type Output = HomogeneousMagnitude;

    fn inner_anti_product(self, other: HomogeneousMagnitude) -> HomogeneousMagnitude {
        HomogeneousMagnitude { groups: HomogeneousMagnitudeGroups { g0: Simd32x2::from(self.group0()[1]) * other.group0() + Simd32x2::from(self.group0()[0]) * swizzle!(other.group0(), 1, 0) * Simd32x2::from([1.0, 0.0]) } }
    }
}

impl LeftContraction<HomogeneousMagnitude> for HomogeneousMagnitude {
    type Output = HomogeneousMagnitude;

    fn left_contraction(self, other: HomogeneousMagnitude) -> HomogeneousMagnitude {
        HomogeneousMagnitude { groups: HomogeneousMagnitudeGroups { g0: Simd32x2::from(self.group0()[0]) * other.group0() } }
    }
}

impl RightContraction<HomogeneousMagnitude> for HomogeneousMagnitude {
    type Output = HomogeneousMagnitude;

    fn right_contraction(self, other: HomogeneousMagnitude) -> HomogeneousMagnitude {
        HomogeneousMagnitude { groups: HomogeneousMagnitudeGroups { g0: self.group0() * Simd32x2::from(other.group0()[0]) } }
    }
}

impl LeftAntiContraction<HomogeneousMagnitude> for HomogeneousMagnitude {
    type Output = HomogeneousMagnitude;

    fn left_anti_contraction(self, other: HomogeneousMagnitude) -> HomogeneousMagnitude {
        HomogeneousMagnitude { groups: HomogeneousMagnitudeGroups { g0: Simd32x2::from(self.group0()[1]) * other.group0() } }
    }
}

impl RightAntiContraction<HomogeneousMagnitude> for HomogeneousMagnitude {
    type Output = HomogeneousMagnitude;

    fn right_anti_contraction(self, other: HomogeneousMagnitude) -> HomogeneousMagnitude {
        HomogeneousMagnitude { groups: HomogeneousMagnitudeGroups { g0: self.group0() * Simd32x2::from(other.group0()[1]) } }
    }
}

impl ScalarProduct<HomogeneousMagnitude> for HomogeneousMagnitude {
    type Output = Scalar;

    fn scalar_product(self, other: HomogeneousMagnitude) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[0] * other.group0()[0] } }
    }
}

impl AntiScalarProduct<HomogeneousMagnitude> for HomogeneousMagnitude {
    type Output = AntiScalar;

    fn anti_scalar_product(self, other: HomogeneousMagnitude) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0()[1] * other.group0()[1] } }
    }
}

impl RegressiveProduct<Point> for HomogeneousMagnitude {
    type Output = Point;

    fn regressive_product(self, other: Point) -> Point {
        Point { groups: PointGroups { g0: Simd32x4::from(self.group0()[1]) * other.group0() } }
    }
}

impl OuterProduct<Point> for HomogeneousMagnitude {
    type Output = Point;

    fn outer_product(self, other: Point) -> Point {
        Point { groups: PointGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() } }
    }
}

impl LeftContraction<Point> for HomogeneousMagnitude {
    type Output = Point;

    fn left_contraction(self, other: Point) -> Point {
        Point { groups: PointGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() } }
    }
}

impl LeftAntiContraction<Point> for HomogeneousMagnitude {
    type Output = Point;

    fn left_anti_contraction(self, other: Point) -> Point {
        Point { groups: PointGroups { g0: Simd32x4::from(self.group0()[1]) * other.group0() } }
    }
}

impl GeometricProduct<Line> for HomogeneousMagnitude {
    type Output = Line;

    fn geometric_product(self, other: Line) -> Line {
        Line { groups: LineGroups { g0: Simd32x3::from(self.group0()[0]) * other.group0() + Simd32x3::from(self.group0()[1]) * other.group1(), g1: Simd32x3::from(self.group0()[0]) * other.group1() } }
    }
}

impl RegressiveProduct<Line> for HomogeneousMagnitude {
    type Output = Line;

    fn regressive_product(self, other: Line) -> Line {
        Line { groups: LineGroups { g0: Simd32x3::from(self.group0()[1]) * other.group0(), g1: Simd32x3::from(self.group0()[1]) * other.group1() } }
    }
}

impl OuterProduct<Line> for HomogeneousMagnitude {
    type Output = Line;

    fn outer_product(self, other: Line) -> Line {
        Line { groups: LineGroups { g0: Simd32x3::from(self.group0()[0]) * other.group0(), g1: Simd32x3::from(self.group0()[0]) * other.group1() } }
    }
}

impl InnerProduct<Line> for HomogeneousMagnitude {
    type Output = Line;

    fn inner_product(self, other: Line) -> Line {
        Line { groups: LineGroups { g0: Simd32x3::from(self.group0()[0]) * other.group0() + Simd32x3::from(self.group0()[1]) * other.group1(), g1: Simd32x3::from(self.group0()[0]) * other.group1() } }
    }
}

impl GeometricAntiProduct<Line> for HomogeneousMagnitude {
    type Output = Line;

    fn geometric_anti_product(self, other: Line) -> Line {
        Line { groups: LineGroups { g0: Simd32x3::from(self.group0()[1]) * other.group0(), g1: Simd32x3::from(self.group0()[0]) * other.group0() + Simd32x3::from(self.group0()[1]) * other.group1() } }
    }
}

impl InnerAntiProduct<Line> for HomogeneousMagnitude {
    type Output = Line;

    fn inner_anti_product(self, other: Line) -> Line {
        Line { groups: LineGroups { g0: Simd32x3::from(self.group0()[1]) * other.group0(), g1: Simd32x3::from(self.group0()[0]) * other.group0() + Simd32x3::from(self.group0()[1]) * other.group1() } }
    }
}

impl LeftContraction<Line> for HomogeneousMagnitude {
    type Output = Line;

    fn left_contraction(self, other: Line) -> Line {
        Line { groups: LineGroups { g0: Simd32x3::from(self.group0()[0]) * other.group0(), g1: Simd32x3::from(self.group0()[0]) * other.group1() } }
    }
}

impl LeftAntiContraction<Line> for HomogeneousMagnitude {
    type Output = Line;

    fn left_anti_contraction(self, other: Line) -> Line {
        Line { groups: LineGroups { g0: Simd32x3::from(self.group0()[1]) * other.group0(), g1: Simd32x3::from(self.group0()[1]) * other.group1() } }
    }
}

impl RegressiveProduct<Plane> for HomogeneousMagnitude {
    type Output = Plane;

    fn regressive_product(self, other: Plane) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()[1]) * other.group0() } }
    }
}

impl OuterProduct<Plane> for HomogeneousMagnitude {
    type Output = Plane;

    fn outer_product(self, other: Plane) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() } }
    }
}

impl LeftContraction<Plane> for HomogeneousMagnitude {
    type Output = Plane;

    fn left_contraction(self, other: Plane) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() } }
    }
}

impl LeftAntiContraction<Plane> for HomogeneousMagnitude {
    type Output = Plane;

    fn left_anti_contraction(self, other: Plane) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()[1]) * other.group0() } }
    }
}

impl GeometricProduct<Motor> for HomogeneousMagnitude {
    type Output = Motor;

    fn geometric_product(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() + Simd32x4::from([self.group0()[1], self.group0()[1], self.group0()[1], self.group0()[0]]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group1()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]), g1: Simd32x3::from(self.group0()[0]) * other.group1() } }
    }
}

impl OuterProduct<Motor> for HomogeneousMagnitude {
    type Output = Motor;

    fn outer_product(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0(), g1: Simd32x3::from(self.group0()[0]) * other.group1() } }
    }
}

impl InnerProduct<Motor> for HomogeneousMagnitude {
    type Output = Motor;

    fn inner_product(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() + Simd32x4::from([self.group0()[1], self.group0()[1], self.group0()[1], self.group0()[0]]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group1()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]), g1: Simd32x3::from(self.group0()[0]) * other.group1() } }
    }
}

impl LeftContraction<Motor> for HomogeneousMagnitude {
    type Output = Motor;

    fn left_contraction(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0(), g1: Simd32x3::from(self.group0()[0]) * other.group1() } }
    }
}

impl LeftAntiContraction<Motor> for HomogeneousMagnitude {
    type Output = Motor;

    fn left_anti_contraction(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[1]) * other.group0(), g1: Simd32x3::from(self.group0()[1]) * other.group1() } }
    }
}

impl AntiScalarProduct<Motor> for HomogeneousMagnitude {
    type Output = AntiScalar;

    fn anti_scalar_product(self, other: Motor) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0()[1] * other.group0()[3] } }
    }
}

impl GeometricProduct<Rotor> for HomogeneousMagnitude {
    type Output = Rotor;

    fn geometric_product(self, other: Rotor) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() } }
    }
}

impl OuterProduct<Rotor> for HomogeneousMagnitude {
    type Output = Rotor;

    fn outer_product(self, other: Rotor) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() } }
    }
}

impl InnerProduct<Rotor> for HomogeneousMagnitude {
    type Output = Rotor;

    fn inner_product(self, other: Rotor) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() } }
    }
}

impl LeftContraction<Rotor> for HomogeneousMagnitude {
    type Output = Rotor;

    fn left_contraction(self, other: Rotor) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() } }
    }
}

impl LeftAntiContraction<Rotor> for HomogeneousMagnitude {
    type Output = Rotor;

    fn left_anti_contraction(self, other: Rotor) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group0()[1]) * other.group0() } }
    }
}

impl AntiScalarProduct<Rotor> for HomogeneousMagnitude {
    type Output = AntiScalar;

    fn anti_scalar_product(self, other: Rotor) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0()[1] * other.group0()[3] } }
    }
}

impl GeometricProduct<Translator> for HomogeneousMagnitude {
    type Output = Motor;

    fn geometric_product(self, other: Translator) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from([self.group0()[1], self.group0()[1], self.group0()[1], self.group0()[0]]) * other.group0(), g1: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) } }
    }
}

impl OuterProduct<Translator> for HomogeneousMagnitude {
    type Output = Translator;

    fn outer_product(self, other: Translator) -> Translator {
        Translator { groups: TranslatorGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() } }
    }
}

impl InnerProduct<Translator> for HomogeneousMagnitude {
    type Output = Motor;

    fn inner_product(self, other: Translator) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from([self.group0()[1], self.group0()[1], self.group0()[1], self.group0()[0]]) * other.group0(), g1: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) } }
    }
}

impl LeftContraction<Translator> for HomogeneousMagnitude {
    type Output = Translator;

    fn left_contraction(self, other: Translator) -> Translator {
        Translator { groups: TranslatorGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() } }
    }
}

impl LeftAntiContraction<Translator> for HomogeneousMagnitude {
    type Output = Translator;

    fn left_anti_contraction(self, other: Translator) -> Translator {
        Translator { groups: TranslatorGroups { g0: Simd32x4::from(self.group0()[1]) * other.group0() } }
    }
}

impl RightAntiContraction<Translator> for HomogeneousMagnitude {
    type Output = HomogeneousMagnitude;

    fn right_anti_contraction(self, other: Translator) -> HomogeneousMagnitude {
        HomogeneousMagnitude { groups: HomogeneousMagnitudeGroups { g0: self.group0() * Simd32x2::from(other.group0()[3]) } }
    }
}

impl AntiScalarProduct<Translator> for HomogeneousMagnitude {
    type Output = AntiScalar;

    fn anti_scalar_product(self, other: Translator) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0()[1] * other.group0()[3] } }
    }
}

impl GeometricProduct<Flector> for HomogeneousMagnitude {
    type Output = Flector;

    fn geometric_product(self, other: Flector) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() + Simd32x4::from([self.group0()[0], self.group0()[0], self.group0()[0], self.group0()[1]]) * swizzle!(other.group1(), 0, 0, 0, 3) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]), g1: Simd32x4::from(self.group0()[0]) * other.group1() + Simd32x4::from([self.group0()[1], self.group0()[1], self.group0()[1], self.group0()[0]]) * swizzle!(other.group0(), 0, 1, 2, 0) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]) } }
    }
}

impl RegressiveProduct<Flector> for HomogeneousMagnitude {
    type Output = Flector;

    fn regressive_product(self, other: Flector) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[1]) * other.group0(), g1: Simd32x4::from(self.group0()[1]) * other.group1() } }
    }
}

impl OuterProduct<Flector> for HomogeneousMagnitude {
    type Output = Flector;

    fn outer_product(self, other: Flector) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0(), g1: Simd32x4::from(self.group0()[0]) * other.group1() } }
    }
}

impl InnerProduct<Flector> for HomogeneousMagnitude {
    type Output = Flector;

    fn inner_product(self, other: Flector) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() + Simd32x4::from([self.group0()[0], self.group0()[0], self.group0()[0], self.group0()[1]]) * swizzle!(other.group1(), 0, 0, 0, 3) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]), g1: Simd32x4::from(self.group0()[0]) * other.group1() + Simd32x4::from([self.group0()[1], self.group0()[1], self.group0()[1], self.group0()[0]]) * swizzle!(other.group0(), 0, 1, 2, 0) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]) } }
    }
}

impl GeometricAntiProduct<Flector> for HomogeneousMagnitude {
    type Output = Flector;

    fn geometric_anti_product(self, other: Flector) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[1]) * other.group0() + Simd32x4::from(self.group0()[0]) * swizzle!(other.group1(), 0, 1, 2, 0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]), g1: Simd32x4::from(self.group0()[1]) * other.group1() + Simd32x4::from(self.group0()[0]) * swizzle!(other.group0(), 0, 0, 0, 3) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) } }
    }
}

impl InnerAntiProduct<Flector> for HomogeneousMagnitude {
    type Output = Flector;

    fn inner_anti_product(self, other: Flector) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[1]) * other.group0() + Simd32x4::from(self.group0()[0]) * swizzle!(other.group1(), 0, 1, 2, 0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]), g1: Simd32x4::from(self.group0()[1]) * other.group1() + Simd32x4::from(self.group0()[0]) * swizzle!(other.group0(), 0, 0, 0, 3) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) } }
    }
}

impl LeftContraction<Flector> for HomogeneousMagnitude {
    type Output = Flector;

    fn left_contraction(self, other: Flector) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0(), g1: Simd32x4::from(self.group0()[0]) * other.group1() } }
    }
}

impl LeftAntiContraction<Flector> for HomogeneousMagnitude {
    type Output = Flector;

    fn left_anti_contraction(self, other: Flector) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[1]) * other.group0(), g1: Simd32x4::from(self.group0()[1]) * other.group1() } }
    }
}

impl ScalarProduct<MultiVector> for HomogeneousMagnitude {
    type Output = Scalar;

    fn scalar_product(self, other: MultiVector) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[0] * other.group0()[0] } }
    }
}

impl AntiScalarProduct<MultiVector> for HomogeneousMagnitude {
    type Output = AntiScalar;

    fn anti_scalar_product(self, other: MultiVector) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0()[1] * other.group0()[1] } }
    }
}

impl SquaredMagnitude for HomogeneousMagnitude {
    type Output = Scalar;

    fn squared_magnitude(self) -> Scalar {
        self.scalar_product(self.reversal())
    }
}

impl Magnitude for HomogeneousMagnitude {
    type Output = Scalar;

    fn magnitude(self) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.squared_magnitude().group0().sqrt() } }
    }
}

impl BulkNorm for HomogeneousMagnitude {
    type Output = Scalar;

    fn bulk_norm(self) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.squared_magnitude().group0().sqrt() } }
    }
}

impl SquaredAntiMagnitude for HomogeneousMagnitude {
    type Output = AntiScalar;

    fn squared_anti_magnitude(self) -> AntiScalar {
        self.anti_scalar_product(self.anti_reversal())
    }
}

impl WeightNorm for HomogeneousMagnitude {
    type Output = AntiScalar;

    fn weight_norm(self) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.squared_anti_magnitude().group0().sqrt() } }
    }
}

impl GeometricNorm for HomogeneousMagnitude {
    type Output = HomogeneousMagnitude;

    fn geometric_norm(self) -> HomogeneousMagnitude {
        self.bulk_norm() + self.weight_norm()
    }
}

impl Scale for HomogeneousMagnitude {
    type Output = HomogeneousMagnitude;

    fn scale(self, other: f32) -> HomogeneousMagnitude {
        self.geometric_product(Scalar { groups: ScalarGroups { g0: other } })
    }
}

impl Signum for HomogeneousMagnitude {
    type Output = HomogeneousMagnitude;

    fn signum(self) -> HomogeneousMagnitude {
        self.geometric_product(Scalar { groups: ScalarGroups { g0: 1.0 / self.magnitude().group0() } })
    }
}

impl Inverse for HomogeneousMagnitude {
    type Output = HomogeneousMagnitude;

    fn inverse(self) -> HomogeneousMagnitude {
        self.reversal().geometric_product(Scalar { groups: ScalarGroups { g0: 1.0 / self.squared_magnitude().group0() } })
    }
}

impl Unitize for HomogeneousMagnitude {
    type Output = HomogeneousMagnitude;

    fn unitize(self) -> HomogeneousMagnitude {
        self.geometric_product(Scalar { groups: ScalarGroups { g0: 1.0 / self.weight_norm().group0() } })
    }
}

impl Zero for Point {
    fn zero() -> Self {
        Point { groups: PointGroups { g0: Simd32x4::from(0.0) } }
    }
}

impl One for Point {
    fn one() -> Self {
        Point { groups: PointGroups { g0: Simd32x4::from(0.0) } }
    }
}

impl Grade for Point {
    type Output = isize;

    fn grade(self) -> isize {
        1
    }
}

impl AntiGrade for Point {
    type Output = isize;

    fn anti_grade(self) -> isize {
        3
    }
}

impl Neg for Point {
    type Output = Point;

    fn neg(self) -> Point {
        Point { groups: PointGroups { g0: self.group0() * Simd32x4::from(-1.0) } }
    }
}

impl Automorphism for Point {
    type Output = Point;

    fn automorphism(self) -> Point {
        Point { groups: PointGroups { g0: self.group0() * Simd32x4::from(-1.0) } }
    }
}

impl Reversal for Point {
    type Output = Point;

    fn reversal(self) -> Point {
        Point { groups: PointGroups { g0: self.group0() } }
    }
}

impl Conjugation for Point {
    type Output = Point;

    fn conjugation(self) -> Point {
        Point { groups: PointGroups { g0: self.group0() * Simd32x4::from(-1.0) } }
    }
}

impl Dual for Point {
    type Output = Plane;

    fn dual(self) -> Plane {
        Plane { groups: PlaneGroups { g0: self.group0() } }
    }
}

impl AntiReversal for Point {
    type Output = Point;

    fn anti_reversal(self) -> Point {
        Point { groups: PointGroups { g0: self.group0() * Simd32x4::from(-1.0) } }
    }
}

impl RightComplement for Point {
    type Output = Plane;

    fn right_complement(self) -> Plane {
        Plane { groups: PlaneGroups { g0: self.group0() } }
    }
}

impl LeftComplement for Point {
    type Output = Plane;

    fn left_complement(self) -> Plane {
        Plane { groups: PlaneGroups { g0: self.group0() * Simd32x4::from(-1.0) } }
    }
}

impl DoubleComplement for Point {
    type Output = Point;

    fn double_complement(self) -> Point {
        Point { groups: PointGroups { g0: self.group0() * Simd32x4::from(-1.0) } }
    }
}

impl GeometricProduct<Scalar> for Point {
    type Output = Point;

    fn geometric_product(self, other: Scalar) -> Point {
        Point { groups: PointGroups { g0: self.group0() * Simd32x4::from(other.group0()) } }
    }
}

impl OuterProduct<Scalar> for Point {
    type Output = Point;

    fn outer_product(self, other: Scalar) -> Point {
        Point { groups: PointGroups { g0: self.group0() * Simd32x4::from(other.group0()) } }
    }
}

impl InnerProduct<Scalar> for Point {
    type Output = Point;

    fn inner_product(self, other: Scalar) -> Point {
        Point { groups: PointGroups { g0: self.group0() * Simd32x4::from(other.group0()) } }
    }
}

impl RightContraction<Scalar> for Point {
    type Output = Point;

    fn right_contraction(self, other: Scalar) -> Point {
        Point { groups: PointGroups { g0: self.group0() * Simd32x4::from(other.group0()) } }
    }
}

impl RegressiveProduct<AntiScalar> for Point {
    type Output = Point;

    fn regressive_product(self, other: AntiScalar) -> Point {
        Point { groups: PointGroups { g0: self.group0() * Simd32x4::from(other.group0()) } }
    }
}

impl GeometricAntiProduct<AntiScalar> for Point {
    type Output = Point;

    fn geometric_anti_product(self, other: AntiScalar) -> Point {
        Point { groups: PointGroups { g0: self.group0() * Simd32x4::from(other.group0()) } }
    }
}

impl InnerAntiProduct<AntiScalar> for Point {
    type Output = Point;

    fn inner_anti_product(self, other: AntiScalar) -> Point {
        Point { groups: PointGroups { g0: self.group0() * Simd32x4::from(other.group0()) } }
    }
}

impl RightAntiContraction<AntiScalar> for Point {
    type Output = Point;

    fn right_anti_contraction(self, other: AntiScalar) -> Point {
        Point { groups: PointGroups { g0: self.group0() * Simd32x4::from(other.group0()) } }
    }
}

impl RegressiveProduct<HomogeneousMagnitude> for Point {
    type Output = Point;

    fn regressive_product(self, other: HomogeneousMagnitude) -> Point {
        Point { groups: PointGroups { g0: self.group0() * Simd32x4::from(other.group0()[1]) } }
    }
}

impl OuterProduct<HomogeneousMagnitude> for Point {
    type Output = Point;

    fn outer_product(self, other: HomogeneousMagnitude) -> Point {
        Point { groups: PointGroups { g0: self.group0() * Simd32x4::from(other.group0()[0]) } }
    }
}

impl RightContraction<HomogeneousMagnitude> for Point {
    type Output = Point;

    fn right_contraction(self, other: HomogeneousMagnitude) -> Point {
        Point { groups: PointGroups { g0: self.group0() * Simd32x4::from(other.group0()[0]) } }
    }
}

impl RightAntiContraction<HomogeneousMagnitude> for Point {
    type Output = Point;

    fn right_anti_contraction(self, other: HomogeneousMagnitude) -> Point {
        Point { groups: PointGroups { g0: self.group0() * Simd32x4::from(other.group0()[1]) } }
    }
}

impl Add<Point> for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point { groups: PointGroups { g0: self.group0() + other.group0() } }
    }
}

impl AddAssign<Point> for Point {
    fn add_assign(&mut self, other: Point) {
        *self = (*self).add(other);
    }
}

impl Sub<Point> for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point { groups: PointGroups { g0: self.group0() - other.group0() } }
    }
}

impl SubAssign<Point> for Point {
    fn sub_assign(&mut self, other: Point) {
        *self = (*self).sub(other);
    }
}

impl Mul<Point> for Point {
    type Output = Point;

    fn mul(self, other: Point) -> Point {
        Point { groups: PointGroups { g0: self.group0() * other.group0() } }
    }
}

impl MulAssign<Point> for Point {
    fn mul_assign(&mut self, other: Point) {
        *self = (*self).mul(other);
    }
}

impl Div<Point> for Point {
    type Output = Point;

    fn div(self, other: Point) -> Point {
        Point { groups: PointGroups { g0: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group0()[3]]) * Simd32x4::from([1.0, 1.0, 1.0, 1.0]) / Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group0()[3]]) * Simd32x4::from([1.0, 1.0, 1.0, 1.0]) } }
    }
}

impl DivAssign<Point> for Point {
    fn div_assign(&mut self, other: Point) {
        *self = (*self).div(other);
    }
}

impl OuterProduct<Point> for Point {
    type Output = Line;

    fn outer_product(self, other: Point) -> Line {
        Line { groups: LineGroups { g0: Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) + Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()[3]) * Simd32x3::from(-1.0), g1: Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], other.group0()[2], other.group0()[0]]) * Simd32x3::from([1.0, 0.0, -1.0]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], other.group0()[0], other.group0()[1]]) * Simd32x3::from([-1.0, 1.0, 0.0]) + Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[0], other.group0()[2], other.group0()[1]]) * Simd32x3::from([0.0, -1.0, 1.0]) } }
    }
}

impl InnerProduct<Point> for Point {
    type Output = Scalar;

    fn inner_product(self, other: Point) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2] } }
    }
}

impl GeometricAntiProduct<Point> for Point {
    type Output = Translator;

    fn geometric_anti_product(self, other: Point) -> Translator {
        Translator { groups: TranslatorGroups { g0: Simd32x4::from(self.group0()[3]) * other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) + swizzle!(self.group0(), 0, 1, 2, 0) * swizzle!(other.group0(), 3, 3, 3, 0) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]) } }
    }
}

impl InnerAntiProduct<Point> for Point {
    type Output = AntiScalar;

    fn inner_anti_product(self, other: Point) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: 0.0 - self.group0()[3] * other.group0()[3] } }
    }
}

impl LeftContraction<Point> for Point {
    type Output = Scalar;

    fn left_contraction(self, other: Point) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2] } }
    }
}

impl RightContraction<Point> for Point {
    type Output = Scalar;

    fn right_contraction(self, other: Point) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2] } }
    }
}

impl LeftAntiContraction<Point> for Point {
    type Output = AntiScalar;

    fn left_anti_contraction(self, other: Point) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: 0.0 - self.group0()[3] * other.group0()[3] } }
    }
}

impl RightAntiContraction<Point> for Point {
    type Output = AntiScalar;

    fn right_anti_contraction(self, other: Point) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: 0.0 - self.group0()[3] * other.group0()[3] } }
    }
}

impl ScalarProduct<Point> for Point {
    type Output = Scalar;

    fn scalar_product(self, other: Point) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2] } }
    }
}

impl AntiScalarProduct<Point> for Point {
    type Output = AntiScalar;

    fn anti_scalar_product(self, other: Point) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: 0.0 - self.group0()[3] * other.group0()[3] } }
    }
}

impl GeometricProduct<Line> for Point {
    type Output = Flector;

    fn geometric_product(self, other: Line) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group1()[2], other.group1()[2], other.group1()[0], other.group0()[1]]) * Simd32x4::from([-1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[1], other.group1()[0], other.group1()[1], other.group0()[2]]) * Simd32x4::from([1.0, -1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group1()[0], other.group1()[2], other.group1()[1], other.group0()[0]]) * Simd32x4::from([0.0, 1.0, -1.0, -1.0]), g1: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], other.group0()[2], other.group0()[0], other.group1()[1]]) * Simd32x4::from([-1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], other.group0()[0], other.group0()[1], other.group1()[2]]) * Simd32x4::from([1.0, -1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group1()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[0], other.group0()[2], other.group0()[1], other.group1()[0]]) * Simd32x4::from([0.0, 1.0, -1.0, -1.0]) } }
    }
}

impl OuterProduct<Line> for Point {
    type Output = Plane;

    fn outer_product(self, other: Line) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], other.group0()[2], other.group0()[0], other.group1()[1]]) * Simd32x4::from([-1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], other.group0()[0], other.group0()[1], other.group1()[2]]) * Simd32x4::from([1.0, -1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group1()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[0], other.group0()[2], other.group0()[1], other.group1()[0]]) * Simd32x4::from([0.0, 1.0, -1.0, -1.0]) } }
    }
}

impl InnerProduct<Line> for Point {
    type Output = Point;

    fn inner_product(self, other: Line) -> Point {
        Point { groups: PointGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group1()[2], other.group1()[2], other.group1()[0], other.group0()[1]]) * Simd32x4::from([-1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[1], other.group1()[0], other.group1()[1], other.group0()[2]]) * Simd32x4::from([1.0, -1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group1()[0], other.group1()[2], other.group1()[1], other.group0()[0]]) * Simd32x4::from([0.0, 1.0, -1.0, -1.0]) } }
    }
}

impl InnerAntiProduct<Line> for Point {
    type Output = Plane;

    fn inner_anti_product(self, other: Line) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + swizzle!(self.group0(), 3, 3, 3, 0) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group0()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) } }
    }
}

impl LeftContraction<Line> for Point {
    type Output = Point;

    fn left_contraction(self, other: Line) -> Point {
        Point { groups: PointGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group1()[2], other.group1()[2], other.group1()[0], other.group0()[1]]) * Simd32x4::from([-1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[1], other.group1()[0], other.group1()[1], other.group0()[2]]) * Simd32x4::from([1.0, -1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group1()[0], other.group1()[2], other.group1()[1], other.group0()[0]]) * Simd32x4::from([0.0, 1.0, -1.0, -1.0]) } }
    }
}

impl RightAntiContraction<Line> for Point {
    type Output = Plane;

    fn right_anti_contraction(self, other: Line) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + swizzle!(self.group0(), 3, 3, 3, 0) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group0()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) } }
    }
}

impl Add<Plane> for Point {
    type Output = Flector;

    fn add(self, other: Plane) -> Flector {
        Flector { groups: FlectorGroups { g0: self.group0(), g1: other.group0() } }
    }
}

impl Sub<Plane> for Point {
    type Output = Flector;

    fn sub(self, other: Plane) -> Flector {
        Flector { groups: FlectorGroups { g0: self.group0(), g1: Simd32x4::from(0.0) - other.group0() } }
    }
}

impl GeometricProduct<Plane> for Point {
    type Output = Motor;

    fn geometric_product(self, other: Plane) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 2, 2, 0, 1) * Simd32x4::from([1.0, 0.0, -1.0, 1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 1, 0, 1, 2) * Simd32x4::from([-1.0, 1.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from(other.group0()[3]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[0]) * swizzle!(other.group0(), 0, 2, 1, 0) * Simd32x4::from([0.0, -1.0, 1.0, 1.0]), g1: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()[3]) * Simd32x3::from(-1.0) } }
    }
}

impl RegressiveProduct<Plane> for Point {
    type Output = Scalar;

    fn regressive_product(self, other: Plane) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2] + self.group0()[3] * other.group0()[3] } }
    }
}

impl OuterProduct<Plane> for Point {
    type Output = AntiScalar;

    fn outer_product(self, other: Plane) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2] + self.group0()[3] * other.group0()[3] } }
    }
}

impl InnerProduct<Plane> for Point {
    type Output = Line;

    fn inner_product(self, other: Plane) -> Line {
        Line { groups: LineGroups { g0: Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], other.group0()[2], other.group0()[0]]) * Simd32x3::from([1.0, 0.0, -1.0]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], other.group0()[0], other.group0()[1]]) * Simd32x3::from([-1.0, 1.0, 0.0]) + Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[0], other.group0()[2], other.group0()[1]]) * Simd32x3::from([0.0, -1.0, 1.0]), g1: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()[3]) * Simd32x3::from(-1.0) } }
    }
}

impl InnerAntiProduct<Plane> for Point {
    type Output = Line;

    fn inner_anti_product(self, other: Plane) -> Line {
        Line { groups: LineGroups { g0: Simd32x3::from(0.0) - Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]), g1: Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], other.group0()[2], other.group0()[0]]) * Simd32x3::from([-1.0, 0.0, 1.0]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], other.group0()[0], other.group0()[1]]) * Simd32x3::from([1.0, -1.0, 0.0]) + Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[0], other.group0()[2], other.group0()[1]]) * Simd32x3::from([0.0, 1.0, -1.0]) } }
    }
}

impl LeftContraction<Plane> for Point {
    type Output = Line;

    fn left_contraction(self, other: Plane) -> Line {
        Line { groups: LineGroups { g0: Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], other.group0()[2], other.group0()[0]]) * Simd32x3::from([1.0, 0.0, -1.0]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], other.group0()[0], other.group0()[1]]) * Simd32x3::from([-1.0, 1.0, 0.0]) + Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[0], other.group0()[2], other.group0()[1]]) * Simd32x3::from([0.0, -1.0, 1.0]), g1: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()[3]) * Simd32x3::from(-1.0) } }
    }
}

impl RightAntiContraction<Plane> for Point {
    type Output = Line;

    fn right_anti_contraction(self, other: Plane) -> Line {
        Line { groups: LineGroups { g0: Simd32x3::from(0.0) - Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]), g1: Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], other.group0()[2], other.group0()[0]]) * Simd32x3::from([-1.0, 0.0, 1.0]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], other.group0()[0], other.group0()[1]]) * Simd32x3::from([1.0, -1.0, 0.0]) + Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[0], other.group0()[2], other.group0()[1]]) * Simd32x3::from([0.0, 1.0, -1.0]) } }
    }
}

impl GeometricProduct<Motor> for Point {
    type Output = Flector;

    fn geometric_product(self, other: Motor) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group1()[2], other.group1()[2], other.group1()[0], other.group0()[1]]) * Simd32x4::from([-1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[1], other.group1()[0], other.group1()[1], other.group0()[2]]) * Simd32x4::from([1.0, -1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group1()[0], other.group1()[2], other.group1()[1], other.group0()[0]]) * Simd32x4::from([0.0, 1.0, -1.0, -1.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[3], other.group0()[2], other.group0()[1], other.group1()[0]]) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], other.group0()[3], other.group0()[0], other.group1()[1]]) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], other.group0()[0], other.group0()[3], other.group1()[2]]) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) + swizzle!(self.group0(), 3, 3, 3, 0) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group1()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) } }
    }
}

impl RegressiveProduct<Motor> for Point {
    type Output = Point;

    fn regressive_product(self, other: Motor) -> Point {
        Point { groups: PointGroups { g0: self.group0() * Simd32x4::from(other.group0()[3]) } }
    }
}

impl OuterProduct<Motor> for Point {
    type Output = Plane;

    fn outer_product(self, other: Motor) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], other.group0()[2], other.group0()[0], other.group1()[1]]) * Simd32x4::from([-1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], other.group0()[0], other.group0()[1], other.group1()[2]]) * Simd32x4::from([1.0, -1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group1()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[0], other.group0()[2], other.group0()[1], other.group1()[0]]) * Simd32x4::from([0.0, 1.0, -1.0, -1.0]) } }
    }
}

impl GeometricAntiProduct<Motor> for Point {
    type Output = Flector;

    fn geometric_anti_product(self, other: Motor) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 2, 3, 0, 2) * Simd32x4::from([1.0, 1.0, -1.0, 0.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 1, 0, 3, 1) * Simd32x4::from([-1.0, 1.0, 1.0, 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group0()[3]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]) + Simd32x4::from(self.group0()[0]) * swizzle!(other.group0(), 3, 2, 1, 0) * Simd32x4::from([1.0, -1.0, 1.0, 0.0]), g1: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + swizzle!(self.group0(), 3, 3, 3, 0) * swizzle!(other.group0(), 0, 1, 2, 0) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) } }
    }
}

impl InnerAntiProduct<Motor> for Point {
    type Output = Flector;

    fn inner_anti_product(self, other: Motor) -> Flector {
        Flector { groups: FlectorGroups { g0: self.group0() * Simd32x4::from(other.group0()[3]), g1: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + swizzle!(self.group0(), 3, 3, 3, 0) * swizzle!(other.group0(), 0, 1, 2, 0) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) } }
    }
}

impl RightAntiContraction<Motor> for Point {
    type Output = Flector;

    fn right_anti_contraction(self, other: Motor) -> Flector {
        Flector { groups: FlectorGroups { g0: self.group0() * Simd32x4::from(other.group0()[3]), g1: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + swizzle!(self.group0(), 3, 3, 3, 0) * swizzle!(other.group0(), 0, 1, 2, 0) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) } }
    }
}

impl RegressiveProduct<Rotor> for Point {
    type Output = Point;

    fn regressive_product(self, other: Rotor) -> Point {
        Point { groups: PointGroups { g0: self.group0() * Simd32x4::from(other.group0()[3]) } }
    }
}

impl GeometricAntiProduct<Rotor> for Point {
    type Output = Flector;

    fn geometric_anti_product(self, other: Rotor) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 2, 3, 0, 2) * Simd32x4::from([1.0, 1.0, -1.0, 0.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 1, 0, 3, 1) * Simd32x4::from([-1.0, 1.0, 1.0, 0.0]) + swizzle!(self.group0(), 0, 0, 0, 3) * swizzle!(other.group0(), 3, 2, 1, 3) * Simd32x4::from([1.0, -1.0, 1.0, 1.0]), g1: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + swizzle!(self.group0(), 3, 3, 3, 0) * swizzle!(other.group0(), 0, 1, 2, 0) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) } }
    }
}

impl InnerAntiProduct<Rotor> for Point {
    type Output = Flector;

    fn inner_anti_product(self, other: Rotor) -> Flector {
        Flector { groups: FlectorGroups { g0: self.group0() * Simd32x4::from(other.group0()[3]), g1: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + swizzle!(self.group0(), 3, 3, 3, 0) * swizzle!(other.group0(), 0, 1, 2, 0) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) } }
    }
}

impl RightAntiContraction<Rotor> for Point {
    type Output = Flector;

    fn right_anti_contraction(self, other: Rotor) -> Flector {
        Flector { groups: FlectorGroups { g0: self.group0() * Simd32x4::from(other.group0()[3]), g1: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + swizzle!(self.group0(), 3, 3, 3, 0) * swizzle!(other.group0(), 0, 1, 2, 0) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) } }
    }
}

impl RegressiveProduct<Translator> for Point {
    type Output = Point;

    fn regressive_product(self, other: Translator) -> Point {
        Point { groups: PointGroups { g0: self.group0() * Simd32x4::from(other.group0()[3]) } }
    }
}

impl OuterProduct<Translator> for Point {
    type Output = Plane;

    fn outer_product(self, other: Translator) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + swizzle!(self.group0(), 3, 3, 3, 0) * swizzle!(other.group0(), 0, 1, 2, 0) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) } }
    }
}

impl GeometricAntiProduct<Translator> for Point {
    type Output = Point;

    fn geometric_anti_product(self, other: Translator) -> Point {
        Point { groups: PointGroups { g0: Simd32x4::from(self.group0()[3]) * other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]) + swizzle!(self.group0(), 0, 1, 2, 0) * swizzle!(other.group0(), 3, 3, 3, 0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) } }
    }
}

impl InnerAntiProduct<Translator> for Point {
    type Output = Point;

    fn inner_anti_product(self, other: Translator) -> Point {
        Point { groups: PointGroups { g0: self.group0() * Simd32x4::from(other.group0()[3]) } }
    }
}

impl RightAntiContraction<Translator> for Point {
    type Output = Point;

    fn right_anti_contraction(self, other: Translator) -> Point {
        Point { groups: PointGroups { g0: self.group0() * Simd32x4::from(other.group0()[3]) } }
    }
}

impl Add<Flector> for Point {
    type Output = Flector;

    fn add(self, other: Flector) -> Flector {
        Flector { groups: FlectorGroups { g0: self.group0() + other.group0(), g1: other.group1() } }
    }
}

impl Sub<Flector> for Point {
    type Output = Flector;

    fn sub(self, other: Flector) -> Flector {
        Flector { groups: FlectorGroups { g0: self.group0() - other.group0(), g1: Simd32x4::from(0.0) - other.group1() } }
    }
}

impl RegressiveProduct<Flector> for Point {
    type Output = Scalar;

    fn regressive_product(self, other: Flector) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[0] * other.group1()[0] + self.group0()[1] * other.group1()[1] + self.group0()[2] * other.group1()[2] + self.group0()[3] * other.group1()[3] } }
    }
}

impl OuterProduct<Flector> for Point {
    type Output = Motor;

    fn outer_product(self, other: Flector) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[3], other.group0()[3], other.group0()[3], other.group1()[1]]) * Simd32x4::from([0.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[3], other.group0()[3], other.group0()[3], other.group1()[2]]) * Simd32x4::from([0.0, 0.0, -1.0, 1.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[3], other.group0()[0], other.group0()[0], other.group1()[0]]) * Simd32x4::from([-1.0, 0.0, 0.0, 1.0]), g1: Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], other.group0()[2], other.group0()[0]]) * Simd32x3::from([1.0, 0.0, -1.0]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], other.group0()[0], other.group0()[1]]) * Simd32x3::from([-1.0, 1.0, 0.0]) + Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[0], other.group0()[2], other.group0()[1]]) * Simd32x3::from([0.0, -1.0, 1.0]) } }
    }
}

impl InnerAntiProduct<Flector> for Point {
    type Output = Motor;

    fn inner_anti_product(self, other: Flector) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(0.0) - Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group0()[3]]), g1: Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group1()[2], other.group1()[2], other.group1()[0]]) * Simd32x3::from([-1.0, 0.0, 1.0]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group1()[1], other.group1()[0], other.group1()[1]]) * Simd32x3::from([1.0, -1.0, 0.0]) + Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group1()[0], other.group1()[2], other.group1()[1]]) * Simd32x3::from([0.0, 1.0, -1.0]) } }
    }
}

impl RightContraction<Flector> for Point {
    type Output = Scalar;

    fn right_contraction(self, other: Flector) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2] } }
    }
}

impl LeftAntiContraction<Flector> for Point {
    type Output = AntiScalar;

    fn left_anti_contraction(self, other: Flector) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: 0.0 - self.group0()[3] * other.group0()[3] } }
    }
}

impl RightAntiContraction<Flector> for Point {
    type Output = Motor;

    fn right_anti_contraction(self, other: Flector) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(0.0) - Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group0()[3]]), g1: Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group1()[2], other.group1()[2], other.group1()[0]]) * Simd32x3::from([-1.0, 0.0, 1.0]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group1()[1], other.group1()[0], other.group1()[1]]) * Simd32x3::from([1.0, -1.0, 0.0]) + Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group1()[0], other.group1()[2], other.group1()[1]]) * Simd32x3::from([0.0, 1.0, -1.0]) } }
    }
}

impl ScalarProduct<Flector> for Point {
    type Output = Scalar;

    fn scalar_product(self, other: Flector) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2] } }
    }
}

impl AntiScalarProduct<Flector> for Point {
    type Output = AntiScalar;

    fn anti_scalar_product(self, other: Flector) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: 0.0 - self.group0()[3] * other.group0()[3] } }
    }
}

impl ScalarProduct<MultiVector> for Point {
    type Output = Scalar;

    fn scalar_product(self, other: MultiVector) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[0] * other.group1()[0] + self.group0()[1] * other.group1()[1] + self.group0()[2] * other.group1()[2] } }
    }
}

impl AntiScalarProduct<MultiVector> for Point {
    type Output = AntiScalar;

    fn anti_scalar_product(self, other: MultiVector) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: 0.0 - self.group0()[3] * other.group1()[3] } }
    }
}

impl SquaredMagnitude for Point {
    type Output = Scalar;

    fn squared_magnitude(self) -> Scalar {
        self.scalar_product(self.reversal())
    }
}

impl Magnitude for Point {
    type Output = Scalar;

    fn magnitude(self) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.squared_magnitude().group0().sqrt() } }
    }
}

impl BulkNorm for Point {
    type Output = Scalar;

    fn bulk_norm(self) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.squared_magnitude().group0().sqrt() } }
    }
}

impl SquaredAntiMagnitude for Point {
    type Output = AntiScalar;

    fn squared_anti_magnitude(self) -> AntiScalar {
        self.anti_scalar_product(self.anti_reversal())
    }
}

impl WeightNorm for Point {
    type Output = AntiScalar;

    fn weight_norm(self) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.squared_anti_magnitude().group0().sqrt() } }
    }
}

impl GeometricNorm for Point {
    type Output = HomogeneousMagnitude;

    fn geometric_norm(self) -> HomogeneousMagnitude {
        self.bulk_norm() + self.weight_norm()
    }
}

impl Scale for Point {
    type Output = Point;

    fn scale(self, other: f32) -> Point {
        self.geometric_product(Scalar { groups: ScalarGroups { g0: other } })
    }
}

impl Signum for Point {
    type Output = Point;

    fn signum(self) -> Point {
        self.geometric_product(Scalar { groups: ScalarGroups { g0: 1.0 / self.magnitude().group0() } })
    }
}

impl Inverse for Point {
    type Output = Point;

    fn inverse(self) -> Point {
        self.reversal().geometric_product(Scalar { groups: ScalarGroups { g0: 1.0 / self.squared_magnitude().group0() } })
    }
}

impl Unitize for Point {
    type Output = Point;

    fn unitize(self) -> Point {
        self.geometric_product(Scalar { groups: ScalarGroups { g0: 1.0 / self.weight_norm().group0() } })
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

impl Grade for Line {
    type Output = isize;

    fn grade(self) -> isize {
        2
    }
}

impl AntiGrade for Line {
    type Output = isize;

    fn anti_grade(self) -> isize {
        2
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
        Line { groups: LineGroups { g0: self.group0(), g1: self.group1() } }
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
        Line { groups: LineGroups { g0: self.group0() * Simd32x3::from(-1.0), g1: self.group1() * Simd32x3::from(-1.0) } }
    }
}

impl Dual for Line {
    type Output = Line;

    fn dual(self) -> Line {
        Line { groups: LineGroups { g0: self.group1() * Simd32x3::from(-1.0), g1: self.group0() * Simd32x3::from(-1.0) } }
    }
}

impl AntiReversal for Line {
    type Output = Line;

    fn anti_reversal(self) -> Line {
        Line { groups: LineGroups { g0: self.group0() * Simd32x3::from(-1.0), g1: self.group1() * Simd32x3::from(-1.0) } }
    }
}

impl RightComplement for Line {
    type Output = Line;

    fn right_complement(self) -> Line {
        Line { groups: LineGroups { g0: self.group1() * Simd32x3::from(-1.0), g1: self.group0() * Simd32x3::from(-1.0) } }
    }
}

impl LeftComplement for Line {
    type Output = Line;

    fn left_complement(self) -> Line {
        Line { groups: LineGroups { g0: self.group1() * Simd32x3::from(-1.0), g1: self.group0() * Simd32x3::from(-1.0) } }
    }
}

impl DoubleComplement for Line {
    type Output = Line;

    fn double_complement(self) -> Line {
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

impl Add<AntiScalar> for Line {
    type Output = Motor;

    fn add(self, other: AntiScalar) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group0()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) + Simd32x4::from(other.group0()) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]), g1: self.group1() } }
    }
}

impl Sub<AntiScalar> for Line {
    type Output = Motor;

    fn sub(self, other: AntiScalar) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group0()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) - Simd32x4::from(other.group0()) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]), g1: self.group1() } }
    }
}

impl RegressiveProduct<AntiScalar> for Line {
    type Output = Line;

    fn regressive_product(self, other: AntiScalar) -> Line {
        Line { groups: LineGroups { g0: self.group0() * Simd32x3::from(other.group0()), g1: self.group1() * Simd32x3::from(other.group0()) } }
    }
}

impl GeometricAntiProduct<AntiScalar> for Line {
    type Output = Line;

    fn geometric_anti_product(self, other: AntiScalar) -> Line {
        Line { groups: LineGroups { g0: self.group0() * Simd32x3::from(other.group0()), g1: self.group1() * Simd32x3::from(other.group0()) } }
    }
}

impl InnerAntiProduct<AntiScalar> for Line {
    type Output = Line;

    fn inner_anti_product(self, other: AntiScalar) -> Line {
        Line { groups: LineGroups { g0: self.group0() * Simd32x3::from(other.group0()), g1: self.group1() * Simd32x3::from(other.group0()) } }
    }
}

impl RightAntiContraction<AntiScalar> for Line {
    type Output = Line;

    fn right_anti_contraction(self, other: AntiScalar) -> Line {
        Line { groups: LineGroups { g0: self.group0() * Simd32x3::from(other.group0()), g1: self.group1() * Simd32x3::from(other.group0()) } }
    }
}

impl GeometricProduct<HomogeneousMagnitude> for Line {
    type Output = Line;

    fn geometric_product(self, other: HomogeneousMagnitude) -> Line {
        Line { groups: LineGroups { g0: Simd32x3::from(self.group1()[0]) * Simd32x3::from(other.group0()[1]) * Simd32x3::from([1.0, 0.0, 0.0]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from(other.group0()[1]) * Simd32x3::from([0.0, 1.0, 0.0]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from(other.group0()[1]) * Simd32x3::from([0.0, 0.0, 1.0]) + self.group0() * Simd32x3::from(other.group0()[0]), g1: self.group1() * Simd32x3::from(other.group0()[0]) } }
    }
}

impl RegressiveProduct<HomogeneousMagnitude> for Line {
    type Output = Line;

    fn regressive_product(self, other: HomogeneousMagnitude) -> Line {
        Line { groups: LineGroups { g0: self.group0() * Simd32x3::from(other.group0()[1]), g1: self.group1() * Simd32x3::from(other.group0()[1]) } }
    }
}

impl OuterProduct<HomogeneousMagnitude> for Line {
    type Output = Line;

    fn outer_product(self, other: HomogeneousMagnitude) -> Line {
        Line { groups: LineGroups { g0: self.group0() * Simd32x3::from(other.group0()[0]), g1: self.group1() * Simd32x3::from(other.group0()[0]) } }
    }
}

impl InnerProduct<HomogeneousMagnitude> for Line {
    type Output = Line;

    fn inner_product(self, other: HomogeneousMagnitude) -> Line {
        Line { groups: LineGroups { g0: Simd32x3::from(self.group1()[0]) * Simd32x3::from(other.group0()[1]) * Simd32x3::from([1.0, 0.0, 0.0]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from(other.group0()[1]) * Simd32x3::from([0.0, 1.0, 0.0]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from(other.group0()[1]) * Simd32x3::from([0.0, 0.0, 1.0]) + self.group0() * Simd32x3::from(other.group0()[0]), g1: self.group1() * Simd32x3::from(other.group0()[0]) } }
    }
}

impl GeometricAntiProduct<HomogeneousMagnitude> for Line {
    type Output = Line;

    fn geometric_anti_product(self, other: HomogeneousMagnitude) -> Line {
        Line { groups: LineGroups { g0: self.group0() * Simd32x3::from(other.group0()[1]), g1: Simd32x3::from(self.group1()[0]) * Simd32x3::from(other.group0()[1]) * Simd32x3::from([1.0, 0.0, 0.0]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from(other.group0()[1]) * Simd32x3::from([0.0, 1.0, 0.0]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from(other.group0()[1]) * Simd32x3::from([0.0, 0.0, 1.0]) + self.group0() * Simd32x3::from(other.group0()[0]) } }
    }
}

impl InnerAntiProduct<HomogeneousMagnitude> for Line {
    type Output = Line;

    fn inner_anti_product(self, other: HomogeneousMagnitude) -> Line {
        Line { groups: LineGroups { g0: self.group0() * Simd32x3::from(other.group0()[1]), g1: Simd32x3::from(self.group1()[0]) * Simd32x3::from(other.group0()[1]) * Simd32x3::from([1.0, 0.0, 0.0]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from(other.group0()[1]) * Simd32x3::from([0.0, 1.0, 0.0]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from(other.group0()[1]) * Simd32x3::from([0.0, 0.0, 1.0]) + self.group0() * Simd32x3::from(other.group0()[0]) } }
    }
}

impl RightContraction<HomogeneousMagnitude> for Line {
    type Output = Line;

    fn right_contraction(self, other: HomogeneousMagnitude) -> Line {
        Line { groups: LineGroups { g0: self.group0() * Simd32x3::from(other.group0()[0]), g1: self.group1() * Simd32x3::from(other.group0()[0]) } }
    }
}

impl RightAntiContraction<HomogeneousMagnitude> for Line {
    type Output = Line;

    fn right_anti_contraction(self, other: HomogeneousMagnitude) -> Line {
        Line { groups: LineGroups { g0: self.group0() * Simd32x3::from(other.group0()[1]), g1: self.group1() * Simd32x3::from(other.group0()[1]) } }
    }
}

impl GeometricProduct<Point> for Line {
    type Output = Flector;

    fn geometric_product(self, other: Point) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group1()[1]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([-1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group0(), 1, 0, 1, 1) * Simd32x4::from([1.0, -1.0, 0.0, 0.0]) + Simd32x4::from([self.group0()[0], self.group1()[0], self.group1()[0], self.group0()[0]]) * swizzle!(other.group0(), 0, 2, 1, 0) * Simd32x4::from([0.0, 1.0, -1.0, 1.0]), g1: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, -1.0, 0.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 1, 0, 1, 1) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[1]) * swizzle!(other.group0(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group0(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from([self.group1()[0], self.group0()[0], self.group0()[0], self.group1()[0]]) * swizzle!(other.group0(), 3, 2, 1, 0) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) } }
    }
}

impl OuterProduct<Point> for Line {
    type Output = Plane;

    fn outer_product(self, other: Point) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, -1.0, 0.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 1, 0, 1, 1) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[1]) * swizzle!(other.group0(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group0(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from([self.group1()[0], self.group0()[0], self.group0()[0], self.group1()[0]]) * swizzle!(other.group0(), 3, 2, 1, 0) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) } }
    }
}

impl InnerProduct<Point> for Line {
    type Output = Point;

    fn inner_product(self, other: Point) -> Point {
        Point { groups: PointGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group1()[1]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([-1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group0(), 1, 0, 1, 1) * Simd32x4::from([1.0, -1.0, 0.0, 0.0]) + Simd32x4::from([self.group0()[0], self.group1()[0], self.group1()[0], self.group0()[0]]) * swizzle!(other.group0(), 0, 2, 1, 0) * Simd32x4::from([0.0, 1.0, -1.0, 1.0]) } }
    }
}

impl InnerAntiProduct<Point> for Line {
    type Output = Plane;

    fn inner_anti_product(self, other: Point) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[0]) * swizzle!(other.group0(), 3, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]) } }
    }
}

impl RightContraction<Point> for Line {
    type Output = Point;

    fn right_contraction(self, other: Point) -> Point {
        Point { groups: PointGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group1()[1]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([-1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group0(), 1, 0, 1, 1) * Simd32x4::from([1.0, -1.0, 0.0, 0.0]) + Simd32x4::from([self.group0()[0], self.group1()[0], self.group1()[0], self.group0()[0]]) * swizzle!(other.group0(), 0, 2, 1, 0) * Simd32x4::from([0.0, 1.0, -1.0, 1.0]) } }
    }
}

impl LeftAntiContraction<Point> for Line {
    type Output = Plane;

    fn left_anti_contraction(self, other: Point) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[0]) * swizzle!(other.group0(), 3, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]) } }
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

impl RegressiveProduct<Line> for Line {
    type Output = Scalar;

    fn regressive_product(self, other: Line) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group0()[0] * other.group1()[0] - self.group0()[1] * other.group1()[1] - self.group0()[2] * other.group1()[2] - self.group1()[0] * other.group0()[0] - self.group1()[1] * other.group0()[1] - self.group1()[2] * other.group0()[2] } }
    }
}

impl OuterProduct<Line> for Line {
    type Output = AntiScalar;

    fn outer_product(self, other: Line) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: 0.0 - self.group0()[0] * other.group1()[0] - self.group0()[1] * other.group1()[1] - self.group0()[2] * other.group1()[2] - self.group1()[0] * other.group0()[0] - self.group1()[1] * other.group0()[1] - self.group1()[2] * other.group0()[2] } }
    }
}

impl InnerProduct<Line> for Line {
    type Output = Scalar;

    fn inner_product(self, other: Line) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group1()[0] * other.group1()[0] - self.group1()[1] * other.group1()[1] - self.group1()[2] * other.group1()[2] } }
    }
}

impl InnerAntiProduct<Line> for Line {
    type Output = AntiScalar;

    fn inner_anti_product(self, other: Line) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2] } }
    }
}

impl LeftContraction<Line> for Line {
    type Output = Scalar;

    fn left_contraction(self, other: Line) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group1()[0] * other.group1()[0] - self.group1()[1] * other.group1()[1] - self.group1()[2] * other.group1()[2] } }
    }
}

impl RightContraction<Line> for Line {
    type Output = Scalar;

    fn right_contraction(self, other: Line) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group1()[0] * other.group1()[0] - self.group1()[1] * other.group1()[1] - self.group1()[2] * other.group1()[2] } }
    }
}

impl LeftAntiContraction<Line> for Line {
    type Output = AntiScalar;

    fn left_anti_contraction(self, other: Line) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2] } }
    }
}

impl RightAntiContraction<Line> for Line {
    type Output = AntiScalar;

    fn right_anti_contraction(self, other: Line) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2] } }
    }
}

impl ScalarProduct<Line> for Line {
    type Output = Scalar;

    fn scalar_product(self, other: Line) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group1()[0] * other.group1()[0] - self.group1()[1] * other.group1()[1] - self.group1()[2] * other.group1()[2] } }
    }
}

impl AntiScalarProduct<Line> for Line {
    type Output = AntiScalar;

    fn anti_scalar_product(self, other: Line) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2] } }
    }
}

impl RegressiveProduct<Plane> for Line {
    type Output = Point;

    fn regressive_product(self, other: Plane) -> Point {
        Point { groups: PointGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[1]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, -1.0, 0.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group0(), 1, 0, 1, 1) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) + Simd32x4::from([self.group0()[0], self.group1()[0], self.group1()[0], self.group0()[0]]) * swizzle!(other.group0(), 3, 2, 1, 0) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) } }
    }
}

impl InnerProduct<Plane> for Line {
    type Output = Point;

    fn inner_product(self, other: Plane) -> Point {
        Point { groups: PointGroups { g0: Simd32x4::from(self.group1()[1]) * swizzle!(other.group0(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group0(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[0]) * swizzle!(other.group0(), 3, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]) } }
    }
}

impl GeometricAntiProduct<Plane> for Line {
    type Output = Flector;

    fn geometric_anti_product(self, other: Plane) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[1]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, -1.0, 0.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group0(), 1, 0, 1, 1) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) + Simd32x4::from([self.group0()[0], self.group1()[0], self.group1()[0], self.group0()[0]]) * swizzle!(other.group0(), 3, 2, 1, 0) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]), g1: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, -1.0, 0.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 1, 0, 1, 1) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from([self.group0()[0], self.group0()[0], self.group0()[0], self.group1()[0]]) * swizzle!(other.group0(), 0, 2, 1, 0) * Simd32x4::from([0.0, -1.0, 1.0, -1.0]) } }
    }
}

impl InnerAntiProduct<Plane> for Line {
    type Output = Plane;

    fn inner_anti_product(self, other: Plane) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, -1.0, 0.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 1, 0, 1, 1) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from([self.group0()[0], self.group0()[0], self.group0()[0], self.group1()[0]]) * swizzle!(other.group0(), 0, 2, 1, 0) * Simd32x4::from([0.0, -1.0, 1.0, -1.0]) } }
    }
}

impl LeftContraction<Plane> for Line {
    type Output = Point;

    fn left_contraction(self, other: Plane) -> Point {
        Point { groups: PointGroups { g0: Simd32x4::from(self.group1()[1]) * swizzle!(other.group0(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group0(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[0]) * swizzle!(other.group0(), 3, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]) } }
    }
}

impl RightAntiContraction<Plane> for Line {
    type Output = Plane;

    fn right_anti_contraction(self, other: Plane) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, -1.0, 0.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 1, 0, 1, 1) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from([self.group0()[0], self.group0()[0], self.group0()[0], self.group1()[0]]) * swizzle!(other.group0(), 0, 2, 1, 0) * Simd32x4::from([0.0, -1.0, 1.0, -1.0]) } }
    }
}

impl Add<Motor> for Line {
    type Output = Motor;

    fn add(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group0()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) + other.group0(), g1: self.group1() + other.group1() } }
    }
}

impl Sub<Motor> for Line {
    type Output = Motor;

    fn sub(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group0()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) - other.group0(), g1: self.group1() - other.group1() } }
    }
}

impl OuterProduct<Motor> for Line {
    type Output = AntiScalar;

    fn outer_product(self, other: Motor) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: 0.0 - self.group0()[0] * other.group1()[0] - self.group0()[1] * other.group1()[1] - self.group0()[2] * other.group1()[2] - self.group1()[0] * other.group0()[0] - self.group1()[1] * other.group0()[1] - self.group1()[2] * other.group0()[2] } }
    }
}

impl InnerAntiProduct<Motor> for Line {
    type Output = Motor;

    fn inner_anti_product(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[0]) * swizzle!(other.group0(), 3, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]), g1: self.group1() * Simd32x3::from(other.group0()[3]) } }
    }
}

impl RightContraction<Motor> for Line {
    type Output = Scalar;

    fn right_contraction(self, other: Motor) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group1()[0] * other.group1()[0] - self.group1()[1] * other.group1()[1] - self.group1()[2] * other.group1()[2] } }
    }
}

impl LeftAntiContraction<Motor> for Line {
    type Output = AntiScalar;

    fn left_anti_contraction(self, other: Motor) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2] } }
    }
}

impl RightAntiContraction<Motor> for Line {
    type Output = Motor;

    fn right_anti_contraction(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[0]) * swizzle!(other.group0(), 3, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]), g1: self.group1() * Simd32x3::from(other.group0()[3]) } }
    }
}

impl ScalarProduct<Motor> for Line {
    type Output = Scalar;

    fn scalar_product(self, other: Motor) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group1()[0] * other.group1()[0] - self.group1()[1] * other.group1()[1] - self.group1()[2] * other.group1()[2] } }
    }
}

impl AntiScalarProduct<Motor> for Line {
    type Output = AntiScalar;

    fn anti_scalar_product(self, other: Motor) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2] } }
    }
}

impl Add<Rotor> for Line {
    type Output = Motor;

    fn add(self, other: Rotor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group0()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) + other.group0(), g1: self.group1() } }
    }
}

impl Sub<Rotor> for Line {
    type Output = Motor;

    fn sub(self, other: Rotor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group0()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) - other.group0(), g1: self.group1() } }
    }
}

impl GeometricProduct<Rotor> for Line {
    type Output = Rotor;

    fn geometric_product(self, other: Rotor) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group1()[0]) * swizzle!(other.group0(), 3, 2, 1, 0) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]) + Simd32x4::from(self.group1()[1]) * swizzle!(other.group0(), 2, 3, 0, 1) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group0(), 1, 0, 3, 2) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) } }
    }
}

impl OuterProduct<Rotor> for Line {
    type Output = AntiScalar;

    fn outer_product(self, other: Rotor) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: 0.0 - self.group1()[0] * other.group0()[0] - self.group1()[1] * other.group0()[1] - self.group1()[2] * other.group0()[2] } }
    }
}

impl InnerAntiProduct<Rotor> for Line {
    type Output = Motor;

    fn inner_anti_product(self, other: Rotor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[0]) * swizzle!(other.group0(), 3, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]), g1: self.group1() * Simd32x3::from(other.group0()[3]) } }
    }
}

impl LeftAntiContraction<Rotor> for Line {
    type Output = AntiScalar;

    fn left_anti_contraction(self, other: Rotor) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2] } }
    }
}

impl RightAntiContraction<Rotor> for Line {
    type Output = Motor;

    fn right_anti_contraction(self, other: Rotor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[0]) * swizzle!(other.group0(), 3, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]), g1: self.group1() * Simd32x3::from(other.group0()[3]) } }
    }
}

impl AntiScalarProduct<Rotor> for Line {
    type Output = AntiScalar;

    fn anti_scalar_product(self, other: Rotor) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2] } }
    }
}

impl Add<Translator> for Line {
    type Output = Motor;

    fn add(self, other: Translator) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group0()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) + swizzle!(other.group0(), 0, 0, 0, 3) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]), g1: self.group1() + Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) } }
    }
}

impl Sub<Translator> for Line {
    type Output = Motor;

    fn sub(self, other: Translator) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group0()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) - swizzle!(other.group0(), 0, 0, 0, 3) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]), g1: self.group1() - Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) } }
    }
}

impl OuterProduct<Translator> for Line {
    type Output = AntiScalar;

    fn outer_product(self, other: Translator) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2] } }
    }
}

impl InnerAntiProduct<Translator> for Line {
    type Output = Line;

    fn inner_anti_product(self, other: Translator) -> Line {
        Line { groups: LineGroups { g0: self.group0() * Simd32x3::from(other.group0()[3]), g1: self.group1() * Simd32x3::from(other.group0()[3]) } }
    }
}

impl RightContraction<Translator> for Line {
    type Output = Scalar;

    fn right_contraction(self, other: Translator) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group1()[0] * other.group0()[0] - self.group1()[1] * other.group0()[1] - self.group1()[2] * other.group0()[2] } }
    }
}

impl RightAntiContraction<Translator> for Line {
    type Output = Line;

    fn right_anti_contraction(self, other: Translator) -> Line {
        Line { groups: LineGroups { g0: self.group0() * Simd32x3::from(other.group0()[3]), g1: self.group1() * Simd32x3::from(other.group0()[3]) } }
    }
}

impl ScalarProduct<Translator> for Line {
    type Output = Scalar;

    fn scalar_product(self, other: Translator) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group1()[0] * other.group0()[0] - self.group1()[1] * other.group0()[1] - self.group1()[2] * other.group0()[2] } }
    }
}

impl GeometricProduct<Flector> for Line {
    type Output = Flector;

    fn geometric_product(self, other: Flector) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group1()[3], other.group0()[2], other.group0()[1], other.group1()[0]]) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group0()[2], other.group1()[3], other.group0()[0], other.group1()[1]]) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group0()[1], other.group0()[0], other.group1()[3], other.group1()[2]]) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]), g1: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], other.group1()[3], other.group0()[0], other.group0()[2]]) * Simd32x4::from([1.0, -1.0, -1.0, 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], other.group0()[0], other.group1()[3], other.group0()[1]]) * Simd32x4::from([-1.0, 1.0, -1.0, 0.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group0()[3], other.group1()[2], other.group1()[1], other.group0()[0]]) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group1()[2], other.group0()[3], other.group1()[0], other.group0()[1]]) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group1()[1], other.group1()[0], other.group0()[3], other.group0()[2]]) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group1()[3], other.group0()[2], other.group0()[1], other.group1()[0]]) * Simd32x4::from([-1.0, -1.0, 1.0, 0.0]) } }
    }
}

impl RegressiveProduct<Flector> for Line {
    type Output = Point;

    fn regressive_product(self, other: Flector) -> Point {
        Point { groups: PointGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group1(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group1(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[1]) * swizzle!(other.group1(), 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, -1.0, 0.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group1(), 1, 0, 1, 1) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) + Simd32x4::from([self.group0()[0], self.group1()[0], self.group1()[0], self.group0()[0]]) * swizzle!(other.group1(), 3, 2, 1, 0) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) } }
    }
}

impl OuterProduct<Flector> for Line {
    type Output = Plane;

    fn outer_product(self, other: Flector) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, -1.0, 0.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 1, 0, 1, 1) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[1]) * swizzle!(other.group0(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group0(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from([self.group1()[0], self.group0()[0], self.group0()[0], self.group1()[0]]) * swizzle!(other.group0(), 3, 2, 1, 0) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) } }
    }
}

impl InnerProduct<Flector> for Line {
    type Output = Point;

    fn inner_product(self, other: Flector) -> Point {
        Point { groups: PointGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group1()[3], other.group0()[2], other.group0()[1], other.group1()[0]]) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group0()[2], other.group1()[3], other.group0()[0], other.group1()[1]]) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group0()[1], other.group0()[0], other.group1()[3], other.group1()[2]]) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) } }
    }
}

impl GeometricAntiProduct<Flector> for Line {
    type Output = Flector;

    fn geometric_anti_product(self, other: Flector) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group1()[3], other.group0()[2], other.group0()[1], other.group1()[0]]) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], other.group1()[3], other.group0()[0], other.group1()[1]]) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], other.group0()[0], other.group1()[3], other.group1()[2]]) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group1()[2], other.group0()[3], other.group1()[0], other.group1()[2]]) * Simd32x4::from([1.0, 1.0, -1.0, 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group1()[1], other.group1()[0], other.group0()[3], other.group1()[1]]) * Simd32x4::from([-1.0, 1.0, 1.0, 0.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group0()[3], other.group1()[2], other.group1()[1], other.group0()[0]]) * Simd32x4::from([1.0, -1.0, 1.0, 0.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[3], other.group1()[2], other.group1()[1], other.group0()[0]]) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group1()[2], other.group0()[3], other.group1()[0], other.group0()[1]]) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[1], other.group1()[0], other.group0()[3], other.group0()[2]]) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from(other.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from(other.group1()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from(other.group1()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) } }
    }
}

impl InnerAntiProduct<Flector> for Line {
    type Output = Plane;

    fn inner_anti_product(self, other: Flector) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[3], other.group1()[2], other.group1()[1], other.group0()[0]]) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group1()[2], other.group0()[3], other.group1()[0], other.group0()[1]]) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[1], other.group1()[0], other.group0()[3], other.group0()[2]]) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from(other.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from(other.group1()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from(other.group1()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) } }
    }
}

impl LeftContraction<Flector> for Line {
    type Output = Point;

    fn left_contraction(self, other: Flector) -> Point {
        Point { groups: PointGroups { g0: Simd32x4::from(self.group1()[1]) * swizzle!(other.group1(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group1(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[0]) * swizzle!(other.group1(), 3, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]) } }
    }
}

impl RightContraction<Flector> for Line {
    type Output = Point;

    fn right_contraction(self, other: Flector) -> Point {
        Point { groups: PointGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group1()[1]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([-1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group0(), 1, 0, 1, 1) * Simd32x4::from([1.0, -1.0, 0.0, 0.0]) + Simd32x4::from([self.group0()[0], self.group1()[0], self.group1()[0], self.group0()[0]]) * swizzle!(other.group0(), 0, 2, 1, 0) * Simd32x4::from([0.0, 1.0, -1.0, 1.0]) } }
    }
}

impl LeftAntiContraction<Flector> for Line {
    type Output = Plane;

    fn left_anti_contraction(self, other: Flector) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[0]) * swizzle!(other.group0(), 3, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]) } }
    }
}

impl RightAntiContraction<Flector> for Line {
    type Output = Plane;

    fn right_anti_contraction(self, other: Flector) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group1(), 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, -1.0, 0.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group1(), 1, 0, 1, 1) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from(other.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from(other.group1()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from([self.group0()[0], self.group0()[0], self.group0()[0], self.group1()[0]]) * swizzle!(other.group1(), 0, 2, 1, 0) * Simd32x4::from([0.0, -1.0, 1.0, -1.0]) } }
    }
}

impl ScalarProduct<MultiVector> for Line {
    type Output = Scalar;

    fn scalar_product(self, other: MultiVector) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group1()[0] * other.group3()[0] - self.group1()[1] * other.group3()[1] - self.group1()[2] * other.group3()[2] } }
    }
}

impl AntiScalarProduct<MultiVector> for Line {
    type Output = AntiScalar;

    fn anti_scalar_product(self, other: MultiVector) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: 0.0 - self.group0()[0] * other.group2()[0] - self.group0()[1] * other.group2()[1] - self.group0()[2] * other.group2()[2] } }
    }
}

impl SquaredMagnitude for Line {
    type Output = Scalar;

    fn squared_magnitude(self) -> Scalar {
        self.scalar_product(self.reversal())
    }
}

impl Magnitude for Line {
    type Output = Scalar;

    fn magnitude(self) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.squared_magnitude().group0().sqrt() } }
    }
}

impl BulkNorm for Line {
    type Output = Scalar;

    fn bulk_norm(self) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.squared_magnitude().group0().sqrt() } }
    }
}

impl SquaredAntiMagnitude for Line {
    type Output = AntiScalar;

    fn squared_anti_magnitude(self) -> AntiScalar {
        self.anti_scalar_product(self.anti_reversal())
    }
}

impl WeightNorm for Line {
    type Output = AntiScalar;

    fn weight_norm(self) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.squared_anti_magnitude().group0().sqrt() } }
    }
}

impl GeometricNorm for Line {
    type Output = HomogeneousMagnitude;

    fn geometric_norm(self) -> HomogeneousMagnitude {
        self.bulk_norm() + self.weight_norm()
    }
}

impl Scale for Line {
    type Output = Line;

    fn scale(self, other: f32) -> Line {
        self.geometric_product(Scalar { groups: ScalarGroups { g0: other } })
    }
}

impl Signum for Line {
    type Output = Line;

    fn signum(self) -> Line {
        self.geometric_product(Scalar { groups: ScalarGroups { g0: 1.0 / self.magnitude().group0() } })
    }
}

impl Inverse for Line {
    type Output = Line;

    fn inverse(self) -> Line {
        self.reversal().geometric_product(Scalar { groups: ScalarGroups { g0: 1.0 / self.squared_magnitude().group0() } })
    }
}

impl Unitize for Line {
    type Output = Line;

    fn unitize(self) -> Line {
        self.geometric_product(Scalar { groups: ScalarGroups { g0: 1.0 / self.weight_norm().group0() } })
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

impl Grade for Plane {
    type Output = isize;

    fn grade(self) -> isize {
        3
    }
}

impl AntiGrade for Plane {
    type Output = isize;

    fn anti_grade(self) -> isize {
        1
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
        Plane { groups: PlaneGroups { g0: self.group0() * Simd32x4::from(-1.0) } }
    }
}

impl Reversal for Plane {
    type Output = Plane;

    fn reversal(self) -> Plane {
        Plane { groups: PlaneGroups { g0: self.group0() * Simd32x4::from(-1.0) } }
    }
}

impl Conjugation for Plane {
    type Output = Plane;

    fn conjugation(self) -> Plane {
        Plane { groups: PlaneGroups { g0: self.group0() } }
    }
}

impl Dual for Plane {
    type Output = Point;

    fn dual(self) -> Point {
        Point { groups: PointGroups { g0: self.group0() * Simd32x4::from(-1.0) } }
    }
}

impl AntiReversal for Plane {
    type Output = Plane;

    fn anti_reversal(self) -> Plane {
        Plane { groups: PlaneGroups { g0: self.group0() } }
    }
}

impl RightComplement for Plane {
    type Output = Point;

    fn right_complement(self) -> Point {
        Point { groups: PointGroups { g0: self.group0() * Simd32x4::from(-1.0) } }
    }
}

impl LeftComplement for Plane {
    type Output = Point;

    fn left_complement(self) -> Point {
        Point { groups: PointGroups { g0: self.group0() } }
    }
}

impl DoubleComplement for Plane {
    type Output = Plane;

    fn double_complement(self) -> Plane {
        Plane { groups: PlaneGroups { g0: self.group0() * Simd32x4::from(-1.0) } }
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

impl RegressiveProduct<AntiScalar> for Plane {
    type Output = Plane;

    fn regressive_product(self, other: AntiScalar) -> Plane {
        Plane { groups: PlaneGroups { g0: self.group0() * Simd32x4::from(other.group0()) } }
    }
}

impl GeometricAntiProduct<AntiScalar> for Plane {
    type Output = Plane;

    fn geometric_anti_product(self, other: AntiScalar) -> Plane {
        Plane { groups: PlaneGroups { g0: self.group0() * Simd32x4::from(other.group0()) } }
    }
}

impl InnerAntiProduct<AntiScalar> for Plane {
    type Output = Plane;

    fn inner_anti_product(self, other: AntiScalar) -> Plane {
        Plane { groups: PlaneGroups { g0: self.group0() * Simd32x4::from(other.group0()) } }
    }
}

impl RightAntiContraction<AntiScalar> for Plane {
    type Output = Plane;

    fn right_anti_contraction(self, other: AntiScalar) -> Plane {
        Plane { groups: PlaneGroups { g0: self.group0() * Simd32x4::from(other.group0()) } }
    }
}

impl RegressiveProduct<HomogeneousMagnitude> for Plane {
    type Output = Plane;

    fn regressive_product(self, other: HomogeneousMagnitude) -> Plane {
        Plane { groups: PlaneGroups { g0: self.group0() * Simd32x4::from(other.group0()[1]) } }
    }
}

impl OuterProduct<HomogeneousMagnitude> for Plane {
    type Output = Plane;

    fn outer_product(self, other: HomogeneousMagnitude) -> Plane {
        Plane { groups: PlaneGroups { g0: self.group0() * Simd32x4::from(other.group0()[0]) } }
    }
}

impl RightContraction<HomogeneousMagnitude> for Plane {
    type Output = Plane;

    fn right_contraction(self, other: HomogeneousMagnitude) -> Plane {
        Plane { groups: PlaneGroups { g0: self.group0() * Simd32x4::from(other.group0()[0]) } }
    }
}

impl RightAntiContraction<HomogeneousMagnitude> for Plane {
    type Output = Plane;

    fn right_anti_contraction(self, other: HomogeneousMagnitude) -> Plane {
        Plane { groups: PlaneGroups { g0: self.group0() * Simd32x4::from(other.group0()[1]) } }
    }
}

impl Add<Point> for Plane {
    type Output = Flector;

    fn add(self, other: Point) -> Flector {
        Flector { groups: FlectorGroups { g0: other.group0(), g1: self.group0() } }
    }
}

impl Sub<Point> for Plane {
    type Output = Flector;

    fn sub(self, other: Point) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(0.0) - other.group0(), g1: self.group0() } }
    }
}

impl GeometricProduct<Point> for Plane {
    type Output = Motor;

    fn geometric_product(self, other: Point) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 2, 2, 0, 1) * Simd32x4::from([-1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 1, 0, 1, 2) * Simd32x4::from([1.0, -1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from(other.group0()[3]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[0]) * swizzle!(other.group0(), 0, 2, 1, 0) * Simd32x4::from([0.0, 1.0, -1.0, -1.0]), g1: Simd32x3::from(0.0) - Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) } }
    }
}

impl RegressiveProduct<Point> for Plane {
    type Output = Scalar;

    fn regressive_product(self, other: Point) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2] - self.group0()[3] * other.group0()[3] } }
    }
}

impl OuterProduct<Point> for Plane {
    type Output = AntiScalar;

    fn outer_product(self, other: Point) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2] - self.group0()[3] * other.group0()[3] } }
    }
}

impl InnerProduct<Point> for Plane {
    type Output = Line;

    fn inner_product(self, other: Point) -> Line {
        Line { groups: LineGroups { g0: Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], other.group0()[2], other.group0()[0]]) * Simd32x3::from([-1.0, 0.0, 1.0]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], other.group0()[0], other.group0()[1]]) * Simd32x3::from([1.0, -1.0, 0.0]) + Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[0], other.group0()[2], other.group0()[1]]) * Simd32x3::from([0.0, 1.0, -1.0]), g1: Simd32x3::from(0.0) - Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) } }
    }
}

impl InnerAntiProduct<Point> for Plane {
    type Output = Line;

    fn inner_anti_product(self, other: Point) -> Line {
        Line { groups: LineGroups { g0: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()[3]) * Simd32x3::from(-1.0), g1: Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], other.group0()[2], other.group0()[0]]) * Simd32x3::from([1.0, 0.0, -1.0]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], other.group0()[0], other.group0()[1]]) * Simd32x3::from([-1.0, 1.0, 0.0]) + Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[0], other.group0()[2], other.group0()[1]]) * Simd32x3::from([0.0, -1.0, 1.0]) } }
    }
}

impl RightContraction<Point> for Plane {
    type Output = Line;

    fn right_contraction(self, other: Point) -> Line {
        Line { groups: LineGroups { g0: Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], other.group0()[2], other.group0()[0]]) * Simd32x3::from([-1.0, 0.0, 1.0]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], other.group0()[0], other.group0()[1]]) * Simd32x3::from([1.0, -1.0, 0.0]) + Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[0], other.group0()[2], other.group0()[1]]) * Simd32x3::from([0.0, 1.0, -1.0]), g1: Simd32x3::from(0.0) - Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) } }
    }
}

impl LeftAntiContraction<Point> for Plane {
    type Output = Line;

    fn left_anti_contraction(self, other: Point) -> Line {
        Line { groups: LineGroups { g0: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()[3]) * Simd32x3::from(-1.0), g1: Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], other.group0()[2], other.group0()[0]]) * Simd32x3::from([1.0, 0.0, -1.0]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], other.group0()[0], other.group0()[1]]) * Simd32x3::from([-1.0, 1.0, 0.0]) + Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[0], other.group0()[2], other.group0()[1]]) * Simd32x3::from([0.0, -1.0, 1.0]) } }
    }
}

impl RegressiveProduct<Line> for Plane {
    type Output = Point;

    fn regressive_product(self, other: Line) -> Point {
        Point { groups: PointGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group1()[2], other.group1()[2], other.group1()[0], other.group0()[1]]) * Simd32x4::from([-1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[1], other.group1()[0], other.group1()[1], other.group0()[2]]) * Simd32x4::from([1.0, -1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group0()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group1()[0], other.group1()[2], other.group1()[1], other.group0()[0]]) * Simd32x4::from([0.0, 1.0, -1.0, -1.0]) } }
    }
}

impl InnerProduct<Line> for Plane {
    type Output = Point;

    fn inner_product(self, other: Line) -> Point {
        Point { groups: PointGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group1()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + swizzle!(self.group0(), 3, 3, 3, 0) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group1()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) } }
    }
}

impl GeometricAntiProduct<Line> for Plane {
    type Output = Flector;

    fn geometric_anti_product(self, other: Line) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group1()[2], other.group1()[2], other.group1()[0], other.group0()[1]]) * Simd32x4::from([-1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[1], other.group1()[0], other.group1()[1], other.group0()[2]]) * Simd32x4::from([1.0, -1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group0()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group1()[0], other.group1()[2], other.group1()[1], other.group0()[0]]) * Simd32x4::from([0.0, 1.0, -1.0, -1.0]), g1: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], other.group0()[2], other.group0()[0], other.group1()[1]]) * Simd32x4::from([1.0, 0.0, -1.0, 1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], other.group0()[0], other.group0()[1], other.group1()[2]]) * Simd32x4::from([-1.0, 1.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[0], other.group0()[2], other.group0()[1], other.group1()[0]]) * Simd32x4::from([0.0, -1.0, 1.0, 1.0]) } }
    }
}

impl InnerAntiProduct<Line> for Plane {
    type Output = Plane;

    fn inner_anti_product(self, other: Line) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], other.group0()[2], other.group0()[0], other.group1()[1]]) * Simd32x4::from([1.0, 0.0, -1.0, 1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], other.group0()[0], other.group0()[1], other.group1()[2]]) * Simd32x4::from([-1.0, 1.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[0], other.group0()[2], other.group0()[1], other.group1()[0]]) * Simd32x4::from([0.0, -1.0, 1.0, 1.0]) } }
    }
}

impl RightContraction<Line> for Plane {
    type Output = Point;

    fn right_contraction(self, other: Line) -> Point {
        Point { groups: PointGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group1()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + swizzle!(self.group0(), 3, 3, 3, 0) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group1()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) } }
    }
}

impl LeftAntiContraction<Line> for Plane {
    type Output = Plane;

    fn left_anti_contraction(self, other: Line) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], other.group0()[2], other.group0()[0], other.group1()[1]]) * Simd32x4::from([1.0, 0.0, -1.0, 1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], other.group0()[0], other.group0()[1], other.group1()[2]]) * Simd32x4::from([-1.0, 1.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[0], other.group0()[2], other.group0()[1], other.group1()[0]]) * Simd32x4::from([0.0, -1.0, 1.0, 1.0]) } }
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

impl RegressiveProduct<Plane> for Plane {
    type Output = Line;

    fn regressive_product(self, other: Plane) -> Line {
        Line { groups: LineGroups { g0: Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], other.group0()[2], other.group0()[0]]) * Simd32x3::from([-1.0, 0.0, 1.0]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], other.group0()[0], other.group0()[1]]) * Simd32x3::from([1.0, -1.0, 0.0]) + Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[0], other.group0()[2], other.group0()[1]]) * Simd32x3::from([0.0, 1.0, -1.0]), g1: Simd32x3::from(0.0) - Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) + Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()[3]) } }
    }
}

impl InnerProduct<Plane> for Plane {
    type Output = Scalar;

    fn inner_product(self, other: Plane) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group0()[3] * other.group0()[3] } }
    }
}

impl GeometricAntiProduct<Plane> for Plane {
    type Output = Motor;

    fn geometric_anti_product(self, other: Plane) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 2, 2, 0, 1) * Simd32x4::from([-1.0, 0.0, 1.0, 1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 1, 0, 1, 2) * Simd32x4::from([1.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[0]) * swizzle!(other.group0(), 0, 2, 1, 0) * Simd32x4::from([0.0, 1.0, -1.0, 1.0]), g1: Simd32x3::from(0.0) - Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) + Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()[3]) } }
    }
}

impl InnerAntiProduct<Plane> for Plane {
    type Output = AntiScalar;

    fn inner_anti_product(self, other: Plane) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2] } }
    }
}

impl LeftContraction<Plane> for Plane {
    type Output = Scalar;

    fn left_contraction(self, other: Plane) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group0()[3] * other.group0()[3] } }
    }
}

impl RightContraction<Plane> for Plane {
    type Output = Scalar;

    fn right_contraction(self, other: Plane) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group0()[3] * other.group0()[3] } }
    }
}

impl LeftAntiContraction<Plane> for Plane {
    type Output = AntiScalar;

    fn left_anti_contraction(self, other: Plane) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2] } }
    }
}

impl RightAntiContraction<Plane> for Plane {
    type Output = AntiScalar;

    fn right_anti_contraction(self, other: Plane) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2] } }
    }
}

impl ScalarProduct<Plane> for Plane {
    type Output = Scalar;

    fn scalar_product(self, other: Plane) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group0()[3] * other.group0()[3] } }
    }
}

impl AntiScalarProduct<Plane> for Plane {
    type Output = AntiScalar;

    fn anti_scalar_product(self, other: Plane) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2] } }
    }
}

impl RegressiveProduct<Motor> for Plane {
    type Output = Flector;

    fn regressive_product(self, other: Motor) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group1()[2], other.group1()[2], other.group1()[0], other.group0()[1]]) * Simd32x4::from([-1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[1], other.group1()[0], other.group1()[1], other.group0()[2]]) * Simd32x4::from([1.0, -1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group0(), 0, 1, 2, 0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group1()[0], other.group1()[2], other.group1()[1], other.group0()[0]]) * Simd32x4::from([0.0, 1.0, -1.0, -1.0]), g1: self.group0() * Simd32x4::from(other.group0()[3]) } }
    }
}

impl InnerProduct<Motor> for Plane {
    type Output = Point;

    fn inner_product(self, other: Motor) -> Point {
        Point { groups: PointGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group1()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group0()[3]]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from(other.group1()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) } }
    }
}

impl GeometricAntiProduct<Motor> for Plane {
    type Output = Flector;

    fn geometric_anti_product(self, other: Motor) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group1()[2], other.group1()[2], other.group1()[0], other.group0()[1]]) * Simd32x4::from([-1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[1], other.group1()[0], other.group1()[1], other.group0()[2]]) * Simd32x4::from([1.0, -1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group0(), 0, 1, 2, 0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group1()[0], other.group1()[2], other.group1()[1], other.group0()[0]]) * Simd32x4::from([0.0, 1.0, -1.0, -1.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[3], other.group0()[2], other.group0()[1], other.group1()[0]]) * Simd32x4::from([1.0, -1.0, 1.0, 1.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], other.group0()[3], other.group0()[0], other.group1()[1]]) * Simd32x4::from([1.0, 1.0, -1.0, 1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], other.group0()[0], other.group0()[3], other.group1()[2]]) * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]) + swizzle!(self.group0(), 0, 0, 0, 3) * swizzle!(other.group0(), 0, 0, 0, 3) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) } }
    }
}

impl InnerAntiProduct<Motor> for Plane {
    type Output = Plane;

    fn inner_anti_product(self, other: Motor) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[3], other.group0()[2], other.group0()[1], other.group1()[0]]) * Simd32x4::from([1.0, -1.0, 1.0, 1.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], other.group0()[3], other.group0()[0], other.group1()[1]]) * Simd32x4::from([1.0, 1.0, -1.0, 1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], other.group0()[0], other.group0()[3], other.group1()[2]]) * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]) + swizzle!(self.group0(), 0, 0, 0, 3) * swizzle!(other.group0(), 0, 0, 0, 3) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) } }
    }
}

impl RightContraction<Motor> for Plane {
    type Output = Point;

    fn right_contraction(self, other: Motor) -> Point {
        Point { groups: PointGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group1()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + swizzle!(self.group0(), 3, 3, 3, 0) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group1()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) } }
    }
}

impl LeftAntiContraction<Motor> for Plane {
    type Output = Plane;

    fn left_anti_contraction(self, other: Motor) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], other.group0()[2], other.group0()[0], other.group1()[1]]) * Simd32x4::from([1.0, 0.0, -1.0, 1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], other.group0()[0], other.group0()[1], other.group1()[2]]) * Simd32x4::from([-1.0, 1.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[0], other.group0()[2], other.group0()[1], other.group1()[0]]) * Simd32x4::from([0.0, -1.0, 1.0, 1.0]) } }
    }
}

impl RightAntiContraction<Motor> for Plane {
    type Output = Plane;

    fn right_anti_contraction(self, other: Motor) -> Plane {
        Plane { groups: PlaneGroups { g0: self.group0() * Simd32x4::from(other.group0()[3]) } }
    }
}

impl RegressiveProduct<Rotor> for Plane {
    type Output = Flector;

    fn regressive_product(self, other: Rotor) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + swizzle!(self.group0(), 3, 3, 3, 0) * swizzle!(other.group0(), 0, 1, 2, 0) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]), g1: self.group0() * Simd32x4::from(other.group0()[3]) } }
    }
}

impl GeometricAntiProduct<Rotor> for Plane {
    type Output = Flector;

    fn geometric_anti_product(self, other: Rotor) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + swizzle!(self.group0(), 3, 3, 3, 0) * swizzle!(other.group0(), 0, 1, 2, 0) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]), g1: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 2, 3, 0, 2) * Simd32x4::from([1.0, 1.0, -1.0, 0.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 1, 0, 3, 1) * Simd32x4::from([-1.0, 1.0, 1.0, 0.0]) + swizzle!(self.group0(), 0, 0, 0, 3) * swizzle!(other.group0(), 3, 2, 1, 3) * Simd32x4::from([1.0, -1.0, 1.0, 1.0]) } }
    }
}

impl InnerAntiProduct<Rotor> for Plane {
    type Output = Plane;

    fn inner_anti_product(self, other: Rotor) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 2, 3, 0, 2) * Simd32x4::from([1.0, 1.0, -1.0, 0.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 1, 0, 3, 1) * Simd32x4::from([-1.0, 1.0, 1.0, 0.0]) + swizzle!(self.group0(), 0, 0, 0, 3) * swizzle!(other.group0(), 3, 2, 1, 3) * Simd32x4::from([1.0, -1.0, 1.0, 1.0]) } }
    }
}

impl RightAntiContraction<Rotor> for Plane {
    type Output = Plane;

    fn right_anti_contraction(self, other: Rotor) -> Plane {
        Plane { groups: PlaneGroups { g0: self.group0() * Simd32x4::from(other.group0()[3]) } }
    }
}

impl InnerProduct<Translator> for Plane {
    type Output = Point;

    fn inner_product(self, other: Translator) -> Point {
        Point { groups: PointGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[3]) * other.group0() + Simd32x4::from(self.group0()[0]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) } }
    }
}

impl InnerAntiProduct<Translator> for Plane {
    type Output = Plane;

    fn inner_anti_product(self, other: Translator) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, 1.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from(other.group0()[3]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[0]) * swizzle!(other.group0(), 3, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, 1.0]) } }
    }
}

impl RightContraction<Translator> for Plane {
    type Output = Point;

    fn right_contraction(self, other: Translator) -> Point {
        Point { groups: PointGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + swizzle!(self.group0(), 3, 3, 3, 0) * swizzle!(other.group0(), 0, 1, 2, 0) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) } }
    }
}

impl RightAntiContraction<Translator> for Plane {
    type Output = Plane;

    fn right_anti_contraction(self, other: Translator) -> Plane {
        Plane { groups: PlaneGroups { g0: self.group0() * Simd32x4::from(other.group0()[3]) } }
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

impl OuterProduct<Flector> for Plane {
    type Output = AntiScalar;

    fn outer_product(self, other: Flector) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2] - self.group0()[3] * other.group0()[3] } }
    }
}

impl InnerAntiProduct<Flector> for Plane {
    type Output = Motor;

    fn inner_anti_product(self, other: Flector) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[3], other.group0()[3], other.group0()[3], other.group1()[1]]) * Simd32x4::from([0.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[3], other.group0()[3], other.group0()[3], other.group1()[2]]) * Simd32x4::from([0.0, 0.0, -1.0, 1.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[3], other.group0()[0], other.group0()[0], other.group1()[0]]) * Simd32x4::from([-1.0, 0.0, 0.0, 1.0]), g1: Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], other.group0()[2], other.group0()[0]]) * Simd32x3::from([1.0, 0.0, -1.0]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], other.group0()[0], other.group0()[1]]) * Simd32x3::from([-1.0, 1.0, 0.0]) + Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[0], other.group0()[2], other.group0()[1]]) * Simd32x3::from([0.0, -1.0, 1.0]) } }
    }
}

impl LeftContraction<Flector> for Plane {
    type Output = Scalar;

    fn left_contraction(self, other: Flector) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group0()[3] * other.group1()[3] } }
    }
}

impl LeftAntiContraction<Flector> for Plane {
    type Output = Motor;

    fn left_anti_contraction(self, other: Flector) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[3], other.group0()[3], other.group0()[3], other.group1()[1]]) * Simd32x4::from([0.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[3], other.group0()[3], other.group0()[3], other.group1()[2]]) * Simd32x4::from([0.0, 0.0, -1.0, 1.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[3], other.group0()[0], other.group0()[0], other.group1()[0]]) * Simd32x4::from([-1.0, 0.0, 0.0, 1.0]), g1: Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], other.group0()[2], other.group0()[0]]) * Simd32x3::from([1.0, 0.0, -1.0]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], other.group0()[0], other.group0()[1]]) * Simd32x3::from([-1.0, 1.0, 0.0]) + Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[0], other.group0()[2], other.group0()[1]]) * Simd32x3::from([0.0, -1.0, 1.0]) } }
    }
}

impl RightAntiContraction<Flector> for Plane {
    type Output = AntiScalar;

    fn right_anti_contraction(self, other: Flector) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0()[0] * other.group1()[0] + self.group0()[1] * other.group1()[1] + self.group0()[2] * other.group1()[2] } }
    }
}

impl ScalarProduct<Flector> for Plane {
    type Output = Scalar;

    fn scalar_product(self, other: Flector) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group0()[3] * other.group1()[3] } }
    }
}

impl AntiScalarProduct<Flector> for Plane {
    type Output = AntiScalar;

    fn anti_scalar_product(self, other: Flector) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0()[0] * other.group1()[0] + self.group0()[1] * other.group1()[1] + self.group0()[2] * other.group1()[2] } }
    }
}

impl ScalarProduct<MultiVector> for Plane {
    type Output = Scalar;

    fn scalar_product(self, other: MultiVector) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group0()[3] * other.group6()[3] } }
    }
}

impl AntiScalarProduct<MultiVector> for Plane {
    type Output = AntiScalar;

    fn anti_scalar_product(self, other: MultiVector) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0()[0] * other.group6()[0] + self.group0()[1] * other.group6()[1] + self.group0()[2] * other.group6()[2] } }
    }
}

impl SquaredMagnitude for Plane {
    type Output = Scalar;

    fn squared_magnitude(self) -> Scalar {
        self.scalar_product(self.reversal())
    }
}

impl Magnitude for Plane {
    type Output = Scalar;

    fn magnitude(self) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.squared_magnitude().group0().sqrt() } }
    }
}

impl BulkNorm for Plane {
    type Output = Scalar;

    fn bulk_norm(self) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.squared_magnitude().group0().sqrt() } }
    }
}

impl SquaredAntiMagnitude for Plane {
    type Output = AntiScalar;

    fn squared_anti_magnitude(self) -> AntiScalar {
        self.anti_scalar_product(self.anti_reversal())
    }
}

impl WeightNorm for Plane {
    type Output = AntiScalar;

    fn weight_norm(self) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.squared_anti_magnitude().group0().sqrt() } }
    }
}

impl GeometricNorm for Plane {
    type Output = HomogeneousMagnitude;

    fn geometric_norm(self) -> HomogeneousMagnitude {
        self.bulk_norm() + self.weight_norm()
    }
}

impl Scale for Plane {
    type Output = Plane;

    fn scale(self, other: f32) -> Plane {
        self.geometric_product(Scalar { groups: ScalarGroups { g0: other } })
    }
}

impl Signum for Plane {
    type Output = Plane;

    fn signum(self) -> Plane {
        self.geometric_product(Scalar { groups: ScalarGroups { g0: 1.0 / self.magnitude().group0() } })
    }
}

impl Inverse for Plane {
    type Output = Plane;

    fn inverse(self) -> Plane {
        self.reversal().geometric_product(Scalar { groups: ScalarGroups { g0: 1.0 / self.squared_magnitude().group0() } })
    }
}

impl Unitize for Plane {
    type Output = Plane;

    fn unitize(self) -> Plane {
        self.geometric_product(Scalar { groups: ScalarGroups { g0: 1.0 / self.weight_norm().group0() } })
    }
}

impl Zero for Motor {
    fn zero() -> Self {
        Motor { groups: MotorGroups { g0: Simd32x4::from(0.0), g1: Simd32x3::from(0.0) } }
    }
}

impl One for Motor {
    fn one() -> Self {
        Motor { groups: MotorGroups { g0: Simd32x4::from(0.0), g1: Simd32x3::from(0.0) } }
    }
}

impl Neg for Motor {
    type Output = Motor;

    fn neg(self) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() * Simd32x4::from(-1.0), g1: self.group1() * Simd32x3::from(-1.0) } }
    }
}

impl Automorphism for Motor {
    type Output = Motor;

    fn automorphism(self) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0(), g1: self.group1() } }
    }
}

impl Reversal for Motor {
    type Output = Motor;

    fn reversal(self) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]), g1: self.group1() * Simd32x3::from(-1.0) } }
    }
}

impl Conjugation for Motor {
    type Output = Motor;

    fn conjugation(self) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]), g1: self.group1() * Simd32x3::from(-1.0) } }
    }
}

impl AntiReversal for Motor {
    type Output = Motor;

    fn anti_reversal(self) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]), g1: self.group1() * Simd32x3::from(-1.0) } }
    }
}

impl DoubleComplement for Motor {
    type Output = Motor;

    fn double_complement(self) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0(), g1: self.group1() } }
    }
}

impl GeometricProduct<Scalar> for Motor {
    type Output = Motor;

    fn geometric_product(self, other: Scalar) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() * Simd32x4::from(other.group0()), g1: self.group1() * Simd32x3::from(other.group0()) } }
    }
}

impl RegressiveProduct<Scalar> for Motor {
    type Output = Scalar;

    fn regressive_product(self, other: Scalar) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[3] * other.group0() } }
    }
}

impl OuterProduct<Scalar> for Motor {
    type Output = Motor;

    fn outer_product(self, other: Scalar) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() * Simd32x4::from(other.group0()), g1: self.group1() * Simd32x3::from(other.group0()) } }
    }
}

impl InnerProduct<Scalar> for Motor {
    type Output = Motor;

    fn inner_product(self, other: Scalar) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() * Simd32x4::from(other.group0()), g1: self.group1() * Simd32x3::from(other.group0()) } }
    }
}

impl RightContraction<Scalar> for Motor {
    type Output = Motor;

    fn right_contraction(self, other: Scalar) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() * Simd32x4::from(other.group0()), g1: self.group1() * Simd32x3::from(other.group0()) } }
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

impl RegressiveProduct<AntiScalar> for Motor {
    type Output = Motor;

    fn regressive_product(self, other: AntiScalar) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() * Simd32x4::from(other.group0()), g1: self.group1() * Simd32x3::from(other.group0()) } }
    }
}

impl GeometricAntiProduct<AntiScalar> for Motor {
    type Output = Motor;

    fn geometric_anti_product(self, other: AntiScalar) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() * Simd32x4::from(other.group0()), g1: self.group1() * Simd32x3::from(other.group0()) } }
    }
}

impl InnerAntiProduct<AntiScalar> for Motor {
    type Output = Motor;

    fn inner_anti_product(self, other: AntiScalar) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() * Simd32x4::from(other.group0()), g1: self.group1() * Simd32x3::from(other.group0()) } }
    }
}

impl LeftAntiContraction<AntiScalar> for Motor {
    type Output = AntiScalar;

    fn left_anti_contraction(self, other: AntiScalar) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0()[3] * other.group0() } }
    }
}

impl RightAntiContraction<AntiScalar> for Motor {
    type Output = Motor;

    fn right_anti_contraction(self, other: AntiScalar) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() * Simd32x4::from(other.group0()), g1: self.group1() * Simd32x3::from(other.group0()) } }
    }
}

impl AntiScalarProduct<AntiScalar> for Motor {
    type Output = AntiScalar;

    fn anti_scalar_product(self, other: AntiScalar) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0()[3] * other.group0() } }
    }
}

impl GeometricProduct<HomogeneousMagnitude> for Motor {
    type Output = Motor;

    fn geometric_product(self, other: HomogeneousMagnitude) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group1()[0]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 1.0, 0.0]) + self.group0() * Simd32x4::from(other.group0()[0]), g1: self.group1() * Simd32x3::from(other.group0()[0]) } }
    }
}

impl OuterProduct<HomogeneousMagnitude> for Motor {
    type Output = Motor;

    fn outer_product(self, other: HomogeneousMagnitude) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() * Simd32x4::from(other.group0()[0]), g1: self.group1() * Simd32x3::from(other.group0()[0]) } }
    }
}

impl InnerProduct<HomogeneousMagnitude> for Motor {
    type Output = Motor;

    fn inner_product(self, other: HomogeneousMagnitude) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group1()[0]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 1.0, 0.0]) + self.group0() * Simd32x4::from(other.group0()[0]), g1: self.group1() * Simd32x3::from(other.group0()[0]) } }
    }
}

impl RightContraction<HomogeneousMagnitude> for Motor {
    type Output = Motor;

    fn right_contraction(self, other: HomogeneousMagnitude) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() * Simd32x4::from(other.group0()[0]), g1: self.group1() * Simd32x3::from(other.group0()[0]) } }
    }
}

impl RightAntiContraction<HomogeneousMagnitude> for Motor {
    type Output = Motor;

    fn right_anti_contraction(self, other: HomogeneousMagnitude) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() * Simd32x4::from(other.group0()[1]), g1: self.group1() * Simd32x3::from(other.group0()[1]) } }
    }
}

impl AntiScalarProduct<HomogeneousMagnitude> for Motor {
    type Output = AntiScalar;

    fn anti_scalar_product(self, other: HomogeneousMagnitude) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0()[3] * other.group0()[1] } }
    }
}

impl GeometricProduct<Point> for Motor {
    type Output = Flector;

    fn geometric_product(self, other: Point) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group1()[0]) * swizzle!(other.group0(), 2, 2, 1, 2) * Simd32x4::from([0.0, 1.0, -1.0, 0.0]) + Simd32x4::from(self.group1()[1]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([-1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group0(), 1, 0, 1, 1) * Simd32x4::from([1.0, -1.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]), g1: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, -1.0, 0.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 1, 0, 1, 1) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group0(), 0, 1, 2, 0) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]) + Simd32x4::from(self.group1()[0]) * swizzle!(other.group0(), 3, 3, 3, 0) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[1]) * swizzle!(other.group0(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group0(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[0]) * swizzle!(other.group0(), 0, 2, 1, 0) * Simd32x4::from([0.0, -1.0, 1.0, 0.0]) } }
    }
}

impl RegressiveProduct<Point> for Motor {
    type Output = Point;

    fn regressive_product(self, other: Point) -> Point {
        Point { groups: PointGroups { g0: Simd32x4::from(self.group0()[3]) * other.group0() } }
    }
}

impl OuterProduct<Point> for Motor {
    type Output = Plane;

    fn outer_product(self, other: Point) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, -1.0, 0.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 1, 0, 1, 1) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[0]) * swizzle!(other.group0(), 3, 3, 3, 0) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[1]) * swizzle!(other.group0(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group0(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[0]) * swizzle!(other.group0(), 0, 2, 1, 0) * Simd32x4::from([0.0, -1.0, 1.0, 0.0]) } }
    }
}

impl GeometricAntiProduct<Point> for Motor {
    type Output = Flector;

    fn geometric_anti_product(self, other: Point) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, -1.0, 0.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 1, 0, 1, 1) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[3]) * other.group0() + Simd32x4::from(self.group1()[0]) * Simd32x4::from(other.group0()[3]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from(other.group0()[3]) * Simd32x4::from([0.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from(other.group0()[3]) * Simd32x4::from([0.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group0()[0]) * swizzle!(other.group0(), 0, 2, 1, 0) * Simd32x4::from([0.0, -1.0, 1.0, 0.0]), g1: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[0]) * swizzle!(other.group0(), 3, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]) } }
    }
}

impl InnerAntiProduct<Point> for Motor {
    type Output = Flector;

    fn inner_anti_product(self, other: Point) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[3]) * other.group0(), g1: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[0]) * swizzle!(other.group0(), 3, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]) } }
    }
}

impl LeftAntiContraction<Point> for Motor {
    type Output = Flector;

    fn left_anti_contraction(self, other: Point) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[3]) * other.group0(), g1: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[0]) * swizzle!(other.group0(), 3, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]) } }
    }
}

impl Into<Line> for Motor {
    fn into(self) -> Line {
        Line { groups: LineGroups { g0: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]), g1: self.group1() } }
    }
}

impl Add<Line> for Motor {
    type Output = Motor;

    fn add(self, other: Line) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() + Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group0()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]), g1: self.group1() + other.group1() } }
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
        Motor { groups: MotorGroups { g0: self.group0() - Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group0()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]), g1: self.group1() - other.group1() } }
    }
}

impl SubAssign<Line> for Motor {
    fn sub_assign(&mut self, other: Line) {
        *self = (*self).sub(other);
    }
}

impl OuterProduct<Line> for Motor {
    type Output = AntiScalar;

    fn outer_product(self, other: Line) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: 0.0 - self.group0()[0] * other.group1()[0] - self.group0()[1] * other.group1()[1] - self.group0()[2] * other.group1()[2] - self.group1()[0] * other.group0()[0] - self.group1()[1] * other.group0()[1] - self.group1()[2] * other.group0()[2] } }
    }
}

impl InnerAntiProduct<Line> for Motor {
    type Output = Motor;

    fn inner_anti_product(self, other: Line) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + swizzle!(self.group0(), 3, 3, 3, 0) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group0()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]), g1: Simd32x3::from(self.group0()[3]) * other.group1() } }
    }
}

impl LeftContraction<Line> for Motor {
    type Output = Scalar;

    fn left_contraction(self, other: Line) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group1()[0] * other.group1()[0] - self.group1()[1] * other.group1()[1] - self.group1()[2] * other.group1()[2] } }
    }
}

impl LeftAntiContraction<Line> for Motor {
    type Output = Motor;

    fn left_anti_contraction(self, other: Line) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + swizzle!(self.group0(), 3, 3, 3, 0) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group0()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]), g1: Simd32x3::from(self.group0()[3]) * other.group1() } }
    }
}

impl RightAntiContraction<Line> for Motor {
    type Output = AntiScalar;

    fn right_anti_contraction(self, other: Line) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2] } }
    }
}

impl ScalarProduct<Line> for Motor {
    type Output = Scalar;

    fn scalar_product(self, other: Line) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group1()[0] * other.group1()[0] - self.group1()[1] * other.group1()[1] - self.group1()[2] * other.group1()[2] } }
    }
}

impl AntiScalarProduct<Line> for Motor {
    type Output = AntiScalar;

    fn anti_scalar_product(self, other: Line) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2] } }
    }
}

impl RegressiveProduct<Plane> for Motor {
    type Output = Flector;

    fn regressive_product(self, other: Plane) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[0]) * swizzle!(other.group0(), 2, 2, 1, 2) * Simd32x4::from([0.0, -1.0, 1.0, 0.0]) + Simd32x4::from(self.group1()[1]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, -1.0, 0.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group0(), 1, 0, 1, 1) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[0]) * swizzle!(other.group0(), 3, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]), g1: Simd32x4::from(self.group0()[3]) * other.group0() } }
    }
}

impl InnerProduct<Plane> for Motor {
    type Output = Point;

    fn inner_product(self, other: Plane) -> Point {
        Point { groups: PointGroups { g0: Simd32x4::from(self.group1()[0]) * swizzle!(other.group0(), 3, 3, 3, 0) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[1]) * swizzle!(other.group0(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group0(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + swizzle!(self.group0(), 0, 0, 0, 3) * swizzle!(other.group0(), 0, 0, 0, 3) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) } }
    }
}

impl GeometricAntiProduct<Plane> for Motor {
    type Output = Flector;

    fn geometric_anti_product(self, other: Plane) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[0]) * swizzle!(other.group0(), 2, 2, 1, 2) * Simd32x4::from([0.0, -1.0, 1.0, 0.0]) + Simd32x4::from(self.group1()[1]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, -1.0, 0.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group0(), 1, 0, 1, 1) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[0]) * swizzle!(other.group0(), 3, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]), g1: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, -1.0, 0.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 1, 0, 1, 1) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[3]) * other.group0() + Simd32x4::from(self.group1()[0]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[0]) * swizzle!(other.group0(), 0, 2, 1, 0) * Simd32x4::from([0.0, -1.0, 1.0, 0.0]) } }
    }
}

impl InnerAntiProduct<Plane> for Motor {
    type Output = Plane;

    fn inner_anti_product(self, other: Plane) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, -1.0, 0.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 1, 0, 1, 1) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[3]) * other.group0() + Simd32x4::from(self.group1()[0]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[0]) * swizzle!(other.group0(), 0, 2, 1, 0) * Simd32x4::from([0.0, -1.0, 1.0, 0.0]) } }
    }
}

impl LeftContraction<Plane> for Motor {
    type Output = Point;

    fn left_contraction(self, other: Plane) -> Point {
        Point { groups: PointGroups { g0: Simd32x4::from(self.group1()[1]) * swizzle!(other.group0(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group0(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[0]) * swizzle!(other.group0(), 3, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]) } }
    }
}

impl LeftAntiContraction<Plane> for Motor {
    type Output = Plane;

    fn left_anti_contraction(self, other: Plane) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()[3]) * other.group0() } }
    }
}

impl RightAntiContraction<Plane> for Motor {
    type Output = Plane;

    fn right_anti_contraction(self, other: Plane) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, -1.0, 0.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 1, 0, 1, 1) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[0]) * swizzle!(other.group0(), 0, 2, 1, 0) * Simd32x4::from([0.0, -1.0, 1.0, 0.0]) } }
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
        Motor { groups: MotorGroups { g0: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group0()[3]]) * Simd32x4::from([1.0, 1.0, 1.0, 1.0]) / Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group0()[3]]) * Simd32x4::from([1.0, 1.0, 1.0, 1.0]), g1: Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]) * Simd32x3::from([1.0, 1.0, 1.0]) / Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x3::from([1.0, 1.0, 1.0]) } }
    }
}

impl DivAssign<Motor> for Motor {
    fn div_assign(&mut self, other: Motor) {
        *self = (*self).div(other);
    }
}

impl OuterProduct<Motor> for Motor {
    type Output = AntiScalar;

    fn outer_product(self, other: Motor) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: 0.0 - self.group0()[0] * other.group1()[0] - self.group0()[1] * other.group1()[1] - self.group0()[2] * other.group1()[2] - self.group1()[0] * other.group0()[0] - self.group1()[1] * other.group0()[1] - self.group1()[2] * other.group0()[2] } }
    }
}

impl InnerAntiProduct<Motor> for Motor {
    type Output = Motor;

    fn inner_anti_product(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[3]) * other.group0() + Simd32x4::from(self.group0()[0]) * swizzle!(other.group0(), 3, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]), g1: Simd32x3::from(self.group0()[3]) * other.group1() + self.group1() * Simd32x3::from(other.group0()[3]) } }
    }
}

impl LeftAntiContraction<Motor> for Motor {
    type Output = Motor;

    fn left_anti_contraction(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[3]) * other.group0() + Simd32x4::from(self.group0()[0]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]), g1: Simd32x3::from(self.group0()[3]) * other.group1() } }
    }
}

impl RightAntiContraction<Motor> for Motor {
    type Output = Motor;

    fn right_anti_contraction(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from(other.group0()[3]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[0]) * swizzle!(other.group0(), 3, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]), g1: self.group1() * Simd32x3::from(other.group0()[3]) } }
    }
}

impl ScalarProduct<Motor> for Motor {
    type Output = Scalar;

    fn scalar_product(self, other: Motor) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group1()[0] * other.group1()[0] - self.group1()[1] * other.group1()[1] - self.group1()[2] * other.group1()[2] } }
    }
}

impl AntiScalarProduct<Motor> for Motor {
    type Output = AntiScalar;

    fn anti_scalar_product(self, other: Motor) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2] + self.group0()[3] * other.group0()[3] } }
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

impl GeometricProduct<Rotor> for Motor {
    type Output = Rotor;

    fn geometric_product(self, other: Rotor) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group1()[0]) * swizzle!(other.group0(), 3, 2, 1, 0) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]) + Simd32x4::from(self.group1()[1]) * swizzle!(other.group0(), 2, 3, 0, 1) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group0(), 1, 0, 3, 2) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) } }
    }
}

impl OuterProduct<Rotor> for Motor {
    type Output = AntiScalar;

    fn outer_product(self, other: Rotor) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: 0.0 - self.group1()[0] * other.group0()[0] - self.group1()[1] * other.group0()[1] - self.group1()[2] * other.group0()[2] } }
    }
}

impl InnerAntiProduct<Rotor> for Motor {
    type Output = Motor;

    fn inner_anti_product(self, other: Rotor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[3]) * other.group0() + Simd32x4::from(self.group0()[0]) * swizzle!(other.group0(), 3, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]), g1: self.group1() * Simd32x3::from(other.group0()[3]) } }
    }
}

impl LeftAntiContraction<Rotor> for Motor {
    type Output = Rotor;

    fn left_anti_contraction(self, other: Rotor) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[3]) * other.group0() + Simd32x4::from(self.group0()[0]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) } }
    }
}

impl RightAntiContraction<Rotor> for Motor {
    type Output = Motor;

    fn right_anti_contraction(self, other: Rotor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from(other.group0()[3]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[0]) * swizzle!(other.group0(), 3, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]), g1: self.group1() * Simd32x3::from(other.group0()[3]) } }
    }
}

impl AntiScalarProduct<Rotor> for Motor {
    type Output = AntiScalar;

    fn anti_scalar_product(self, other: Rotor) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2] + self.group0()[3] * other.group0()[3] } }
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
        Motor { groups: MotorGroups { g0: self.group0() + swizzle!(other.group0(), 0, 0, 0, 3) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]), g1: self.group1() + Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) } }
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
        Motor { groups: MotorGroups { g0: self.group0() - swizzle!(other.group0(), 0, 0, 0, 3) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]), g1: self.group1() - Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) } }
    }
}

impl SubAssign<Translator> for Motor {
    fn sub_assign(&mut self, other: Translator) {
        *self = (*self).sub(other);
    }
}

impl OuterProduct<Translator> for Motor {
    type Output = AntiScalar;

    fn outer_product(self, other: Translator) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2] } }
    }
}

impl InnerAntiProduct<Translator> for Motor {
    type Output = Motor;

    fn inner_anti_product(self, other: Translator) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() * Simd32x4::from(other.group0()[3]), g1: Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) + self.group1() * Simd32x3::from(other.group0()[3]) } }
    }
}

impl LeftAntiContraction<Translator> for Motor {
    type Output = Translator;

    fn left_anti_contraction(self, other: Translator) -> Translator {
        Translator { groups: TranslatorGroups { g0: Simd32x4::from(self.group0()[3]) * other.group0() } }
    }
}

impl RightAntiContraction<Translator> for Motor {
    type Output = Motor;

    fn right_anti_contraction(self, other: Translator) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() * Simd32x4::from(other.group0()[3]), g1: self.group1() * Simd32x3::from(other.group0()[3]) } }
    }
}

impl ScalarProduct<Translator> for Motor {
    type Output = Scalar;

    fn scalar_product(self, other: Translator) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group1()[0] * other.group0()[0] - self.group1()[1] * other.group0()[1] - self.group1()[2] * other.group0()[2] } }
    }
}

impl AntiScalarProduct<Translator> for Motor {
    type Output = AntiScalar;

    fn anti_scalar_product(self, other: Translator) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0()[3] * other.group0()[3] } }
    }
}

impl GeometricProduct<Flector> for Motor {
    type Output = Flector;

    fn geometric_product(self, other: Flector) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from(other.group1()[3]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group1()[3], other.group0()[2], other.group0()[1], other.group1()[0]]) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group0()[2], other.group1()[3], other.group0()[0], other.group1()[1]]) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group0()[1], other.group0()[0], other.group1()[3], other.group1()[2]]) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]), g1: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], other.group1()[3], other.group0()[0], other.group0()[2]]) * Simd32x4::from([1.0, -1.0, -1.0, 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], other.group0()[0], other.group1()[3], other.group0()[1]]) * Simd32x4::from([-1.0, 1.0, -1.0, 0.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group0(), 0, 1, 2, 0) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group0()[3], other.group1()[2], other.group1()[1], other.group0()[0]]) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group1()[2], other.group0()[3], other.group1()[0], other.group0()[1]]) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group1()[1], other.group1()[0], other.group0()[3], other.group0()[2]]) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group1()[3], other.group0()[2], other.group0()[1], other.group1()[0]]) * Simd32x4::from([-1.0, -1.0, 1.0, 0.0]) } }
    }
}

impl RegressiveProduct<Flector> for Motor {
    type Output = Flector;

    fn regressive_product(self, other: Flector) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group1(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group1(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[3]) * other.group0() + Simd32x4::from(self.group1()[0]) * swizzle!(other.group1(), 2, 2, 1, 2) * Simd32x4::from([0.0, -1.0, 1.0, 0.0]) + Simd32x4::from(self.group1()[1]) * swizzle!(other.group1(), 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, -1.0, 0.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group1(), 1, 0, 1, 1) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[0]) * swizzle!(other.group1(), 3, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]), g1: Simd32x4::from(self.group0()[3]) * other.group1() } }
    }
}

impl OuterProduct<Flector> for Motor {
    type Output = Plane;

    fn outer_product(self, other: Flector) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, -1.0, 0.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 1, 0, 1, 1) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[0]) * swizzle!(other.group0(), 3, 3, 3, 0) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[1]) * swizzle!(other.group0(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group0(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[0]) * swizzle!(other.group0(), 0, 2, 1, 0) * Simd32x4::from([0.0, -1.0, 1.0, 0.0]) } }
    }
}

impl GeometricAntiProduct<Flector> for Motor {
    type Output = Flector;

    fn geometric_anti_product(self, other: Flector) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group1()[3], other.group0()[2], other.group0()[1], other.group1()[0]]) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], other.group1()[3], other.group0()[0], other.group1()[1]]) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], other.group0()[0], other.group1()[3], other.group1()[2]]) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[3]) * other.group0() + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group1()[2], other.group0()[3], other.group1()[0], other.group1()[2]]) * Simd32x4::from([1.0, 1.0, -1.0, 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group1()[1], other.group1()[0], other.group0()[3], other.group1()[1]]) * Simd32x4::from([-1.0, 1.0, 1.0, 0.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group0()[3], other.group1()[2], other.group1()[1], other.group0()[0]]) * Simd32x4::from([1.0, -1.0, 1.0, 0.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[3], other.group1()[2], other.group1()[1], other.group0()[0]]) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group1()[2], other.group0()[3], other.group1()[0], other.group0()[1]]) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[1], other.group1()[0], other.group0()[3], other.group0()[2]]) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[3]) * other.group1() + Simd32x4::from(self.group1()[1]) * Simd32x4::from(other.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from(other.group1()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from(other.group1()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) } }
    }
}

impl InnerAntiProduct<Flector> for Motor {
    type Output = Flector;

    fn inner_anti_product(self, other: Flector) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[3]) * other.group0(), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[3], other.group1()[2], other.group1()[1], other.group0()[0]]) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group1()[2], other.group0()[3], other.group1()[0], other.group0()[1]]) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[1], other.group1()[0], other.group0()[3], other.group0()[2]]) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[3]) * other.group1() + Simd32x4::from(self.group1()[1]) * Simd32x4::from(other.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from(other.group1()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from(other.group1()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) } }
    }
}

impl LeftContraction<Flector> for Motor {
    type Output = Point;

    fn left_contraction(self, other: Flector) -> Point {
        Point { groups: PointGroups { g0: Simd32x4::from(self.group1()[1]) * swizzle!(other.group1(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group1(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[0]) * swizzle!(other.group1(), 3, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]) } }
    }
}

impl LeftAntiContraction<Flector> for Motor {
    type Output = Flector;

    fn left_anti_contraction(self, other: Flector) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[3]) * other.group0(), g1: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[3]) * other.group1() + Simd32x4::from(self.group0()[0]) * swizzle!(other.group0(), 3, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]) } }
    }
}

impl RightAntiContraction<Flector> for Motor {
    type Output = Plane;

    fn right_anti_contraction(self, other: Flector) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group1(), 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, -1.0, 0.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group1(), 1, 0, 1, 1) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from(other.group1()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from(other.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from(other.group1()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[0]) * swizzle!(other.group1(), 0, 2, 1, 0) * Simd32x4::from([0.0, -1.0, 1.0, 0.0]) } }
    }
}

impl ScalarProduct<MultiVector> for Motor {
    type Output = Scalar;

    fn scalar_product(self, other: MultiVector) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group1()[0] * other.group3()[0] - self.group1()[1] * other.group3()[1] - self.group1()[2] * other.group3()[2] } }
    }
}

impl AntiScalarProduct<MultiVector> for Motor {
    type Output = AntiScalar;

    fn anti_scalar_product(self, other: MultiVector) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: 0.0 - self.group0()[0] * other.group2()[0] - self.group0()[1] * other.group2()[1] - self.group0()[2] * other.group2()[2] + self.group0()[3] * other.group0()[1] } }
    }
}

impl SquaredMagnitude for Motor {
    type Output = Scalar;

    fn squared_magnitude(self) -> Scalar {
        self.scalar_product(self.reversal())
    }
}

impl Magnitude for Motor {
    type Output = Scalar;

    fn magnitude(self) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.squared_magnitude().group0().sqrt() } }
    }
}

impl BulkNorm for Motor {
    type Output = Scalar;

    fn bulk_norm(self) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.squared_magnitude().group0().sqrt() } }
    }
}

impl SquaredAntiMagnitude for Motor {
    type Output = AntiScalar;

    fn squared_anti_magnitude(self) -> AntiScalar {
        self.anti_scalar_product(self.anti_reversal())
    }
}

impl WeightNorm for Motor {
    type Output = AntiScalar;

    fn weight_norm(self) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.squared_anti_magnitude().group0().sqrt() } }
    }
}

impl GeometricNorm for Motor {
    type Output = HomogeneousMagnitude;

    fn geometric_norm(self) -> HomogeneousMagnitude {
        self.bulk_norm() + self.weight_norm()
    }
}

impl Scale for Motor {
    type Output = Motor;

    fn scale(self, other: f32) -> Motor {
        self.geometric_product(Scalar { groups: ScalarGroups { g0: other } })
    }
}

impl Signum for Motor {
    type Output = Motor;

    fn signum(self) -> Motor {
        self.geometric_product(Scalar { groups: ScalarGroups { g0: 1.0 / self.magnitude().group0() } })
    }
}

impl Inverse for Motor {
    type Output = Motor;

    fn inverse(self) -> Motor {
        self.reversal().geometric_product(Scalar { groups: ScalarGroups { g0: 1.0 / self.squared_magnitude().group0() } })
    }
}

impl Unitize for Motor {
    type Output = Motor;

    fn unitize(self) -> Motor {
        self.geometric_product(Scalar { groups: ScalarGroups { g0: 1.0 / self.weight_norm().group0() } })
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
        Rotor { groups: RotorGroups { g0: self.group0() } }
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
        Rotor { groups: RotorGroups { g0: self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]) } }
    }
}

impl AntiReversal for Rotor {
    type Output = Rotor;

    fn anti_reversal(self) -> Rotor {
        Rotor { groups: RotorGroups { g0: self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]) } }
    }
}

impl DoubleComplement for Rotor {
    type Output = Rotor;

    fn double_complement(self) -> Rotor {
        Rotor { groups: RotorGroups { g0: self.group0() } }
    }
}

impl GeometricProduct<Scalar> for Rotor {
    type Output = Rotor;

    fn geometric_product(self, other: Scalar) -> Rotor {
        Rotor { groups: RotorGroups { g0: self.group0() * Simd32x4::from(other.group0()) } }
    }
}

impl RegressiveProduct<Scalar> for Rotor {
    type Output = Scalar;

    fn regressive_product(self, other: Scalar) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[3] * other.group0() } }
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

impl RegressiveProduct<AntiScalar> for Rotor {
    type Output = Rotor;

    fn regressive_product(self, other: AntiScalar) -> Rotor {
        Rotor { groups: RotorGroups { g0: self.group0() * Simd32x4::from(other.group0()) } }
    }
}

impl GeometricAntiProduct<AntiScalar> for Rotor {
    type Output = Rotor;

    fn geometric_anti_product(self, other: AntiScalar) -> Rotor {
        Rotor { groups: RotorGroups { g0: self.group0() * Simd32x4::from(other.group0()) } }
    }
}

impl InnerAntiProduct<AntiScalar> for Rotor {
    type Output = Rotor;

    fn inner_anti_product(self, other: AntiScalar) -> Rotor {
        Rotor { groups: RotorGroups { g0: self.group0() * Simd32x4::from(other.group0()) } }
    }
}

impl LeftAntiContraction<AntiScalar> for Rotor {
    type Output = AntiScalar;

    fn left_anti_contraction(self, other: AntiScalar) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0()[3] * other.group0() } }
    }
}

impl RightAntiContraction<AntiScalar> for Rotor {
    type Output = Rotor;

    fn right_anti_contraction(self, other: AntiScalar) -> Rotor {
        Rotor { groups: RotorGroups { g0: self.group0() * Simd32x4::from(other.group0()) } }
    }
}

impl AntiScalarProduct<AntiScalar> for Rotor {
    type Output = AntiScalar;

    fn anti_scalar_product(self, other: AntiScalar) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0()[3] * other.group0() } }
    }
}

impl GeometricProduct<HomogeneousMagnitude> for Rotor {
    type Output = Rotor;

    fn geometric_product(self, other: HomogeneousMagnitude) -> Rotor {
        Rotor { groups: RotorGroups { g0: self.group0() * Simd32x4::from(other.group0()[0]) } }
    }
}

impl OuterProduct<HomogeneousMagnitude> for Rotor {
    type Output = Rotor;

    fn outer_product(self, other: HomogeneousMagnitude) -> Rotor {
        Rotor { groups: RotorGroups { g0: self.group0() * Simd32x4::from(other.group0()[0]) } }
    }
}

impl InnerProduct<HomogeneousMagnitude> for Rotor {
    type Output = Rotor;

    fn inner_product(self, other: HomogeneousMagnitude) -> Rotor {
        Rotor { groups: RotorGroups { g0: self.group0() * Simd32x4::from(other.group0()[0]) } }
    }
}

impl RightContraction<HomogeneousMagnitude> for Rotor {
    type Output = Rotor;

    fn right_contraction(self, other: HomogeneousMagnitude) -> Rotor {
        Rotor { groups: RotorGroups { g0: self.group0() * Simd32x4::from(other.group0()[0]) } }
    }
}

impl RightAntiContraction<HomogeneousMagnitude> for Rotor {
    type Output = Rotor;

    fn right_anti_contraction(self, other: HomogeneousMagnitude) -> Rotor {
        Rotor { groups: RotorGroups { g0: self.group0() * Simd32x4::from(other.group0()[1]) } }
    }
}

impl AntiScalarProduct<HomogeneousMagnitude> for Rotor {
    type Output = AntiScalar;

    fn anti_scalar_product(self, other: HomogeneousMagnitude) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0()[3] * other.group0()[1] } }
    }
}

impl RegressiveProduct<Point> for Rotor {
    type Output = Point;

    fn regressive_product(self, other: Point) -> Point {
        Point { groups: PointGroups { g0: Simd32x4::from(self.group0()[3]) * other.group0() } }
    }
}

impl GeometricAntiProduct<Point> for Rotor {
    type Output = Flector;

    fn geometric_anti_product(self, other: Point) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, -1.0, 0.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 1, 0, 1, 1) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[3]) * other.group0() + Simd32x4::from(self.group0()[0]) * swizzle!(other.group0(), 0, 2, 1, 0) * Simd32x4::from([0.0, -1.0, 1.0, 0.0]), g1: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[0]) * swizzle!(other.group0(), 3, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]) } }
    }
}

impl InnerAntiProduct<Point> for Rotor {
    type Output = Flector;

    fn inner_anti_product(self, other: Point) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[3]) * other.group0(), g1: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[0]) * swizzle!(other.group0(), 3, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]) } }
    }
}

impl LeftAntiContraction<Point> for Rotor {
    type Output = Flector;

    fn left_anti_contraction(self, other: Point) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[3]) * other.group0(), g1: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[0]) * swizzle!(other.group0(), 3, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]) } }
    }
}

impl Add<Line> for Rotor {
    type Output = Motor;

    fn add(self, other: Line) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() + Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group0()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]), g1: other.group1() } }
    }
}

impl Sub<Line> for Rotor {
    type Output = Motor;

    fn sub(self, other: Line) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() - Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group0()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]), g1: Simd32x3::from(0.0) - other.group1() } }
    }
}

impl GeometricProduct<Line> for Rotor {
    type Output = Rotor;

    fn geometric_product(self, other: Line) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group1()[2], other.group1()[2], other.group1()[0], other.group1()[1]]) * Simd32x4::from([-1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[1], other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x4::from([1.0, -1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group1()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group1()[0], other.group1()[2], other.group1()[1], other.group1()[0]]) * Simd32x4::from([0.0, 1.0, -1.0, -1.0]) } }
    }
}

impl OuterProduct<Line> for Rotor {
    type Output = AntiScalar;

    fn outer_product(self, other: Line) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: 0.0 - self.group0()[0] * other.group1()[0] - self.group0()[1] * other.group1()[1] - self.group0()[2] * other.group1()[2] } }
    }
}

impl InnerAntiProduct<Line> for Rotor {
    type Output = Motor;

    fn inner_anti_product(self, other: Line) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + swizzle!(self.group0(), 3, 3, 3, 0) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group0()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]), g1: Simd32x3::from(self.group0()[3]) * other.group1() } }
    }
}

impl LeftAntiContraction<Line> for Rotor {
    type Output = Motor;

    fn left_anti_contraction(self, other: Line) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + swizzle!(self.group0(), 3, 3, 3, 0) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group0()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]), g1: Simd32x3::from(self.group0()[3]) * other.group1() } }
    }
}

impl RightAntiContraction<Line> for Rotor {
    type Output = AntiScalar;

    fn right_anti_contraction(self, other: Line) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2] } }
    }
}

impl AntiScalarProduct<Line> for Rotor {
    type Output = AntiScalar;

    fn anti_scalar_product(self, other: Line) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2] } }
    }
}

impl RegressiveProduct<Plane> for Rotor {
    type Output = Flector;

    fn regressive_product(self, other: Plane) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[0]) * swizzle!(other.group0(), 3, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]), g1: Simd32x4::from(self.group0()[3]) * other.group0() } }
    }
}

impl GeometricAntiProduct<Plane> for Rotor {
    type Output = Flector;

    fn geometric_anti_product(self, other: Plane) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[0]) * swizzle!(other.group0(), 3, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]), g1: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, -1.0, 0.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 1, 0, 1, 1) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[3]) * other.group0() + Simd32x4::from(self.group0()[0]) * swizzle!(other.group0(), 0, 2, 1, 0) * Simd32x4::from([0.0, -1.0, 1.0, 0.0]) } }
    }
}

impl InnerAntiProduct<Plane> for Rotor {
    type Output = Plane;

    fn inner_anti_product(self, other: Plane) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, -1.0, 0.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 1, 0, 1, 1) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[3]) * other.group0() + Simd32x4::from(self.group0()[0]) * swizzle!(other.group0(), 0, 2, 1, 0) * Simd32x4::from([0.0, -1.0, 1.0, 0.0]) } }
    }
}

impl LeftAntiContraction<Plane> for Rotor {
    type Output = Plane;

    fn left_anti_contraction(self, other: Plane) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()[3]) * other.group0() } }
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
        Motor { groups: MotorGroups { g0: self.group0() - other.group0(), g1: Simd32x3::from(0.0) - other.group1() } }
    }
}

impl GeometricProduct<Motor> for Rotor {
    type Output = Rotor;

    fn geometric_product(self, other: Motor) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group1()[2], other.group1()[2], other.group1()[0], other.group1()[1]]) * Simd32x4::from([-1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[1], other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x4::from([1.0, -1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group1()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group1()[0], other.group1()[2], other.group1()[1], other.group1()[0]]) * Simd32x4::from([0.0, 1.0, -1.0, -1.0]) } }
    }
}

impl OuterProduct<Motor> for Rotor {
    type Output = AntiScalar;

    fn outer_product(self, other: Motor) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: 0.0 - self.group0()[0] * other.group1()[0] - self.group0()[1] * other.group1()[1] - self.group0()[2] * other.group1()[2] } }
    }
}

impl InnerAntiProduct<Motor> for Rotor {
    type Output = Motor;

    fn inner_anti_product(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[3]) * other.group0() + Simd32x4::from(self.group0()[0]) * swizzle!(other.group0(), 3, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]), g1: Simd32x3::from(self.group0()[3]) * other.group1() } }
    }
}

impl LeftAntiContraction<Motor> for Rotor {
    type Output = Motor;

    fn left_anti_contraction(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[3]) * other.group0() + Simd32x4::from(self.group0()[0]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]), g1: Simd32x3::from(self.group0()[3]) * other.group1() } }
    }
}

impl RightAntiContraction<Motor> for Rotor {
    type Output = Rotor;

    fn right_anti_contraction(self, other: Motor) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from(other.group0()[3]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[0]) * swizzle!(other.group0(), 3, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]) } }
    }
}

impl AntiScalarProduct<Motor> for Rotor {
    type Output = AntiScalar;

    fn anti_scalar_product(self, other: Motor) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2] + self.group0()[3] * other.group0()[3] } }
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

impl RegressiveProduct<Rotor> for Rotor {
    type Output = Rotor;

    fn regressive_product(self, other: Rotor) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group0()[3]) * other.group0() + swizzle!(self.group0(), 0, 1, 2, 0) * swizzle!(other.group0(), 3, 3, 3, 0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) } }
    }
}

impl GeometricAntiProduct<Rotor> for Rotor {
    type Output = Rotor;

    fn geometric_anti_product(self, other: Rotor) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group0()[0]) * swizzle!(other.group0(), 3, 2, 1, 0) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 2, 3, 0, 1) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 1, 0, 3, 2) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[3]) * other.group0() } }
    }
}

impl InnerAntiProduct<Rotor> for Rotor {
    type Output = Rotor;

    fn inner_anti_product(self, other: Rotor) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[3]) * other.group0() + Simd32x4::from(self.group0()[0]) * swizzle!(other.group0(), 3, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]) } }
    }
}

impl LeftAntiContraction<Rotor> for Rotor {
    type Output = Rotor;

    fn left_anti_contraction(self, other: Rotor) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[3]) * other.group0() + Simd32x4::from(self.group0()[0]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) } }
    }
}

impl RightAntiContraction<Rotor> for Rotor {
    type Output = Rotor;

    fn right_anti_contraction(self, other: Rotor) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from(other.group0()[3]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[0]) * swizzle!(other.group0(), 3, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]) } }
    }
}

impl AntiScalarProduct<Rotor> for Rotor {
    type Output = AntiScalar;

    fn anti_scalar_product(self, other: Rotor) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2] + self.group0()[3] * other.group0()[3] } }
    }
}

impl Add<Translator> for Rotor {
    type Output = Motor;

    fn add(self, other: Translator) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() + swizzle!(other.group0(), 0, 0, 0, 3) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]), g1: Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) } }
    }
}

impl Sub<Translator> for Rotor {
    type Output = Motor;

    fn sub(self, other: Translator) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() - swizzle!(other.group0(), 0, 0, 0, 3) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]), g1: Simd32x3::from(0.0) - Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) } }
    }
}

impl GeometricProduct<Translator> for Rotor {
    type Output = Rotor;

    fn geometric_product(self, other: Translator) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 2, 2, 0, 1) * Simd32x4::from([-1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 1, 0, 1, 2) * Simd32x4::from([1.0, -1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group0(), 0, 1, 2, 0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) + Simd32x4::from(self.group0()[0]) * swizzle!(other.group0(), 0, 2, 1, 0) * Simd32x4::from([0.0, 1.0, -1.0, -1.0]) } }
    }
}

impl OuterProduct<Translator> for Rotor {
    type Output = AntiScalar;

    fn outer_product(self, other: Translator) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2] } }
    }
}

impl InnerAntiProduct<Translator> for Rotor {
    type Output = Motor;

    fn inner_anti_product(self, other: Translator) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() * Simd32x4::from(other.group0()[3]), g1: Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) } }
    }
}

impl LeftAntiContraction<Translator> for Rotor {
    type Output = Translator;

    fn left_anti_contraction(self, other: Translator) -> Translator {
        Translator { groups: TranslatorGroups { g0: Simd32x4::from(self.group0()[3]) * other.group0() } }
    }
}

impl RightAntiContraction<Translator> for Rotor {
    type Output = Rotor;

    fn right_anti_contraction(self, other: Translator) -> Rotor {
        Rotor { groups: RotorGroups { g0: self.group0() * Simd32x4::from(other.group0()[3]) } }
    }
}

impl AntiScalarProduct<Translator> for Rotor {
    type Output = AntiScalar;

    fn anti_scalar_product(self, other: Translator) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0()[3] * other.group0()[3] } }
    }
}

impl RegressiveProduct<Flector> for Rotor {
    type Output = Flector;

    fn regressive_product(self, other: Flector) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group1(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group1(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[3]) * other.group0() + Simd32x4::from(self.group0()[0]) * swizzle!(other.group1(), 3, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]), g1: Simd32x4::from(self.group0()[3]) * other.group1() } }
    }
}

impl GeometricAntiProduct<Flector> for Rotor {
    type Output = Flector;

    fn geometric_anti_product(self, other: Flector) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group1()[3], other.group0()[2], other.group0()[1], other.group1()[0]]) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], other.group1()[3], other.group0()[0], other.group1()[1]]) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], other.group0()[0], other.group1()[3], other.group1()[2]]) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[3]) * other.group0(), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[3], other.group1()[2], other.group1()[1], other.group0()[0]]) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group1()[2], other.group0()[3], other.group1()[0], other.group0()[1]]) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[1], other.group1()[0], other.group0()[3], other.group0()[2]]) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[3]) * other.group1() } }
    }
}

impl InnerAntiProduct<Flector> for Rotor {
    type Output = Flector;

    fn inner_anti_product(self, other: Flector) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[3]) * other.group0(), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[3], other.group1()[2], other.group1()[1], other.group0()[0]]) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group1()[2], other.group0()[3], other.group1()[0], other.group0()[1]]) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[1], other.group1()[0], other.group0()[3], other.group0()[2]]) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[3]) * other.group1() } }
    }
}

impl LeftAntiContraction<Flector> for Rotor {
    type Output = Flector;

    fn left_anti_contraction(self, other: Flector) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[3]) * other.group0(), g1: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[3]) * other.group1() + Simd32x4::from(self.group0()[0]) * swizzle!(other.group0(), 3, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]) } }
    }
}

impl AntiScalarProduct<MultiVector> for Rotor {
    type Output = AntiScalar;

    fn anti_scalar_product(self, other: MultiVector) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: 0.0 - self.group0()[0] * other.group2()[0] - self.group0()[1] * other.group2()[1] - self.group0()[2] * other.group2()[2] + self.group0()[3] * other.group0()[1] } }
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
        Translator { groups: TranslatorGroups { g0: self.group0() } }
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
        Translator { groups: TranslatorGroups { g0: self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]) } }
    }
}

impl AntiReversal for Translator {
    type Output = Translator;

    fn anti_reversal(self) -> Translator {
        Translator { groups: TranslatorGroups { g0: self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]) } }
    }
}

impl DoubleComplement for Translator {
    type Output = Translator;

    fn double_complement(self) -> Translator {
        Translator { groups: TranslatorGroups { g0: self.group0() } }
    }
}

impl GeometricProduct<Scalar> for Translator {
    type Output = Translator;

    fn geometric_product(self, other: Scalar) -> Translator {
        Translator { groups: TranslatorGroups { g0: self.group0() * Simd32x4::from(other.group0()) } }
    }
}

impl RegressiveProduct<Scalar> for Translator {
    type Output = Scalar;

    fn regressive_product(self, other: Scalar) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[3] * other.group0() } }
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

impl GeometricAntiProduct<Scalar> for Translator {
    type Output = Scalar;

    fn geometric_anti_product(self, other: Scalar) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[3] * other.group0() } }
    }
}

impl InnerAntiProduct<Scalar> for Translator {
    type Output = Scalar;

    fn inner_anti_product(self, other: Scalar) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[3] * other.group0() } }
    }
}

impl RightContraction<Scalar> for Translator {
    type Output = Translator;

    fn right_contraction(self, other: Scalar) -> Translator {
        Translator { groups: TranslatorGroups { g0: self.group0() * Simd32x4::from(other.group0()) } }
    }
}

impl LeftAntiContraction<Scalar> for Translator {
    type Output = Scalar;

    fn left_anti_contraction(self, other: Scalar) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[3] * other.group0() } }
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

impl RegressiveProduct<AntiScalar> for Translator {
    type Output = Translator;

    fn regressive_product(self, other: AntiScalar) -> Translator {
        Translator { groups: TranslatorGroups { g0: self.group0() * Simd32x4::from(other.group0()) } }
    }
}

impl GeometricAntiProduct<AntiScalar> for Translator {
    type Output = Translator;

    fn geometric_anti_product(self, other: AntiScalar) -> Translator {
        Translator { groups: TranslatorGroups { g0: self.group0() * Simd32x4::from(other.group0()) } }
    }
}

impl InnerAntiProduct<AntiScalar> for Translator {
    type Output = Translator;

    fn inner_anti_product(self, other: AntiScalar) -> Translator {
        Translator { groups: TranslatorGroups { g0: self.group0() * Simd32x4::from(other.group0()) } }
    }
}

impl LeftAntiContraction<AntiScalar> for Translator {
    type Output = AntiScalar;

    fn left_anti_contraction(self, other: AntiScalar) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0()[3] * other.group0() } }
    }
}

impl RightAntiContraction<AntiScalar> for Translator {
    type Output = Translator;

    fn right_anti_contraction(self, other: AntiScalar) -> Translator {
        Translator { groups: TranslatorGroups { g0: self.group0() * Simd32x4::from(other.group0()) } }
    }
}

impl AntiScalarProduct<AntiScalar> for Translator {
    type Output = AntiScalar;

    fn anti_scalar_product(self, other: AntiScalar) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0()[3] * other.group0() } }
    }
}

impl GeometricProduct<HomogeneousMagnitude> for Translator {
    type Output = Motor;

    fn geometric_product(self, other: HomogeneousMagnitude) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() * Simd32x4::from([other.group0()[1], other.group0()[1], other.group0()[1], other.group0()[0]]), g1: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()[0]) } }
    }
}

impl OuterProduct<HomogeneousMagnitude> for Translator {
    type Output = Translator;

    fn outer_product(self, other: HomogeneousMagnitude) -> Translator {
        Translator { groups: TranslatorGroups { g0: self.group0() * Simd32x4::from(other.group0()[0]) } }
    }
}

impl InnerProduct<HomogeneousMagnitude> for Translator {
    type Output = Motor;

    fn inner_product(self, other: HomogeneousMagnitude) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() * Simd32x4::from([other.group0()[1], other.group0()[1], other.group0()[1], other.group0()[0]]), g1: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()[0]) } }
    }
}

impl RightContraction<HomogeneousMagnitude> for Translator {
    type Output = Translator;

    fn right_contraction(self, other: HomogeneousMagnitude) -> Translator {
        Translator { groups: TranslatorGroups { g0: self.group0() * Simd32x4::from(other.group0()[0]) } }
    }
}

impl LeftAntiContraction<HomogeneousMagnitude> for Translator {
    type Output = HomogeneousMagnitude;

    fn left_anti_contraction(self, other: HomogeneousMagnitude) -> HomogeneousMagnitude {
        HomogeneousMagnitude { groups: HomogeneousMagnitudeGroups { g0: Simd32x2::from(self.group0()[3]) * other.group0() } }
    }
}

impl RightAntiContraction<HomogeneousMagnitude> for Translator {
    type Output = Translator;

    fn right_anti_contraction(self, other: HomogeneousMagnitude) -> Translator {
        Translator { groups: TranslatorGroups { g0: self.group0() * Simd32x4::from(other.group0()[1]) } }
    }
}

impl AntiScalarProduct<HomogeneousMagnitude> for Translator {
    type Output = AntiScalar;

    fn anti_scalar_product(self, other: HomogeneousMagnitude) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0()[3] * other.group0()[1] } }
    }
}

impl RegressiveProduct<Point> for Translator {
    type Output = Point;

    fn regressive_product(self, other: Point) -> Point {
        Point { groups: PointGroups { g0: Simd32x4::from(self.group0()[3]) * other.group0() } }
    }
}

impl OuterProduct<Point> for Translator {
    type Output = Plane;

    fn outer_product(self, other: Point) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[0]) * swizzle!(other.group0(), 3, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]) } }
    }
}

impl GeometricAntiProduct<Point> for Translator {
    type Output = Point;

    fn geometric_anti_product(self, other: Point) -> Point {
        Point { groups: PointGroups { g0: Simd32x4::from(self.group0()[3]) * other.group0() + swizzle!(self.group0(), 0, 1, 2, 0) * swizzle!(other.group0(), 3, 3, 3, 0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) } }
    }
}

impl InnerAntiProduct<Point> for Translator {
    type Output = Point;

    fn inner_anti_product(self, other: Point) -> Point {
        Point { groups: PointGroups { g0: Simd32x4::from(self.group0()[3]) * other.group0() } }
    }
}

impl LeftAntiContraction<Point> for Translator {
    type Output = Point;

    fn left_anti_contraction(self, other: Point) -> Point {
        Point { groups: PointGroups { g0: Simd32x4::from(self.group0()[3]) * other.group0() } }
    }
}

impl Add<Line> for Translator {
    type Output = Motor;

    fn add(self, other: Line) -> Motor {
        Motor { groups: MotorGroups { g0: swizzle!(self.group0(), 0, 0, 0, 3) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group0()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]), g1: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) + other.group1() } }
    }
}

impl Sub<Line> for Translator {
    type Output = Motor;

    fn sub(self, other: Line) -> Motor {
        Motor { groups: MotorGroups { g0: swizzle!(self.group0(), 0, 0, 0, 3) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) - Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group0()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]), g1: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) - other.group1() } }
    }
}

impl OuterProduct<Line> for Translator {
    type Output = AntiScalar;

    fn outer_product(self, other: Line) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2] } }
    }
}

impl InnerAntiProduct<Line> for Translator {
    type Output = Line;

    fn inner_anti_product(self, other: Line) -> Line {
        Line { groups: LineGroups { g0: Simd32x3::from(self.group0()[3]) * other.group0(), g1: Simd32x3::from(self.group0()[3]) * other.group1() } }
    }
}

impl LeftContraction<Line> for Translator {
    type Output = Scalar;

    fn left_contraction(self, other: Line) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group0()[0] * other.group1()[0] - self.group0()[1] * other.group1()[1] - self.group0()[2] * other.group1()[2] } }
    }
}

impl LeftAntiContraction<Line> for Translator {
    type Output = Line;

    fn left_anti_contraction(self, other: Line) -> Line {
        Line { groups: LineGroups { g0: Simd32x3::from(self.group0()[3]) * other.group0(), g1: Simd32x3::from(self.group0()[3]) * other.group1() } }
    }
}

impl ScalarProduct<Line> for Translator {
    type Output = Scalar;

    fn scalar_product(self, other: Line) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group0()[0] * other.group1()[0] - self.group0()[1] * other.group1()[1] - self.group0()[2] * other.group1()[2] } }
    }
}

impl InnerProduct<Plane> for Translator {
    type Output = Point;

    fn inner_product(self, other: Plane) -> Point {
        Point { groups: PointGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from(other.group0()[3]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[0]) * swizzle!(other.group0(), 3, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]) } }
    }
}

impl InnerAntiProduct<Plane> for Translator {
    type Output = Plane;

    fn inner_anti_product(self, other: Plane) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[3]) * other.group0() + Simd32x4::from(self.group0()[0]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) } }
    }
}

impl LeftContraction<Plane> for Translator {
    type Output = Point;

    fn left_contraction(self, other: Plane) -> Point {
        Point { groups: PointGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[0]) * swizzle!(other.group0(), 3, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]) } }
    }
}

impl LeftAntiContraction<Plane> for Translator {
    type Output = Plane;

    fn left_anti_contraction(self, other: Plane) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()[3]) * other.group0() } }
    }
}

impl Add<Motor> for Translator {
    type Output = Motor;

    fn add(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: swizzle!(self.group0(), 0, 0, 0, 3) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + other.group0(), g1: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) + other.group1() } }
    }
}

impl Sub<Motor> for Translator {
    type Output = Motor;

    fn sub(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: swizzle!(self.group0(), 0, 0, 0, 3) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) - other.group0(), g1: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) - other.group1() } }
    }
}

impl OuterProduct<Motor> for Translator {
    type Output = AntiScalar;

    fn outer_product(self, other: Motor) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2] } }
    }
}

impl InnerAntiProduct<Motor> for Translator {
    type Output = Motor;

    fn inner_anti_product(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[3]) * other.group0(), g1: Simd32x3::from(self.group0()[3]) * other.group1() + Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()[3]) } }
    }
}

impl LeftAntiContraction<Motor> for Translator {
    type Output = Motor;

    fn left_anti_contraction(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[3]) * other.group0(), g1: Simd32x3::from(self.group0()[3]) * other.group1() } }
    }
}

impl RightAntiContraction<Motor> for Translator {
    type Output = Translator;

    fn right_anti_contraction(self, other: Motor) -> Translator {
        Translator { groups: TranslatorGroups { g0: self.group0() * Simd32x4::from(other.group0()[3]) } }
    }
}

impl ScalarProduct<Motor> for Translator {
    type Output = Scalar;

    fn scalar_product(self, other: Motor) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group0()[0] * other.group1()[0] - self.group0()[1] * other.group1()[1] - self.group0()[2] * other.group1()[2] } }
    }
}

impl AntiScalarProduct<Motor> for Translator {
    type Output = AntiScalar;

    fn anti_scalar_product(self, other: Motor) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0()[3] * other.group0()[3] } }
    }
}

impl Add<Rotor> for Translator {
    type Output = Motor;

    fn add(self, other: Rotor) -> Motor {
        Motor { groups: MotorGroups { g0: swizzle!(self.group0(), 0, 0, 0, 3) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + other.group0(), g1: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) } }
    }
}

impl Sub<Rotor> for Translator {
    type Output = Motor;

    fn sub(self, other: Rotor) -> Motor {
        Motor { groups: MotorGroups { g0: swizzle!(self.group0(), 0, 0, 0, 3) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) - other.group0(), g1: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) } }
    }
}

impl GeometricProduct<Rotor> for Translator {
    type Output = Rotor;

    fn geometric_product(self, other: Rotor) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group0()[0]) * swizzle!(other.group0(), 3, 2, 1, 0) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]) + Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 2, 3, 0, 1) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 1, 0, 3, 2) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) } }
    }
}

impl OuterProduct<Rotor> for Translator {
    type Output = AntiScalar;

    fn outer_product(self, other: Rotor) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2] } }
    }
}

impl InnerAntiProduct<Rotor> for Translator {
    type Output = Motor;

    fn inner_anti_product(self, other: Rotor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[3]) * other.group0(), g1: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()[3]) } }
    }
}

impl LeftAntiContraction<Rotor> for Translator {
    type Output = Rotor;

    fn left_anti_contraction(self, other: Rotor) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group0()[3]) * other.group0() } }
    }
}

impl RightAntiContraction<Rotor> for Translator {
    type Output = Translator;

    fn right_anti_contraction(self, other: Rotor) -> Translator {
        Translator { groups: TranslatorGroups { g0: self.group0() * Simd32x4::from(other.group0()[3]) } }
    }
}

impl AntiScalarProduct<Rotor> for Translator {
    type Output = AntiScalar;

    fn anti_scalar_product(self, other: Rotor) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0()[3] * other.group0()[3] } }
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

impl RegressiveProduct<Translator> for Translator {
    type Output = Translator;

    fn regressive_product(self, other: Translator) -> Translator {
        Translator { groups: TranslatorGroups { g0: Simd32x4::from(self.group0()[3]) * other.group0() + swizzle!(self.group0(), 0, 1, 2, 0) * swizzle!(other.group0(), 3, 3, 3, 0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) } }
    }
}

impl GeometricAntiProduct<Translator> for Translator {
    type Output = Translator;

    fn geometric_anti_product(self, other: Translator) -> Translator {
        Translator { groups: TranslatorGroups { g0: Simd32x4::from(self.group0()[3]) * other.group0() + swizzle!(self.group0(), 0, 1, 2, 0) * swizzle!(other.group0(), 3, 3, 3, 0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) } }
    }
}

impl InnerAntiProduct<Translator> for Translator {
    type Output = Translator;

    fn inner_anti_product(self, other: Translator) -> Translator {
        Translator { groups: TranslatorGroups { g0: Simd32x4::from(self.group0()[3]) * other.group0() + swizzle!(self.group0(), 0, 1, 2, 0) * swizzle!(other.group0(), 3, 3, 3, 0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) } }
    }
}

impl LeftAntiContraction<Translator> for Translator {
    type Output = Translator;

    fn left_anti_contraction(self, other: Translator) -> Translator {
        Translator { groups: TranslatorGroups { g0: Simd32x4::from(self.group0()[3]) * other.group0() } }
    }
}

impl RightAntiContraction<Translator> for Translator {
    type Output = Translator;

    fn right_anti_contraction(self, other: Translator) -> Translator {
        Translator { groups: TranslatorGroups { g0: self.group0() * Simd32x4::from(other.group0()[3]) } }
    }
}

impl ScalarProduct<Translator> for Translator {
    type Output = Scalar;

    fn scalar_product(self, other: Translator) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2] } }
    }
}

impl AntiScalarProduct<Translator> for Translator {
    type Output = AntiScalar;

    fn anti_scalar_product(self, other: Translator) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0()[3] * other.group0()[3] } }
    }
}

impl GeometricProduct<Flector> for Translator {
    type Output = Flector;

    fn geometric_product(self, other: Flector) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group1()[3], other.group0()[2], other.group0()[1], other.group1()[0]]) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], other.group1()[3], other.group0()[0], other.group1()[1]]) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], other.group0()[0], other.group1()[3], other.group1()[2]]) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) + swizzle!(self.group0(), 0, 0, 0, 3) * swizzle!(other.group1(), 0, 0, 0, 3) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[3], other.group1()[2], other.group1()[1], other.group0()[0]]) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group1()[2], other.group0()[3], other.group1()[0], other.group0()[1]]) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[1], other.group1()[0], other.group0()[3], other.group0()[2]]) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) + swizzle!(self.group0(), 3, 3, 3, 0) * swizzle!(other.group0(), 0, 1, 2, 0) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]) } }
    }
}

impl RegressiveProduct<Flector> for Translator {
    type Output = Flector;

    fn regressive_product(self, other: Flector) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group1(), 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, -1.0, 0.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group1(), 1, 0, 1, 1) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[3]) * other.group0() + Simd32x4::from(self.group0()[0]) * swizzle!(other.group1(), 0, 2, 1, 0) * Simd32x4::from([0.0, -1.0, 1.0, 0.0]), g1: Simd32x4::from(self.group0()[3]) * other.group1() } }
    }
}

impl OuterProduct<Flector> for Translator {
    type Output = Plane;

    fn outer_product(self, other: Flector) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[0]) * swizzle!(other.group0(), 3, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]) } }
    }
}

impl GeometricAntiProduct<Flector> for Translator {
    type Output = Flector;

    fn geometric_anti_product(self, other: Flector) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group1()[2], other.group0()[3], other.group1()[0], other.group1()[2]]) * Simd32x4::from([1.0, 1.0, -1.0, 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[1], other.group1()[0], other.group0()[3], other.group1()[1]]) * Simd32x4::from([-1.0, 1.0, 1.0, 0.0]) + Simd32x4::from(self.group0()[3]) * other.group0() + Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[3], other.group1()[2], other.group1()[1], other.group0()[0]]) * Simd32x4::from([1.0, -1.0, 1.0, 0.0]), g1: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group1()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[3]) * other.group1() + Simd32x4::from(self.group0()[0]) * Simd32x4::from(other.group1()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) } }
    }
}

impl InnerAntiProduct<Flector> for Translator {
    type Output = Flector;

    fn inner_anti_product(self, other: Flector) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[3]) * other.group0(), g1: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group1()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[3]) * other.group1() + Simd32x4::from(self.group0()[0]) * Simd32x4::from(other.group1()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) } }
    }
}

impl LeftContraction<Flector> for Translator {
    type Output = Point;

    fn left_contraction(self, other: Flector) -> Point {
        Point { groups: PointGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group1(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group1(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[0]) * swizzle!(other.group1(), 3, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]) } }
    }
}

impl LeftAntiContraction<Flector> for Translator {
    type Output = Flector;

    fn left_anti_contraction(self, other: Flector) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[3]) * other.group0(), g1: Simd32x4::from(self.group0()[3]) * other.group1() } }
    }
}

impl ScalarProduct<MultiVector> for Translator {
    type Output = Scalar;

    fn scalar_product(self, other: MultiVector) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group0()[0] * other.group3()[0] - self.group0()[1] * other.group3()[1] - self.group0()[2] * other.group3()[2] } }
    }
}

impl AntiScalarProduct<MultiVector> for Translator {
    type Output = AntiScalar;

    fn anti_scalar_product(self, other: MultiVector) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0()[3] * other.group0()[1] } }
    }
}

impl SquaredMagnitude for Translator {
    type Output = Scalar;

    fn squared_magnitude(self) -> Scalar {
        self.scalar_product(self.reversal())
    }
}

impl Magnitude for Translator {
    type Output = Scalar;

    fn magnitude(self) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.squared_magnitude().group0().sqrt() } }
    }
}

impl BulkNorm for Translator {
    type Output = Scalar;

    fn bulk_norm(self) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.squared_magnitude().group0().sqrt() } }
    }
}

impl SquaredAntiMagnitude for Translator {
    type Output = AntiScalar;

    fn squared_anti_magnitude(self) -> AntiScalar {
        self.anti_scalar_product(self.anti_reversal())
    }
}

impl WeightNorm for Translator {
    type Output = AntiScalar;

    fn weight_norm(self) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.squared_anti_magnitude().group0().sqrt() } }
    }
}

impl GeometricNorm for Translator {
    type Output = HomogeneousMagnitude;

    fn geometric_norm(self) -> HomogeneousMagnitude {
        self.bulk_norm() + self.weight_norm()
    }
}

impl Scale for Translator {
    type Output = Translator;

    fn scale(self, other: f32) -> Translator {
        self.geometric_product(Scalar { groups: ScalarGroups { g0: other } })
    }
}

impl Signum for Translator {
    type Output = Translator;

    fn signum(self) -> Translator {
        self.geometric_product(Scalar { groups: ScalarGroups { g0: 1.0 / self.magnitude().group0() } })
    }
}

impl Inverse for Translator {
    type Output = Translator;

    fn inverse(self) -> Translator {
        self.reversal().geometric_product(Scalar { groups: ScalarGroups { g0: 1.0 / self.squared_magnitude().group0() } })
    }
}

impl Unitize for Translator {
    type Output = Translator;

    fn unitize(self) -> Translator {
        self.geometric_product(Scalar { groups: ScalarGroups { g0: 1.0 / self.weight_norm().group0() } })
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
        Flector { groups: FlectorGroups { g0: self.group0() * Simd32x4::from(-1.0), g1: self.group1() * Simd32x4::from(-1.0) } }
    }
}

impl Reversal for Flector {
    type Output = Flector;

    fn reversal(self) -> Flector {
        Flector { groups: FlectorGroups { g0: self.group0(), g1: self.group1() * Simd32x4::from(-1.0) } }
    }
}

impl Conjugation for Flector {
    type Output = Flector;

    fn conjugation(self) -> Flector {
        Flector { groups: FlectorGroups { g0: self.group0() * Simd32x4::from(-1.0), g1: self.group1() } }
    }
}

impl Dual for Flector {
    type Output = Flector;

    fn dual(self) -> Flector {
        Flector { groups: FlectorGroups { g0: self.group1() * Simd32x4::from(-1.0), g1: self.group0() } }
    }
}

impl AntiReversal for Flector {
    type Output = Flector;

    fn anti_reversal(self) -> Flector {
        Flector { groups: FlectorGroups { g0: self.group0() * Simd32x4::from(-1.0), g1: self.group1() } }
    }
}

impl RightComplement for Flector {
    type Output = Flector;

    fn right_complement(self) -> Flector {
        Flector { groups: FlectorGroups { g0: self.group1() * Simd32x4::from(-1.0), g1: self.group0() } }
    }
}

impl LeftComplement for Flector {
    type Output = Flector;

    fn left_complement(self) -> Flector {
        Flector { groups: FlectorGroups { g0: self.group1(), g1: self.group0() * Simd32x4::from(-1.0) } }
    }
}

impl DoubleComplement for Flector {
    type Output = Flector;

    fn double_complement(self) -> Flector {
        Flector { groups: FlectorGroups { g0: self.group0() * Simd32x4::from(-1.0), g1: self.group1() * Simd32x4::from(-1.0) } }
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

impl RegressiveProduct<AntiScalar> for Flector {
    type Output = Flector;

    fn regressive_product(self, other: AntiScalar) -> Flector {
        Flector { groups: FlectorGroups { g0: self.group0() * Simd32x4::from(other.group0()), g1: self.group1() * Simd32x4::from(other.group0()) } }
    }
}

impl GeometricAntiProduct<AntiScalar> for Flector {
    type Output = Flector;

    fn geometric_anti_product(self, other: AntiScalar) -> Flector {
        Flector { groups: FlectorGroups { g0: self.group0() * Simd32x4::from(other.group0()), g1: self.group1() * Simd32x4::from(other.group0()) } }
    }
}

impl InnerAntiProduct<AntiScalar> for Flector {
    type Output = Flector;

    fn inner_anti_product(self, other: AntiScalar) -> Flector {
        Flector { groups: FlectorGroups { g0: self.group0() * Simd32x4::from(other.group0()), g1: self.group1() * Simd32x4::from(other.group0()) } }
    }
}

impl RightAntiContraction<AntiScalar> for Flector {
    type Output = Flector;

    fn right_anti_contraction(self, other: AntiScalar) -> Flector {
        Flector { groups: FlectorGroups { g0: self.group0() * Simd32x4::from(other.group0()), g1: self.group1() * Simd32x4::from(other.group0()) } }
    }
}

impl GeometricProduct<HomogeneousMagnitude> for Flector {
    type Output = Flector;

    fn geometric_product(self, other: HomogeneousMagnitude) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group1()[3]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + self.group0() * Simd32x4::from(other.group0()[0]), g1: Simd32x4::from(self.group1()[0]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 1.0, 0.0]) + Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]) * Simd32x4::from([other.group0()[1], other.group0()[1], other.group0()[1], other.group0()[0]]) } }
    }
}

impl RegressiveProduct<HomogeneousMagnitude> for Flector {
    type Output = Flector;

    fn regressive_product(self, other: HomogeneousMagnitude) -> Flector {
        Flector { groups: FlectorGroups { g0: self.group0() * Simd32x4::from(other.group0()[1]), g1: self.group1() * Simd32x4::from(other.group0()[1]) } }
    }
}

impl OuterProduct<HomogeneousMagnitude> for Flector {
    type Output = Flector;

    fn outer_product(self, other: HomogeneousMagnitude) -> Flector {
        Flector { groups: FlectorGroups { g0: self.group0() * Simd32x4::from(other.group0()[0]), g1: self.group1() * Simd32x4::from(other.group0()[0]) } }
    }
}

impl InnerProduct<HomogeneousMagnitude> for Flector {
    type Output = Flector;

    fn inner_product(self, other: HomogeneousMagnitude) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group1()[3]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + self.group0() * Simd32x4::from(other.group0()[0]), g1: Simd32x4::from(self.group1()[0]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 1.0, 0.0]) + Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]) * Simd32x4::from([other.group0()[1], other.group0()[1], other.group0()[1], other.group0()[0]]) } }
    }
}

impl GeometricAntiProduct<HomogeneousMagnitude> for Flector {
    type Output = Flector;

    fn geometric_anti_product(self, other: HomogeneousMagnitude) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group1()[0]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, -1.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, -1.0, 0.0]) + self.group0() * Simd32x4::from(other.group0()[1]), g1: Simd32x4::from(self.group1()[3]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group0()[3]]) * Simd32x4::from([other.group0()[1], other.group0()[1], other.group0()[1], other.group0()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) } }
    }
}

impl InnerAntiProduct<HomogeneousMagnitude> for Flector {
    type Output = Flector;

    fn inner_anti_product(self, other: HomogeneousMagnitude) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group1()[0]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, -1.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, -1.0, 0.0]) + self.group0() * Simd32x4::from(other.group0()[1]), g1: Simd32x4::from(self.group1()[3]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group0()[3]]) * Simd32x4::from([other.group0()[1], other.group0()[1], other.group0()[1], other.group0()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) } }
    }
}

impl RightContraction<HomogeneousMagnitude> for Flector {
    type Output = Flector;

    fn right_contraction(self, other: HomogeneousMagnitude) -> Flector {
        Flector { groups: FlectorGroups { g0: self.group0() * Simd32x4::from(other.group0()[0]), g1: self.group1() * Simd32x4::from(other.group0()[0]) } }
    }
}

impl RightAntiContraction<HomogeneousMagnitude> for Flector {
    type Output = Flector;

    fn right_anti_contraction(self, other: HomogeneousMagnitude) -> Flector {
        Flector { groups: FlectorGroups { g0: self.group0() * Simd32x4::from(other.group0()[1]), g1: self.group1() * Simd32x4::from(other.group0()[1]) } }
    }
}

impl Into<Point> for Flector {
    fn into(self) -> Point {
        Point { groups: PointGroups { g0: self.group0() } }
    }
}

impl Add<Point> for Flector {
    type Output = Flector;

    fn add(self, other: Point) -> Flector {
        Flector { groups: FlectorGroups { g0: self.group0() + other.group0(), g1: self.group1() } }
    }
}

impl AddAssign<Point> for Flector {
    fn add_assign(&mut self, other: Point) {
        *self = (*self).add(other);
    }
}

impl Sub<Point> for Flector {
    type Output = Flector;

    fn sub(self, other: Point) -> Flector {
        Flector { groups: FlectorGroups { g0: self.group0() - other.group0(), g1: self.group1() } }
    }
}

impl SubAssign<Point> for Flector {
    fn sub_assign(&mut self, other: Point) {
        *self = (*self).sub(other);
    }
}

impl RegressiveProduct<Point> for Flector {
    type Output = Scalar;

    fn regressive_product(self, other: Point) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group1()[0] * other.group0()[0] - self.group1()[1] * other.group0()[1] - self.group1()[2] * other.group0()[2] - self.group1()[3] * other.group0()[3] } }
    }
}

impl OuterProduct<Point> for Flector {
    type Output = Motor;

    fn outer_product(self, other: Point) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[3]) * swizzle!(other.group0(), 0, 1, 2, 0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from(other.group0()[3]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[0]]) * swizzle!(other.group0(), 3, 3, 3, 0) * Simd32x4::from(-1.0), g1: Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], other.group0()[2], other.group0()[0]]) * Simd32x3::from([1.0, 0.0, -1.0]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], other.group0()[0], other.group0()[1]]) * Simd32x3::from([-1.0, 1.0, 0.0]) + Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[0], other.group0()[2], other.group0()[1]]) * Simd32x3::from([0.0, -1.0, 1.0]) } }
    }
}

impl InnerAntiProduct<Point> for Flector {
    type Output = Motor;

    fn inner_anti_product(self, other: Point) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group0()[3]]) * Simd32x4::from(other.group0()[3]) * Simd32x4::from(-1.0), g1: Simd32x3::from(self.group1()[1]) * Simd32x3::from([other.group0()[2], other.group0()[2], other.group0()[0]]) * Simd32x3::from([1.0, 0.0, -1.0]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group0()[1], other.group0()[0], other.group0()[1]]) * Simd32x3::from([-1.0, 1.0, 0.0]) + Simd32x3::from(self.group1()[0]) * Simd32x3::from([other.group0()[0], other.group0()[2], other.group0()[1]]) * Simd32x3::from([0.0, -1.0, 1.0]) } }
    }
}

impl LeftContraction<Point> for Flector {
    type Output = Scalar;

    fn left_contraction(self, other: Point) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2] } }
    }
}

impl LeftAntiContraction<Point> for Flector {
    type Output = Motor;

    fn left_anti_contraction(self, other: Point) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group0()[3]]) * Simd32x4::from(other.group0()[3]) * Simd32x4::from(-1.0), g1: Simd32x3::from(self.group1()[1]) * Simd32x3::from([other.group0()[2], other.group0()[2], other.group0()[0]]) * Simd32x3::from([1.0, 0.0, -1.0]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group0()[1], other.group0()[0], other.group0()[1]]) * Simd32x3::from([-1.0, 1.0, 0.0]) + Simd32x3::from(self.group1()[0]) * Simd32x3::from([other.group0()[0], other.group0()[2], other.group0()[1]]) * Simd32x3::from([0.0, -1.0, 1.0]) } }
    }
}

impl RightAntiContraction<Point> for Flector {
    type Output = AntiScalar;

    fn right_anti_contraction(self, other: Point) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: 0.0 - self.group0()[3] * other.group0()[3] } }
    }
}

impl ScalarProduct<Point> for Flector {
    type Output = Scalar;

    fn scalar_product(self, other: Point) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2] } }
    }
}

impl AntiScalarProduct<Point> for Flector {
    type Output = AntiScalar;

    fn anti_scalar_product(self, other: Point) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: 0.0 - self.group0()[3] * other.group0()[3] } }
    }
}

impl GeometricProduct<Line> for Flector {
    type Output = Flector;

    fn geometric_product(self, other: Line) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group1()[2], other.group1()[2], other.group1()[0], other.group0()[1]]) * Simd32x4::from([-1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[1], other.group1()[0], other.group1()[1], other.group0()[2]]) * Simd32x4::from([1.0, -1.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from(other.group1()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from(other.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from(other.group1()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group1()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group1()[0], other.group1()[2], other.group1()[1], other.group0()[0]]) * Simd32x4::from([0.0, 1.0, -1.0, -1.0]), g1: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], other.group0()[2], other.group0()[0], other.group1()[1]]) * Simd32x4::from([-1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], other.group0()[0], other.group0()[1], other.group1()[2]]) * Simd32x4::from([1.0, -1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group1()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group1()[2], other.group1()[2], other.group1()[1], other.group1()[2]]) * Simd32x4::from([0.0, 1.0, -1.0, 0.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group1()[2], other.group1()[2], other.group1()[0], other.group1()[2]]) * Simd32x4::from([-1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group1()[1], other.group1()[0], other.group1()[1], other.group1()[1]]) * Simd32x4::from([1.0, -1.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group0()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[0], other.group0()[2], other.group0()[1], other.group1()[0]]) * Simd32x4::from([0.0, 1.0, -1.0, -1.0]) } }
    }
}

impl RegressiveProduct<Line> for Flector {
    type Output = Point;

    fn regressive_product(self, other: Line) -> Point {
        Point { groups: PointGroups { g0: Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group1()[2], other.group1()[2], other.group1()[0], other.group0()[1]]) * Simd32x4::from([-1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group1()[1], other.group1()[0], other.group1()[1], other.group0()[2]]) * Simd32x4::from([1.0, -1.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group0()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group1()[0], other.group1()[2], other.group1()[1], other.group0()[0]]) * Simd32x4::from([0.0, 1.0, -1.0, -1.0]) } }
    }
}

impl OuterProduct<Line> for Flector {
    type Output = Plane;

    fn outer_product(self, other: Line) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], other.group0()[2], other.group0()[0], other.group1()[1]]) * Simd32x4::from([-1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], other.group0()[0], other.group0()[1], other.group1()[2]]) * Simd32x4::from([1.0, -1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group1()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[0], other.group0()[2], other.group0()[1], other.group1()[0]]) * Simd32x4::from([0.0, 1.0, -1.0, -1.0]) } }
    }
}

impl InnerProduct<Line> for Flector {
    type Output = Point;

    fn inner_product(self, other: Line) -> Point {
        Point { groups: PointGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group1()[2], other.group1()[2], other.group1()[0], other.group0()[1]]) * Simd32x4::from([-1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[1], other.group1()[0], other.group1()[1], other.group0()[2]]) * Simd32x4::from([1.0, -1.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from(other.group1()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from(other.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from(other.group1()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group1()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group1()[0], other.group1()[2], other.group1()[1], other.group0()[0]]) * Simd32x4::from([0.0, 1.0, -1.0, -1.0]) } }
    }
}

impl GeometricAntiProduct<Line> for Flector {
    type Output = Flector;

    fn geometric_anti_product(self, other: Line) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], other.group0()[2], other.group0()[0], other.group0()[2]]) * Simd32x4::from([1.0, 0.0, -1.0, 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], other.group0()[0], other.group0()[1], other.group0()[1]]) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group1()[0]]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group1()[2], other.group1()[2], other.group1()[1], other.group0()[0]]) * Simd32x4::from([0.0, 1.0, -1.0, -1.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group1()[2], other.group1()[2], other.group1()[0], other.group0()[1]]) * Simd32x4::from([-1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group1()[1], other.group1()[0], other.group1()[1], other.group0()[2]]) * Simd32x4::from([1.0, -1.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group0()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[0], other.group0()[2], other.group0()[1], other.group0()[0]]) * Simd32x4::from([0.0, -1.0, 1.0, 0.0]), g1: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group0()[2], other.group0()[2], other.group0()[1], other.group1()[0]]) * Simd32x4::from([0.0, -1.0, 1.0, 1.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group0()[2], other.group0()[2], other.group0()[0], other.group1()[1]]) * Simd32x4::from([1.0, 0.0, -1.0, 1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group0()[1], other.group0()[0], other.group0()[1], other.group1()[2]]) * Simd32x4::from([-1.0, 1.0, 0.0, 1.0]) + swizzle!(self.group0(), 3, 3, 3, 0) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group0()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) } }
    }
}

impl InnerAntiProduct<Line> for Flector {
    type Output = Plane;

    fn inner_anti_product(self, other: Line) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group0()[2], other.group0()[2], other.group0()[1], other.group1()[0]]) * Simd32x4::from([0.0, -1.0, 1.0, 1.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group0()[2], other.group0()[2], other.group0()[0], other.group1()[1]]) * Simd32x4::from([1.0, 0.0, -1.0, 1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group0()[1], other.group0()[0], other.group0()[1], other.group1()[2]]) * Simd32x4::from([-1.0, 1.0, 0.0, 1.0]) + swizzle!(self.group0(), 3, 3, 3, 0) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group0()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) } }
    }
}

impl LeftContraction<Line> for Flector {
    type Output = Point;

    fn left_contraction(self, other: Line) -> Point {
        Point { groups: PointGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group1()[2], other.group1()[2], other.group1()[0], other.group0()[1]]) * Simd32x4::from([-1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[1], other.group1()[0], other.group1()[1], other.group0()[2]]) * Simd32x4::from([1.0, -1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group1()[0], other.group1()[2], other.group1()[1], other.group0()[0]]) * Simd32x4::from([0.0, 1.0, -1.0, -1.0]) } }
    }
}

impl RightContraction<Line> for Flector {
    type Output = Point;

    fn right_contraction(self, other: Line) -> Point {
        Point { groups: PointGroups { g0: Simd32x4::from(self.group1()[1]) * Simd32x4::from(other.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from(other.group1()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + swizzle!(self.group1(), 3, 3, 3, 0) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group1()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) } }
    }
}

impl LeftAntiContraction<Line> for Flector {
    type Output = Plane;

    fn left_anti_contraction(self, other: Line) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group0()[2], other.group0()[2], other.group0()[0], other.group1()[1]]) * Simd32x4::from([1.0, 0.0, -1.0, 1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group0()[1], other.group0()[0], other.group0()[1], other.group1()[2]]) * Simd32x4::from([-1.0, 1.0, 0.0, 1.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group0()[0], other.group0()[2], other.group0()[1], other.group1()[0]]) * Simd32x4::from([0.0, -1.0, 1.0, 1.0]) } }
    }
}

impl RightAntiContraction<Line> for Flector {
    type Output = Plane;

    fn right_anti_contraction(self, other: Line) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + swizzle!(self.group0(), 3, 3, 3, 0) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group0()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) } }
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

impl OuterProduct<Plane> for Flector {
    type Output = AntiScalar;

    fn outer_product(self, other: Plane) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2] + self.group0()[3] * other.group0()[3] } }
    }
}

impl InnerAntiProduct<Plane> for Flector {
    type Output = Motor;

    fn inner_anti_product(self, other: Plane) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group1()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from([self.group0()[3], self.group0()[3], self.group0()[3], self.group1()[0]]) * swizzle!(other.group0(), 0, 1, 2, 0) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]), g1: Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], other.group0()[2], other.group0()[0]]) * Simd32x3::from([-1.0, 0.0, 1.0]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], other.group0()[0], other.group0()[1]]) * Simd32x3::from([1.0, -1.0, 0.0]) + Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[0], other.group0()[2], other.group0()[1]]) * Simd32x3::from([0.0, 1.0, -1.0]) } }
    }
}

impl RightContraction<Plane> for Flector {
    type Output = Scalar;

    fn right_contraction(self, other: Plane) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group1()[3] * other.group0()[3] } }
    }
}

impl LeftAntiContraction<Plane> for Flector {
    type Output = AntiScalar;

    fn left_anti_contraction(self, other: Plane) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group1()[0] * other.group0()[0] + self.group1()[1] * other.group0()[1] + self.group1()[2] * other.group0()[2] } }
    }
}

impl RightAntiContraction<Plane> for Flector {
    type Output = Motor;

    fn right_anti_contraction(self, other: Plane) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group1()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from([self.group0()[3], self.group0()[3], self.group0()[3], self.group1()[0]]) * swizzle!(other.group0(), 0, 1, 2, 0) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]), g1: Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], other.group0()[2], other.group0()[0]]) * Simd32x3::from([-1.0, 0.0, 1.0]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], other.group0()[0], other.group0()[1]]) * Simd32x3::from([1.0, -1.0, 0.0]) + Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[0], other.group0()[2], other.group0()[1]]) * Simd32x3::from([0.0, 1.0, -1.0]) } }
    }
}

impl ScalarProduct<Plane> for Flector {
    type Output = Scalar;

    fn scalar_product(self, other: Plane) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group1()[3] * other.group0()[3] } }
    }
}

impl AntiScalarProduct<Plane> for Flector {
    type Output = AntiScalar;

    fn anti_scalar_product(self, other: Plane) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group1()[0] * other.group0()[0] + self.group1()[1] * other.group0()[1] + self.group1()[2] * other.group0()[2] } }
    }
}

impl GeometricProduct<Motor> for Flector {
    type Output = Flector;

    fn geometric_product(self, other: Motor) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group1()[2], other.group1()[2], other.group1()[0], other.group0()[1]]) * Simd32x4::from([-1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[1], other.group1()[0], other.group1()[1], other.group0()[2]]) * Simd32x4::from([1.0, -1.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from(other.group1()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from(other.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from(other.group1()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group0()[3]]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group1()[0], other.group1()[2], other.group1()[1], other.group0()[0]]) * Simd32x4::from([0.0, 1.0, -1.0, -1.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[3], other.group0()[2], other.group0()[1], other.group1()[0]]) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], other.group0()[3], other.group0()[0], other.group1()[1]]) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], other.group0()[0], other.group0()[3], other.group1()[2]]) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group1()[2], other.group1()[2], other.group1()[1], other.group1()[2]]) * Simd32x4::from([0.0, 1.0, -1.0, 0.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group1()[2], other.group1()[2], other.group1()[0], other.group1()[2]]) * Simd32x4::from([-1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group1()[1], other.group1()[0], other.group1()[1], other.group1()[1]]) * Simd32x4::from([1.0, -1.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[3]) * swizzle!(other.group0(), 0, 1, 2, 0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) + swizzle!(self.group0(), 3, 3, 3, 0) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group1()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) } }
    }
}

impl RegressiveProduct<Motor> for Flector {
    type Output = Flector;

    fn regressive_product(self, other: Motor) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group1()[2], other.group1()[2], other.group1()[1], other.group0()[0]]) * Simd32x4::from([0.0, 1.0, -1.0, -1.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group1()[2], other.group1()[2], other.group1()[0], other.group0()[1]]) * Simd32x4::from([-1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group1()[1], other.group1()[0], other.group1()[1], other.group0()[2]]) * Simd32x4::from([1.0, -1.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[3]) * swizzle!(other.group0(), 0, 1, 2, 0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) + self.group0() * Simd32x4::from(other.group0()[3]), g1: self.group1() * Simd32x4::from(other.group0()[3]) } }
    }
}

impl OuterProduct<Motor> for Flector {
    type Output = Plane;

    fn outer_product(self, other: Motor) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], other.group0()[2], other.group0()[0], other.group1()[1]]) * Simd32x4::from([-1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], other.group0()[0], other.group0()[1], other.group1()[2]]) * Simd32x4::from([1.0, -1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group1()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[0], other.group0()[2], other.group0()[1], other.group1()[0]]) * Simd32x4::from([0.0, 1.0, -1.0, -1.0]) } }
    }
}

impl GeometricAntiProduct<Motor> for Flector {
    type Output = Flector;

    fn geometric_anti_product(self, other: Motor) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 2, 3, 0, 2) * Simd32x4::from([1.0, 1.0, -1.0, 0.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 1, 0, 3, 1) * Simd32x4::from([-1.0, 1.0, 1.0, 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group0()[3]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group1()[2], other.group1()[2], other.group1()[1], other.group0()[0]]) * Simd32x4::from([0.0, 1.0, -1.0, -1.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group1()[2], other.group1()[2], other.group1()[0], other.group0()[1]]) * Simd32x4::from([-1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group1()[1], other.group1()[0], other.group1()[1], other.group0()[2]]) * Simd32x4::from([1.0, -1.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[3]) * swizzle!(other.group0(), 0, 1, 2, 0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) + Simd32x4::from(self.group0()[0]) * swizzle!(other.group0(), 3, 2, 1, 0) * Simd32x4::from([1.0, -1.0, 1.0, 0.0]), g1: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group0()[3], other.group0()[2], other.group0()[1], other.group1()[0]]) * Simd32x4::from([1.0, -1.0, 1.0, 1.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group0()[2], other.group0()[3], other.group0()[0], other.group1()[1]]) * Simd32x4::from([1.0, 1.0, -1.0, 1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group0()[1], other.group0()[0], other.group0()[3], other.group1()[2]]) * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from(other.group0()[3]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + swizzle!(self.group0(), 3, 3, 3, 0) * swizzle!(other.group0(), 0, 1, 2, 0) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) } }
    }
}

impl InnerAntiProduct<Motor> for Flector {
    type Output = Flector;

    fn inner_anti_product(self, other: Motor) -> Flector {
        Flector { groups: FlectorGroups { g0: self.group0() * Simd32x4::from(other.group0()[3]), g1: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group0()[3], other.group0()[2], other.group0()[1], other.group1()[0]]) * Simd32x4::from([1.0, -1.0, 1.0, 1.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group0()[2], other.group0()[3], other.group0()[0], other.group1()[1]]) * Simd32x4::from([1.0, 1.0, -1.0, 1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group0()[1], other.group0()[0], other.group0()[3], other.group1()[2]]) * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from(other.group0()[3]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + swizzle!(self.group0(), 3, 3, 3, 0) * swizzle!(other.group0(), 0, 1, 2, 0) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) } }
    }
}

impl RightContraction<Motor> for Flector {
    type Output = Point;

    fn right_contraction(self, other: Motor) -> Point {
        Point { groups: PointGroups { g0: Simd32x4::from(self.group1()[1]) * Simd32x4::from(other.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from(other.group1()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + swizzle!(self.group1(), 3, 3, 3, 0) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group1()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) } }
    }
}

impl LeftAntiContraction<Motor> for Flector {
    type Output = Plane;

    fn left_anti_contraction(self, other: Motor) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group0()[2], other.group0()[2], other.group0()[0], other.group1()[1]]) * Simd32x4::from([1.0, 0.0, -1.0, 1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group0()[1], other.group0()[0], other.group0()[1], other.group1()[2]]) * Simd32x4::from([-1.0, 1.0, 0.0, 1.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group0()[0], other.group0()[2], other.group0()[1], other.group1()[0]]) * Simd32x4::from([0.0, -1.0, 1.0, 1.0]) } }
    }
}

impl RightAntiContraction<Motor> for Flector {
    type Output = Flector;

    fn right_anti_contraction(self, other: Motor) -> Flector {
        Flector { groups: FlectorGroups { g0: self.group0() * Simd32x4::from(other.group0()[3]), g1: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from(other.group0()[3]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from(other.group0()[3]) * Simd32x4::from([0.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from(other.group0()[3]) * Simd32x4::from([0.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from(other.group0()[3]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + swizzle!(self.group0(), 3, 3, 3, 0) * swizzle!(other.group0(), 0, 1, 2, 0) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) } }
    }
}

impl RegressiveProduct<Rotor> for Flector {
    type Output = Flector;

    fn regressive_product(self, other: Rotor) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group1()[0]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[3]) * swizzle!(other.group0(), 0, 1, 2, 0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) + self.group0() * Simd32x4::from(other.group0()[3]), g1: self.group1() * Simd32x4::from(other.group0()[3]) } }
    }
}

impl GeometricAntiProduct<Rotor> for Flector {
    type Output = Flector;

    fn geometric_anti_product(self, other: Rotor) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 2, 3, 0, 2) * Simd32x4::from([1.0, 1.0, -1.0, 0.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 1, 0, 3, 1) * Simd32x4::from([-1.0, 1.0, 1.0, 0.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[3]) * swizzle!(other.group0(), 0, 1, 2, 0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) + swizzle!(self.group0(), 0, 0, 0, 3) * swizzle!(other.group0(), 3, 2, 1, 3) * Simd32x4::from([1.0, -1.0, 1.0, 1.0]), g1: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[0]) * swizzle!(other.group0(), 3, 2, 1, 3) * Simd32x4::from([1.0, -1.0, 1.0, 0.0]) + Simd32x4::from(self.group1()[1]) * swizzle!(other.group0(), 2, 3, 0, 2) * Simd32x4::from([1.0, 1.0, -1.0, 0.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group0(), 1, 0, 3, 1) * Simd32x4::from([-1.0, 1.0, 1.0, 0.0]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from(other.group0()[3]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + swizzle!(self.group0(), 3, 3, 3, 0) * swizzle!(other.group0(), 0, 1, 2, 0) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) } }
    }
}

impl InnerAntiProduct<Rotor> for Flector {
    type Output = Flector;

    fn inner_anti_product(self, other: Rotor) -> Flector {
        Flector { groups: FlectorGroups { g0: self.group0() * Simd32x4::from(other.group0()[3]), g1: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[0]) * swizzle!(other.group0(), 3, 2, 1, 3) * Simd32x4::from([1.0, -1.0, 1.0, 0.0]) + Simd32x4::from(self.group1()[1]) * swizzle!(other.group0(), 2, 3, 0, 2) * Simd32x4::from([1.0, 1.0, -1.0, 0.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group0(), 1, 0, 3, 1) * Simd32x4::from([-1.0, 1.0, 1.0, 0.0]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from(other.group0()[3]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + swizzle!(self.group0(), 3, 3, 3, 0) * swizzle!(other.group0(), 0, 1, 2, 0) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) } }
    }
}

impl RightAntiContraction<Rotor> for Flector {
    type Output = Flector;

    fn right_anti_contraction(self, other: Rotor) -> Flector {
        Flector { groups: FlectorGroups { g0: self.group0() * Simd32x4::from(other.group0()[3]), g1: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from(other.group0()[3]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from(other.group0()[3]) * Simd32x4::from([0.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from(other.group0()[3]) * Simd32x4::from([0.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from(other.group0()[3]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + swizzle!(self.group0(), 3, 3, 3, 0) * swizzle!(other.group0(), 0, 1, 2, 0) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) } }
    }
}

impl GeometricProduct<Translator> for Flector {
    type Output = Flector;

    fn geometric_product(self, other: Translator) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([-1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 1, 0, 1, 1) * Simd32x4::from([1.0, -1.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[3]) * other.group0() + Simd32x4::from([self.group0()[0], self.group0()[0], self.group0()[0], self.group1()[0]]) * swizzle!(other.group0(), 0, 2, 1, 0) * Simd32x4::from([0.0, 1.0, -1.0, -1.0]), g1: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group0(), 0, 1, 2, 0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) + Simd32x4::from(self.group1()[1]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([-1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group0(), 1, 0, 1, 1) * Simd32x4::from([1.0, -1.0, 0.0, 0.0]) + Simd32x4::from([self.group0()[0], self.group1()[0], self.group1()[0], self.group0()[0]]) * swizzle!(other.group0(), 3, 2, 1, 0) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]) } }
    }
}

impl RegressiveProduct<Translator> for Flector {
    type Output = Flector;

    fn regressive_product(self, other: Translator) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group1()[0]) * swizzle!(other.group0(), 2, 2, 1, 2) * Simd32x4::from([0.0, 1.0, -1.0, 0.0]) + Simd32x4::from(self.group1()[1]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([-1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group0(), 1, 0, 1, 1) * Simd32x4::from([1.0, -1.0, 0.0, 0.0]) + self.group0() * Simd32x4::from(other.group0()[3]), g1: self.group1() * Simd32x4::from(other.group0()[3]) } }
    }
}

impl OuterProduct<Translator> for Flector {
    type Output = Plane;

    fn outer_product(self, other: Translator) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + swizzle!(self.group0(), 3, 3, 3, 0) * swizzle!(other.group0(), 0, 1, 2, 0) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) } }
    }
}

impl GeometricAntiProduct<Translator> for Flector {
    type Output = Flector;

    fn geometric_anti_product(self, other: Translator) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[3]) * other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]) + Simd32x4::from(self.group1()[0]) * swizzle!(other.group0(), 2, 2, 1, 2) * Simd32x4::from([0.0, 1.0, -1.0, 0.0]) + Simd32x4::from(self.group1()[1]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([-1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group0(), 1, 0, 1, 1) * Simd32x4::from([1.0, -1.0, 0.0, 0.0]) + swizzle!(self.group0(), 0, 1, 2, 0) * swizzle!(other.group0(), 3, 3, 3, 0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]), g1: Simd32x4::from(self.group1()[1]) * swizzle!(other.group0(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, 1.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group0(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, 1.0]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from(other.group0()[3]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group1()[0]) * swizzle!(other.group0(), 3, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, 1.0]) } }
    }
}

impl InnerAntiProduct<Translator> for Flector {
    type Output = Flector;

    fn inner_anti_product(self, other: Translator) -> Flector {
        Flector { groups: FlectorGroups { g0: self.group0() * Simd32x4::from(other.group0()[3]), g1: Simd32x4::from(self.group1()[1]) * swizzle!(other.group0(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, 1.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group0(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, 1.0]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from(other.group0()[3]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group1()[0]) * swizzle!(other.group0(), 3, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, 1.0]) } }
    }
}

impl RightContraction<Translator> for Flector {
    type Output = Point;

    fn right_contraction(self, other: Translator) -> Point {
        Point { groups: PointGroups { g0: Simd32x4::from(self.group1()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + swizzle!(self.group1(), 3, 3, 3, 0) * swizzle!(other.group0(), 0, 1, 2, 0) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) } }
    }
}

impl RightAntiContraction<Translator> for Flector {
    type Output = Flector;

    fn right_anti_contraction(self, other: Translator) -> Flector {
        Flector { groups: FlectorGroups { g0: self.group0() * Simd32x4::from(other.group0()[3]), g1: self.group1() * Simd32x4::from(other.group0()[3]) } }
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

impl OuterProduct<Flector> for Flector {
    type Output = Motor;

    fn outer_product(self, other: Flector) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[3], other.group0()[3], other.group0()[3], other.group1()[1]]) * Simd32x4::from([0.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[3], other.group0()[3], other.group0()[3], other.group1()[2]]) * Simd32x4::from([0.0, 0.0, -1.0, 1.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from(other.group0()[3]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[3], other.group0()[0], other.group0()[0], other.group1()[0]]) * Simd32x4::from([-1.0, 0.0, 0.0, 1.0]), g1: Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], other.group0()[2], other.group0()[0]]) * Simd32x3::from([1.0, 0.0, -1.0]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], other.group0()[0], other.group0()[1]]) * Simd32x3::from([-1.0, 1.0, 0.0]) + Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[0], other.group0()[2], other.group0()[1]]) * Simd32x3::from([0.0, -1.0, 1.0]) } }
    }
}

impl InnerAntiProduct<Flector> for Flector {
    type Output = Motor;

    fn inner_anti_product(self, other: Flector) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(0.0) - Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group0()[3]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group0()[3], other.group0()[3], other.group0()[3], other.group1()[1]]) * Simd32x4::from([0.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group0()[3], other.group0()[3], other.group0()[3], other.group1()[2]]) * Simd32x4::from([0.0, 0.0, -1.0, 1.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group0()[3], other.group0()[0], other.group0()[0], other.group1()[0]]) * Simd32x4::from([-1.0, 0.0, 0.0, 1.0]), g1: Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group1()[2], other.group1()[2], other.group1()[0]]) * Simd32x3::from([-1.0, 0.0, 1.0]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group1()[1], other.group1()[0], other.group1()[1]]) * Simd32x3::from([1.0, -1.0, 0.0]) + Simd32x3::from(self.group1()[0]) * Simd32x3::from([other.group0()[2], other.group0()[2], other.group0()[1]]) * Simd32x3::from([0.0, -1.0, 1.0]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from([other.group0()[2], other.group0()[2], other.group0()[0]]) * Simd32x3::from([1.0, 0.0, -1.0]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group0()[1], other.group0()[0], other.group0()[1]]) * Simd32x3::from([-1.0, 1.0, 0.0]) + Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group1()[0], other.group1()[2], other.group1()[1]]) * Simd32x3::from([0.0, 1.0, -1.0]) } }
    }
}

impl LeftAntiContraction<Flector> for Flector {
    type Output = Motor;

    fn left_anti_contraction(self, other: Flector) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group0()[3], other.group0()[3], other.group0()[3], other.group1()[0]]) * Simd32x4::from([-1.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group0()[3], other.group0()[3], other.group0()[3], other.group1()[1]]) * Simd32x4::from([0.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group0()[3], other.group0()[3], other.group0()[3], other.group1()[2]]) * Simd32x4::from([0.0, 0.0, -1.0, 1.0]) + swizzle!(self.group0(), 0, 0, 0, 3) * swizzle!(other.group0(), 0, 0, 0, 3) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]), g1: Simd32x3::from(self.group1()[1]) * Simd32x3::from([other.group0()[2], other.group0()[2], other.group0()[0]]) * Simd32x3::from([1.0, 0.0, -1.0]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group0()[1], other.group0()[0], other.group0()[1]]) * Simd32x3::from([-1.0, 1.0, 0.0]) + Simd32x3::from(self.group1()[0]) * Simd32x3::from([other.group0()[0], other.group0()[2], other.group0()[1]]) * Simd32x3::from([0.0, -1.0, 1.0]) } }
    }
}

impl RightAntiContraction<Flector> for Flector {
    type Output = Motor;

    fn right_anti_contraction(self, other: Flector) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(0.0) - Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group0()[3]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from(other.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from(other.group1()[2]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from(other.group1()[0]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]), g1: Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group1()[2], other.group1()[2], other.group1()[0]]) * Simd32x3::from([-1.0, 0.0, 1.0]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group1()[1], other.group1()[0], other.group1()[1]]) * Simd32x3::from([1.0, -1.0, 0.0]) + Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group1()[0], other.group1()[2], other.group1()[1]]) * Simd32x3::from([0.0, 1.0, -1.0]) } }
    }
}

impl ScalarProduct<Flector> for Flector {
    type Output = Scalar;

    fn scalar_product(self, other: Flector) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2] - self.group1()[3] * other.group1()[3] } }
    }
}

impl AntiScalarProduct<Flector> for Flector {
    type Output = AntiScalar;

    fn anti_scalar_product(self, other: Flector) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: 0.0 - self.group0()[3] * other.group0()[3] + self.group1()[0] * other.group1()[0] + self.group1()[1] * other.group1()[1] + self.group1()[2] * other.group1()[2] } }
    }
}

impl ScalarProduct<MultiVector> for Flector {
    type Output = Scalar;

    fn scalar_product(self, other: MultiVector) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[0] * other.group1()[0] + self.group0()[1] * other.group1()[1] + self.group0()[2] * other.group1()[2] - self.group1()[3] * other.group6()[3] } }
    }
}

impl AntiScalarProduct<MultiVector> for Flector {
    type Output = AntiScalar;

    fn anti_scalar_product(self, other: MultiVector) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: 0.0 - self.group0()[3] * other.group1()[3] + self.group1()[0] * other.group6()[0] + self.group1()[1] * other.group6()[1] + self.group1()[2] * other.group6()[2] } }
    }
}

impl SquaredMagnitude for Flector {
    type Output = Scalar;

    fn squared_magnitude(self) -> Scalar {
        self.scalar_product(self.reversal())
    }
}

impl Magnitude for Flector {
    type Output = Scalar;

    fn magnitude(self) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.squared_magnitude().group0().sqrt() } }
    }
}

impl BulkNorm for Flector {
    type Output = Scalar;

    fn bulk_norm(self) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.squared_magnitude().group0().sqrt() } }
    }
}

impl SquaredAntiMagnitude for Flector {
    type Output = AntiScalar;

    fn squared_anti_magnitude(self) -> AntiScalar {
        self.anti_scalar_product(self.anti_reversal())
    }
}

impl WeightNorm for Flector {
    type Output = AntiScalar;

    fn weight_norm(self) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.squared_anti_magnitude().group0().sqrt() } }
    }
}

impl GeometricNorm for Flector {
    type Output = HomogeneousMagnitude;

    fn geometric_norm(self) -> HomogeneousMagnitude {
        self.bulk_norm() + self.weight_norm()
    }
}

impl Scale for Flector {
    type Output = Flector;

    fn scale(self, other: f32) -> Flector {
        self.geometric_product(Scalar { groups: ScalarGroups { g0: other } })
    }
}

impl Signum for Flector {
    type Output = Flector;

    fn signum(self) -> Flector {
        self.geometric_product(Scalar { groups: ScalarGroups { g0: 1.0 / self.magnitude().group0() } })
    }
}

impl Inverse for Flector {
    type Output = Flector;

    fn inverse(self) -> Flector {
        self.reversal().geometric_product(Scalar { groups: ScalarGroups { g0: 1.0 / self.squared_magnitude().group0() } })
    }
}

impl Unitize for Flector {
    type Output = Flector;

    fn unitize(self) -> Flector {
        self.geometric_product(Scalar { groups: ScalarGroups { g0: 1.0 / self.weight_norm().group0() } })
    }
}

impl Zero for MultiVector {
    fn zero() -> Self {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x3::from(0.0), g1: Simd32x4::from(0.0), g2: Simd32x3::from(0.0), g3: Simd32x3::from(0.0), g4: Simd32x3::from(0.0), g5: Simd32x3::from(0.0), g6: Simd32x4::from(0.0), g7: Simd32x4::from(0.0) } }
    }
}

impl One for MultiVector {
    fn one() -> Self {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x3::from([1.0, 0.0, 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(0.0), g3: Simd32x3::from(0.0), g4: Simd32x3::from(0.0), g5: Simd32x3::from(0.0), g6: Simd32x4::from(0.0), g7: Simd32x4::from(0.0) } }
    }
}

impl Neg for MultiVector {
    type Output = MultiVector;

    fn neg(self) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: swizzle!(self.group0(), 0, 1, 1) * Simd32x3::from(-1.0), g1: self.group1() * Simd32x4::from(-1.0), g2: self.group2() * Simd32x3::from(-1.0), g3: self.group3() * Simd32x3::from(-1.0), g4: self.group2() * Simd32x3::from(-1.0), g5: self.group3() * Simd32x3::from(-1.0), g6: self.group6() * Simd32x4::from(-1.0), g7: self.group6() * Simd32x4::from(-1.0) } }
    }
}

impl Automorphism for MultiVector {
    type Output = MultiVector;

    fn automorphism(self) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: swizzle!(self.group0(), 0, 1, 1), g1: self.group1() * Simd32x4::from(-1.0), g2: self.group2(), g3: self.group3(), g4: self.group2(), g5: self.group3(), g6: self.group6() * Simd32x4::from(-1.0), g7: self.group6() * Simd32x4::from(-1.0) } }
    }
}

impl Reversal for MultiVector {
    type Output = MultiVector;

    fn reversal(self) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: swizzle!(self.group0(), 0, 1, 1), g1: self.group1(), g2: self.group2() * Simd32x3::from(-1.0), g3: self.group3() * Simd32x3::from(-1.0), g4: self.group2() * Simd32x3::from(-1.0), g5: self.group3() * Simd32x3::from(-1.0), g6: self.group6() * Simd32x4::from(-1.0), g7: self.group6() * Simd32x4::from(-1.0) } }
    }
}

impl Conjugation for MultiVector {
    type Output = MultiVector;

    fn conjugation(self) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: swizzle!(self.group0(), 0, 1, 1), g1: self.group1() * Simd32x4::from(-1.0), g2: self.group2() * Simd32x3::from(-1.0), g3: self.group3() * Simd32x3::from(-1.0), g4: self.group2() * Simd32x3::from(-1.0), g5: self.group3() * Simd32x3::from(-1.0), g6: self.group6(), g7: self.group6() } }
    }
}

impl AntiReversal for MultiVector {
    type Output = MultiVector;

    fn anti_reversal(self) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: swizzle!(self.group0(), 0, 1, 1), g1: self.group1() * Simd32x4::from(-1.0), g2: self.group2() * Simd32x3::from(-1.0), g3: self.group3() * Simd32x3::from(-1.0), g4: self.group2() * Simd32x3::from(-1.0), g5: self.group3() * Simd32x3::from(-1.0), g6: self.group6(), g7: self.group6() } }
    }
}

impl DoubleComplement for MultiVector {
    type Output = MultiVector;

    fn double_complement(self) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: swizzle!(self.group0(), 0, 1, 1), g1: self.group1() * Simd32x4::from(-1.0), g2: self.group2(), g3: self.group3(), g4: self.group2(), g5: self.group3(), g6: self.group6() * Simd32x4::from(-1.0), g7: self.group6() * Simd32x4::from(-1.0) } }
    }
}

impl Into<Scalar> for MultiVector {
    fn into(self) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[0] } }
    }
}

impl RegressiveProduct<Scalar> for MultiVector {
    type Output = Scalar;

    fn regressive_product(self, other: Scalar) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[1] * other.group0() } }
    }
}

impl LeftContraction<Scalar> for MultiVector {
    type Output = Scalar;

    fn left_contraction(self, other: Scalar) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[0] * other.group0() } }
    }
}

impl ScalarProduct<Scalar> for MultiVector {
    type Output = Scalar;

    fn scalar_product(self, other: Scalar) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[0] * other.group0() } }
    }
}

impl OuterProduct<AntiScalar> for MultiVector {
    type Output = AntiScalar;

    fn outer_product(self, other: AntiScalar) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0()[0] * other.group0() } }
    }
}

impl LeftAntiContraction<AntiScalar> for MultiVector {
    type Output = AntiScalar;

    fn left_anti_contraction(self, other: AntiScalar) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0()[1] * other.group0() } }
    }
}

impl AntiScalarProduct<AntiScalar> for MultiVector {
    type Output = AntiScalar;

    fn anti_scalar_product(self, other: AntiScalar) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0()[1] * other.group0() } }
    }
}

impl ScalarProduct<HomogeneousMagnitude> for MultiVector {
    type Output = Scalar;

    fn scalar_product(self, other: HomogeneousMagnitude) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[0] * other.group0()[0] } }
    }
}

impl AntiScalarProduct<HomogeneousMagnitude> for MultiVector {
    type Output = AntiScalar;

    fn anti_scalar_product(self, other: HomogeneousMagnitude) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0()[1] * other.group0()[1] } }
    }
}

impl Into<Point> for MultiVector {
    fn into(self) -> Point {
        Point { groups: PointGroups { g0: self.group1() } }
    }
}

impl ScalarProduct<Point> for MultiVector {
    type Output = Scalar;

    fn scalar_product(self, other: Point) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group1()[0] * other.group0()[0] + self.group1()[1] * other.group0()[1] + self.group1()[2] * other.group0()[2] } }
    }
}

impl AntiScalarProduct<Point> for MultiVector {
    type Output = AntiScalar;

    fn anti_scalar_product(self, other: Point) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: 0.0 - self.group1()[3] * other.group0()[3] } }
    }
}

impl ScalarProduct<Line> for MultiVector {
    type Output = Scalar;

    fn scalar_product(self, other: Line) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group3()[0] * other.group1()[0] - self.group3()[1] * other.group1()[1] - self.group3()[2] * other.group1()[2] } }
    }
}

impl AntiScalarProduct<Line> for MultiVector {
    type Output = AntiScalar;

    fn anti_scalar_product(self, other: Line) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: 0.0 - self.group2()[0] * other.group0()[0] - self.group2()[1] * other.group0()[1] - self.group2()[2] * other.group0()[2] } }
    }
}

impl ScalarProduct<Plane> for MultiVector {
    type Output = Scalar;

    fn scalar_product(self, other: Plane) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group6()[3] * other.group0()[3] } }
    }
}

impl AntiScalarProduct<Plane> for MultiVector {
    type Output = AntiScalar;

    fn anti_scalar_product(self, other: Plane) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group6()[0] * other.group0()[0] + self.group6()[1] * other.group0()[1] + self.group6()[2] * other.group0()[2] } }
    }
}

impl ScalarProduct<Motor> for MultiVector {
    type Output = Scalar;

    fn scalar_product(self, other: Motor) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group3()[0] * other.group1()[0] - self.group3()[1] * other.group1()[1] - self.group3()[2] * other.group1()[2] } }
    }
}

impl AntiScalarProduct<Motor> for MultiVector {
    type Output = AntiScalar;

    fn anti_scalar_product(self, other: Motor) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0()[1] * other.group0()[3] - self.group2()[0] * other.group0()[0] - self.group2()[1] * other.group0()[1] - self.group2()[2] * other.group0()[2] } }
    }
}

impl AntiScalarProduct<Rotor> for MultiVector {
    type Output = AntiScalar;

    fn anti_scalar_product(self, other: Rotor) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0()[1] * other.group0()[3] - self.group2()[0] * other.group0()[0] - self.group2()[1] * other.group0()[1] - self.group2()[2] * other.group0()[2] } }
    }
}

impl ScalarProduct<Translator> for MultiVector {
    type Output = Scalar;

    fn scalar_product(self, other: Translator) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group3()[0] * other.group0()[0] - self.group3()[1] * other.group0()[1] - self.group3()[2] * other.group0()[2] } }
    }
}

impl AntiScalarProduct<Translator> for MultiVector {
    type Output = AntiScalar;

    fn anti_scalar_product(self, other: Translator) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0()[1] * other.group0()[3] } }
    }
}

impl ScalarProduct<Flector> for MultiVector {
    type Output = Scalar;

    fn scalar_product(self, other: Flector) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group1()[0] * other.group0()[0] + self.group1()[1] * other.group0()[1] + self.group1()[2] * other.group0()[2] - self.group6()[3] * other.group1()[3] } }
    }
}

impl AntiScalarProduct<Flector> for MultiVector {
    type Output = AntiScalar;

    fn anti_scalar_product(self, other: Flector) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: 0.0 - self.group1()[3] * other.group0()[3] + self.group6()[0] * other.group1()[0] + self.group6()[1] * other.group1()[1] + self.group6()[2] * other.group1()[2] } }
    }
}

impl ScalarProduct<MultiVector> for MultiVector {
    type Output = Scalar;

    fn scalar_product(self, other: MultiVector) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[0] * other.group0()[0] + self.group1()[0] * other.group1()[0] + self.group1()[1] * other.group1()[1] + self.group1()[2] * other.group1()[2] - self.group3()[0] * other.group3()[0] - self.group3()[1] * other.group3()[1] - self.group3()[2] * other.group3()[2] - self.group6()[3] * other.group6()[3] } }
    }
}

impl AntiScalarProduct<MultiVector> for MultiVector {
    type Output = AntiScalar;

    fn anti_scalar_product(self, other: MultiVector) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0()[1] * other.group0()[1] - self.group1()[3] * other.group1()[3] - self.group2()[0] * other.group2()[0] - self.group2()[1] * other.group2()[1] - self.group2()[2] * other.group2()[2] + self.group6()[0] * other.group6()[0] + self.group6()[1] * other.group6()[1] + self.group6()[2] * other.group6()[2] } }
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

impl BulkNorm for MultiVector {
    type Output = Scalar;

    fn bulk_norm(self) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.squared_magnitude().group0().sqrt() } }
    }
}

impl SquaredAntiMagnitude for MultiVector {
    type Output = AntiScalar;

    fn squared_anti_magnitude(self) -> AntiScalar {
        self.anti_scalar_product(self.anti_reversal())
    }
}

impl WeightNorm for MultiVector {
    type Output = AntiScalar;

    fn weight_norm(self) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.squared_anti_magnitude().group0().sqrt() } }
    }
}

impl GeometricNorm for MultiVector {
    type Output = HomogeneousMagnitude;

    fn geometric_norm(self) -> HomogeneousMagnitude {
        self.bulk_norm() + self.weight_norm()
    }
}

impl GeometricQuotient<HomogeneousMagnitude> for AntiScalar {
    type Output = AntiScalar;

    fn geometric_quotient(self, other: HomogeneousMagnitude) -> AntiScalar {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for AntiScalar {
    type Output = AntiScalar;

    fn geometric_quotient(self, other: Scalar) -> AntiScalar {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<HomogeneousMagnitude> for Flector {
    type Output = Flector;

    fn geometric_quotient(self, other: HomogeneousMagnitude) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Line> for Flector {
    type Output = Flector;

    fn geometric_quotient(self, other: Line) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Motor> for Flector {
    type Output = Flector;

    fn geometric_quotient(self, other: Motor) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for Flector {
    type Output = Flector;

    fn geometric_quotient(self, other: Scalar) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Translator> for Flector {
    type Output = Flector;

    fn geometric_quotient(self, other: Translator) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<AntiScalar> for HomogeneousMagnitude {
    type Output = AntiScalar;

    fn transformation(self, other: AntiScalar) -> AntiScalar {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl GeometricQuotient<Flector> for HomogeneousMagnitude {
    type Output = Flector;

    fn geometric_quotient(self, other: Flector) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Flector> for HomogeneousMagnitude {
    type Output = Flector;

    fn transformation(self, other: Flector) -> Flector {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl Powi for HomogeneousMagnitude {
    type Output = HomogeneousMagnitude;

    fn powi(self, exponent: isize) -> HomogeneousMagnitude {
        if exponent == 0 {
            return HomogeneousMagnitude::one();
        }
        let mut x: HomogeneousMagnitude = if exponent < 0 { self.inverse() } else { self };
        let mut y: HomogeneousMagnitude = HomogeneousMagnitude::one();
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

impl GeometricQuotient<HomogeneousMagnitude> for HomogeneousMagnitude {
    type Output = HomogeneousMagnitude;

    fn geometric_quotient(self, other: HomogeneousMagnitude) -> HomogeneousMagnitude {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<HomogeneousMagnitude> for HomogeneousMagnitude {
    type Output = HomogeneousMagnitude;

    fn transformation(self, other: HomogeneousMagnitude) -> HomogeneousMagnitude {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl GeometricQuotient<Line> for HomogeneousMagnitude {
    type Output = Line;

    fn geometric_quotient(self, other: Line) -> Line {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Line> for HomogeneousMagnitude {
    type Output = Line;

    fn transformation(self, other: Line) -> Line {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl GeometricQuotient<Motor> for HomogeneousMagnitude {
    type Output = Motor;

    fn geometric_quotient(self, other: Motor) -> Motor {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Motor> for HomogeneousMagnitude {
    type Output = Motor;

    fn transformation(self, other: Motor) -> Motor {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl Transformation<Rotor> for HomogeneousMagnitude {
    type Output = Rotor;

    fn transformation(self, other: Rotor) -> Rotor {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl GeometricQuotient<Scalar> for HomogeneousMagnitude {
    type Output = HomogeneousMagnitude;

    fn geometric_quotient(self, other: Scalar) -> HomogeneousMagnitude {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Scalar> for HomogeneousMagnitude {
    type Output = Scalar;

    fn transformation(self, other: Scalar) -> Scalar {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl GeometricQuotient<Translator> for HomogeneousMagnitude {
    type Output = Motor;

    fn geometric_quotient(self, other: Translator) -> Motor {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Translator> for HomogeneousMagnitude {
    type Output = Translator;

    fn transformation(self, other: Translator) -> Translator {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl GeometricQuotient<Flector> for Line {
    type Output = Flector;

    fn geometric_quotient(self, other: Flector) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Flector> for Line {
    type Output = Flector;

    fn transformation(self, other: Flector) -> Flector {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl GeometricQuotient<HomogeneousMagnitude> for Line {
    type Output = Line;

    fn geometric_quotient(self, other: HomogeneousMagnitude) -> Line {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Point> for Line {
    type Output = Flector;

    fn geometric_quotient(self, other: Point) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Point> for Line {
    type Output = Point;

    fn transformation(self, other: Point) -> Point {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl Transformation<Rotor> for Line {
    type Output = Rotor;

    fn transformation(self, other: Rotor) -> Rotor {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl GeometricQuotient<Scalar> for Line {
    type Output = Line;

    fn geometric_quotient(self, other: Scalar) -> Line {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Flector> for Motor {
    type Output = Flector;

    fn geometric_quotient(self, other: Flector) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Flector> for Motor {
    type Output = Flector;

    fn transformation(self, other: Flector) -> Flector {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl GeometricQuotient<HomogeneousMagnitude> for Motor {
    type Output = Motor;

    fn geometric_quotient(self, other: HomogeneousMagnitude) -> Motor {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Point> for Motor {
    type Output = Flector;

    fn geometric_quotient(self, other: Point) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Point> for Motor {
    type Output = Point;

    fn transformation(self, other: Point) -> Point {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl Transformation<Rotor> for Motor {
    type Output = Rotor;

    fn transformation(self, other: Rotor) -> Rotor {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl GeometricQuotient<Scalar> for Motor {
    type Output = Motor;

    fn geometric_quotient(self, other: Scalar) -> Motor {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Point> for Plane {
    type Output = Motor;

    fn geometric_quotient(self, other: Point) -> Motor {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for Plane {
    type Output = Plane;

    fn geometric_quotient(self, other: Scalar) -> Plane {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Line> for Point {
    type Output = Flector;

    fn geometric_quotient(self, other: Line) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Motor> for Point {
    type Output = Flector;

    fn geometric_quotient(self, other: Motor) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Plane> for Point {
    type Output = Motor;

    fn geometric_quotient(self, other: Plane) -> Motor {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Plane> for Point {
    type Output = Plane;

    fn transformation(self, other: Plane) -> Plane {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl GeometricQuotient<Scalar> for Point {
    type Output = Point;

    fn geometric_quotient(self, other: Scalar) -> Point {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<HomogeneousMagnitude> for Rotor {
    type Output = Rotor;

    fn geometric_quotient(self, other: HomogeneousMagnitude) -> Rotor {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Line> for Rotor {
    type Output = Rotor;

    fn geometric_quotient(self, other: Line) -> Rotor {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Motor> for Rotor {
    type Output = Rotor;

    fn geometric_quotient(self, other: Motor) -> Rotor {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for Rotor {
    type Output = Rotor;

    fn geometric_quotient(self, other: Scalar) -> Rotor {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Translator> for Rotor {
    type Output = Rotor;

    fn geometric_quotient(self, other: Translator) -> Rotor {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<AntiScalar> for Scalar {
    type Output = AntiScalar;

    fn transformation(self, other: AntiScalar) -> AntiScalar {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl GeometricQuotient<Flector> for Scalar {
    type Output = Flector;

    fn geometric_quotient(self, other: Flector) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Flector> for Scalar {
    type Output = Flector;

    fn transformation(self, other: Flector) -> Flector {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl GeometricQuotient<HomogeneousMagnitude> for Scalar {
    type Output = HomogeneousMagnitude;

    fn geometric_quotient(self, other: HomogeneousMagnitude) -> HomogeneousMagnitude {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<HomogeneousMagnitude> for Scalar {
    type Output = HomogeneousMagnitude;

    fn transformation(self, other: HomogeneousMagnitude) -> HomogeneousMagnitude {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl GeometricQuotient<Line> for Scalar {
    type Output = Line;

    fn geometric_quotient(self, other: Line) -> Line {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Line> for Scalar {
    type Output = Line;

    fn transformation(self, other: Line) -> Line {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl GeometricQuotient<Motor> for Scalar {
    type Output = Motor;

    fn geometric_quotient(self, other: Motor) -> Motor {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Motor> for Scalar {
    type Output = Motor;

    fn transformation(self, other: Motor) -> Motor {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl GeometricQuotient<Plane> for Scalar {
    type Output = Plane;

    fn geometric_quotient(self, other: Plane) -> Plane {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Plane> for Scalar {
    type Output = Plane;

    fn transformation(self, other: Plane) -> Plane {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl GeometricQuotient<Point> for Scalar {
    type Output = Point;

    fn geometric_quotient(self, other: Point) -> Point {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Point> for Scalar {
    type Output = Point;

    fn transformation(self, other: Point) -> Point {
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

impl GeometricQuotient<Translator> for Scalar {
    type Output = Translator;

    fn geometric_quotient(self, other: Translator) -> Translator {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Translator> for Scalar {
    type Output = Translator;

    fn transformation(self, other: Translator) -> Translator {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl GeometricQuotient<Flector> for Translator {
    type Output = Flector;

    fn geometric_quotient(self, other: Flector) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Flector> for Translator {
    type Output = Flector;

    fn transformation(self, other: Flector) -> Flector {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl GeometricQuotient<HomogeneousMagnitude> for Translator {
    type Output = Motor;

    fn geometric_quotient(self, other: HomogeneousMagnitude) -> Motor {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Rotor> for Translator {
    type Output = Rotor;

    fn transformation(self, other: Rotor) -> Rotor {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl GeometricQuotient<Scalar> for Translator {
    type Output = Translator;

    fn geometric_quotient(self, other: Scalar) -> Translator {
        self.geometric_product(other.inverse())
    }
}
