#![cfg_attr(
    all(
        any(target_arch = "arm", target_arch = "aarch64"),
        target_feature = "neon"
    ),
    feature(stdsimd)
)]

// pub mod cga3d;
pub mod cga3d;
pub mod rga3d;
pub mod simd;

/// All elements set to `0.0`
pub trait Zero {
    fn zero() -> Self;
}

/// All elements set to `0.0`, except for the scalar, which is set to `1.0`
pub trait One {
    fn one() -> Self;
}

#[cfg(test)]
mod tests {
    #[test]
    fn dummy_test() {
        //
    }
}
