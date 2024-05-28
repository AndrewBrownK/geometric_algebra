use std::num::NonZeroU8;
use crate::algebra2::basis::{BasisElement, GeneratorSquares, PrimaryBasis};
use crate::algebra2::metric::{ExoMetric, Metric};

pub trait GeoAlg {
    fn true_generator_squares(&self) -> GeneratorSquares;

    // But what about????
    // - anti_scalar: not needed because it's in true_generator_squares
    // - wedge_product: not needed because BasisElement primitive wedge is just it
    // - metric/exo_metric: the whole point of this foundation layer trait is to not have a metric yet.
}
pub trait GeoAlgWithMetric {
    fn anti_scalar(&self) -> BasisElement;
    fn metric(&self) -> Metric;
    fn exo_metric(&self) -> ExoMetric;
}




pub struct VanillaArrows {
    dimensions: u8,
    squares: GeneratorSquares
}
impl VanillaArrows {
    pub fn new(first_basis: PrimaryBasis, dimensions: NonZeroU8) -> Self {
        let dimensions = dimensions.get();
        let mut d_remaining = dimensions;
        let mut squares = GeneratorSquares::new([(first_basis, 1i8)]);
        d_remaining -= 1;
        while d_remaining > 0 {
            d_remaining -= 1;
            let _: anyhow::Result<()> = try {
                let next_basis = squares.next_available_basis()?;
                squares = squares.append([(next_basis, 1i8)])?;
                ()
            };
        }
        VanillaArrows { squares, dimensions }
    }
}
impl GeoAlg for VanillaArrows {
    fn true_generator_squares(&self) -> GeneratorSquares {
        self.squares
    }
}