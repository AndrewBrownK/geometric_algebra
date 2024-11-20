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
    pub use crate::traits::anti_fix::AntiFix;
    pub use crate::traits::anti_reverse::AntiReverse;
    pub use crate::traits::anti_square_root::AntiSquareRoot;
    pub use crate::traits::center_norm::CenterNorm;
    pub use crate::traits::center_norm_squared::CenterNormSquared;
    pub use crate::traits::constraint_violation::ConstraintViolation;
    pub use crate::traits::fix::Fix;
    pub use crate::traits::flat_bulk::FlatBulk;
    pub use crate::traits::flat_bulk_norm::FlatBulkNorm;
    pub use crate::traits::flat_bulk_norm_squared::FlatBulkNormSquared;
    pub use crate::traits::flat_norm::FlatNorm;
    pub use crate::traits::flat_norm_squared::FlatNormSquared;
    pub use crate::traits::flat_weight::FlatWeight;
    pub use crate::traits::flat_weight_norm::FlatWeightNorm;
    pub use crate::traits::flat_weight_norm_squared::FlatWeightNormSquared;
    pub use crate::traits::radius_norm::RadiusNorm;
    pub use crate::traits::radius_norm_squared::RadiusNormSquared;
    pub use crate::traits::reverse::Reverse;
    pub use crate::traits::right_anti_dual::RightAntiDual;
    pub use crate::traits::right_dual::RightDual;
    pub use crate::traits::round_bulk::RoundBulk;
    pub use crate::traits::round_bulk_norm::RoundBulkNorm;
    pub use crate::traits::round_bulk_norm_squared::RoundBulkNormSquared;
    pub use crate::traits::round_norm::RoundNorm;
    pub use crate::traits::round_norm_squared::RoundNormSquared;
    pub use crate::traits::round_weight::RoundWeight;
    pub use crate::traits::round_weight_norm::RoundWeightNorm;
    pub use crate::traits::round_weight_norm_squared::RoundWeightNormSquared;
    pub use crate::traits::square_root::SquareRoot;
    pub use crate::traits::unitized_center_norm::UnitizedCenterNorm;
    pub use crate::traits::unitized_center_norm_squared::UnitizedCenterNormSquared;
    pub use crate::traits::unitized_flat_norm::UnitizedFlatNorm;
    pub use crate::traits::unitized_flat_norm_squared::UnitizedFlatNormSquared;
    pub use crate::traits::unitized_radius_norm::UnitizedRadiusNorm;
    pub use crate::traits::unitized_radius_norm_squared::UnitizedRadiusNormSquared;
    pub use crate::traits::unitized_round_norm::UnitizedRoundNorm;
    pub use crate::traits::unitized_round_norm_squared::UnitizedRoundNormSquared;
}
pub mod arity_2 {
    pub use crate::traits::anti_sandwich::AntiSandwich;
    pub use crate::traits::anti_scalar_product::AntiScalarProduct;
    pub use crate::traits::anti_wedge::AntiWedge;
    pub use crate::traits::geometric_anti_product::GeometricAntiProduct;
    pub use crate::traits::geometric_product::GeometricProduct;
    pub use crate::traits::sandwich::Sandwich;
    pub use crate::traits::scalar_product::ScalarProduct;
    pub use crate::traits::wedge::Wedge;
}
pub mod infix {
    pub use crate::traits::anti_constraint_violation::anti_constraint_violation;
    pub use crate::traits::anti_fix::anti_fix;
    pub use crate::traits::anti_reverse::anti_reverse;
    pub use crate::traits::anti_sandwich::anti_sandwich;
    pub use crate::traits::anti_sandwich::anti_sandwich_partial;
    pub use crate::traits::anti_scalar_product::anti_scalar_product;
    pub use crate::traits::anti_scalar_product::anti_scalar_product_partial;
    pub use crate::traits::anti_square_root::anti_square_root;
    pub use crate::traits::anti_wedge::anti_wedge;
    pub use crate::traits::anti_wedge::anti_wedge_partial;
    pub use crate::traits::center_norm::center_norm;
    pub use crate::traits::center_norm_squared::center_norm_squared;
    pub use crate::traits::constraint_violation::constraint_violation;
    pub use crate::traits::fix::fix;
    pub use crate::traits::flat_bulk::flat_bulk;
    pub use crate::traits::flat_bulk_norm::flat_bulk_norm;
    pub use crate::traits::flat_bulk_norm_squared::flat_bulk_norm_squared;
    pub use crate::traits::flat_norm::flat_norm;
    pub use crate::traits::flat_norm_squared::flat_norm_squared;
    pub use crate::traits::flat_weight::flat_weight;
    pub use crate::traits::flat_weight_norm::flat_weight_norm;
    pub use crate::traits::flat_weight_norm_squared::flat_weight_norm_squared;
    pub use crate::traits::geometric_anti_product::geometric_anti_product;
    pub use crate::traits::geometric_anti_product::geometric_anti_product_partial;
    pub use crate::traits::geometric_product::geometric_product;
    pub use crate::traits::geometric_product::geometric_product_partial;
    pub use crate::traits::radius_norm::radius_norm;
    pub use crate::traits::radius_norm_squared::radius_norm_squared;
    pub use crate::traits::reverse::reverse;
    pub use crate::traits::right_anti_dual::right_anti_dual;
    pub use crate::traits::right_dual::right_dual;
    pub use crate::traits::round_bulk::round_bulk;
    pub use crate::traits::round_bulk_norm::round_bulk_norm;
    pub use crate::traits::round_bulk_norm_squared::round_bulk_norm_squared;
    pub use crate::traits::round_norm::round_norm;
    pub use crate::traits::round_norm_squared::round_norm_squared;
    pub use crate::traits::round_weight::round_weight;
    pub use crate::traits::round_weight_norm::round_weight_norm;
    pub use crate::traits::round_weight_norm_squared::round_weight_norm_squared;
    pub use crate::traits::sandwich::sandwich;
    pub use crate::traits::sandwich::sandwich_partial;
    pub use crate::traits::scalar_product::scalar_product;
    pub use crate::traits::scalar_product::scalar_product_partial;
    pub use crate::traits::square_root::square_root;
    pub use crate::traits::unitized_center_norm::unitized_center_norm;
    pub use crate::traits::unitized_center_norm_squared::unitized_center_norm_squared;
    pub use crate::traits::unitized_flat_norm::unitized_flat_norm;
    pub use crate::traits::unitized_flat_norm_squared::unitized_flat_norm_squared;
    pub use crate::traits::unitized_radius_norm::unitized_radius_norm;
    pub use crate::traits::unitized_radius_norm_squared::unitized_radius_norm_squared;
    pub use crate::traits::unitized_round_norm::unitized_round_norm;
    pub use crate::traits::unitized_round_norm_squared::unitized_round_norm_squared;
    pub use crate::traits::wedge::wedge;
    pub use crate::traits::wedge::wedge_partial;
}
mod anti_constraint_violation;
pub use anti_constraint_violation::AntiConstraintViolation;
mod anti_fix;
pub use anti_fix::AntiFix;
mod anti_grade;
pub use anti_grade::AntiGrade;
mod anti_one;
pub use anti_one::AntiOne;
mod anti_reverse;
pub use anti_reverse::AntiReverse;
mod anti_sandwich;
pub use anti_sandwich::AntiSandwich;
mod anti_scalar_product;
pub use anti_scalar_product::AntiScalarProduct;
mod anti_square_root;
pub use anti_square_root::AntiSquareRoot;
mod anti_wedge;
pub use anti_wedge::AntiWedge;
mod center_norm;
pub use center_norm::CenterNorm;
mod center_norm_squared;
pub use center_norm_squared::CenterNormSquared;
mod constraint_violation;
pub use constraint_violation::ConstraintViolation;
mod fix;
pub use fix::Fix;
mod flat_bulk;
pub use flat_bulk::FlatBulk;
mod flat_bulk_norm;
pub use flat_bulk_norm::FlatBulkNorm;
mod flat_bulk_norm_squared;
pub use flat_bulk_norm_squared::FlatBulkNormSquared;
mod flat_norm;
pub use flat_norm::FlatNorm;
mod flat_norm_squared;
pub use flat_norm_squared::FlatNormSquared;
mod flat_weight;
pub use flat_weight::FlatWeight;
mod flat_weight_norm;
pub use flat_weight_norm::FlatWeightNorm;
mod flat_weight_norm_squared;
pub use flat_weight_norm_squared::FlatWeightNormSquared;
mod geometric_anti_product;
pub use geometric_anti_product::GeometricAntiProduct;
mod geometric_product;
pub use geometric_product::GeometricProduct;
mod grade;
pub use grade::Grade;
mod one;
pub use one::One;
mod radius_norm;
pub use radius_norm::RadiusNorm;
mod radius_norm_squared;
pub use radius_norm_squared::RadiusNormSquared;
mod reverse;
pub use reverse::Reverse;
mod right_anti_dual;
pub use right_anti_dual::RightAntiDual;
mod right_dual;
pub use right_dual::RightDual;
mod round_bulk;
pub use round_bulk::RoundBulk;
mod round_bulk_norm;
pub use round_bulk_norm::RoundBulkNorm;
mod round_bulk_norm_squared;
pub use round_bulk_norm_squared::RoundBulkNormSquared;
mod round_norm;
pub use round_norm::RoundNorm;
mod round_norm_squared;
pub use round_norm_squared::RoundNormSquared;
mod round_weight;
pub use round_weight::RoundWeight;
mod round_weight_norm;
pub use round_weight_norm::RoundWeightNorm;
mod round_weight_norm_squared;
pub use round_weight_norm_squared::RoundWeightNormSquared;
mod sandwich;
pub use sandwich::Sandwich;
mod scalar_product;
pub use scalar_product::ScalarProduct;
mod square_root;
pub use square_root::SquareRoot;
mod unit;
pub use unit::Unit;
mod unitized_center_norm;
pub use unitized_center_norm::UnitizedCenterNorm;
mod unitized_center_norm_squared;
pub use unitized_center_norm_squared::UnitizedCenterNormSquared;
mod unitized_flat_norm;
pub use unitized_flat_norm::UnitizedFlatNorm;
mod unitized_flat_norm_squared;
pub use unitized_flat_norm_squared::UnitizedFlatNormSquared;
mod unitized_radius_norm;
pub use unitized_radius_norm::UnitizedRadiusNorm;
mod unitized_radius_norm_squared;
pub use unitized_radius_norm_squared::UnitizedRadiusNormSquared;
mod unitized_round_norm;
pub use unitized_round_norm::UnitizedRoundNorm;
mod unitized_round_norm_squared;
pub use unitized_round_norm_squared::UnitizedRoundNormSquared;
mod wedge;
pub use wedge::Wedge;
mod zero;
pub use zero::Zero;
