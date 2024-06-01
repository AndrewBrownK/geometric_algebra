use std::num::NonZeroU8;
use crate::algebra2::basis::{BasisElement, GeneratorSquares, PrimaryBasis};
use crate::algebra2::metric::{ExoMetric, Metric};

pub trait GeoAlg {
    fn exomorphism_metric(&self, a: BasisElement, b: BasisElement) -> i8;
    fn anti_scalar(&self) -> BasisElement;
}

pub trait DiagonalGeoAlg {
    fn generator_squares(&self) -> GeneratorSquares;
    fn anti_scalar(&self) -> BasisElement;
}




pub struct DiagonalSquares<GA: DiagonalGeoAlg>(GA);
impl<GA: DiagonalGeoAlg> GeoAlg for DiagonalSquares<GA> {
    fn exomorphism_metric(&self, a: BasisElement, b: BasisElement) -> i8 {
        if a.grade() != b.grade() {
            return 0;
        }
        if a.signature() != b.signature() {
            return 0;
        }
        let gs = self.0.generator_squares();
        a.coefficient() * b.coefficient() * gs.square_element(a)
    }
    fn anti_scalar(&self) -> BasisElement {
        self.0.anti_scalar()
    }
}


pub struct GA_<GA: GeoAlg>(GA);
impl<GA: GeoAlg> GA_<GA> {
    // TODO tests
    pub fn geometric_product(&self, a: BasisElement, b: BasisElement) -> Vec<BasisElement> {
        // TODO implementation
        let w = a.wedge(b);
        let d = self.0.exomorphism_metric(a, b);
        vec![]
    }
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
// impl GeoAlg for VanillaArrows {
//     fn exomorphism_metric(&self, a: BasisElement, b: BasisElement) -> BasisElement {
//         todo!()
//     }
// }