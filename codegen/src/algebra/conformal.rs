
#[cfg(test)]
mod tests;

use crate::algebra::basis_element::{BasisElement, BasisElementIndex};
use crate::algebra::dialect::Dialect;
use crate::algebra::GeometricAlgebraTrait;
use crate::algebra::rigid::RigidGeometricAlgebra;

pub struct ConformalGeometricAlgebra {
    // origin_retaining_generator_squares: Vec<isize>,
    // infinity_retaining_generator_squares: Vec<isize>,
    surface_generator_squares: Vec<isize>,
    origin: usize,
    infinity: usize,
    pub name: &'static str,
    pub dialect: Dialect,
}

impl ConformalGeometricAlgebra {
    pub fn new(name: &'static str, dimensions: usize, dialect: Dialect) -> Self {
        // TODO maybe one of these retaining squares should be -1 instead of 1

        let mut origin_retaining_generator_squares = vec![1; dimensions];
        origin_retaining_generator_squares.push(1);
        origin_retaining_generator_squares.push(0);

        let mut infinity_retaining_generator_squares = vec![1; dimensions];
        infinity_retaining_generator_squares.push(0);
        infinity_retaining_generator_squares.push(1);

        let mut surface_generator_squares = vec![1; dimensions];
        surface_generator_squares.push(0);
        surface_generator_squares.push(0);

        let origin = surface_generator_squares.len() - 2;
        let infinity = surface_generator_squares.len() - 1;
        ConformalGeometricAlgebra {
            // origin_retaining_generator_squares, infinity_retaining_generator_squares,
            surface_generator_squares,
            origin, infinity, name, dialect,
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

    fn right_complement(&self, a: &BasisElement) -> BasisElement {
        let index = a.index;
        let origin = (1 as BasisElementIndex) << self.origin;
        let infinity = (1 as BasisElementIndex) << self.infinity;
        let projective = origin | infinity;
        let anti_scalar = self.anti_scalar_element().index;

        // Basis element has both e4 and e5 -> regular dual
        let is_projective = index & projective == projective;
        if is_projective {
            let new_index = anti_scalar - index;
            return BasisElement {
                // TODO coefficient
                coefficient: 1,
                index: new_index,
            }
        }

        // Basis element includes e5 but not e4 -> dual keeps e5 and does not get e4
        let is_flat = index & infinity == infinity;
        if is_flat {
            let mut new_index = anti_scalar - index;
            new_index = new_index - origin;
            new_index = new_index + infinity;
            return BasisElement {
                // TODO coefficient
                coefficient: 1,
                index: new_index,
            }
        }

        // Basis element includes e4 but not e5 -> dual keeps e4 but does not get e5
        let is_round = index & origin == origin;
        if is_round {
            let mut new_index = anti_scalar - index;
            new_index = new_index - infinity;
            new_index = new_index + origin;
            return BasisElement {
                // TODO coefficient
                coefficient: 1,
                index: new_index,
            }
        }

        // Neither e4 nor e5 -> regular dual (acquires e4 and e5)
        assert_eq!(index & projective, 0);
        let mut new_index = anti_scalar - index;
        return BasisElement {
            // TODO coefficient
            coefficient: 1,
            index: new_index,
        }
    }

    fn left_complement(&self, a: &BasisElement) -> BasisElement {
        todo!()
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

        // TODO remove if unused
        let origin_basis = BasisElement::from_index(origin);
        let infinity_basis = BasisElement::from_index(infinity);
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
            assert_eq!(result.index && projective, 0);
            // TODO not sure if this coefficient is accurate in higher dimensions. Or heck... this dimension...
            result.coefficient = a.coefficient * b.coefficient * (-1isize) ^ result.grade();
            return vec![result]
        }

        let a_is_along_origin = ((a.index & origin) == origin) && !a_is_projective;
        let a_is_along_infinity =  ((a.index & infinity) == infinity) && !a_is_projective;
        let a_is_half_projective = a_is_along_origin || a_is_along_infinity;

        let b_is_along_origin = ((b.index & origin) == origin) && !b_is_projective;
        let b_is_along_infinity =  ((b.index & infinity) == infinity) && !b_is_projective;
        let b_is_half_projective = b_is_along_origin || b_is_along_infinity;

        if (a_is_along_origin && b_is_along_infinity) || (a_is_along_infinity && b_is_along_origin) {
            // There will be two components to the result
            // So we should already have one of the results
            assert_eq!(result.len(), 1);
            let second_product_component = result[0].primitive_product(&projective_basis, &self.surface_generator_squares);
            // TODO coefficient?
            result.push(second_product_component);
            return result
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
            // todo coefficient?
            return vec![result]
        }

        // TODO otherwise we can do the trivial product (I think)
        return result
    }
}





















//