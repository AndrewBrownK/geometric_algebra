pub mod base {
    pub use crate::data::origin::Origin;
    pub use crate::data::point::Point;
}
pub mod join_0 {
    pub use crate::data::origin::Origin;
    pub use crate::data::point::Point;
}
pub mod join_1 {
    pub use crate::data::line::Line;
}
pub mod join_2 {
    pub use crate::data::horizon::Horizon;
    pub use crate::data::plane::Plane;
}
pub mod meet_0 {
    pub use crate::data::horizon::Horizon;
    pub use crate::data::plane::Plane;
}
pub mod meet_1 {
    pub use crate::data::line::Line;
}
pub mod meet_2 {
    pub use crate::data::origin::Origin;
    pub use crate::data::point::Point;
}
pub mod reflection_0 {
    pub use crate::data::anti_scalar::AntiScalar;
}
pub mod reflection_1 {
    pub use crate::data::horizon::Horizon;
    pub use crate::data::plane::Plane;
}
pub mod reflection_2 {
    pub use crate::data::line::Line;
}
pub mod reflection_3 {
    pub use crate::data::flector::Flector;
    pub use crate::data::origin::Origin;
    pub use crate::data::point::Point;
}
pub mod reflection_4 {
    pub use crate::data::dual_num::DualNum;
    pub use crate::data::motor::Motor;
    pub use crate::data::scalar::Scalar;
}
pub mod vector_0 {
    pub use crate::data::scalar::Scalar;
}
pub mod vector_1 {
    pub use crate::data::origin::Origin;
    pub use crate::data::point::Point;
}
pub mod vector_2 {
    pub use crate::data::line::Line;
}
pub mod vector_3 {
    pub use crate::data::horizon::Horizon;
    pub use crate::data::plane::Plane;
}
pub mod vector_4 {
    pub use crate::data::anti_scalar::AntiScalar;
}
pub mod vector_mixed {
    pub use crate::data::dual_num::DualNum;
    pub use crate::data::flector::Flector;
    pub use crate::data::motor::Motor;
    pub use crate::data::multi_vector::MultiVector;
}
mod anti_scalar;
pub use anti_scalar::AntiScalar;
mod dual_num;
pub use dual_num::DualNum;
mod flector;
pub use flector::Flector;
mod horizon;
pub use horizon::Horizon;
mod line;
pub use line::Line;
mod motor;
pub use motor::Motor;
mod multi_vector;
pub use multi_vector::MultiVector;
mod origin;
pub use origin::Origin;
mod plane;
pub use plane::Plane;
mod point;
pub use point::Point;
mod scalar;
pub use scalar::Scalar;
