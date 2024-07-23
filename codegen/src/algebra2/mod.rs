use std::marker::PhantomData;
use std::sync::Arc;

use parking_lot::RwLock;

use crate::algebra2::basis::{BasisElement, BasisElementNames};
use crate::algebra2::basis::arithmetic::Sum;
use crate::algebra2::basis::generators::GeneratorSquares;
use crate::algebra2::basis::substitutes::Substitutions;
use crate::algebra2::multivector::{MultiVec};

pub mod basis;
pub mod multivector;


pub struct GeometricAlgebra<const AntiScalar: BasisElement> {
    repo: Substitutions,
    named_bases: RwLock<BasisElementNames>,
}



#[macro_export]
macro_rules! ga {
    ($anti_scalar:ident; $( $i8_lit:expr => $( $generator:expr ),+ $(,)? );+ $(;)? ) => {
        {
            use $crate::algebra2::basis::generators::*;
            let gs = {
                use $crate::algebra2::basis::generators::*;
                let mut gs = $crate::algebra2::basis::generators::GeneratorSquares::empty();
                $($(gs = gs.overwrite([($generator, $i8_lit)]);)+)+
                gs
            };
            $crate::algebra2::GeometricAlgebra::<$anti_scalar>::from_squares(gs)
        }
    };
    ($anti_scalar:ident; $( $i8_lit:expr => $( $generator:expr ),+ $(,)? );+ ; where $( $generator_element:expr => $sum:expr );+ $(;)? ) => {
        {
            use $crate::algebra2::basis::generators::*;
            let gs = {
                let mut gs = $crate::algebra2::basis::generators::GeneratorSquares::empty();
                $($(gs = gs.overwrite([($generator, $i8_lit)]);)+)+
                gs
            };
            let subs = {
                vec![$(($generator_element, $sum)),+]
            };
            let subs = $crate::algebra2::basis::substitutes::Substitutions::new(gs, subs);
            $crate::algebra2::GeometricAlgebra::<$anti_scalar>::from_substitutions(subs)
        }
    };
}

impl<const AntiScalar: BasisElement> GeometricAlgebra<AntiScalar> {
    pub fn from_squares(generator_squares: GeneratorSquares) -> Arc<Self> {
        Self::from_substitutions(Substitutions::new(generator_squares, vec![]))
    }

    pub fn from_substitutions(substitutions: Substitutions) -> Arc<Self> {
        let ty_anti_scalar = AntiScalar;
        let rt_anti_scalar = substitutions.anti_scalar();
        if ty_anti_scalar.signature() != rt_anti_scalar.signature() {
            panic!("Substitutions AntiScalar({rt_anti_scalar}) does not match GeometricAlgebra AntiScalar({ty_anti_scalar})");
        }
        Arc::new(Self { repo: substitutions, named_bases: RwLock::new(BasisElementNames::new()) })
    }

    fn name_in(&self, el: BasisElement) -> BasisElement {
        if !self.named_bases.read().may_accept(el) {
            return el;
        }
        match self.named_bases.write().accept_name(el) {
            Ok(_) => el,
            Err(err) => panic!("Conflicting element names for {el:?}: {err:?}"),
        }
    }

    fn name_out(&self, el: BasisElement) -> BasisElement {
        self.named_bases.read().provide_name(el)
    }

    fn name_and_sign_out(&self, el: BasisElement) -> BasisElement {
        self.named_bases.read().provide_name_and_sign(el)
    }

    fn name_out_sum(&self, mut sum: Sum) -> Sum {
        for p in sum.sum.iter_mut() {
            p.element = self.name_out(p.element);
        }
        sum
    }

    pub fn anti_scalar(&self) -> BasisElement {
        self.name_out(self.repo.anti_scalar())
    }

    pub fn product(&self, a: BasisElement, b: BasisElement) -> Sum {
        let a = self.name_in(a);
        let b =self.name_in(b);
        self.name_out_sum(self.repo.product(a, b))
    }

    pub fn anti_product(&self, a: BasisElement, b: BasisElement) -> Sum {
        let a = self.name_in(a);
        let b =self.name_in(b);
        self.name_out_sum(self.repo.anti_product(a, b))
    }

    pub fn scalar_product(&self, a: BasisElement, b: BasisElement) -> Sum {
        let a = self.name_in(a);
        let b =self.name_in(b);
        self.name_out_sum(self.repo.scalar_product(a, b))
    }

    pub fn anti_scalar_product(&self, a: BasisElement, b: BasisElement) -> Sum {
        let a = self.name_in(a);
        let b =self.name_in(b);
        self.name_out_sum(self.repo.anti_scalar_product(a, b))
    }

    pub fn apply_metric(&self, a: BasisElement) -> BasisElement {
        let a = self.name_in(a);
        self.name_out(self.repo.apply_metric(a))
    }

    pub fn apply_anti_metric(&self, a: BasisElement) -> BasisElement {
        let a = self.name_in(a);
        self.name_out(self.repo.apply_anti_metric(a))
    }

    pub fn dual(&self, a: BasisElement) -> BasisElement {
        let a = self.name_in(a);
        self.name_out(self.repo.apply_metric(a).right_complement(AntiScalar))
    }

    pub fn anti_dual(&self, a: BasisElement) -> BasisElement {
        let a = self.name_in(a);
        self.name_out(self.repo.apply_metric(a).right_complement(AntiScalar))
    }

    fn internalize_element_names(&self, mv: &MultiVec<AntiScalar>) {

        for el in MultiVec::<AntiScalar>::elements(&mv).into_iter() {
            self.name_in(el);
        }
    }

    pub fn all_elements(&self) -> impl Iterator<Item=BasisElement> {
        let mut results = vec![];
        let anti_scalar = self.anti_scalar();
        let independent_signatures = anti_scalar.signature().into_grade_1_signatures();
        let qty_elements = 1 << independent_signatures.len();
        let names = self.named_bases.read();
        for i in 0..qty_elements {
            let mut element = BasisElement::scalar();
            for (j, &sig) in independent_signatures.iter().enumerate() {
                if i & (1 << j) != 0 {
                    element = element.wedge(BasisElement::from(sig));
                }
            }
            results.push(names.provide_name_and_sign(element));
        }
        results.sort();
        results.into_iter()
    }
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