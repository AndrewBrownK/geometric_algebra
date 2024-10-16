pub mod arity_0 {
    pub use crate::traits::anti_grade::AntiGrade;
    pub use crate::traits::anti_one::AntiOne;
    pub use crate::traits::grade::Grade;
    pub use crate::traits::one::One;
    pub use crate::traits::unit::Unit;
    pub use crate::traits::zero::Zero;
}
pub mod arity_1 {
    pub use crate::traits::anti_constraint_violation::AntiConstraintViolation;
    pub use crate::traits::anti_fix_impl::AntiFixImpl;
    pub use crate::traits::anti_reverse::AntiReverse;
    pub use crate::traits::constraint_violation::ConstraintViolation;
    pub use crate::traits::fix::Fix;
    pub use crate::traits::reverse::Reverse;
    pub use crate::traits::right_anti_dual::RightAntiDual;
    pub use crate::traits::right_dual::RightDual;
}
pub mod arity_2 {
    pub use crate::traits::anti_sandwich::AntiSandwich;
    pub use crate::traits::anti_wedge::AntiWedge;
    pub use crate::traits::geometric_anti_product::GeometricAntiProduct;
    pub use crate::traits::geometric_product::GeometricProduct;
    pub use crate::traits::sandwich::Sandwich;
    pub use crate::traits::wedge::Wedge;
}
pub mod infix {
    pub use crate::traits::anti_constraint_violation::anti_constraint_violation;
    pub use crate::traits::anti_fix_impl::anti_fix_impl;
    pub use crate::traits::anti_reverse::anti_reverse;
    pub use crate::traits::anti_sandwich::anti_sandwich;
    pub use crate::traits::anti_sandwich::anti_sandwich_partial;
    pub use crate::traits::anti_wedge::anti_wedge;
    pub use crate::traits::anti_wedge::anti_wedge_partial;
    pub use crate::traits::constraint_violation::constraint_violation;
    pub use crate::traits::fix::fix;
    pub use crate::traits::geometric_anti_product::geometric_anti_product;
    pub use crate::traits::geometric_anti_product::geometric_anti_product_partial;
    pub use crate::traits::geometric_product::geometric_product;
    pub use crate::traits::geometric_product::geometric_product_partial;
    pub use crate::traits::reverse::reverse;
    pub use crate::traits::right_anti_dual::right_anti_dual;
    pub use crate::traits::right_dual::right_dual;
    pub use crate::traits::sandwich::sandwich;
    pub use crate::traits::sandwich::sandwich_partial;
    pub use crate::traits::wedge::wedge;
    pub use crate::traits::wedge::wedge_partial;
}
mod anti_constraint_violation;
pub use anti_constraint_violation::AntiConstraintViolation;
mod anti_fix_impl;
pub use anti_fix_impl::AntiFixImpl;
mod anti_grade;
pub use anti_grade::AntiGrade;
mod anti_one;
pub use anti_one::AntiOne;
mod anti_reverse;
pub use anti_reverse::AntiReverse;
mod anti_sandwich;
pub use anti_sandwich::AntiSandwich;
mod anti_wedge;
pub use anti_wedge::AntiWedge;
mod constraint_violation;
pub use constraint_violation::ConstraintViolation;
mod fix;
pub use fix::Fix;
mod geometric_anti_product;
pub use geometric_anti_product::GeometricAntiProduct;
mod geometric_product;
pub use geometric_product::GeometricProduct;
mod grade;
pub use grade::Grade;
mod one;
pub use one::One;
mod reverse;
pub use reverse::Reverse;
mod right_anti_dual;
pub use right_anti_dual::RightAntiDual;
mod right_dual;
pub use right_dual::RightDual;
mod sandwich;
pub use sandwich::Sandwich;
mod unit;
pub use unit::Unit;
mod wedge;
pub use wedge::Wedge;
mod zero;
pub use zero::Zero;
