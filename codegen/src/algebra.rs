use std::fmt::{Debug, Display, Formatter, Write};
use crate::algebra::dialect::Dialect;
use crate::ast::{DataType, Parameter};
use basis_element::{BasisElement, BasisElementIndex};

pub mod basis_element;
pub mod conformal;
pub mod dialect;
pub mod rigid;

pub trait GeometricAlgebraTrait {
    fn algebra_name(&self) -> &'static str;
    fn dialect(&self) -> &Dialect;
    fn parse(&self, name: &str) -> BasisElement;
    fn represented_dimensions(&self) -> usize;

    fn basis_size(&self) -> usize;

    fn basis(&self) -> Vec<BasisElement> {
        let mut v = vec![];
        for index in 0..self.basis_size() as BasisElementIndex {
            let mut element = BasisElement::from_index(index);
            let mut dual = self.dual(&element);
            if dual.coefficient == 0 {
                dual = self.right_complement(&element);
            }
            if dual.cmp(&element) == std::cmp::Ordering::Less {
                element.coefficient = dual.coefficient;
            }
            v.push(element);
        }
        v
    }

    fn scalar_element(&self) -> BasisElement {
        BasisElement::from_index(0 as BasisElementIndex)
    }
    fn anti_scalar_element(&self) -> BasisElement {
        BasisElement::from_index(self.basis_size() as BasisElementIndex - 1)
    }

    // Does applying the metric erase information?
    fn is_degenerate(&self) -> bool;

    // Even number of dimensions = yes
    // Odd number of dimensions = no
    fn has_multiple_complements(&self) -> bool;
    fn right_complement(&self, a: &BasisElement) -> BasisElement;
    fn left_complement(&self, a: &BasisElement) -> BasisElement;
    fn dual(&self, a: &BasisElement) -> BasisElement;
    fn anti_dual(&self, a: &BasisElement) -> BasisElement;
    fn product(&self, a: &BasisElement, b: &BasisElement) -> Vec<BasisElement>;
    fn anti_product(&self, a: &BasisElement, b: &BasisElement) -> Vec<BasisElement>;
}

#[derive(Clone)]
pub struct Involution {
    pub terms: Vec<(BasisElement, BasisElement)>,
}

impl Involution {
    pub fn identity<GA: GeometricAlgebraTrait>(algebra: &GA) -> Self {
        Self {
            terms: algebra.basis().into_iter().map(|element| (element.clone(), element)).collect(),
        }
    }

    pub fn projection(class: &MultiVectorClass) -> Self {
        Self {
            terms: class.flat_basis().iter().map(|element| (element.clone(), element.clone())).collect(),
        }
    }

    pub fn negated<F>(&self, grade_negation: F) -> Self
    where
        F: Fn(usize, BasisElementIndex) -> bool,
    {
        Self {
            terms: self
                .terms
                .iter()
                .map(|(key, value)| {
                    let mut element = value.clone();
                    element.set_coefficient(element.coefficient * if grade_negation(value.grade(), element.index) { -1 } else { 1 });
                    (key.clone(), element)
                })
                .collect(),
        }
    }

    pub fn dual<GA: GeometricAlgebraTrait>(&self, algebra: &GA) -> Self {
        Self {
            terms: self.terms.iter().map(|(key, value)| (key.clone(), algebra.dual(value))).collect(),
        }
    }

    pub fn anti_dual<GA: GeometricAlgebraTrait>(&self, algebra: &GA) -> Self {
        Self {
            terms: self.terms.iter().map(|(key, value)| (key.clone(), algebra.anti_dual(value))).collect(),
        }
    }

    pub fn right_complement<GA: GeometricAlgebraTrait>(&self, algebra: &GA) -> Self {
        Self {
            terms: self.terms.iter().map(|(key, value)| (key.clone(), algebra.right_complement(value))).collect(),
        }
    }

    pub fn left_complement<GA: GeometricAlgebraTrait>(&self, algebra: &GA) -> Self {
        Self {
            terms: self.terms.iter().map(|(key, value)| (key.clone(), algebra.left_complement(value))).collect(),
        }
    }

    pub fn double_complement<GA: GeometricAlgebraTrait>(&self, algebra: &GA) -> Self {
        Self {
            terms: self
                .terms
                .iter()
                .map(|(key, value)| (key.clone(), algebra.right_complement(&algebra.right_complement(value))))
                .collect(),
        }
    }

