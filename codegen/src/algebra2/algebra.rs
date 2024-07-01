use crate::algebra2::basis::arithmetic::{GradedProduct, Product, Sum};
use crate::algebra2::basis::{BasisElement, BasisSignature};
use crate::algebra2::basis::generators::GeneratorSquares;
use crate::algebra2::basis::grades::grade0;
use crate::algebra2::basis::substitute::SubstitutionRepository;

// TODO maybe you want to give it a name? and some other properties too?
//  If you can't come up with something, you might want to leave it at SubstitutionRepository
//  after all. Could also use this location to recover the names of BasisElements that might
//  have been lost in calculation.
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

    // TODO SEE PAGE 237 THAT IS CONFIRMATION THAT SUBSTITUTIONS WERE NECESSARY IN ORDER
    //  TO EVALUATE THE METRIC OKEY DOKEY

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

/*
Big discussions from very smart people
Nobody can agree what a dot product is
Or if there's a distinction between inner product and interior products
So it's stuff like this that makes a multi-dialect library extremely challenging,
but also hopefully extremely useful. What utility multi-dialect loses in its
ability to resolve conflicts, hopefully it can make up by allowing rich experimentation.



Sudgy (asked for distinction between dot product and scalar product):

"The scalar part of the geometric product" is usually called the scalar product (although some
authors define the scalar product to also reverse the first argument, but that's a minor detail)
The dot product of a j-vector A and a k-vector B is ⟨AB⟩_|j - k|





Eric (speaking on interior vs inner products, and grade selection of geometric product):

BY DEFINITION, an inner product generates a scalar value.
Those are interior products.
Furthermore, the inner product defined by most GA authors is not valid, even if you only take the scalar part of the result.
If you want to define the inner product in terms of the geometric product, then it must be a ⋅ b = ⟨ab̃⟩₀, with one of the factors reversed.
However, this is really just an identity, not a definition. A proper definition does not depend on the geometric product at all.

Depending on whether the metric is degenerate, there are either two or four types of interior product
that are different in nontrivial ways. <ab̃>_|gr(a)-gr(b)| gives you one (the bulk contraction), but only up to sign. It's a huge topic.

(blah blah blah, can't only use wedge, must also include dualized operand)

The term "interior product" dates back to Grassmann himself, and it is well understood in related
fields like differential geometry. Most GA folks, by deriving definitions of contraction and inner
product from the geometric product, have accidentally gotten it a little wrong, and they have no
idea how these relate to exterior products and duals. The whole intrinsic / extrinsic thing
unwittingly relies on relationships like Equation (2.119) in my book, which says that for a and b
of the same grade, a ∨ bulkdual(b) = a • b, where • is the inner product. This hides an implicit
dualization and obfuscates the actual operations that are happening, all because of a belief that
the geometric product is fundamental. IMO, focusing on OPNS vs IPNS is the wrong approach, and a
proper understanding of the complete exterior algebra makes it clear that the real focus should be
product vs antiproduct.

(image of identities for expansoin/contraction with grade selection of geometric product and fancy sign derivation)

With a nondegenerate metric (as in CGA), some dualizing can be used to compute the right side of
the bottom identity with the geometric product instead of the antiproduct, but that won't hold up
in a degenerate setting (like PGA).

The existence of the second identity means that any weight expansion calculated with the wedge
product and weight dual could also be calculated with a geometric product, grade selection, and
sign adjustment. A possibly dualized and/or negated version of the answer you're looking for shows
up inside the geometric product, but it's clearly nowhere near as clean as the left sides of these
equations, and once again, the geometric product is overcalculating because it is not fundamental.
(I see the need for grade selection in any formula as an indication that a simpler operation exists.)
I do find it very interesting that the geometric product contains the bulk contraction,
which is defined by an antiwedge product.

Btw, everybody, many texts define the "right contraction" by the right side of the top equation
without the sign adjustment, but that is wrong because it doesn't agree with the inner product
mandated by the extension of the metric to all grades. The sign adjustment must be there.
This is another example where it's easy to make a mistake by deriving simpler operations from the
geometric product. It's much better to use the left side of the equation without leaving the more
fundamental exterior algebra.

A contraction defined by <AB>_|grade(A)-grade(B)| often has the wrong sign, and it doesn't
always reduce to the inner product with the correct sign when gr(A) = gr(B).

That's a rather long discussion, and it starts with ⟨AB⟩₀ not producing the correct sign of the
inner product. It does involve norms, but also angles and the extension of linear transformations
from the base vector space to the entire exterior algebra.
The proper foundations are all laid out very thorou




 */