use std::collections::BTreeMap;
use crate::{
    algebra::{Involution, MultiVectorClass, MultiVectorClassRegistry, Product},
    ast::{AstNode, DataType, Expression, ExpressionContent, Parameter},
};
use crate::algebra::basis_element::{BasisElement, BasisElementIndex};
use crate::algebra::GeometricAlgebraTrait;

#[macro_export]
macro_rules! result_of_trait {
    ($ast_node:expr) => {
        match $ast_node {
            AstNode::TraitImplementation { ref result, .. } => result,
            the_fuck => unreachable!("There must be a TraitImplementation result {the_fuck:?}"),
        }
    };
}

pub fn simplify_and_legalize(expression: Box<Expression>) -> Box<Expression> {
    match expression.content {
        ExpressionContent::Gather(mut inner_expression, indices) => {
            if let Some(first_index_pair) = indices.first() {
                inner_expression = simplify_and_legalize(inner_expression);
                if indices.iter().all(|index_pair| index_pair == first_index_pair) {
                    Box::new(Expression {
                        size: expression.size,
                        content: ExpressionContent::Gather(inner_expression, vec![*first_index_pair]),
                        data_type_hint: None
                    })
                } else if inner_expression.size == expression.size && indices.iter().all(|(array_index, _)| *array_index == first_index_pair.0) {
                    inner_expression = Box::new(Expression {
                        size: expression.size,
                        content: ExpressionContent::Access(inner_expression, first_index_pair.0),
                        data_type_hint: None
                    });
                    if indices.iter().enumerate().any(|(i, (_, component_index))| i != *component_index) {
                        Box::new(Expression {
                            size: expression.size,
                            content: ExpressionContent::Swizzle(
                                inner_expression,
                                indices.iter().map(|(_, component_index)| *component_index).collect(),
                            ),
                            data_type_hint: None
                        })
                    } else {
                        inner_expression
                    }
                } else {
                    Box::new(Expression {
                        size: expression.size,
                        content: ExpressionContent::Gather(inner_expression, indices),
                        data_type_hint: None
                    })
                }
            } else {
                Box::new(Expression {
                    size: expression.size,
                    content: ExpressionContent::None,
                    data_type_hint: None
                })
            }
        }
        ExpressionContent::Constant(ref data_type, ref values) => {
            let first_value = values.first().unwrap();
            if values.iter().all(|value| value == first_value) {
                Box::new(Expression {
                    size: expression.size,
                    content: ExpressionContent::Constant(data_type.clone(), vec![*first_value]),
                    data_type_hint: Some(data_type.clone())
                })
            } else {
                expression
            }
        }
        ExpressionContent::Add(mut a, mut b) => {
            if let ExpressionContent::Multiply(ref c, ref d) = b.content {
                if let ExpressionContent::Multiply(ref e, ref f) = d.content {
                    if let ExpressionContent::Constant(_data_type, values) = &f.content {
                        if values.iter().all(|value| *value == -1) {
                            b = Box::new(Expression {
                                size: expression.size,
                                content: ExpressionContent::Multiply(c.clone(), e.clone()),
                                data_type_hint: None
                            });
                            return simplify_and_legalize(Box::new(Expression {
                                size: expression.size,
                                content: ExpressionContent::Subtract(a, b),
                                data_type_hint: None
                            }));
                        }
                    }
                }
            }
            a = simplify_and_legalize(a);
            b = simplify_and_legalize(b);
            if a.content == ExpressionContent::None {
                b
            } else if b.content == ExpressionContent::None {
                a
            } else {
                let data_type_hint = if a.data_type_hint == b.data_type_hint { a.data_type_hint.clone() } else { None };
                Box::new(Expression {
                    size: expression.size,
                    content: ExpressionContent::Add(a, b),
                    data_type_hint
                })
            }
        }
        ExpressionContent::Subtract(mut a, mut b) => {
            a = simplify_and_legalize(a);
            b = simplify_and_legalize(b);
            if a.content == ExpressionContent::None {
                let constant = Expression {
                    size: expression.size,
                    content: ExpressionContent::Constant(DataType::SimdVector(expression.size), vec![0]),
                    data_type_hint: None,
                };
                Box::new(Expression {
                    size: expression.size,
                    data_type_hint: b.data_type_hint.clone(),
                    content: ExpressionContent::Subtract(Box::new(constant), b),
                })
            } else if b.content == ExpressionContent::None {
                a
            } else {
                Box::new(Expression {
                    size: expression.size,
                    data_type_hint: a.data_type_hint.clone(),
                    content: ExpressionContent::Subtract(a, b),
                })
            }
        }
        ExpressionContent::Multiply(mut a, mut b) => {
            a = simplify_and_legalize(a);
            b = simplify_and_legalize(b);
            if let ExpressionContent::Constant(_, _) = a.content {
                std::mem::swap(&mut a, &mut b)
            }
            if a.content == ExpressionContent::None {
                b
            } else {
                match b.content {
                    ExpressionContent::None => a,
                    ExpressionContent::Constant(_data_type, c) if c.iter().all(|c| *c == 1) => a,
                    ExpressionContent::Constant(_data_type, c) if c.iter().all(|c| *c == 0) => Box::new(Expression {
                        size: expression.size,
                        content: ExpressionContent::None,
                        data_type_hint: None
                    }),
                    _ => {
                        let data_type_hint = if a.data_type_hint == b.data_type_hint { a.data_type_hint.clone() } else { None };
                        Box::new(Expression {
                            size: expression.size,
                            content: ExpressionContent::Multiply(a, b),
                            data_type_hint
                        })
                    },
                }
            }
        }
        _ => expression,
    }
}

impl MultiVectorClass {
    pub fn flat_basis(&self) -> Vec<BasisElement> {
        self.grouped_basis.iter().flatten().cloned().collect()
    }

    pub fn signature(&self) -> Vec<BasisElementIndex> {
        let mut signature: Vec<BasisElementIndex> = self.grouped_basis.iter().flatten().map(|element| element.index).collect();
        signature.sort_unstable();
        signature
    }

    pub fn index_in_group(&self, mut index: usize) -> (usize, usize) {
        for (group_index, group) in self.grouped_basis.iter().enumerate() {
            if index >= group.len() {
                index -= group.len();
            } else {
                return (group_index, index);
            }
        }
        unreachable!()
    }

    pub fn constant<'a>(&'a self, name: &'static str) -> AstNode<'a> {
        let (scalar_value, other_values) = match name {
            "Zero" => (0, 0),
            "One" => (1, 0),
            _ => unreachable!(),
        };
        let mut body = Vec::new();
        for result_group in self.grouped_basis.iter() {
            let size = result_group.len();
            let expression = Expression {
                size,
                content: ExpressionContent::Constant(
                    DataType::SimdVector(size),
                    result_group
                        .iter()
                        .map(|element| if element.index == 0 { scalar_value } else { other_values })
                        .collect(),
                ),
                data_type_hint: Some(DataType::SimdVector(size))
            };
            body.push((DataType::SimdVector(size), *simplify_and_legalize(Box::new(expression))));
        }
        AstNode::TraitImplementation {
            result: Parameter {
                name,
                data_type: DataType::MultiVector(self),
            },
            parameters: vec![],
            body: vec![AstNode::ReturnStatement {
                expression: Box::new(Expression {
                    size: 1,
                    content: ExpressionContent::InvokeClassMethod(self, "Constructor", body),
                    data_type_hint: Some(DataType::MultiVector(self))
                }),
            }],
        }
    }

