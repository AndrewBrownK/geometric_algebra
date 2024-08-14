pub mod arity_0 {
    pub use crate::traits::anti_grade::AntiGrade;
    pub use crate::traits::anti_one::AntiOne;
    pub use crate::traits::grade::Grade;
    pub use crate::traits::one::One;
    pub use crate::traits::unit::Unit;
    pub use crate::traits::zero::Zero;
}
pub mod arity_1 {
    pub use crate::traits::anti_dual::anti_dual;
    pub use crate::traits::anti_dual::AntiDual;
    pub use crate::traits::anti_reverse::anti_reverse;
    pub use crate::traits::anti_reverse::AntiReverse;
    pub use crate::traits::dual::dual;
    pub use crate::traits::dual::Dual;
    pub use crate::traits::reverse::reverse;
    pub use crate::traits::reverse::Reverse;
}
pub mod arity_2 {
    pub use crate::traits::anti_sandwich::anti_sandwich;
    pub use crate::traits::anti_sandwich::anti_sandwich_partial;
    pub use crate::traits::anti_sandwich::AntiSandwich;
    pub use crate::traits::anti_wedge::anti_wedge;
    pub use crate::traits::anti_wedge::anti_wedge_partial;
    pub use crate::traits::anti_wedge::AntiWedge;
    pub use crate::traits::geometric_anti_product::geometric_anti_product;
    pub use crate::traits::geometric_anti_product::geometric_anti_product_partial;
    pub use crate::traits::geometric_anti_product::GeometricAntiProduct;
    pub use crate::traits::geometric_product::geometric_product;
    pub use crate::traits::geometric_product::geometric_product_partial;
    pub use crate::traits::geometric_product::GeometricProduct;
    pub use crate::traits::sandwich::sandwich;
    pub use crate::traits::sandwich::sandwich_partial;
    pub use crate::traits::sandwich::Sandwich;
    pub use crate::traits::wedge::wedge;
    pub use crate::traits::wedge::wedge_partial;
    pub use crate::traits::wedge::Wedge;
}
mod anti_dual;
pub use crate::traits::anti_dual::anti_dual;
pub use anti_dual::AntiDual;
mod anti_grade;
pub use anti_grade::AntiGrade;
mod anti_one;
pub use anti_one::AntiOne;
mod anti_reverse;
pub use crate::traits::anti_reverse::anti_reverse;
pub use anti_reverse::AntiReverse;
mod anti_sandwich;
pub use crate::traits::anti_sandwich::anti_sandwich;
pub use crate::traits::anti_sandwich::anti_sandwich_partial;
pub use anti_sandwich::AntiSandwich;
mod anti_wedge;
pub use crate::traits::anti_wedge::anti_wedge;
pub use crate::traits::anti_wedge::anti_wedge_partial;
pub use anti_wedge::AntiWedge;
mod dual;
pub use crate::traits::dual::dual;
pub use dual::Dual;
mod geometric_anti_product;
pub use crate::traits::geometric_anti_product::geometric_anti_product;
pub use crate::traits::geometric_anti_product::geometric_anti_product_partial;
pub use geometric_anti_product::GeometricAntiProduct;
mod geometric_product;
pub use crate::traits::geometric_product::geometric_product;
pub use crate::traits::geometric_product::geometric_product_partial;
pub use geometric_product::GeometricProduct;
mod grade;
pub use grade::Grade;
mod one;
pub use one::One;
mod reverse;
pub use crate::traits::reverse::reverse;
pub use reverse::Reverse;
mod sandwich;
pub use crate::traits::sandwich::sandwich;
pub use crate::traits::sandwich::sandwich_partial;
pub use sandwich::Sandwich;
mod unit;
pub use unit::Unit;
mod wedge;
pub use crate::traits::wedge::wedge;
pub use crate::traits::wedge::wedge_partial;
pub use wedge::Wedge;
mod zero;
pub use zero::Zero;