    pub fn involutions<GA: GeometricAlgebraTrait>(algebra: &GA) -> Vec<(&'static str, Self, &'static str)> {
        let involution = Self::identity(algebra);
        let dimensions = algebra.anti_scalar_element().grade();
        let mut result = vec![
            ("Neg", involution.negated(|_grade, _| true), ""),
            (
                "Automorphism",
                involution.negated(|grade, _| grade % 2 == 1),
                "\nNegates elements with `grade % 2 == 1`\n\nAlso called main involution",
            ),
            ("Conjugation", involution.negated(|grade, _| (grade + 3) % 4 < 2), "\nNegates elements with `(grade + 3) % 4 < 2`"),
            (
                "Reversal",
                involution.negated(|grade, _| grade % 4 >= 2),
                "\nNegates elements with `grade % 4 >= 2`\n\nAlso called transpose\nhttps://rigidgeometricalgebra.org/wiki/index.php?title=Reverses",
            ),
            (
                "AntiReversal",
                involution.negated(|grade, _| {
                    let anti_grade = dimensions - grade;
                    anti_grade % 4 >= 2
                }),
                "\nNegates elements with `grade % 4 >= 2`\n\nhttps://rigidgeometricalgebra.org/wiki/index.php?title=Reverses",
            ),
            (
                "Dual",
                involution.dual(algebra),
                "\nElement order reversed\nAlso known as Right Complement\nhttps://rigidgeometricalgebra.org/wiki/index.php?title=Complements",
            ),
            (
                "AntiDual",
                involution.anti_dual(algebra),
                "\nAntiDuals are a special kind a Dual.\nhttps://conformalgeometricalgebra.org/wiki/index.php?title=Duals\nhttps://rigidgeometricalgebra.org/wiki/index.php?title=Complements",
            ),
            (
                "DoubleComplement",
                involution.double_complement(algebra),
                "\nDouble Complement\nhttps://rigidgeometricalgebra.org/wiki/index.php?title=Complements",
            ),
        ];

        if algebra.has_multiple_complements() {
            result.push((
                "RightComplement",
                involution.right_complement(algebra),
                "\nRight Complement\nhttps://rigidgeometricalgebra.org/wiki/index.php?title=Complements",
            ));
            result.push((
                "LeftComplement",
                involution.left_complement(algebra),
                "\nLeft Complement\nhttps://rigidgeometricalgebra.org/wiki/index.php?title=Complements",
            ));
        } else {
            result.push((
                "Complement",
                involution.right_complement(algebra),
                "\nComplement\nhttps://rigidgeometricalgebra.org/wiki/index.php?title=Complements",
            ));
        }
        if algebra.algebra_name().starts_with("cga") {
            let infinity = (1 as BasisElementIndex) << (dimensions - 1);
            result.push((
                "ConformalConjugate",
                involution.negated(|grade, index| {
                    (index & infinity) == infinity
                }),
                "\nConformal Conjugates\nSee chapter 4.5.4 of the book (page 204).",
            ));
        }

        return result;
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ProductTerm {
    pub product: Vec<BasisElement>,
    pub factor_a: BasisElement,
    pub factor_b: BasisElement,
}

impl Display for ProductTerm {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.factor_a, f)?;
        f.write_str(" * ")?;
        Display::fmt(&self.factor_b, f)?;
        f.write_str(" = ")?;
        let mut first  = true;
        for p in &self.product {
            if !first { f.write_str(" + ")?; }
            Display::fmt(p, f)?;
            first = false;
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct Product {
    pub terms: Vec<ProductTerm>,
}

impl Product {
    pub fn new<GA: GeometricAlgebraTrait>(a: &[BasisElement], b: &[BasisElement], algebra: &GA) -> Self {
        Self {
            terms: a
                .iter()
                .flat_map(|a| {
                    b.iter().map(move |b| ProductTerm {
                        product: algebra.product(a, b),
                        factor_a: a.clone(),
                        factor_b: b.clone(),
                    })
                })
                .filter(|term| !term.product.is_empty())
                .collect(),
        }
    }

    pub fn anti_new<GA: GeometricAlgebraTrait>(a: &[BasisElement], b: &[BasisElement], algebra: &GA) -> Self {
        Self {
            terms: a
                .iter()
                .flat_map(|a| {
                    b.iter().map(move |b| ProductTerm {
                        product: algebra.anti_product(a, b),
                        factor_a: a.clone(),
                        factor_b: b.clone(),
                    })
                })
                .filter(|term| !term.product.is_empty())
                .collect(),
        }
    }

    pub fn projected<F>(&self, max_grade: usize, grade_projection: F) -> Self
    where
        F: Fn(usize, usize, usize, usize) -> bool,
    {

        Self {
            terms: self
                .terms
                .iter()
                .filter(|term| !term.product.is_empty())
                .flat_map(|term| {
                    if term.product.len() == 1 {
                        return vec![term.clone()];
                    }
                    let mut terms = vec![];
                    for p in term.product.iter() {
                        terms.push(ProductTerm {
                            product: vec![p.clone()],
                            factor_a: term.factor_a.clone(),
                            factor_b: term.factor_b.clone(),
                        })
                    }
                    terms
                })
                .filter(|term| grade_projection(max_grade, term.factor_a.grade(), term.factor_b.grade(), term.product[0].grade()))
                .collect(),
        }
    }

    fn synonyms(names: &Vec<&'static str>) -> String {
        if names.len() <= 1 {
            return "".to_string();
        }
        let synonyms: String = names.iter().map(|it| it.to_string()).intersperse(", ".to_string()).collect();
        format!("Synonyms included: {synonyms}")
    }

    pub fn products<GA: GeometricAlgebraTrait>(algebra: &GA) -> Vec<(&'static str, Self, String)> {
        let basis = algebra.basis();
        let product = Self::new(&basis, &basis, algebra);
        let anti_product = Self::anti_new(&basis, &basis, algebra);
        let max_grade = algebra.anti_scalar_element().grade();

        let wedge_like: fn(usize, usize, usize, usize) -> bool = |_, factor_a_grade, factor_b_grade, product_grade| {
            product_grade == factor_a_grade + factor_b_grade
        };
        let anti_wedge_like: fn(usize, usize, usize, usize) -> bool = move |max_grade, factor_a_grade, factor_b_grade, product_grade| {
            let anti_grade_a = max_grade - factor_a_grade;
            let anti_grade_b = max_grade - factor_b_grade;
            let anti_grade_product = max_grade - product_grade;
            anti_grade_product == anti_grade_a + anti_grade_b
        };
        let scalar_result_only: fn(usize, usize, usize, usize) -> bool = |_, factor_a_grade, factor_b_grade, product_grade| {
            product_grade == 0 && (factor_a_grade == factor_b_grade)
        };
        let anti_scalar_result_only: fn(usize, usize, usize, usize) -> bool = |max_grade, factor_a_grade, factor_b_grade, product_grade| {
            product_grade == max_grade
        };

        let dialect = algebra.dialect();
        let mut products = vec![];

        // https://rigidgeometricalgebra.org/wiki/index.php?title=Geometric_products

        let synonyms = Product::synonyms(&dialect.geometric_product);
        let docs = format!(
            "
            Geometric Product
            {synonyms}
            https://rigidgeometricalgebra.org/wiki/index.php?title=Geometric_products
        "
        );
        for name in &dialect.geometric_product {
            products.push((*name, product.clone(), docs.clone()))
        }

        let synonyms = Product::synonyms(&dialect.geometric_anti_product);
        let docs = format!(
            "
            Geometric Anti-Product
            {synonyms}
            https://rigidgeometricalgebra.org/wiki/index.php?title=Geometric_products
        "
        );
        for name in &dialect.geometric_anti_product {
            products.push((*name, anti_product.clone(), docs.clone()))
        }

        // https://rigidgeometricalgebra.org/wiki/index.php?title=Exterior_products

        let synonyms = Product::synonyms(&dialect.exterior_product);
        let docs = format!(
            "
            Exterior Product
            {synonyms}
            https://rigidgeometricalgebra.org/wiki/index.php?title=Exterior_products
        "
        );
        for name in &dialect.exterior_product {
            products.push((*name, product.projected(max_grade, wedge_like), docs.clone()))
        }
        let synonyms = Product::synonyms(&dialect.exterior_anti_product);
        let docs = format!(
            "
            Geometric Anti-Product
            {synonyms}
            https://rigidgeometricalgebra.org/wiki/index.php?title=Exterior_products
        "
        );
        for name in &dialect.exterior_anti_product {
            products.push((*name, anti_product.projected(max_grade, anti_wedge_like), docs.clone()))
        }

        // TODO these dot products don't look SIMD optimal (as far as I know, which is not much)
        //  for example see `impl Dot<Point> for Point` in rga3d
        //  It (might?) be more efficient to multiply first, and add second. Even if we multiply
        //  and unused SIMD element, it might be faster than extracting crap out manually and then
        //  multiplying and adding. I heard GPUs can perform the dot product into addition like
        //  really efficiently.
        // https://rigidgeometricalgebra.org/wiki/index.php?title=Dot_products

        let synonyms = Product::synonyms(&dialect.dot_product);
        let docs = format!(
            "
            Dot Product
            {synonyms}
            https://rigidgeometricalgebra.org/wiki/index.php?title=Dot_products
        "
        );
        for name in &dialect.dot_product {
            products.push((*name, product.projected(max_grade, scalar_result_only), docs.clone()))
        }
        let synonyms = Product::synonyms(&dialect.anti_dot_product);
        let docs = format!(
            "
            Anti-Dot Product
            {synonyms}
            https://rigidgeometricalgebra.org/wiki/index.php?title=Dot_products
        "
        );
        for name in &dialect.anti_dot_product {
            products.push((*name, anti_product.projected(max_grade, anti_scalar_result_only), docs.clone()))
        }

        products
    }
}

impl Display for Product {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("Product(")?;
        let mut first = true;
        for term in &self.terms {
            if !first { f.write_str(", ")?; }
            Display::fmt(term, f)?;
            first = false;
        }
        f.write_str(")")
    }
}

#[derive(Default)]
pub struct MultiVectorClassRegistry {
    pub classes: Vec<MultiVectorClass>,
    index_by_signature: std::collections::HashMap<Vec<BasisElementIndex>, usize>,
}

impl MultiVectorClassRegistry {
    pub fn single_parameters<'r>(&'r self) -> impl Iterator<Item = Parameter<'r>> {
        self.classes.iter().map(|class_a| Parameter {
            name: "self",
            data_type: DataType::MultiVector(class_a),
        })
    }
    pub fn pair_parameters(&self) -> impl Iterator<Item = (Parameter, Parameter)> {
        self.classes
            .iter()
            .map(|class_a| {
                let param_a = Parameter {
                    name: "self",
                    data_type: DataType::MultiVector(class_a),
                };
                self.classes.iter().map(move |class_b| {
                    let param_b = Parameter {
                        name: "other",
                        data_type: DataType::MultiVector(class_b),
                    };
                    (param_a.clone(), param_b)
                })
            })
            .flatten()
    }
}