    pub fn involution<'a>(
        name: &'static str,
        involution: &Involution,
        parameter_a: &Parameter<'a>,
        registry: &'a MultiVectorClassRegistry,
        project: bool,
    ) -> AstNode<'a> {
        let a_flat_basis = parameter_a.multi_vector_class().flat_basis();
        let mut result_signature = Vec::new();
        for a_element in a_flat_basis.iter() {
            for (in_element, out_element) in involution.terms.iter() {
                if in_element.index == a_element.index {
                    result_signature.push(out_element.index);
                    break;
                }
            }
        }
        if project {
            for (in_element, _out_element) in involution.terms.iter() {
                if !a_flat_basis.iter().any(|element| element.index == in_element.index) {
                    return AstNode::None;
                }
            }
        }
        result_signature.sort_unstable();
        let result_class = match registry.get(&result_signature) {
            None => return AstNode::None,
            Some(rc) => rc,
        };
        let result_flat_basis = result_class.flat_basis();
        let mut body = Vec::new();
        let mut base_index = 0;
        for result_group in result_class.grouped_basis.iter() {
            let size = result_group.len();
            let (factors, a_indices): (Vec<_>, Vec<_>) = (0..size)
                .map(|index_in_group| {
                    let result_element = &result_flat_basis[base_index + index_in_group];
                    let involution_element = involution
                        .terms
                        .iter()
                        .position(|(_in_element, out_element)| out_element.index == result_element.index)
                        .unwrap();
                    let (in_element, out_element) = &involution.terms[involution_element];
                    let index_in_a = a_flat_basis.iter().position(|a_element| a_element.index == in_element.index).unwrap();
                    (
                        out_element.coefficient * result_element.coefficient * in_element.coefficient * a_flat_basis[index_in_a].coefficient,
                        parameter_a.multi_vector_class().index_in_group(index_in_a),
                    )
                })
                .unzip();
            let a_group_index = a_indices[0].0;
            let expression = Expression {
                size,
                content: ExpressionContent::Multiply(
                    Box::new(Expression {
                        size,
                        content: ExpressionContent::Gather(
                            Box::new(Expression {
                                size: parameter_a.multi_vector_class().grouped_basis[a_group_index].len(),
                                content: ExpressionContent::Variable(parameter_a.name),
                                data_type_hint: None
                            }),
                            a_indices,
                        ),
                        data_type_hint: None,
                    }),
                    Box::new(Expression {
                        size,
                        content: ExpressionContent::Constant(DataType::SimdVector(size), factors),
                        data_type_hint: Some(DataType::SimdVector(size))
                    }),
                ),
                data_type_hint: Some(DataType::SimdVector(size))
            };
            body.push((DataType::SimdVector(size), *simplify_and_legalize(Box::new(expression))));
            base_index += size;
        }
        AstNode::TraitImplementation {
            result: Parameter {
                name,
                data_type: DataType::MultiVector(result_class),
            },
            parameters: vec![parameter_a.clone()],
            body: vec![AstNode::ReturnStatement {
                expression: Box::new(Expression {
                    size: 1,
                    content: ExpressionContent::InvokeClassMethod(result_class, "Constructor", body),
                    data_type_hint: Some(DataType::MultiVector(result_class))
                }),
            }],
        }
    }

    pub fn element_wise<'a>(
        name: &'static str,
        parameter_a: &Parameter<'a>,
        parameter_b: &Parameter<'a>,
        registry: &'a MultiVectorClassRegistry,
    ) -> AstNode<'a> {
        let a_flat_basis = parameter_a.multi_vector_class().flat_basis();
        let b_flat_basis = parameter_b.multi_vector_class().flat_basis();
        let result_signature = a_flat_basis
            .iter()
            .chain(b_flat_basis.iter())
            .cloned()
            .collect::<std::collections::HashSet<_>>();
        let mut result_signature = result_signature.into_iter().map(|element| element.index).collect::<Vec<_>>();
        result_signature.sort_unstable();
        if let Some(result_class) = registry.get(&result_signature) {
            let parameters = [(parameter_a, &a_flat_basis), (parameter_b, &b_flat_basis)];
            let mut body = Vec::new();
            for result_group in result_class.grouped_basis.iter() {
                let size = result_group.len();
                let mut expressions = parameters.iter().map(|(parameter, flat_basis)| {
                    let mut parameter_group_index = None;
                    let terms: Vec<_> = result_group
                        .iter()
                        .map(|result_element| {
                            if let Some(index_in_flat_basis) = flat_basis.iter().position(|element| element.index == result_element.index) {
                                let index_pair = parameter.multi_vector_class().index_in_group(index_in_flat_basis);
                                parameter_group_index = Some(index_pair.0);
                                (result_element.coefficient * flat_basis[index_in_flat_basis].coefficient, index_pair)
                            } else {
                                (0, (0, 0))
                            }
                        })
                        .collect();
                    Expression {
                        size,
                        content: ExpressionContent::Multiply(
                            Box::new(Expression {
                                size,
                                content: ExpressionContent::Gather(
                                    Box::new(Expression {
                                        size: if let Some(index) = parameter_group_index {
                                            parameter.multi_vector_class().grouped_basis[index].len()
                                        } else {
                                            size
                                        },
                                        content: ExpressionContent::Variable(parameter.name),
                                        data_type_hint: None
                                    }),
                                    terms.iter().map(|(_factor, index_pair)| index_pair).cloned().collect(),
                                ),
                                data_type_hint: None
                            }),
                            Box::new(Expression {
                                size,
                                content: ExpressionContent::Constant(
                                    DataType::SimdVector(size),
                                    terms.iter().map(|(factor, _index_pair)| *factor).collect::<Vec<_>>(),
                                ),
                                data_type_hint: Some(DataType::SimdVector(size))
                            }),
                        ),
                        data_type_hint: Some(DataType::SimdVector(size))
                    }
                });
                body.push((
                    DataType::SimdVector(size),
                    *simplify_and_legalize(Box::new(Expression {
                        size,
                        content: match name {
                            "Add" => ExpressionContent::Add(Box::new(expressions.next().unwrap()), Box::new(expressions.next().unwrap())),
                            "Sub" => ExpressionContent::Subtract(Box::new(expressions.next().unwrap()), Box::new(expressions.next().unwrap())),
                            "Mul" => ExpressionContent::Multiply(Box::new(expressions.next().unwrap()), Box::new(expressions.next().unwrap())),
                            "Div" => ExpressionContent::Divide(Box::new(expressions.next().unwrap()), Box::new(expressions.next().unwrap())),
                            _ => unreachable!(),
                        },
                        data_type_hint: Some(DataType::SimdVector(size))
                    })),
                ));
            }
            AstNode::TraitImplementation {
                result: Parameter {
                    name,
                    data_type: DataType::MultiVector(result_class),
                },
                parameters: vec![parameter_a.clone(), parameter_b.clone()],
                body: vec![AstNode::ReturnStatement {
                    expression: Box::new(Expression {
                        size: 1,
                        content: ExpressionContent::InvokeClassMethod(result_class, "Constructor", body),
                        data_type_hint: Some(DataType::MultiVector(result_class))
                    }),
                }],
            }
        } else {
            AstNode::None
        }
    }

    pub fn product<'a>(
        name: &'static str,
        product: &Product,
        parameter_a: &Parameter<'a>,
        parameter_b: &Parameter<'a>,
        registry: &'a MultiVectorClassRegistry,
    ) -> AstNode<'a> {
        let a_flat_basis = parameter_a.multi_vector_class().flat_basis();
        let b_flat_basis = parameter_b.multi_vector_class().flat_basis();
        let mut result_signature = std::collections::HashSet::new();
        for product_term in product.terms.iter() {
            if a_flat_basis.iter().any(|e| e.index == product_term.factor_a.index)
                && b_flat_basis.iter().any(|e| e.index == product_term.factor_b.index)
            {
                for pt in product_term.product {
                    result_signature.insert(pt.index);
                }
            }
        }
        let mut result_signature = result_signature.into_iter().collect::<Vec<_>>();
        result_signature.sort_unstable();

        // Be a little bit more flexible when finding a result type
        // Needed (for example) in order to get geometric product on Motor x Line
        // without having to predefine every intermediate type of product

        let mut result_class = registry.get(&result_signature);

        if result_class.is_none() && !result_signature.is_empty() {
            let mut viable_classes: Vec<_> = registry.classes.iter().filter(|it| {
                let sig = it.0.signature();
                result_signature.iter().all(|it| sig.contains(it))
            }).collect();
            viable_classes.sort_by_key(|it| it.0.signature().len());
            result_class = viable_classes.first().map(|it| &it.0);
        }

        let result_class = match result_class {
            None => return AstNode::None,
            Some(rc) => rc
        };

        let result_flat_basis = result_class.flat_basis();
        let mut new_sorted_terms: BTreeMap<usize, Vec<(isize, usize, usize)>> = BTreeMap::new();
        let mut stuff = product.terms.iter().flat_map(|it| {
            it.product.iter().map(|p| {
                (it.factor_a.clone(), it.factor_b.clone(), p.clone())
            })
        });
        for (factor_a, factor_b, product) in stuff {
            let a_position = a_flat_basis.iter().position(|e| e.index == factor_a.index);
            let b_position = b_flat_basis.iter().position(|e| e.index == factor_b.index);
            let result_position = result_flat_basis.iter().position(|e| e.index == product.index);
            let (a_flat_index, b_flat_index, result_flat_index) = match (a_position, b_position, result_position) {
                (Some(a), Some(b), Some(r)) => (a, b, r),
                _ => continue
            };
            let coefficient
                = result_flat_basis[result_flat_index].coefficient
                * product.coefficient
                * a_flat_basis[a_flat_index].coefficient
                * factor_a.coefficient
                * b_flat_basis[b_flat_index].coefficient
                * factor_b.coefficient;
            new_sorted_terms.entry(result_flat_index)
                .and_modify(|v| v.push((coefficient, a_flat_index, b_flat_index)))
                .or_insert(vec![(coefficient, a_flat_index, b_flat_index)]);
        }


        let mut body = Vec::new();
        let mut base_index = 0;
        for result_group in result_class.grouped_basis.iter() {
            let result_group_size = result_group.len();
            let mut expression = Expression {
                size: result_group_size,
                content: ExpressionContent::None,
                data_type_hint: None
            };

            let mut contraction_expr_a = Expression {
                size: result_group_size,
                content: ExpressionContent::None,
                data_type_hint: None
            };
            let mut contraction_expr_b = Expression {
                size: result_group_size,
                content: ExpressionContent::None,
                data_type_hint: None
            };
            let mut contraction_expr_a_gather_indices = vec![(0, 0); expression.size];
            let mut contraction_expr_b_gather_indices = vec![(0, 0); expression.size];
            let mut last_term_coefficients = vec![0; expression.size];

            // TODO this is divider line of refactor progress (refactor is incomplete below, and complete above)
            //  Don't forget that, whatever is going on, I'll need to add an "Add" expression. So expect that.

            // TODO result_terms[0].len() is trying to figure out how many factor_a terms contribute to this result term
            //  because just beware that result_terms is not the same data type before and after this refactor pass, but
            //  len() exists for both.
            //  So... what transposed_terms was PREVIOUSLY trying to express, was....
            //  well, result_terms was to the effect of Map<a_factor_position, (coefficient, b_factor_position)>
            //  except instead of a Map, it's a Vec accessed by index. Anyway result_terms is a Vec<> of these
            //  "effective Maps", that correspond to this result_group in the result_class.
            //  So what is transposed_terms?
            //  transposed_terms would have been... for each position_in_a...
            //  Okay, jesus fucking christ this is annoying. All the trouble of using Vecs instead of properly keyed
            //  maps, to then turn the Vecs into iters, and recover the index with an enumerate. Christ.
            //  Okay so you can't truly comprehend transposed_terms without also the foresight that enumerate is called
            //  on it. Come on man you could have just made the tuple in the map() invocation instead of making
            //  this so convoluted and obscure. So there's an implicit factor_a_position (aka index_in_a), then we...
            //  Collect the (coefficient, factor_b_position) for all result_terms at each factor_a_position..
            //  it's goddamn confusing, because "inner[i]" is per result_term, but also per i (the outer loop)
            //  goddamn it this is frustrating


            // Prior to refactor...
            // sorted_terms:
            //      Vec<Vec<(isize, usize)>> as in map(result_basis_index, map(a_basis_index, (coefficient, b_basis_index)))
            // result_terms:
            //      (per result_group, with size of result_group.len()), in other words vec length/index is index_in_group
            //      Vec<Vec<(isize, usize)>> as in map(index_in_group, map(a_basis_index, (coefficient, b_basis_index)))
            //      so compared to sorted_terms, result_terms has an index within the result_group, instead of an index within the entire flat_basis
            // transposed_terms:
            //      Seems to assume all components of this result_group depend on the same quantity of a components
            //      So i is a_basis_index, from 0 to max_a_basis_index
            //      If we assume the outer map can be collected into a Vec too, it looks like this expresses roughly Vec<Vec<(isize, usize)>>
            //      Actually.... yeah.... it's not just assuming every component of this result_group depends on the same quantity of a components... it is enforcing that...
            //      it does an iter().map() on result_terms, but just accesses inside the thing... is it like....
            //      is it changing map(index_in_group, map(a_basis_index, (coefficient, b_basis_index))) into map(a_basis_index, map(index_in_group, (coefficient, b_basis_index)))?
            //      no not quite.... it is more like... creating the following...
            //      map(a_basis_index, vec(coefficient, b_basis_index))
            //      the index of the inner vec doesnt mean anything. I think so, anyway.
            //      well no.... so look ahead. We have the hypothesis on what a_terms means. And enumerate isn't
            //      usually used on it, but is sometimes. Okay. It seems to represent "just some index within the result_group"
            //      ...particularly when it comes to the last term?

            // After refactor...
            // new_sorted_terms:
            //      BTreeMap<usize, Vec<(isize, usize, usize)>> as in map(result_basis_index, (coefficient, a_basis_index, b_basis_index))
            //







            // TODO refactor line





            // index_in_group -> sum of terms: coefficient, a_flat_index, b_flat_index
            // (this makes no assumptions about how many times each factor_flat_index can show up in each index_in_group)

            let result_terms_per_index_in_group = (0..result_group_size)
                .map(|index_in_group| new_sorted_terms.remove(&(base_index + index_in_group)).unwrap_or_default())
                .collect::<Vec<_>>();

            //

            let transposed_terms: Vec<Vec<(isize, usize)>> = vec![];







            // For each position_in_a used in this result_group
            for (a_flat_index, coefficient_and_b_index_per_result_group_index) in transposed_terms.iter().enumerate() {

                // If all coefficients are zero, then move on. We'll put in a const simd4 of 0s after this for loop.
                if coefficient_and_b_index_per_result_group_index.iter().all(|(factor, _)| *factor == 0) {
                    continue;
                }

                // a_indices and b_indices are the "index_in_group" for a and b
                // TODO this is the only use of index_in_a.... am I sure I'm not mixing up index_in_a and index_in_result_group?
                let (a_group_index, a_index_in_group) = parameter_a.multi_vector_class().index_in_group(a_flat_index);
                let a_indices = coefficient_and_b_index_per_result_group_index.iter().map(|_| (a_group_index, a_index_in_group)).collect::<Vec<_>>();
                let b_indices = coefficient_and_b_index_per_result_group_index
                    .iter()
                    .map(|(_, b_flat_index)| parameter_b.multi_vector_class().index_in_group(*b_flat_index))
                    .collect::<Vec<_>>();

                // Get the first entry with a non-zero coefficient
                let non_zero_index = coefficient_and_b_index_per_result_group_index.iter().position(|(factor, _)| *factor != 0).unwrap();

                // then get the group index of b having that non-zero coefficent
                let b_group_index = b_indices[non_zero_index].0;
                // Then overwrite (shadow) b_indices..... by.... replacing any zero terms with the non-zero term found?
                let b_indices = coefficient_and_b_index_per_result_group_index
                    .iter()
                    .enumerate()
                    .map(|(index, (factor, _))| b_indices[if *factor == 0 { non_zero_index } else { index }])
                    .collect::<Vec<_>>();

                // What conditions to be contractible?
                // - all factors per result group are zero
                // - we have not yet initialized contraction_expr_a, or its size is the same as the parameter_a group
                // - we have not yet initialized contraction_expr_b, or its size is the same as the parameter_b group
                let is_contractible = coefficient_and_b_index_per_result_group_index.iter().enumerate().all(
                    |(result_group_index, (factor, _))| *factor == 0 || last_term_coefficients[result_group_index] == 0
                )
                    && (contraction_expr_a.content == ExpressionContent::None
                        || contraction_expr_a.size == parameter_a.multi_vector_class().grouped_basis[a_group_index].len())
                    && (contraction_expr_b.content == ExpressionContent::None
                        || contraction_expr_b.size == parameter_b.multi_vector_class().grouped_basis[b_group_index].len());




                if is_contractible && coefficient_and_b_index_per_result_group_index.iter().any(|(factor, _)| *factor == 0) {
                    if contraction_expr_a.content == ExpressionContent::None {
                        assert!(contraction_expr_b.content == ExpressionContent::None);

                        // TODO so this is the real business when it comes to contraction
                        //  as you can see, it just takes Variable(parameter_a) and Variable(parameter_b) raw?
                        //  Well.. it's just the start, this branch will only be entered once per result_group.
                        //  Okay so, it is used in a "Gather" expression.
                        //  contraction_expr_a is gathered at contraction_expr_a_gather_indices
                        //  contraction_expr_b is gathered at contraction_expr_b_gather_indices
                        //  You can also see where these are finally used, it is only in the so called "last_term"
                        //  where it is multiplied by "last_term_coefficients"
                        contraction_expr_a = Expression {
                            size: parameter_a.multi_vector_class().grouped_basis[a_group_index].len(),
                            content: ExpressionContent::Variable(parameter_a.name),
                            data_type_hint: Some(parameter_a.data_type.clone())
                        };
                        contraction_expr_b = Expression {
                            size: parameter_b.multi_vector_class().grouped_basis[b_group_index].len(),
                            content: ExpressionContent::Variable(parameter_b.name),
                            data_type_hint: Some(parameter_b.data_type.clone())
                        };
                        contraction_expr_a_gather_indices = a_indices.iter().map(|(a_group_index, _)| (*a_group_index, 0)).collect();
                        contraction_expr_b_gather_indices = b_indices.iter().map(|(b_group_index, _)| (*b_group_index, 0)).collect();
                    }
                    for (result_group_index, (factor, _index_in_b)) in coefficient_and_b_index_per_result_group_index.iter().enumerate() {
                        if *factor != 0 {
                            contraction_expr_a_gather_indices[result_group_index] = a_indices[result_group_index];
                            contraction_expr_b_gather_indices[result_group_index] = b_indices[result_group_index];
                            last_term_coefficients[result_group_index] = *factor;
                        }
                    }
                } else {

                    // expression = expression + (gather(parameter_a, a_indices) * gather(parameter_b, b_indices) * coefficients)
                    expression = Expression {
                        size: result_group_size,
                        data_type_hint: None,
                        content: ExpressionContent::Add(
                            Box::new(expression),
                            Box::new(Expression {
                                size: result_group_size,
                                data_type_hint: None,
                                content: ExpressionContent::Multiply(
                                    Box::new(Expression {
                                        size: result_group_size,
                                        data_type_hint: None,
                                        content: ExpressionContent::Gather(
                                            Box::new(Expression {
                                                size: parameter_a.multi_vector_class().grouped_basis[a_group_index].len(),
                                                content: ExpressionContent::Variable(parameter_a.name),
                                                data_type_hint: Some(parameter_a.data_type.clone())
                                            }),
                                            a_indices,
                                        ),
                                    }),
                                    Box::new(Expression {
                                        size: result_group_size,
                                        data_type_hint: Some(DataType::SimdVector(result_group_size)),
                                        content: ExpressionContent::Multiply(
                                            Box::new(Expression {
                                                size: result_group_size,
                                                data_type_hint: None,
                                                content: ExpressionContent::Gather(
                                                    Box::new(Expression {
                                                        size: parameter_b.multi_vector_class().grouped_basis[b_group_index].len(),
                                                        data_type_hint: None,
                                                        content: ExpressionContent::Variable(parameter_b.name),
                                                    }),
                                                    b_indices,
                                                ),
                                            }),
                                            Box::new(Expression {
                                                size: result_group_size,
                                                data_type_hint: Some(DataType::SimdVector(result_group_size)),
                                                content: ExpressionContent::Constant(
                                                    DataType::SimdVector(result_group_size),
                                                    coefficient_and_b_index_per_result_group_index.iter().map(|(factor, _)| *factor).collect::<Vec<_>>(),
                                                ),
                                            }),
                                        ),
                                    }),
                                ),
                            }),
                        ),
                    };
                }
            }


            // TODO refactoring above this line
            // TODO don't forget to simplify_and_legalize all components of the sum (if applicable)


            // If there are any (non-zero) coefficients for the last term in this result_group...
            if last_term_coefficients.iter().any(|scalar| *scalar != 0) {
                expression = Expression {
                    size: result_group_size,
                    content: ExpressionContent::Add(

                        // then add the expression so far...
                        Box::new(expression),

                        // ...and the last_term_coefficients times the contraction factors
                        Box::new(Expression {
                            size: result_group_size,
                            content: ExpressionContent::Multiply(
                                Box::new(Expression {
                                    size: result_group_size,
                                    content: ExpressionContent::Multiply(
                                        Box::new(Expression {
                                            size: result_group_size,
                                            content: ExpressionContent::Gather(Box::new(contraction_expr_a), contraction_expr_a_gather_indices),
                                            data_type_hint: None
                                        }),
                                        Box::new(Expression {
                                            size: result_group_size,
                                            content: ExpressionContent::Gather(Box::new(contraction_expr_b), contraction_expr_b_gather_indices),
                                            data_type_hint: None
                                        }),
                                    ),
                                    data_type_hint: None
                                }),
                                Box::new(Expression {
                                    size: result_group_size,
                                    content: ExpressionContent::Constant(DataType::SimdVector(result_group_size), last_term_coefficients),
                                    data_type_hint: Some(DataType::SimdVector(result_group_size))
                                }),
                            ),
                            data_type_hint: None
                        }),
                    ),
                    data_type_hint: None
                };
            }

            // If this entire result_group has not been expressed yet...
            if expression.content == ExpressionContent::None {

                // ...then it is zero
                expression = Expression {
                    size: result_group_size,
                    content: ExpressionContent::Constant(DataType::SimdVector(result_group_size), (0..result_group_size).map(|_| 0).collect()),
                    data_type_hint: Some(DataType::SimdVector(result_group_size))
                };
            }

            // Push the expression for this result group
            body.push((DataType::SimdVector(result_group_size), *simplify_and_legalize(Box::new(expression))));

            // and move on to the next result_group
            base_index += result_group_size;
        }
        if body.is_empty() {
            return AstNode::None
        }
        AstNode::TraitImplementation {
            result: Parameter {
                name,
                data_type: DataType::MultiVector(result_class),
            },
            parameters: vec![parameter_a.clone(), parameter_b.clone()],
            body: vec![AstNode::ReturnStatement {
                expression: Box::new(Expression {
                    size: 1,
                    content: ExpressionContent::InvokeClassMethod(result_class, "Constructor", body),
                    data_type_hint: Some(DataType::MultiVector(result_class))
                }),
            }],
        }


        /* Example from rga3d

impl WedgeDot<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn wedge_dot(self, other: MultiVector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {

                g0: Simd32x2::from(self.group0()[0]) * other.group0()
                    + Simd32x2::from(self.group1()[0]) * Simd32x2::from([other.group1()[0], other.group4()[0]])
                    + Simd32x2::from(self.group1()[1]) * Simd32x2::from([other.group1()[1], other.group4()[1]])
                    + Simd32x2::from(self.group1()[2]) * Simd32x2::from([other.group1()[2], other.group4()[2]])
                    + Simd32x2::from(self.group1()[3]) * Simd32x2::from(                     other.group4()[3]) * Simd32x2::from([0.0, 1.0])
                    + Simd32x2::from(self.group2()[0]) * Simd32x2::from(                     other.group3()[0]) * Simd32x2::from([0.0, -1.0])
                    + Simd32x2::from(self.group2()[1]) * Simd32x2::from(                     other.group3()[1]) * Simd32x2::from([0.0, -1.0])
                    + Simd32x2::from(self.group2()[2]) * Simd32x2::from(                     other.group3()[2]) * Simd32x2::from([0.0, -1.0])
                    - Simd32x2::from(self.group3()[0]) * Simd32x2::from([other.group3()[0], other.group2()[0]])
                    - Simd32x2::from(self.group3()[1]) * Simd32x2::from([other.group3()[1], other.group2()[1]])
                    - Simd32x2::from(self.group3()[2]) * Simd32x2::from([other.group3()[2], other.group2()[2]])
                    + Simd32x2::from(self.group4()[0]) * Simd32x2::from(                     other.group1()[0]) * Simd32x2::from([0.0, -1.0])
                    + Simd32x2::from(self.group4()[1]) * Simd32x2::from(                     other.group1()[1]) * Simd32x2::from([0.0, -1.0])
                    + Simd32x2::from(self.group4()[2]) * Simd32x2::from(                     other.group1()[2]) * Simd32x2::from([0.0, -1.0])
                    - Simd32x2::from(self.group4()[3]) * Simd32x2::from([other.group4()[3], other.group1()[3]])
                    +                    self.group0() * Simd32x2::from(                     other.group0()[0]) * Simd32x2::from([0.0, 1.0]),

                g1: Simd32x4::from(self.group0()[0]) * other.group1()
                    + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group0()[0], other.group3()[2], other.group3()[1], other.group2()[0]]) * Simd32x4::from([1.0, 1.0, -1.0, -1.0])
                    + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group3()[2], other.group0()[0], other.group3()[0], other.group2()[1]]) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0])
                    + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group3()[1], other.group3()[0], other.group0()[0], other.group2()[2]]) * Simd32x4::from([1.0, -1.0, 1.0, -1.0])
                    + Simd32x4::from(self.group1()[3]) * Simd32x4::from(                                                           other.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0])
                    + Simd32x4::from(self.group2()[0]) * Simd32x4::from(                                                           other.group1()[0]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0])
                    + Simd32x4::from(self.group2()[1]) * Simd32x4::from(                                                           other.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0])
                    + Simd32x4::from(self.group2()[2]) * Simd32x4::from(                                                           other.group1()[2]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0])
                    + Simd32x4::from(self.group3()[0]) * Simd32x4::from([other.group4()[3], other.group1()[2], other.group1()[1], other.group4()[0]]) * Simd32x4::from([1.0, 1.0, -1.0, -1.0])
                    + Simd32x4::from(self.group3()[1]) * Simd32x4::from([other.group1()[2], other.group4()[3], other.group1()[0], other.group4()[1]]) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0])
                    + Simd32x4::from(self.group3()[2]) * Simd32x4::from([other.group1()[1], other.group1()[0], other.group4()[3], other.group4()[2]]) * Simd32x4::from([1.0, -1.0, 1.0, -1.0])
                    + Simd32x4::from(self.group4()[0]) * Simd32x4::from(                                                           other.group3()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0])
                    + Simd32x4::from(self.group4()[1]) * Simd32x4::from(                                                           other.group3()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0])
                    + Simd32x4::from(self.group4()[2]) * Simd32x4::from(                                                           other.group3()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0])
                    + Simd32x4::from(self.group4()[3]) * Simd32x4::from([other.group3()[0], other.group3()[1], other.group3()[2], other.group0()[1]])
                    + Simd32x4::from([self.group0()[0], self.group0()[0], self.group0()[0], self.group0()[1]]) * swizzle!(other.group4(), 0, 0, 0, 3) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),

                g2: Simd32x3::from(self.group0()[0]) * other.group2() + Simd32x3::from(self.group0()[1]) * other.group3()
                    + Simd32x3::from(self.group1()[0]) * Simd32x3::from([other.group1()[3], other.group4()[2], other.group4()[1]]) * Simd32x3::from([-1.0, -1.0, 1.0])
                    + Simd32x3::from(self.group1()[1]) * Simd32x3::from([other.group4()[2], other.group1()[3], other.group4()[0]]) * Simd32x3::from([1.0, -1.0, -1.0])
                    + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group4()[1], other.group4()[0], other.group1()[3]]) * Simd32x3::from([-1.0, 1.0, -1.0])
                    + Simd32x3::from(self.group1()[3]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]])
                    + Simd32x3::from(self.group2()[0]) * Simd32x3::from([other.group0()[0], other.group3()[2], other.group3()[1]]) * Simd32x3::from([1.0, 1.0, -1.0])
                    + Simd32x3::from(self.group2()[1]) * Simd32x3::from([other.group3()[2], other.group0()[0], other.group3()[0]]) * Simd32x3::from([-1.0, 1.0, 1.0])
                    + Simd32x3::from(self.group2()[2]) * Simd32x3::from([other.group3()[1], other.group3()[0], other.group0()[0]]) * Simd32x3::from([1.0, -1.0, 1.0])
                    + Simd32x3::from(self.group3()[0]) * Simd32x3::from([other.group0()[1], other.group2()[2], other.group2()[1]]) * Simd32x3::from([1.0, 1.0, -1.0])
                    + Simd32x3::from(self.group3()[1]) * Simd32x3::from([other.group2()[2], other.group0()[1], other.group2()[0]]) * Simd32x3::from([-1.0, 1.0, 1.0])
                    + Simd32x3::from(self.group3()[2]) * Simd32x3::from([other.group2()[1], other.group2()[0], other.group0()[1]]) * Simd32x3::from([1.0, -1.0, 1.0])
                    + Simd32x3::from(self.group4()[0]) * Simd32x3::from([other.group4()[3], other.group1()[2], other.group1()[1]]) * Simd32x3::from([1.0, 1.0, -1.0])
                    + Simd32x3::from(self.group4()[1]) * Simd32x3::from([other.group1()[2], other.group4()[3], other.group1()[0]]) * Simd32x3::from([-1.0, 1.0, 1.0])
                    + Simd32x3::from(self.group4()[2]) * Simd32x3::from([other.group1()[1], other.group1()[0], other.group4()[3]]) * Simd32x3::from([1.0, -1.0, 1.0])
                    - Simd32x3::from(self.group4()[3]) * Simd32x3::from([other.group4()[0], other.group4()[1], other.group4()[2]]),

                g3: Simd32x3::from(self.group0()[0]) * other.group3()
                    + Simd32x3::from(self.group1()[0]) * Simd32x3::from([other.group4()[3], other.group1()[2], other.group1()[1]]) * Simd32x3::from([-1.0, -1.0, 1.0])
                    + Simd32x3::from(self.group1()[1]) * Simd32x3::from([other.group1()[2], other.group4()[3], other.group1()[0]]) * Simd32x3::from([1.0, -1.0, -1.0])
                    + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group1()[1], other.group1()[0], other.group4()[3]]) * Simd32x3::from([-1.0, 1.0, -1.0])
                    + Simd32x3::from(self.group3()[0]) * Simd32x3::from([other.group0()[0], other.group3()[2], other.group3()[1]]) * Simd32x3::from([1.0, 1.0, -1.0])
                    + Simd32x3::from(self.group3()[1]) * Simd32x3::from([other.group3()[2], other.group0()[0], other.group3()[0]]) * Simd32x3::from([-1.0, 1.0, 1.0])
                    + Simd32x3::from(self.group3()[2]) * Simd32x3::from([other.group3()[1], other.group3()[0], other.group0()[0]]) * Simd32x3::from([1.0, -1.0, 1.0])
                    - Simd32x3::from(self.group4()[3]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),

                g4: Simd32x4::from(self.group0()[0]) * other.group4()
                    + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group0()[1], other.group2()[2], other.group2()[1], other.group3()[0]]) * Simd32x4::from([1.0, 1.0, -1.0, -1.0])
                    + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group2()[2], other.group0()[1], other.group2()[0], other.group3()[1]]) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0])
                    + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group2()[1], other.group2()[0], other.group0()[1], other.group3()[2]]) * Simd32x4::from([1.0, -1.0, 1.0, -1.0])
                    + Simd32x4::from(self.group1()[3]) * Simd32x4::from([other.group3()[0], other.group3()[1], other.group3()[2], other.group3()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0])
                    + Simd32x4::from(self.group2()[0]) * Simd32x4::from([other.group4()[3], other.group1()[2], other.group1()[1], other.group4()[3]]) * Simd32x4::from([-1.0, -1.0, 1.0, 0.0])
                    + Simd32x4::from(self.group2()[1]) * Simd32x4::from([other.group1()[2], other.group4()[3], other.group1()[0], other.group1()[2]]) * Simd32x4::from([1.0, -1.0, -1.0, 0.0])
                    + Simd32x4::from(self.group2()[2]) * Simd32x4::from([other.group1()[1], other.group1()[0], other.group4()[3], other.group1()[1]]) * Simd32x4::from([-1.0, 1.0, -1.0, 0.0])
                    + Simd32x4::from(self.group3()[0]) * Simd32x4::from([other.group1()[3], other.group4()[2], other.group4()[1], other.group1()[0]]) * Simd32x4::from([1.0, 1.0, -1.0, -1.0])
                    + Simd32x4::from(self.group3()[1]) * Simd32x4::from([other.group4()[2], other.group1()[3], other.group4()[0], other.group1()[1]]) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0])
                    + Simd32x4::from(self.group3()[2]) * Simd32x4::from([other.group4()[1], other.group4()[0], other.group1()[3], other.group1()[2]]) * Simd32x4::from([1.0, -1.0, 1.0, -1.0])
                    + Simd32x4::from(self.group4()[0]) * Simd32x4::from([other.group0()[0], other.group3()[2], other.group3()[1], other.group0()[0]]) * Simd32x4::from([1.0, 1.0, -1.0, 0.0])
                    + Simd32x4::from(self.group4()[1]) * Simd32x4::from([other.group3()[2], other.group0()[0], other.group3()[0], other.group3()[2]]) * Simd32x4::from([-1.0, 1.0, 1.0, 0.0])
                    + Simd32x4::from(self.group4()[2]) * Simd32x4::from([other.group3()[1], other.group3()[0], other.group0()[0], other.group3()[1]]) * Simd32x4::from([1.0, -1.0, 1.0, 0.0])
                    + Simd32x4::from(self.group4()[3]) * Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], other.group0()[0]])
                    + Simd32x4::from([self.group0()[1], self.group0()[1], self.group0()[1], self.group0()[0]]) * swizzle!(other.group1(), 0, 1, 2, 0) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0])
            }
        }
    }
}
         */
    }

    pub fn derive_unitize<'a>(
        name: &'static str,
        geometric_product: &AstNode<'a>,
        weight_norm: &AstNode<'a>,
        parameter_a: &Parameter<'a>,
        parameter_b: &Parameter<'a>,
    ) -> AstNode<'a> {
        let geometric_product_result = result_of_trait!(geometric_product);
        let weight_norm_result = result_of_trait!(weight_norm);
        AstNode::TraitImplementation {
            result: Parameter { name, data_type: geometric_product_result.data_type.clone() },
            parameters: vec![parameter_a.clone()],
            body: vec![AstNode::ReturnStatement {
                expression: Box::new(Expression {
                    size: 1,
                    content: ExpressionContent::InvokeInstanceMethod(
                        parameter_a.data_type.clone(),
                        Box::new(Expression {
                            size: 1,
                            content: ExpressionContent::Variable(parameter_a.name),
                            data_type_hint: Some(parameter_a.data_type.clone())
                        }),
                        geometric_product_result.name,
                        vec![(
                            DataType::MultiVector(parameter_b.multi_vector_class()),
                            Expression {
                                size: 1,
                                content: ExpressionContent::InvokeClassMethod(
                                    parameter_b.multi_vector_class(),
                                    "Constructor",
                                    vec![(
                                        DataType::SimdVector(1),
                                        Expression {
                                            size: 1,
                                            content: ExpressionContent::Divide(
                                                Box::new(Expression {
                                                    size: 1,
                                                    content: ExpressionContent::Constant(DataType::SimdVector(1), vec![1]),
                                                    data_type_hint: Some(DataType::SimdVector(1))
                                                }),
                                                Box::new(Expression {
                                                    size: 1,
                                                    content: ExpressionContent::Access(
                                                        Box::new(Expression {
                                                            size: 1,
                                                            content: ExpressionContent::InvokeInstanceMethod(
                                                                parameter_a.data_type.clone(),
                                                                Box::new(Expression {
                                                                    size: 1,
                                                                    content: ExpressionContent::Variable(parameter_a.name),
                                                                    data_type_hint: Some(parameter_a.data_type.clone())
                                                                }),
                                                                weight_norm_result.name,
                                                                vec![],
                                                            ),
                                                            data_type_hint: Some(weight_norm_result.data_type.clone())
                                                        }),
                                                        0,
                                                    ),
                                                    data_type_hint: None
                                                }),
                                            ),
                                            data_type_hint: None
                                        },
                                    )],
                                ),
                                data_type_hint: Some(parameter_b.data_type.clone())
                            },

                        )]
                    ),
                    data_type_hint: Some(geometric_product_result.data_type.clone())
                }),
            }],
        }
    }

    pub fn derive_scale<'a>(
        name: &'static str,
        geometric_product: &AstNode<'a>,
        parameter_a: &Parameter<'a>,
        parameter_b: &Parameter<'a>,
    ) -> AstNode<'a> {
        let geometric_product_result = result_of_trait!(geometric_product);
        AstNode::TraitImplementation {
            result: Parameter {
                name,
                data_type: geometric_product_result.data_type.clone(),
            },
            parameters: vec![
                parameter_a.clone(),
                Parameter {
                    name: "other",
                    data_type: DataType::SimdVector(1),
                },
            ],
            body: vec![AstNode::ReturnStatement {
                expression: Box::new(Expression {
                    size: 1,
                    content: ExpressionContent::InvokeInstanceMethod(
                        parameter_a.data_type.clone(),
                        Box::new(Expression {
                            size: 1,
                            content: ExpressionContent::Variable(parameter_a.name),
                            data_type_hint: Some(parameter_a.data_type.clone())
                        }),
                        geometric_product_result.name,
                        vec![(
                            DataType::MultiVector(parameter_b.multi_vector_class()),
                            Expression {
                                size: 1,
                                content: ExpressionContent::InvokeClassMethod(
                                    parameter_b.multi_vector_class(),
                                    "Constructor",
                                    vec![(
                                        DataType::SimdVector(1),
                                        Expression {
                                            size: 1,
                                            content: ExpressionContent::Variable(parameter_b.name),
                                            data_type_hint: Some(parameter_b.data_type.clone())
                                        },
                                    )],
                                ),
                                data_type_hint: Some(parameter_b.data_type.clone())
                            },
                        )],
                    ),
                    data_type_hint: Some(geometric_product_result.data_type.clone())
                }),
            }],
        }
    }

    pub fn derive_signum<'a>(
        name: &'static str,
        geometric_product: &AstNode<'a>,
        magnitude: &AstNode<'a>,
        parameter_a: &Parameter<'a>,
    ) -> AstNode<'a> {
        let geometric_product_result = result_of_trait!(geometric_product);
        let magnitude_result = result_of_trait!(magnitude);
        AstNode::TraitImplementation {
            result: Parameter {
                name,
                data_type: geometric_product_result.data_type.clone(),
            },
            parameters: vec![parameter_a.clone()],
            body: vec![AstNode::ReturnStatement {
                expression: Box::new(Expression {
                    size: 1,
                    data_type_hint: Some(geometric_product_result.data_type.clone()),
                    content: ExpressionContent::InvokeInstanceMethod(
                        parameter_a.data_type.clone(),
                        Box::new(Expression {
                            size: 1,
                            data_type_hint: Some(parameter_a.data_type.clone()),
                            content: ExpressionContent::Variable(parameter_a.name),
                        }),
                        geometric_product_result.name,
                        vec![(
                            DataType::MultiVector(magnitude_result.multi_vector_class()),
                            Expression {
                                size: 1,
                                data_type_hint: Some(magnitude_result.data_type.clone()),
                                content: ExpressionContent::InvokeClassMethod(
                                    magnitude_result.multi_vector_class(),
                                    "Constructor",
                                    vec![(
                                        DataType::SimdVector(1),
                                        Expression {
                                            size: 1,
                                            data_type_hint: Some(DataType::SimdVector(1)),
                                            content: ExpressionContent::Divide(
                                                Box::new(Expression {
                                                    size: 1,
                                                    data_type_hint: Some(DataType::SimdVector(1)),
                                                    content: ExpressionContent::Constant(DataType::SimdVector(1), vec![1]),
                                                }),
                                                Box::new(Expression {
                                                    size: 1,
                                                    data_type_hint: None,
                                                    content: ExpressionContent::Access(
                                                        Box::new(Expression {
                                                            size: 1,
                                                            content: ExpressionContent::InvokeInstanceMethod(
                                                                parameter_a.data_type.clone(),
                                                                Box::new(Expression {
                                                                    size: 1,
                                                                    content: ExpressionContent::Variable(parameter_a.name),
                                                                    data_type_hint: Some(parameter_a.data_type.clone())
                                                                }),
                                                                magnitude_result.name,
                                                                vec![],
                                                            ),
                                                            data_type_hint: Some(magnitude_result.data_type.clone())
                                                        }),
                                                        0,
                                                    ),
                                                }),
                                            ),
                                        },
                                    )],
                                ),
                            },
                        )],
                    ),
                }),
            }],
        }
    }

    pub fn derive_inverse<'a>(
        name: &'static str,
        geometric_product: &AstNode<'a>,
        squared_magnitude: &AstNode<'a>,
        involution: &AstNode<'a>,
        parameter_a: &Parameter<'a>,
    ) -> AstNode<'a> {
        let geometric_product_result = result_of_trait!(geometric_product);
        let squared_magnitude_result = result_of_trait!(squared_magnitude);
        let involution_result = result_of_trait!(involution);
        AstNode::TraitImplementation {
            result: Parameter {
                name,
                data_type: geometric_product_result.data_type.clone(),
            },
            parameters: vec![parameter_a.clone()],
            body: vec![AstNode::ReturnStatement {
                expression: Box::new(Expression {
                    size: 1,
                    data_type_hint: Some(geometric_product_result.data_type.clone()),
                    content: ExpressionContent::InvokeInstanceMethod(
                        involution_result.data_type.clone(),
                        Box::new(Expression {
                            size: 1,
                            data_type_hint: Some(involution_result.data_type.clone()),
                            content: ExpressionContent::InvokeInstanceMethod(
                                parameter_a.data_type.clone(),
                                Box::new(Expression {
                                    size: 1,
                                    data_type_hint: Some(parameter_a.data_type.clone()),
                                    content: ExpressionContent::Variable(parameter_a.name),
                                }),
                                involution_result.name,
                                vec![],
                            ),
                        }),
                        geometric_product_result.name,
                        vec![(
                            DataType::MultiVector(squared_magnitude_result.multi_vector_class()),
                            Expression {
                                size: 1,
                                data_type_hint: Some(squared_magnitude_result.data_type.clone()),
                                content: ExpressionContent::InvokeClassMethod(
                                    squared_magnitude_result.multi_vector_class(),
                                    "Constructor",
                                    vec![(
                                        DataType::SimdVector(1),
                                        Expression {
                                            size: 1,
                                            data_type_hint: Some(DataType::SimdVector(1)),
                                            content: ExpressionContent::Divide(
                                                Box::new(Expression {
                                                    size: 1,
                                                    data_type_hint: Some(DataType::SimdVector(1)),
                                                    content: ExpressionContent::Constant(DataType::SimdVector(1), vec![1]),
                                                }),
                                                Box::new(Expression {
                                                    size: 1,
                                                    data_type_hint: None,
                                                    content: ExpressionContent::Access(
                                                        Box::new(Expression {
                                                            size: 1,
                                                            data_type_hint: Some(squared_magnitude_result.data_type.clone()),
                                                            content: ExpressionContent::InvokeInstanceMethod(
                                                                parameter_a.data_type.clone(),
                                                                Box::new(Expression {
                                                                    size: 1,
                                                                    data_type_hint: Some(parameter_a.data_type.clone()),
                                                                    content: ExpressionContent::Variable(parameter_a.name),
                                                                }),
                                                                squared_magnitude_result.name,
                                                                vec![],
                                                            ),
                                                        }),
                                                        0,
                                                    ),
                                                }),
                                            ),
                                        },
                                    )],
                                ),
                            },
                        )],
                    ),
                }),
            }],
        }
    }

    pub fn derive_power_of_integer<'a>(
        name: &'static str,
        geometric_product: &AstNode<'a>,
        constant_one: &AstNode<'a>,
        inverse: &AstNode<'a>,
        parameter_a: &Parameter<'a>,
        parameter_b: &Parameter<'a>,
    ) -> AstNode<'a> {
        let geometric_product_result = result_of_trait!(geometric_product);
        let constant_one_result = result_of_trait!(constant_one);
        let inverse_result = result_of_trait!(inverse);
        AstNode::TraitImplementation {
            result: Parameter {
                name,
                data_type: parameter_a.data_type.clone(),
            },
            parameters: vec![parameter_a.clone(), parameter_b.clone()],
            body: vec![
                AstNode::IfThenBlock {
                    condition: Box::new(Expression {
                        size: 1,
                        data_type_hint: None,
                        content: ExpressionContent::Equal(
                            Box::new(Expression {
                                size: 1,
                                data_type_hint: Some(parameter_b.data_type.clone()),
                                content: ExpressionContent::Variable(parameter_b.name),
                            }),
                            Box::new(Expression {
                                size: 1,
                                data_type_hint: Some(DataType::Integer),
                                content: ExpressionContent::Constant(DataType::Integer, vec![0]),
                            }),
                        ),
                    }),
                    body: vec![AstNode::ReturnStatement {
                        expression: Box::new(Expression {
                            size: 1,
                            data_type_hint: Some(parameter_a.data_type.clone()),
                            content: ExpressionContent::InvokeClassMethod(parameter_a.multi_vector_class(), constant_one_result.name, vec![]),
                        }),
                    }],
                },
                AstNode::VariableAssignment {
                    name: "x",
                    data_type: Some(parameter_a.data_type.clone()),
                    expression: Box::new(Expression {
                        size: 1,
                        data_type_hint: None,
                        content: ExpressionContent::Select(
                            Box::new(Expression {
                                size: 1,
                                data_type_hint: None,
                                content: ExpressionContent::LessThan(
                                    Box::new(Expression {
                                        size: 1,
                                        data_type_hint: None,
                                        content: ExpressionContent::Variable(parameter_b.name),
                                    }),
                                    Box::new(Expression {
                                        size: 1,
                                        data_type_hint: None,
                                        content: ExpressionContent::Constant(DataType::Integer, vec![0]),
                                    }),
                                ),
                            }),
                            Box::new(Expression {
                                size: 1,
                                data_type_hint: Some(inverse_result.data_type.clone()),
                                content: ExpressionContent::InvokeInstanceMethod(
                                    parameter_a.data_type.clone(),
                                    Box::new(Expression {
                                        size: 1,
                                        data_type_hint: None,
                                        content: ExpressionContent::Variable(parameter_a.name),
                                    }),
                                    inverse_result.name,
                                    vec![],
                                ),
                            }),
                            Box::new(Expression {
                                size: 1,
                                data_type_hint: None,
                                content: ExpressionContent::Variable(parameter_a.name),
                            }),
                        ),
                    }),
                },
                AstNode::VariableAssignment {
                    name: "y",
                    data_type: Some(parameter_a.data_type.clone()),
                    expression: Box::new(Expression {
                        size: 1,
                        data_type_hint: None,
                        content: ExpressionContent::InvokeClassMethod(parameter_a.multi_vector_class(), constant_one_result.name, vec![]),
                    }),
                },
                AstNode::VariableAssignment {
                    name: "n",
                    data_type: Some(DataType::Integer),
                    expression: Box::new(Expression {
                        size: 1,
                        data_type_hint: None,
                        content: ExpressionContent::InvokeInstanceMethod(
                            DataType::Integer,
                            Box::new(Expression {
                                size: 1,
                                data_type_hint: None,
                                content: ExpressionContent::Variable(parameter_b.name),
                            }),
                            "Abs",
                            vec![],
                        ),
                    }),
                },
                AstNode::WhileLoopBlock {
                    condition: Box::new(Expression {
                        size: 1,
                        data_type_hint: None,
                        content: ExpressionContent::LessThan(
                            Box::new(Expression {
                                size: 1,
                                data_type_hint: None,
                                content: ExpressionContent::Constant(DataType::Integer, vec![1]),
                            }),
                            Box::new(Expression {
                                size: 1,
                                data_type_hint: None,
                                content: ExpressionContent::Variable("n"),
                            }),
                        ),
                    }),
                    body: vec![
                        AstNode::IfThenBlock {
                            condition: Box::new(Expression {
                                size: 1,
                                data_type_hint: None,
                                content: ExpressionContent::Equal(
                                    Box::new(Expression {
                                        size: 1,
                                        data_type_hint: None,
                                        content: ExpressionContent::LogicAnd(
                                            Box::new(Expression {
                                                size: 1,
                                                data_type_hint: None,
                                                content: ExpressionContent::Variable("n"),
                                            }),
                                            Box::new(Expression {
                                                size: 1,
                                                data_type_hint: None,
                                                content: ExpressionContent::Constant(DataType::Integer, vec![1]),
                                            }),
                                        ),
                                    }),
                                    Box::new(Expression {
                                        size: 1,
                                        data_type_hint: None,
                                        content: ExpressionContent::Constant(DataType::Integer, vec![1]),
                                    }),
                                ),
                            }),
                            body: vec![AstNode::VariableAssignment {
                                name: "y",
                                data_type: None,
                                expression: Box::new(Expression {
                                    size: 1,
                                    data_type_hint: None,
                                    content: ExpressionContent::InvokeInstanceMethod(
                                        parameter_a.data_type.clone(),
                                        Box::new(Expression {
                                            size: 1,
                                            data_type_hint: None,
                                            content: ExpressionContent::Variable("x"),
                                        }),
                                        geometric_product_result.name,
                                        vec![(
                                            DataType::MultiVector(parameter_a.multi_vector_class()),
                                            Expression {
                                                size: 1,
                                                data_type_hint: None,
                                                content: ExpressionContent::Variable("y"),
                                            },
                                        )],
                                    ),
                                }),
                            }],
                        },
                        AstNode::VariableAssignment {
                            name: "x",
                            data_type: None,
                            expression: Box::new(Expression {
                                size: 1,
                                data_type_hint: None,
                                content: ExpressionContent::InvokeInstanceMethod(
                                    parameter_a.data_type.clone(),
                                    Box::new(Expression {
                                        size: 1,
                                        data_type_hint: None,
                                        content: ExpressionContent::Variable("x"),
                                    }),
                                    geometric_product_result.name,
                                    vec![(
                                        DataType::MultiVector(parameter_a.multi_vector_class()),
                                        Expression {
                                            size: 1,
                                            data_type_hint: None,
                                            content: ExpressionContent::Variable("x"),
                                        },
                                    )],
                                ),
                            }),
                        },
                        AstNode::VariableAssignment {
                            name: "n",
                            data_type: None,
                            expression: Box::new(Expression {
                                size: 1,
                                data_type_hint: None,
                                content: ExpressionContent::BitShiftRight(
                                    Box::new(Expression {
                                        size: 1,
                                        data_type_hint: None,
                                        content: ExpressionContent::Variable("n"),
                                    }),
                                    Box::new(Expression {
                                        size: 1,
                                        data_type_hint: None,
                                        content: ExpressionContent::Constant(DataType::Integer, vec![1]),
                                    }),
                                ),
                            }),
                        },
                    ],
                },
                AstNode::ReturnStatement {
                    expression: Box::new(Expression {
                        size: 1,
                        data_type_hint: None,
                        content: ExpressionContent::InvokeInstanceMethod(
                            parameter_a.data_type.clone(),
                            Box::new(Expression {
                                size: 1,
                                data_type_hint: None,
                                content: ExpressionContent::Variable("x"),
                            }),
                            geometric_product_result.name,
                            vec![(
                                DataType::MultiVector(parameter_a.multi_vector_class()),
                                Expression {
                                    size: 1,
                                    data_type_hint: None,
                                    content: ExpressionContent::Variable("y"),
                                },
                            )],
                        ),
                    }),
                },
            ],
        }
    }

    pub fn derive_division<'a>(
        name: &'static str,
        geometric_product: &AstNode<'a>,
        inverse: &AstNode<'a>,
        parameter_a: &Parameter<'a>,
        parameter_b: &Parameter<'a>,
    ) -> AstNode<'a> {
        let geometric_product_result = result_of_trait!(geometric_product);
        let inverse_result = result_of_trait!(inverse);
        AstNode::TraitImplementation {
            result: Parameter {
                name,
                data_type: geometric_product_result.data_type.clone(),
            },
            parameters: vec![parameter_a.clone(), parameter_b.clone()],
            body: vec![AstNode::ReturnStatement {
                expression: Box::new(Expression {
                    size: 1,
                    data_type_hint: None,
                    content: ExpressionContent::InvokeInstanceMethod(
                        parameter_a.data_type.clone(),
                        Box::new(Expression {
                            size: 1,
                            data_type_hint: None,
                            content: ExpressionContent::Variable(parameter_a.name),
                        }),
                        geometric_product_result.name,
                        vec![(
                            DataType::MultiVector(inverse_result.multi_vector_class()),
                            Expression {
                                size: 1,
                                data_type_hint: None,
                                content: ExpressionContent::InvokeInstanceMethod(
                                    parameter_b.data_type.clone(),
                                    Box::new(Expression {
                                        size: 1,
                                        data_type_hint: None,
                                        content: ExpressionContent::Variable(parameter_b.name),
                                    }),
                                    inverse_result.name,
                                    vec![],
                                ),
                            },
                        )],
                    ),
                }),
            }],
        }
    }

    // TODO I can probably make this more succinct, even with the conditional Into stuff
    pub fn derive_sandwich_product<'a>(
        name: &'static str,
        geometric_product: &AstNode<'a>,
        geometric_product_2: &AstNode<'a>,
        involution: &AstNode<'a>,
        conversion: Option<&AstNode<'a>>,
        parameter_a: &Parameter<'a>,
        parameter_b: &Parameter<'a>,
    ) -> AstNode<'a> {
        let geometric_product_result = result_of_trait!(geometric_product);
        let geometric_product_2_result = result_of_trait!(geometric_product_2);
        let involution_result = result_of_trait!(involution);
        let product = Box::new(Expression {
            size: 1,
            data_type_hint: Some(geometric_product_2_result.data_type.clone()),
            content: ExpressionContent::InvokeInstanceMethod(
                geometric_product_result.data_type.clone(),
                Box::new(Expression {
                    size: 1,
                    data_type_hint: Some(geometric_product_result.data_type.clone()),
                    content: ExpressionContent::InvokeInstanceMethod(
                        parameter_a.data_type.clone(),
                        Box::new(Expression {
                            size: 1,
                            data_type_hint: Some(parameter_a.data_type.clone()),
                            content: ExpressionContent::Variable(parameter_a.name),
                        }),
                        geometric_product_result.name,
                        vec![(
                            DataType::MultiVector(parameter_b.multi_vector_class()),
                            Expression {
                                size: 1,
                                data_type_hint: Some(parameter_b.data_type.clone()),
                                content: ExpressionContent::Variable(parameter_b.name),
                            },
                        )],
                    ),
                }),
                geometric_product_2_result.name,
                vec![(
                    DataType::MultiVector(involution_result.multi_vector_class()),
                    Expression {
                        size: 1,
                        data_type_hint: Some(involution_result.data_type.clone()),
                        content: ExpressionContent::InvokeInstanceMethod(
                            parameter_a.data_type.clone(),
                            Box::new(Expression {
                                size: 1,
                                data_type_hint: Some(parameter_a.data_type.clone()),
                                content: ExpressionContent::Variable(parameter_a.name),
                            }),
                            involution_result.name,
                            vec![],
                        ),
                    },
                )],
            ),
        });
        let conversion_result = if let Some(conversion) = conversion {
            result_of_trait!(conversion)
        } else {
            geometric_product_2_result
        };
        AstNode::TraitImplementation {
            result: Parameter {
                name,
                data_type: conversion_result.data_type.clone(),
            },
            parameters: vec![parameter_a.clone(), parameter_b.clone()],
            body: vec![AstNode::ReturnStatement {
                expression: if conversion.is_some() {
                    Box::new(Expression {
                        size: 1,
                        data_type_hint: Some(conversion_result.data_type.clone()),
                        content: ExpressionContent::Conversion(
                            geometric_product_2_result.multi_vector_class(),
                            conversion_result.multi_vector_class(),
                            product,
                        ),
                    })
                } else {
                    product
                },
            }],
        }
    }

    pub fn derive_grade<'a>(
        name: &'static str,
        parameter_a: &Parameter<'a>,
        grade: usize
    ) -> AstNode<'a> {
        AstNode::TraitImplementation {
            result: Parameter { name, data_type: DataType::Integer },
            parameters: vec![parameter_a.clone()],
            body: vec![AstNode::ReturnStatement {
                expression: Box::new(Expression {
                    size: 0,
                    data_type_hint: Some(DataType::Integer),
                    content: ExpressionContent::Constant(DataType::Integer, vec![grade as isize])
                }),
            }],
        }
    }

    pub fn derive_bulk_or_weight<'a, GA: GeometricAlgebraTrait>(
        name: &'static str,
        parameter_a: &Parameter<'a>,
        projective_basis: &BasisElement,
        is_projective: bool,
        algebra: &GA,
        registry: &'a MultiVectorClassRegistry,
    ) -> AstNode<'a> {

        let mut result_signature = Vec::new();
        let a_flat_basis = parameter_a.multi_vector_class().flat_basis();
        for a_element in a_flat_basis.iter() {
            let products = algebra.product(projective_basis, a_element);
            if is_projective && products.is_empty() {
                result_signature.push(a_element.index)
            } else if !is_projective && !products.is_empty() {
                result_signature.push(a_element.index)
            } else {
                continue
            }
        }
        result_signature.sort_unstable();
        if result_signature.is_empty() {
            return AstNode::None
        }
        let mut param_a_signature = parameter_a.multi_vector_class().signature();
        param_a_signature.sort_unstable();
        if param_a_signature == result_signature {
            return single_expression_single_trait_impl(name, &parameter_a, variable(&parameter_a))
        }

        // Most objects have bulk and weight.
        // We'll try to find an exact match for the result class.
        // If there is no exact match, we'll try to find the closest match.
        // If nothing else, the starting class should always suffice.

        let mut result_class = registry.get(&result_signature);
        if result_class.is_none() && !result_signature.is_empty() {
            let mut viable_classes: Vec<_> = registry.classes.iter().filter(|it| {
                let sig = it.0.signature();
                result_signature.iter().all(|it| sig.contains(it)) &&
                    // Bulk of Line could be represented as Translator with zero anti-scalar, but that is weird
                    sig.iter().all(|it| param_a_signature.contains(it))
            }).collect();
            viable_classes.sort_by_key(|it| it.0.signature().len());
            result_class = viable_classes.first().map(|it| &it.0);
        }
        let result_class = result_class.unwrap_or_else(|| parameter_a.multi_vector_class());

        let result_flat_basis = result_class.flat_basis();
        let mut base_index = 0;
        let mut body = Vec::new();
        for result_group in result_class.grouped_basis.iter() {
            let size = result_group.len();

            let (factors, a_indices): (Vec<_>, Vec<_>) = (0..size)
                .map(|index_in_group| {
                    let result_element = &result_flat_basis[base_index + index_in_group];
                    let index_in_a = a_flat_basis.iter().position(|a_element| a_element == result_element).unwrap();
                    let result_element_is_projective = algebra.product(projective_basis, result_element).is_empty();
                    let scalar = if is_projective == result_element_is_projective {
                            1isize
                        } else {
                            0isize
                        };
                    let a_index = parameter_a.multi_vector_class().index_in_group(index_in_a);
                    (scalar, a_index)
                })
                .unzip();


            let a_group_index = a_indices[0].0;
            let expression = Expression {
                size,
                data_type_hint: None,
                content: ExpressionContent::Multiply(
                    Box::new(Expression {
                        size,
                        data_type_hint: None,
                        content: ExpressionContent::Gather(
                            Box::new(Expression {
                                size: parameter_a.multi_vector_class().grouped_basis[a_group_index].len(),
                                data_type_hint: Some(parameter_a.data_type.clone()),
                                content: ExpressionContent::Variable(parameter_a.name),
                            }),
                            a_indices,
                        ),
                    }),
                    Box::new(Expression {
                        size,
                        data_type_hint: Some(DataType::SimdVector(size)),
                        content: ExpressionContent::Constant(DataType::SimdVector(size), factors),
                    }),
                ),
            };
            // body.push((DataType::SimdVector(size), *simplify_and_legalize(Box::new(expression))));
            body.push((DataType::SimdVector(size), expression));
            base_index += size;
        }
        AstNode::TraitImplementation {
            result: Parameter { name, data_type: DataType::MultiVector(result_class) },
            parameters: vec![parameter_a.clone()],
            body: vec![AstNode::ReturnStatement {
                expression: Box::new(Expression {
                    size: 1,
                    data_type_hint: Some(DataType::MultiVector(result_class)),
                    content: ExpressionContent::InvokeClassMethod(result_class, "Constructor", body),
                }),
            }],
        }
    }
}



