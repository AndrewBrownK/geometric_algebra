use std::ops::{Add, AddAssign, Mul, MulAssign};

#[derive(Clone, Copy)]
pub struct OperationsTracker {
    pub add_sub: usize,
    pub mul: usize,
    pub div: usize,
    // TODO add more
}



impl OperationsTracker {
    pub fn zero() -> Self {
        Self {
            add_sub: 0,
            mul: 0,
            div: 0,
        }
    }
}

impl Mul<usize> for OperationsTracker {
    type Output = OperationsTracker;

    fn mul(self, rhs: usize) -> Self::Output {
        Self {
            add_sub: rhs * self.add_sub,
            mul: rhs * self.mul,
            div: rhs * self.div,
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
}
impl VectoredOperationsTracker {
    pub fn zero() -> Self {
        Self {
            floats: OperationsTracker::zero(),
            simd2: OperationsTracker::zero(),
            simd3: OperationsTracker::zero(),
            simd4: OperationsTracker::zero(),
        }
    }

    pub fn without_simd(self) -> OperationsTracker {
        let mut f = self.floats;
        f += self.simd2 * 2;
        f += self.simd3 * 3;
        f += self.simd4 * 4;
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
        }
    }
}
impl AddAssign<VectoredOperationsTracker> for VectoredOperationsTracker {
    fn add_assign(&mut self, rhs: VectoredOperationsTracker) {
        self.floats += rhs.floats;
        self.simd2 += rhs.simd2;
        self.simd3 += rhs.simd3;
        self.simd4 += rhs.simd4;
    }
}