impl MultiVectorClassRegistry {
    pub fn register(&mut self, class: MultiVectorClass) {
        self.index_by_signature.insert(class.signature(), self.classes.len());
        self.classes.push(class);
    }

    pub fn get_exact(&self, signature: &[BasisElementIndex]) -> Option<&MultiVectorClass> {
        self.index_by_signature.get(signature).map(|index| &self.classes[*index])
    }

    pub fn get_at_least(&self, signature: &[BasisElementIndex]) -> Option<&MultiVectorClass> {
        let mut result_class = self.get_exact(&signature);
        if result_class.is_some() {
            return result_class
        }
        if signature.is_empty() {
            return None
        }

        let mut viable_classes: Vec<_> = self
            .classes
            .iter()
            .filter(|it| {
                let sig = it.signature();
                signature.iter().all(|it| sig.contains(it))
            })
            .collect();
        viable_classes.sort_by_key(|it| it.signature().len());
        result_class = viable_classes.first().map(|it| *it);
        return result_class
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct MultiVectorClass {
    pub class_name: String,
    pub grouped_basis: Vec<Vec<BasisElement>>,
}

pub fn read_multi_vector_from_str<GA: GeometricAlgebraTrait>(multi_vector_descriptor: &str, algebra: &GA) -> MultiVectorClass {
    let mut multi_vector_descriptor_iter = multi_vector_descriptor.split(':');
    let mut class_name = multi_vector_descriptor_iter.next().unwrap().to_owned();
    let mvc = MultiVectorClass {
        class_name,
        grouped_basis: multi_vector_descriptor_iter
            .next()
            .unwrap()
            .split('|')
            .map(|group_descriptor| group_descriptor.split(',').map(|element_name| algebra.parse(element_name)).collect::<Vec<_>>())
            .collect::<Vec<_>>(),
    };
    mvc
}
