pub mod arity_0 {
    pub use crate::traits::anti_grade::AntiGrade;
    pub use crate::traits::anti_one::AntiOne;
    pub use crate::traits::grade::Grade;
    pub use crate::traits::one::One;
    pub use crate::traits::unit::Unit;
    pub use crate::traits::zero::Zero;
}
pub mod arity_1 {
    pub use crate::traits::dual::dual;
    pub use crate::traits::dual::Dual;
}
pub mod arity_2 {
    pub use crate::traits::anti_wedge::anti_wedge;
    pub use crate::traits::anti_wedge::anti_wedge_partial;
    pub use crate::traits::anti_wedge::AntiWedge;
    pub use crate::traits::bulk_expansion::bulk_expansion;
    pub use crate::traits::bulk_expansion::bulk_expansion_partial;
    pub use crate::traits::bulk_expansion::BulkExpansion;
    pub use crate::traits::geometric_anti_product::geometric_anti_product;
    pub use crate::traits::geometric_anti_product::geometric_anti_product_partial;
    pub use crate::traits::geometric_anti_product::GeometricAntiProduct;
    pub use crate::traits::geometric_product::geometric_product;
    pub use crate::traits::geometric_product::geometric_product_partial;
    pub use crate::traits::geometric_product::GeometricProduct;
    pub use crate::traits::wedge::wedge;
    pub use crate::traits::wedge::wedge_partial;
    pub use crate::traits::wedge::Wedge;
}
mod anti_grade;
pub use anti_grade::AntiGrade;
mod anti_one;
pub use anti_one::AntiOne;
mod anti_wedge;
pub use crate::traits::anti_wedge::anti_wedge;
pub use crate::traits::anti_wedge::anti_wedge_partial;
pub use anti_wedge::AntiWedge;
mod bulk_expansion;
pub use crate::traits::bulk_expansion::bulk_expansion;
pub use crate::traits::bulk_expansion::bulk_expansion_partial;
pub use bulk_expansion::BulkExpansion;
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
mod unit;
pub use unit::Unit;
mod wedge;
pub use crate::traits::wedge::wedge;
pub use crate::traits::wedge::wedge_partial;
pub use wedge::Wedge;
mod zero;
pub use zero::Zero;
