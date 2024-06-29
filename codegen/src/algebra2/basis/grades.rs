

use std::marker::{ConstParamTy};

// Cannot use macro because that makes it incompatible with ConstParamTy
// bitflags::bitflags! {
//     #[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
//     pub struct Grades: u16 {
//         const g0 = 0x1;
//         const g1 = 0x2;
//         const g2 = 0x4;
//         const g3 = 0x8;
//         const g4 = 0x10;
//         const g5 = 0x20;
//         const g6 = 0x40;
//         const g7 = 0x80;
//         const g8 = 0x100;
//         const g9 = 0x200;
//         const g10 = 0x400;
//         const g11 = 0x800;
//         const g12 = 0x1000;
//         const g13 = 0x2000;
//         const g14 = 0x4000;
//         const g15 = 0x8000;
//     }
// }


use std::cmp::{PartialEq};
use std::hash::{Hash};
use std::ops::{BitOr, BitAnd, Not};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Grades(u16);

#[allow(non_upper_case_globals)]
impl Grades {
    pub const g0: Self = Grades(0x1);
    pub const g1: Self = Grades(0x2);
    pub const g2: Self = Grades(0x4);
    pub const g3: Self = Grades(0x8);
    pub const g4: Self = Grades(0x10);
    pub const g5: Self = Grades(0x20);
    pub const g6: Self = Grades(0x40);
    pub const g7: Self = Grades(0x80);
    pub const g8: Self = Grades(0x100);
    pub const g9: Self = Grades(0x200);
    pub const g10: Self = Grades(0x400);
    pub const g11: Self = Grades(0x800);
    pub const g12: Self = Grades(0x1000);
    pub const g13: Self = Grades(0x2000);
    pub const g14: Self = Grades(0x4000);
    pub const g15: Self = Grades(0x8000);

    pub const fn into_bits(self) -> u16 {
        self.0
    }

    pub const fn const_bitor(self, rhs: Self) -> Self {
        Grades(self.0 | rhs.0)
    }

    pub const fn const_bitand(self, rhs: Self) -> Self {
        Grades(self.0 & rhs.0)
    }
    pub const fn const_not(self) -> Self {
        Grades(!self.0)
    }
}
impl BitOr for Grades {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Grades(self.0 | rhs.0)
    }
}
impl BitAnd for Grades {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Grades(self.0 & rhs.0)
    }
}
impl Not for Grades {
    type Output = Self;

    fn not(self) -> Self::Output {
        Grades(!self.0)
    }
}


// Implement `ConstParamTy` for `Grades`
impl ConstParamTy for Grades {}


#[macro_export]
macro_rules! grade_constraint {
    ($g:ty, $h:ty) => {
        [(); <crate::algebra2::basis::grades::AddGradesImpl as crate::algebra2::basis::grades::AddGradesTrait<$g, $h>>::OUTPUT.into_bits() as usize]
    };
}

pub struct AddGradesImpl;
pub trait AddGradesTrait<const G: Grades, const H: Grades> {
    type Output;
    const OUTPUT: Self::Output;
}
impl<const G: Grades, const H: Grades> AddGradesTrait<G, H> for AddGradesImpl {
    type Output = Grades;
    const OUTPUT: Grades = G.const_bitor(H);
}