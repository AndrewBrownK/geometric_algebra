pub mod base {
    pub use crate::data::anti_plane::AntiPlane;
    pub use crate::data::round_point::RoundPoint;
}
pub mod join_0 {
    pub use crate::data::anti_plane::AntiPlane;
    pub use crate::data::round_point::RoundPoint;
}
pub mod join_1 {
    pub use crate::data::anti_line::AntiLine;
    pub use crate::data::dipole::Dipole;
    pub use crate::data::flat_point::FlatPoint;
}
pub mod join_2 {
    pub use crate::data::anti_flat_point::AntiFlatPoint;
    pub use crate::data::circle::Circle;
    pub use crate::data::line::Line;
}
pub mod join_3 {
    pub use crate::data::plane::Plane;
    pub use crate::data::sphere::Sphere;
}
pub mod meet_0 {
    pub use crate::data::plane::Plane;
    pub use crate::data::sphere::Sphere;
}
pub mod meet_1 {
    pub use crate::data::anti_flat_point::AntiFlatPoint;
    pub use crate::data::circle::Circle;
    pub use crate::data::line::Line;
}
pub mod meet_2 {
    pub use crate::data::anti_line::AntiLine;
    pub use crate::data::dipole::Dipole;
    pub use crate::data::flat_point::FlatPoint;
}
pub mod meet_3 {
    pub use crate::data::anti_plane::AntiPlane;
    pub use crate::data::round_point::RoundPoint;
}
pub mod reflection_0 {
    pub use crate::data::anti_scalar::AntiScalar;
}
pub mod reflection_1 {
    pub use crate::data::plane::Plane;
    pub use crate::data::sphere::Sphere;
}
pub mod reflection_2 {
    pub use crate::data::anti_flat_point::AntiFlatPoint;
    pub use crate::data::circle::Circle;
    pub use crate::data::circle_rotor::CircleRotor;
    pub use crate::data::line::Line;
}
pub mod reflection_3 {
    pub use crate::data::anti_line::AntiLine;
    pub use crate::data::dipole::Dipole;
    pub use crate::data::dipole_inversion::DipoleInversion;
    pub use crate::data::flat_point::FlatPoint;
    pub use crate::data::flector::Flector;
}
pub mod reflection_4 {
    pub use crate::data::anti_dipole_inversion::AntiDipoleInversion;
    pub use crate::data::anti_flector::AntiFlector;
    pub use crate::data::anti_plane::AntiPlane;
    pub use crate::data::dual_num::DualNum;
    pub use crate::data::motor::Motor;
    pub use crate::data::round_point::RoundPoint;
    pub use crate::data::versor_even::VersorEven;
}
pub mod reflection_5 {
    pub use crate::data::anti_circle_rotor::AntiCircleRotor;
    pub use crate::data::anti_dual_num::AntiDualNum;
    pub use crate::data::anti_motor::AntiMotor;
    pub use crate::data::scalar::Scalar;
    pub use crate::data::versor_odd::VersorOdd;
}
pub mod vector_0 {
    pub use crate::data::scalar::Scalar;
}
pub mod vector_1 {
    pub use crate::data::anti_plane::AntiPlane;
    pub use crate::data::round_point::RoundPoint;
}
pub mod vector_2 {
    pub use crate::data::anti_line::AntiLine;
    pub use crate::data::dipole::Dipole;
    pub use crate::data::flat_point::FlatPoint;
}
pub mod vector_3 {
    pub use crate::data::anti_flat_point::AntiFlatPoint;
    pub use crate::data::circle::Circle;
    pub use crate::data::line::Line;
}
pub mod vector_4 {
    pub use crate::data::plane::Plane;
    pub use crate::data::sphere::Sphere;
}
pub mod vector_5 {
    pub use crate::data::anti_scalar::AntiScalar;
}
pub mod vector_mixed {
    pub use crate::data::anti_circle_rotor::AntiCircleRotor;
    pub use crate::data::anti_dipole_inversion::AntiDipoleInversion;
    pub use crate::data::anti_dual_num::AntiDualNum;
    pub use crate::data::anti_flector::AntiFlector;
    pub use crate::data::anti_motor::AntiMotor;
    pub use crate::data::circle_rotor::CircleRotor;
    pub use crate::data::dipole_inversion::DipoleInversion;
    pub use crate::data::dual_num::DualNum;
    pub use crate::data::flector::Flector;
    pub use crate::data::motor::Motor;
    pub use crate::data::multi_vector::MultiVector;
    pub use crate::data::versor_even::VersorEven;
    pub use crate::data::versor_odd::VersorOdd;
}
mod anti_circle_rotor;
pub use anti_circle_rotor::AntiCircleRotor;
mod anti_dipole_inversion;
pub use anti_dipole_inversion::AntiDipoleInversion;
mod anti_dual_num;
pub use anti_dual_num::AntiDualNum;
mod anti_flat_point;
pub use anti_flat_point::AntiFlatPoint;
mod anti_flector;
pub use anti_flector::AntiFlector;
mod anti_line;
pub use anti_line::AntiLine;
mod anti_motor;
pub use anti_motor::AntiMotor;
mod anti_plane;
pub use anti_plane::AntiPlane;
mod anti_scalar;
pub use anti_scalar::AntiScalar;
mod circle;
pub use circle::Circle;
mod circle_rotor;
pub use circle_rotor::CircleRotor;
mod dipole;
pub use dipole::Dipole;
mod dipole_inversion;
pub use dipole_inversion::DipoleInversion;
mod dual_num;
pub use dual_num::DualNum;
mod flat_point;
pub use flat_point::FlatPoint;
mod flector;
pub use flector::Flector;
mod line;
pub use line::Line;
mod motor;
pub use motor::Motor;
mod multi_vector;
pub use multi_vector::MultiVector;
mod plane;
pub use plane::Plane;
mod round_point;
pub use round_point::RoundPoint;
mod scalar;
pub use scalar::Scalar;
mod sphere;
pub use sphere::Sphere;
mod versor_even;
pub use versor_even::VersorEven;
mod versor_odd;
pub use versor_odd::VersorOdd;
