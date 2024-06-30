use crate::algebra2::basis::arithmetic::{GradedProduct, Product, Sum};
use crate::algebra2::basis::{BasisElement, BasisSignature};
use crate::algebra2::basis::generators::GeneratorSquares;
use crate::algebra2::basis::grades::grade0;
use crate::algebra2::basis::substitute::SubstitutionRepository;

pub struct GeometricAlgebra(
    SubstitutionRepository
);

impl GeometricAlgebra {
    pub fn from_squares(generator_squares: GeneratorSquares) -> Self {
        Self::from_substitutions(SubstitutionRepository::new(generator_squares, vec![]))
    }

    pub fn from_substitutions(substitution_repository: SubstitutionRepository) -> Self {
        Self(substitution_repository)
    }

    pub fn anti_scalar(&self) -> BasisElement {
        self.0.anti_scalar()
    }

    pub fn product(&self, a: BasisElement, b: BasisElement) -> Sum {
        self.0.product(a, b)
    }

    pub fn anti_product(&self, a: BasisElement, b: BasisElement) -> Sum {
        self.0.anti_product(a, b)
    }

    // // TODO you might want to check out page 134 before you settle into this.
    // //  Additionally, different factions can't even agree on what the dot product is
    // //  I'm probably better off deriving an exomorphism metric instead of grade selecting the product.
    // pub fn dot(&self, a: BasisElement, b: BasisElement) -> Product {
    //     // Feels a bit dirty and expensive to implement dot product this way.
    //     // But since this is code generation and not actual number crunching,
    //     // it should be fine... Not that I wouldn't happily accept something
    //     // better.
    //     let mut s = self.product(a, b);
    //     s.sum.retain(|p| p.element.grade() == 0);
    //     s.sort_and_simplify();
    //     if s.sum.is_empty() {
    //         return Product {
    //             coefficient: 0.0,
    //             element: BasisElement::scalar(),
    //         };
    //     }
    //     assert_eq!(s.sum.len(), 1);
    //     let p = s.sum[0];
    //     assert_eq!(p.element.signature(), BasisSignature::scalar);
    //     p
    // }
    //
    // pub fn anti_dot(&self, a: BasisElement, b: BasisElement) -> Product {
    //     // Feels a bit dirty and expensive to implement dot product this way.
    //     // But since this is code generation and not actual number crunching,
    //     // it should be fine... Not that I wouldn't happily accept something
    //     // better.
    //     let anti_scalar = self.anti_scalar();
    //     let anti_scalar_grade = anti_scalar.grade();
    //     let mut s = self.anti_product(a, b);
    //     s.sum.retain(|p| p.element.grade() == anti_scalar_grade);
    //     s.sort_and_simplify();
    //     if s.sum.is_empty() {
    //         return Product {
    //             coefficient: 0.0,
    //             element: anti_scalar,
    //         };
    //     }
    //     assert_eq!(s.sum.len(), 1);
    //     let p = s.sum[0];
    //     assert_eq!(p.element.signature(), anti_scalar.signature());
    //     p
    // }

    // pub fn right_complement(&self, a: BasisElement) -> BasisElement {
    //     self.
    // }

    // pub fn wedge(&self, a: BasisElement, b: BasisElement) -> Option<BasisElement> {
    //     let c = a.wedge(b);
    //     if c.coefficient() == 0 {
    //         return None;
    //     }
    //     Some(c)
    // }
    //
    // pub fn anti_wedge(&self, a: BasisElement, b: BasisElement) -> Option<BasisElement> {
    //
    // }
}

