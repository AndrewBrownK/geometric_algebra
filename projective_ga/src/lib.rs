
pub mod simd;

// TODO possibly make copies of these traits and put them in the dedicated crates instead of here
//  Same for simd

/// All elements set to `0.0`
pub trait Zero {
    fn zero() -> Self;
}

/// All elements set to `0.0`, except for the scalar, which is set to `1.0`
pub trait One {
    fn one() -> Self;
}

/// All elements set to `1.0`
pub trait Unit {
    fn unit() -> Self;
}