use crate::algebra::basis_element::{BasisElement, BasisElementIndex};
use crate::algebra::dialect::Dialect;
use crate::algebra::GeometricAlgebraTrait;

#[cfg(test)]
mod tests;

pub struct ConformalGeometricAlgebra {
    all_retaining_generator_squares: Vec<isize>,
    surface_generator_squares: Vec<isize>,
    origin: usize,
    infinity: usize,
    pub name: &'static str,
    pub dialect: Dialect,
}

impl ConformalGeometricAlgebra {
    pub fn new(name: &'static str, dimensions: usize, dialect: Dialect) -> Self {
        let all_retaining_generator_squares = vec![1; dimensions + 2];
        let mut surface_generator_squares = vec![1; dimensions];
        surface_generator_squares.push(0);
        surface_generator_squares.push(0);

        let origin = surface_generator_squares.len() - 2;
        let infinity = surface_generator_squares.len() - 1;
        ConformalGeometricAlgebra {
            all_retaining_generator_squares,
            surface_generator_squares,
            origin,
            infinity,
            name,
            dialect,
        }
    }

    pub fn sorted_basis(&self) -> Vec<BasisElement> {
        let mut basis_elements = self.basis();
        basis_elements.sort();
        basis_elements
    }
}

impl GeometricAlgebraTrait for ConformalGeometricAlgebra {
    fn algebra_name(&self) -> &'static str {
        self.name
    }

    fn dialect(&self) -> &Dialect {
        &self.dialect
    }

    //noinspection DuplicatedCode
    fn parse(&self, mut name: &str) -> BasisElement {
        let mut result = BasisElement::from_index(0);
        if name.starts_with('-') {
            name = &name[1..];
            result.coefficient = -1;
        }
        if name == "1" {
            return result;
        }
        let mut generator_indices = name.chars();
        assert_eq!(generator_indices.next().unwrap(), 'e');
        for generator_index in generator_indices {
            let generator_index = generator_index.to_digit(16).unwrap();
            if generator_index == 0 {
                panic!("Please do not use e0, and instead start from e1")
            }
            let generator_index = generator_index - 1;
            assert!((generator_index as usize) < self.surface_generator_squares.len());
            result = result.primitive_product(&BasisElement::from_index(1 << generator_index), &self.surface_generator_squares);
        }
        result
    }

    fn basis_size(&self) -> usize {
        1 << self.surface_generator_squares.len()
    }

    // NOTE THAT RIGHT COMPLEMENTS AND DUALS ARE NOT THE SAME IN CGA
    //  See thread: https://twitter.com/Foo55443320/status/1768401735410958594
    fn right_complement(&self, a: &BasisElement) -> BasisElement {
        let coefficient = a.coefficient;
        let index = a.index;
        let origin = (1 as BasisElementIndex) << self.origin;
        let infinity = (1 as BasisElementIndex) << self.infinity;
        let projective = origin | infinity;
        let anti_scalar = self.anti_scalar_element().index;

        // Basis element has both e4 and e5 -> regular right complement
        let is_projective = index & projective == projective;
        if is_projective {
            // Start with the BasisElement index, yet uncertain of coefficient
            let new_index = anti_scalar - index;
            let mut candidate_complement = BasisElement { coefficient: 1, index: new_index };

            // Fix the coefficient
            let anti_scalar_product = self
                .product(&a, &candidate_complement)
                .into_iter()
                .find(|it| it.index == anti_scalar)
                .expect("Must find anti_scalar result when wedging complements");
            candidate_complement.coefficient = anti_scalar_product.coefficient;

            // Return fixed result
            return candidate_complement;
        }

        // Basis element includes e5 but not e4 -> regular right complement
        let is_flat = index & infinity == infinity;
        if is_flat {
            // Start with the BasisElement index, yet uncertain of coefficient
            let new_index = anti_scalar - index;
            // new_index = new_index - origin;
            // new_index = new_index + infinity;
            let mut candidate_complement = BasisElement { coefficient, index: new_index };

            // Fix the coefficient
            let anti_scalar_product = self
                .product(&a, &candidate_complement)
                .into_iter()
                .find(|it| it.index == anti_scalar)
                .expect("Must find anti_scalar result when wedging complements");
            candidate_complement.coefficient = anti_scalar_product.coefficient;

            // Return fixed result
            return candidate_complement;
        }

        // Basis element includes e4 but not e5 -> regular right complement
        let is_round = index & origin == origin;
        if is_round {
            // Start with the BasisElement index, yet uncertain of coefficient
            let new_index = anti_scalar - index;
            // new_index = new_index - infinity;
            // new_index = new_index + origin;
            let mut candidate_complement = BasisElement { coefficient, index: new_index };

            // Fix the coefficient
            let anti_scalar_product = self
                .product(&a, &candidate_complement)
                .into_iter()
                .find(|it| it.index == anti_scalar)
                .expect("Must find anti_scalar result when wedging complements");
            candidate_complement.coefficient = anti_scalar_product.coefficient;

            // Return fixed result
            return candidate_complement;
        }

        // Neither e4 nor e5 -> regular right complement (acquires e4 and e5)
        assert_eq!(index & projective, 0);

        // Start with the BasisElement index, yet uncertain of coefficient
        let new_index = anti_scalar - index;
        let mut candidate_complement = BasisElement { coefficient, index: new_index };

        // Fix the coefficient
        let anti_scalar_product = self
            .product(&a, &candidate_complement)
            .into_iter()
            .find(|it| it.index == anti_scalar)
            .expect("Must find anti_scalar result when wedging complements");
        candidate_complement.coefficient = anti_scalar_product.coefficient;

        // Return fixed result
        return candidate_complement;
    }

    fn left_complement(&self, a: &BasisElement) -> BasisElement {
        self.right_complement(&self.right_complement(&self.right_complement(a)))
    }

    fn product(&self, a: &BasisElement, b: &BasisElement) -> Vec<BasisElement> {
        // We need to reproduce this Cayley table.
        // https://conformalgeometricalgebra.org/wiki/index.php?title=Geometric_products

        let mut result = vec![];

        let trivial_product = a.primitive_product(b, &self.surface_generator_squares);
        if trivial_product.coefficient != 0 {
            result.push(trivial_product.clone());
        }

        let origin = (1 as BasisElementIndex) << self.origin;
        let infinity = (1 as BasisElementIndex) << self.infinity;
        let projective = origin | infinity;
        let anti_scalar = self.anti_scalar_element().index;
        let non_projective = anti_scalar - projective;

        let projective_basis = BasisElement::from_index(projective);

        let a_is_projective = (a.index & projective) == projective;
        let b_is_projective = (b.index & projective) == projective;
        if a_is_projective && b_is_projective {
            // primitive product should be zero
            assert!(result.is_empty());
            // Then we add the non-trivial part of this product (the only part of this product)
            let mut a = a.clone();
            let mut b = b.clone();
            a.index = a.index & non_projective;
            b.index = b.index & non_projective;
            let mut result = a.primitive_product(&b, &self.surface_generator_squares);
            assert_eq!(result.index & projective, 0);
            result.coefficient = a.coefficient * b.coefficient * result.coefficient;
            return vec![result];
        }

        let a_is_along_origin = ((a.index & origin) == origin) && !a_is_projective;
        let a_is_along_infinity = ((a.index & infinity) == infinity) && !a_is_projective;
        let a_is_half_projective = a_is_along_origin || a_is_along_infinity;

        let b_is_along_origin = ((b.index & origin) == origin) && !b_is_projective;
        let b_is_along_infinity = ((b.index & infinity) == infinity) && !b_is_projective;
        let b_is_half_projective = b_is_along_origin || b_is_along_infinity;

        if (a_is_along_origin && b_is_along_infinity) || (a_is_along_infinity && b_is_along_origin) {
            // There will be two components to the result
            // So we should already have one of the results
            assert_eq!(result.len(), 1);
            // Note the use of different generator squares here
            let mut second_product_component = result[0].primitive_product(&projective_basis, &self.all_retaining_generator_squares);
            // TODO not yet sure if this coefficient logic correctly generalizes to higher dimensions
            if a_is_along_infinity && b_is_along_origin {
                second_product_component.coefficient = second_product_component.coefficient * -1;
            }
            result.push(second_product_component);
            return result;
        }

        if (a_is_half_projective && b_is_projective) || (a_is_projective && b_is_half_projective) {
            // primitive product should be zero
            assert!(result.is_empty());
            // Then we add the non-trivial part of this product (the only part of this product)
            let mut a = a.clone();
            let mut b = b.clone();
            if a_is_projective {
                a.index = a.index & non_projective;
            }
            if b_is_projective {
                b.index = b.index & non_projective;
            }
            let mut result = a.primitive_product(&b, &self.surface_generator_squares);
            // TODO not yet sure if this coefficient logic correctly generalizes to higher dimensions
            if a_is_along_infinity && b_is_projective || a_is_projective && b_is_along_origin {
                result.coefficient = result.coefficient * -1;
            }
            return vec![result];
        }

        return result;
    }

    fn anti_product(&self, a: &BasisElement, b: &BasisElement) -> Vec<BasisElement> {
        // We need to reproduce this Cayley table.
        // https://conformalgeometricalgebra.org/wiki/index.php?title=Geometric_products

        let mut result = vec![];

        let trivial_product = a.primitive_anti_product(b, &self.surface_generator_squares);
        if trivial_product.coefficient != 0 {
            result.push(trivial_product.clone());
        }

        let anti_scalar = self.anti_scalar_element().index;
        let origin = (1 as BasisElementIndex) << self.origin;
        let anti_origin = anti_scalar - origin;
        let infinity = (1 as BasisElementIndex) << self.infinity;
        let anti_infinity = anti_scalar - infinity;
        let projective = origin | infinity;
        let anti_projective = anti_scalar - projective;

        let projective_basis = BasisElement::from_index(projective);
        let anti_projective_basis = BasisElement::from_index(anti_projective);

        // let a_is_anti_projective = (a.index & anti_projective) == anti_projective;
        // let b_is_anti_projective = (b.index & anti_projective) == anti_projective;
        // if a_is_anti_projective && b_is_anti_projective {
        //     // primitive product should be zero
        //     assert!(result.is_empty());
        //     // Then we add the non-trivial part of this product (the only part of this product)
        //     let mut a = a.clone();
        //     let mut b = b.clone();
        //     a.index = a.index & projective;
        //     b.index = b.index & projective;
        //     let mut result = a.primitive_product(&b, &self.surface_generator_squares);
        //     assert_eq!(result.index & anti_projective, 0);
        //     result.coefficient = a.coefficient * b.coefficient * result.coefficient;
        //     return vec![result];
        // }



        // TODO

        return result;
    }
}

//
