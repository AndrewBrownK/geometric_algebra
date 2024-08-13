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

    // noinspection DuplicatedCode
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

    fn represented_dimensions(&self) -> usize {
        self.surface_generator_squares.len() - 2
    }

    fn basis_size(&self) -> usize {
        1 << self.surface_generator_squares.len()
    }

    fn basis(&self) -> Vec<BasisElement> {
        let mut v = vec![];
        for index in 0..self.basis_size() as BasisElementIndex {
            let mut element = BasisElement::from_index(index);
            let dual = self.dual(&element);
            if dual.cmp(&element) == std::cmp::Ordering::Less {
                element.coefficient = self.dual(&element).coefficient;
            }
            v.push(element);
        }
        v
    }

    fn is_degenerate(&self) -> bool {
        return false;
    }

    fn has_multiple_complements(&self) -> bool {
        return self.surface_generator_squares.len() % 2 == 0;
    }

    fn right_complement(&self, a: &BasisElement) -> BasisElement {
        let mut result = BasisElement {
            coefficient: a.coefficient,
            index: self.basis_size() as BasisElementIndex - 1 - a.index,
        };
        result.coefficient *= a.primitive_product(&result, &self.surface_generator_squares).coefficient;
        result
    }

    fn left_complement(&self, a: &BasisElement) -> BasisElement {
        let mut result = BasisElement {
            coefficient: a.coefficient,
            index: self.basis_size() as BasisElementIndex - 1 - a.index,
        };
        result.coefficient *= a.primitive_product(&result, &self.surface_generator_squares).coefficient;
        result
    }

    fn dual(&self, a: &BasisElement) -> BasisElement {
        let origin = (1 as BasisElementIndex) << self.origin;
        let infinity = (1 as BasisElementIndex) << self.infinity;

        // exomoriphism metric result
        let mut result = a.clone();
        let aligned_origin = result.index & origin == origin;
        let aligned_infinity = result.index & infinity == infinity;

        match (aligned_origin, aligned_infinity) {
            (true, false) => {
                result.index -= origin;
                result.index += infinity;
                result.coefficient *= -1
            }
            (false, true) => {
                result.index -= infinity;
                result.index += origin;
                result.coefficient *= -1
            }
            (true, true) => {
                result.coefficient *= -1;
            }
            (false, false) => {
                // No funny business
            }
        }

        // Take (right) complement of the exomorphism metric result
        return self.right_complement(&result);
    }

    fn anti_dual(&self, a: &BasisElement) -> BasisElement {
        // Wat is anti_dual in conformal?
        //  See pages 68, 81, 83, and 185 of the book
        //  Could be a lot simpler in the conventional cga3d case, but I want
        //  to generalize to higher dimensional CGA

        // First take the left complement according to the latter formulation of the
        // anti-metric on page 68
        let mut result = self.left_complement(&a);

        // Then apply the exo-metric and right complement at once by applying dual
        // This gives us the result of applying the exo-anti-metric
        result = self.dual(&result);

        // Then right complement again to get the anti-dual
        result = self.right_complement(&result);

        return result;
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
            let result = a.primitive_product(&b, &self.surface_generator_squares);
            assert_eq!(result.index & projective, 0);
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

        let a = self.right_complement(a);
        let b = self.right_complement(b);
        let mut results = self.product(&a, &b);
        for r in results.iter_mut() {
            *r = self.left_complement(&r);
        }
        return results;
    }
}

//
