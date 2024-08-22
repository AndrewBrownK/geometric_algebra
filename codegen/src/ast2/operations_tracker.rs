use crate::ast2::datatype::MultiVector;
use crate::ast2::traits::{RawTraitImplementation, TraitKey};
use std::collections::HashMap;
use std::ops::{Add, AddAssign, Mul, MulAssign};
use std::sync::Arc;

#[derive(Clone, Copy)]
pub struct OperationsTracker {
    pub add_sub: usize,
    pub mul: usize,
    pub div: usize,
    pub pow: usize,
    // TODO add more
}

impl OperationsTracker {
    pub fn zero() -> Self {
        Self { add_sub: 0, mul: 0, div: 0, pow: 0 }
    }
    pub fn is_zero(&self) -> bool {
        self.add_sub == 0 && self.mul == 0 && self.div == 0
    }
}

impl Mul<usize> for OperationsTracker {
    type Output = OperationsTracker;

    fn mul(self, rhs: usize) -> Self::Output {
        Self {
            add_sub: rhs * self.add_sub,
            mul: rhs * self.mul,
            div: rhs * self.div,
            pow: rhs * self.pow
        }
    }
}
impl MulAssign<usize> for OperationsTracker {
    fn mul_assign(&mut self, rhs: usize) {
        self.add_sub *= rhs;
        self.mul *= rhs;
        self.div *= rhs;
    }
}
impl Add<OperationsTracker> for OperationsTracker {
    type Output = OperationsTracker;

    fn add(self, rhs: OperationsTracker) -> Self::Output {
        Self {
            add_sub: self.add_sub + rhs.add_sub,
            mul: self.mul + rhs.mul,
            div: self.div + rhs.div,
            pow: self.pow + rhs.pow,
        }
    }
}
impl AddAssign<OperationsTracker> for OperationsTracker {
    fn add_assign(&mut self, rhs: OperationsTracker) {
        self.add_sub += rhs.add_sub;
        self.mul += rhs.mul;
        self.div += rhs.div;
    }
}

#[derive(Clone, Copy)]
pub struct VectoredOperationsTracker {
    pub floats: OperationsTracker,
    pub simd2: OperationsTracker,
    pub simd3: OperationsTracker,
    pub simd4: OperationsTracker,
    pub basis_element_struct_access: bool,
}
impl VectoredOperationsTracker {
    pub fn zero() -> Self {
        Self {
            floats: OperationsTracker::zero(),
            simd2: OperationsTracker::zero(),
            simd3: OperationsTracker::zero(),
            simd4: OperationsTracker::zero(),
            basis_element_struct_access: false,
        }
    }

    pub fn without_simd(self) -> OperationsTracker {
        let mut f = self.floats;
        f += self.simd2 * 2;
        f += self.simd3 * 3;
        f += self.simd4 * 4;
        f
    }

    pub fn with_simd(self) -> OperationsTracker {
        let mut f = self.floats;
        f += self.simd2;
        f += self.simd3;
        f += self.simd4;
        f
    }
}

impl Add<VectoredOperationsTracker> for VectoredOperationsTracker {
    type Output = Self;

    fn add(self, rhs: VectoredOperationsTracker) -> Self::Output {
        Self {
            floats: self.floats + rhs.floats,
            simd2: self.simd2 + rhs.simd2,
            simd3: self.simd3 + rhs.simd3,
            simd4: self.simd4 + rhs.simd4,
            basis_element_struct_access: self.basis_element_struct_access || rhs.basis_element_struct_access,
        }
    }
}
impl AddAssign<VectoredOperationsTracker> for VectoredOperationsTracker {
    fn add_assign(&mut self, rhs: VectoredOperationsTracker) {
        self.floats += rhs.floats;
        self.simd2 += rhs.simd2;
        self.simd3 += rhs.simd3;
        self.simd4 += rhs.simd4;
        self.basis_element_struct_access |= rhs.basis_element_struct_access;
    }
}

pub(crate) trait TrackOperations {
    fn count_operations(&self, lookup: &TraitOperationsLookup) -> VectoredOperationsTracker;
}

pub(crate) struct TraitOperationsLookup<'a> {
    pub(crate) traits10: &'a HashMap<(TraitKey, MultiVector), Arc<RawTraitImplementation>>,
    pub(crate) traits11: &'a HashMap<(TraitKey, MultiVector), Arc<RawTraitImplementation>>,
    pub(crate) traits21: &'a HashMap<(TraitKey, MultiVector, MultiVector), Arc<RawTraitImplementation>>,
    pub(crate) traits22: &'a HashMap<(TraitKey, MultiVector, MultiVector), Arc<RawTraitImplementation>>,
}

impl<'a> TraitOperationsLookup<'a> {
    pub fn trait_10_ops(&self, k: &TraitKey, a: &MultiVector) -> VectoredOperationsTracker {
        if let Some(rti) = self.traits10.get(&(*k, *a)) {
            return rti.statistics;
        }
        panic!(
            "Attempted to look up the VectorOperationsTracker of a trait_10 that was not \
            found as a dependency. This is probably a bug in forgetting to record a dependency."
        )
    }
    pub fn trait_11_ops(&self, k: &TraitKey, a: &MultiVector) -> VectoredOperationsTracker {
        if let Some(rti) = self.traits11.get(&(*k, *a)) {
            return rti.statistics;
        }
        panic!(
            "Attempted to look up the VectorOperationsTracker of a trait_11 that was not \
            found as a dependency. This is probably a bug in forgetting to record a dependency."
        )
    }
    pub fn trait_21_ops(&self, k: &TraitKey, a: &MultiVector, b: &MultiVector) -> VectoredOperationsTracker {
        if let Some(rti) = self.traits21.get(&(*k, *a, *b)) {
            return rti.statistics;
        }
        panic!(
            "Attempted to look up the VectorOperationsTracker of a trait_21 that was not \
            found as a dependency. This is probably a bug in forgetting to record a dependency."
        )
    }
    pub fn trait_22_ops(&self, k: &TraitKey, a: &MultiVector, b: &MultiVector) -> VectoredOperationsTracker {
        if let Some(rti) = self.traits22.get(&(*k, *a, *b)) {
            return rti.statistics;
        }
        panic!(
            "Attempted to look up the VectorOperationsTracker of a trait_22 that was not \
            found as a dependency. This is probably a bug in forgetting to record a dependency."
        )
    }
}