pub fn variable<'a>(param: &Parameter<'a>) -> Expression<'a> {
    Expression { size: 1, content: ExpressionContent::Variable(param.name), data_type_hint: Some(param.data_type.clone()) }
}

pub fn single_expression_pair_trait_impl<'a>(
    name: &'static str,

    parameter_a: &Parameter<'a>,
    parameter_b: &Parameter<'a>,
    expression: Expression<'a>
) -> AstNode<'a> {
    let data_type = match &expression.data_type_hint {
        Some(dt) => dt.clone(),
        _ => panic!("single_expression_pair_trait_impl for {name} requires data_type_hint on \"expression\" {expression:?}"),
    };
    AstNode::TraitImplementation {
        result: Parameter { name, data_type },
        parameters: vec![parameter_a.clone(), parameter_b.clone()],
        body: vec![
            AstNode::ReturnStatement { expression: Box::new(expression) }
        ]
    }
}

pub fn single_expression_single_trait_impl<'a>(
    name: &'static str,

    parameter_a: &Parameter<'a>,
    expression: Expression<'a>
) -> AstNode<'a> {
    let data_type = match &expression.data_type_hint {
        Some(dt) => dt.clone(),
        _ => panic!("single_expression_single_trait_impl for {name} requires data_type_hint on \"expression\" {expression:?}"),
    };
    AstNode::TraitImplementation {
        result: Parameter { name, data_type },
        parameters: vec![parameter_a.clone()],
        body: vec![
            AstNode::ReturnStatement { expression: Box::new(expression) }
        ]
    }
}

pub fn single_expression_class_trait_impl<'a>(
    name: &'static str,
    mvc: &'a MultiVectorClass,
    expression: Expression<'a>
) -> AstNode<'a> {
    AstNode::TraitImplementation {
        result: Parameter { name, data_type: DataType::MultiVector(mvc) },
        parameters: vec![],
        body: vec![
            AstNode::ReturnStatement { expression: Box::new(expression) }
        ]
    }
}