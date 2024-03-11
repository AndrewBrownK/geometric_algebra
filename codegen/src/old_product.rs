use crate::algebra::{MultiVectorClassRegistry, Product};
use crate::ast::{AstNode, DataType, Expression, ExpressionContent, Parameter};
use crate::compile::simplify_and_legalize;

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
            result_signature.insert(product_term.product.index);
        }
    }
    let mut result_signature = result_signature.into_iter().collect::<Vec<_>>();
    result_signature.sort_unstable();
    if let Some(result_class) = registry.get(&result_signature) {
        let result_flat_basis = result_class.flat_basis();
        let mut sorted_terms = vec![vec![(0, 0); a_flat_basis.len()]; result_flat_basis.len()];
        for product_term in product.terms.iter() {
            if let Some(y) = result_flat_basis.iter().position(|e| e.index == product_term.product.index) {
                if let Some(x) = a_flat_basis.iter().position(|e| e.index == product_term.factor_a.index) {
                    if let Some(gather_index) = b_flat_basis.iter().position(|e| e.index == product_term.factor_b.index) {
                        sorted_terms[y][x] = (
                            result_flat_basis[y].scalar
                                * product_term.product.scalar
                                * a_flat_basis[x].scalar
                                * product_term.factor_a.scalar
                                * b_flat_basis[gather_index].scalar
                                * product_term.factor_b.scalar,
                            gather_index,
                        );
                    }
                }
            }
        }
        let mut body = Vec::new();
        let mut base_index = 0;
        for result_group in result_class.grouped_basis.iter() {
            let size = result_group.len();
            let mut expression = Expression {
                size,
                content: ExpressionContent::None,
                data_type_hint: None,
            };
            let result_terms = (0..size)
                .map(|index_in_group| &sorted_terms[base_index + index_in_group])
                .collect::<Vec<_>>();
            let transposed_terms = (0..result_terms[0].len()).map(|i| result_terms.iter().map(|inner| inner[i]).collect::<Vec<_>>());
            let mut contraction = (
                Expression {
                    size,
                    content: ExpressionContent::None,
                    data_type_hint: None,
                },
                Expression {
                    size,
                    content: ExpressionContent::None,
                    data_type_hint: None,
                },
                vec![(0, 0); expression.size],
                vec![(0, 0); expression.size],
                vec![0; expression.size],
            );
            for (index_in_a, a_terms) in transposed_terms.enumerate() {
                if a_terms.iter().all(|(factor, _)| *factor == 0) {
                    continue;
                }
                let (a_group_index, a_index_in_group) = parameter_a.multi_vector_class().index_in_group(index_in_a);
                let a_indices = a_terms.iter().map(|_| (a_group_index, a_index_in_group)).collect::<Vec<_>>();
                let b_indices = a_terms
                    .iter()
                    .map(|(_, index_in_b)| parameter_b.multi_vector_class().index_in_group(*index_in_b))
                    .collect::<Vec<_>>();
                let non_zero_index = a_terms.iter().position(|(factor, _index_pair)| *factor != 0).unwrap();
                let b_group_index = b_indices[non_zero_index].0;
                let b_indices = a_terms
                    .iter()
                    .enumerate()
                    .map(|(index, (factor, _index_pair))| b_indices[if *factor == 0 { non_zero_index } else { index }])
                    .collect::<Vec<_>>();
                let is_contractable = a_terms.iter().enumerate().all(|(i, (factor, _))| *factor == 0 || contraction.4[i] == 0)
                    && (contraction.0.content == ExpressionContent::None
                    || contraction.0.size == parameter_a.multi_vector_class().grouped_basis[a_group_index].len())
                    && (contraction.1.content == ExpressionContent::None
                    || contraction.1.size == parameter_b.multi_vector_class().grouped_basis[b_group_index].len());
                if is_contractable && a_terms.iter().any(|(factor, _)| *factor == 0) {
                    if contraction.0.content == ExpressionContent::None {
                        assert!(contraction.1.content == ExpressionContent::None);
                        contraction.0 = Expression {
                            size: parameter_a.multi_vector_class().grouped_basis[a_group_index].len(),
                            content: ExpressionContent::Variable(parameter_a.data_type.clone(), parameter_a.name),
                            data_type_hint: None,
                        };
                        contraction.1 = Expression {
                            size: parameter_b.multi_vector_class().grouped_basis[b_group_index].len(),
                            content: ExpressionContent::Variable(parameter_b.data_type.clone(), parameter_b.name),
                            data_type_hint: None,
                        };
                        contraction.2 = a_indices.iter().map(|(a_group_index, _)| (*a_group_index, 0)).collect();
                        contraction.3 = b_indices.iter().map(|(b_group_index, _)| (*b_group_index, 0)).collect();
                    }
                    for (i, (factor, _index_in_b)) in a_terms.iter().enumerate() {
                        if *factor != 0 {
                            contraction.2[i] = a_indices[i];
                            contraction.3[i] = b_indices[i];
                            contraction.4[i] = *factor;
                        }
                    }
                } else {
                    expression = Expression {
                        size,
                        content: ExpressionContent::Add(
                            Box::new(expression),
                            Box::new(Expression {
                                size,
                                content: ExpressionContent::Multiply(
                                    Box::new(Expression {
                                        size,
                                        content: ExpressionContent::Gather(
                                            Box::new(Expression {
                                                size: parameter_a.multi_vector_class().grouped_basis[a_group_index].len(),
                                                content: ExpressionContent::Variable(parameter_a.data_type.clone(), parameter_a.name),
                                                data_type_hint: None,
                                            }),
                                            a_indices,
                                        ),
                                        data_type_hint: None,
                                    }),
                                    Box::new(Expression {
                                        size,
                                        content: ExpressionContent::Multiply(
                                            Box::new(Expression {
                                                size,
                                                content: ExpressionContent::Gather(
                                                    Box::new(Expression {
                                                        size: parameter_b.multi_vector_class().grouped_basis[b_group_index].len(),
                                                        content: ExpressionContent::Variable(parameter_b.data_type.clone(), parameter_b.name),
                                                        data_type_hint: None,
                                                    }),
                                                    b_indices,
                                                ),
                                                data_type_hint: None,
                                            }),
                                            Box::new(Expression {
                                                size,
                                                content: ExpressionContent::Constant(
                                                    DataType::SimdVector(size),
                                                    a_terms.iter().map(|(factor, _)| *factor).collect::<Vec<_>>(),
                                                ),
                                                data_type_hint: None,
                                            }),
                                        ),
                                        data_type_hint: None,
                                    }),
                                ),
                                data_type_hint: None,
                            }),
                        ),
                        data_type_hint: None,
                    };
                }
            }
            if contraction.4.iter().any(|scalar| *scalar != 0) {
                expression = Expression {
                    size,
                    content: ExpressionContent::Add(
                        Box::new(expression),
                        Box::new(Expression {
                            size,
                            content: ExpressionContent::Multiply(
                                Box::new(Expression {
                                    size,
                                    content: ExpressionContent::Multiply(
                                        Box::new(Expression {
                                            size,
                                            content: ExpressionContent::Gather(Box::new(contraction.0), contraction.2),
                                            data_type_hint: None,
                                        }),
                                        Box::new(Expression {
                                            size,
                                            content: ExpressionContent::Gather(Box::new(contraction.1), contraction.3),
                                            data_type_hint: None,
                                        }),
                                    ),
                                    data_type_hint: None,
                                }),
                                Box::new(Expression {
                                    size,
                                    content: ExpressionContent::Constant(DataType::SimdVector(size), contraction.4),
                                    data_type_hint: None,
                                }),
                            ),
                            data_type_hint: None,
                        }),
                    ),
                    data_type_hint: None,
                };
            }
            if expression.content == ExpressionContent::None {
                expression = Expression {
                    size,
                    content: ExpressionContent::Constant(DataType::SimdVector(size), (0..size).map(|_| 0).collect()),
                    data_type_hint: None,
                };
            }
            body.push((DataType::SimdVector(size), *simplify_and_legalize(Box::new(expression))));
            base_index += size;
        }
        if body.is_empty() {
            AstNode::None
        } else {
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
                        data_type_hint: None,
                    }),
                }],
            }
        }
    } else {
        AstNode::None
    }
}
