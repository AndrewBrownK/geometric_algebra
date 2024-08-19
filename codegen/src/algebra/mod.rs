use crate::algebra::dialect::Dialect;
use crate::ast::{DataType, Parameter};
use basis_element::{BasisElement, BasisElementIndex};
use std::fmt::{Debug, Display, Formatter};

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
        let mut first = true;
        for p in &self.product {
            if !first {
                f.write_str(" + ")?;
            }
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
}

impl Display for Product {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("Product(")?;
        let mut first = true;
        for term in &self.terms {
            if !first {
                f.write_str(", ")?;
            }
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
            return result_class;
        }
        if signature.is_empty() {
            return None;
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
        return result_class;
    }
}

#[derive(PartialEq, Eq, Debug, PartialOrd, Ord, Hash)]
pub struct MultiVectorClass {
    pub class_name: String,
    pub grouped_basis: Vec<Vec<BasisElement>>,
}

pub fn read_multi_vector_from_str<GA: GeometricAlgebraTrait>(multi_vector_descriptor: &str, algebra: &GA) -> MultiVectorClass {
    let mut multi_vector_descriptor_iter = multi_vector_descriptor.split(':');
    let class_name = multi_vector_descriptor_iter.next().unwrap().to_owned();
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
