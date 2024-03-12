use crate::algebra::{MultiVectorClassRegistry, Product};
use crate::algebra::basis_element::BasisElementIndex;
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
    let mut result_signature = result_signature.into_iter().collect::<Vec<BasisElementIndex>>();
    result_signature.sort_unstable();

    let result_class = match registry.get(&result_signature) {
        None => return AstNode::None,
        Some(rc) => rc,
    };


    let result_flat_basis = result_class.flat_basis();
    let mut sorted_terms: Vec<Vec<(isize, usize)>> = vec![vec![(0, 0); a_flat_basis.len()]; result_flat_basis.len()];
    for product_term in product.terms.iter() {
        if let Some(result_flat_index) = result_flat_basis.iter().position(|e| e.index == product_term.product.index) {
            if let Some(a_flat_index) = a_flat_basis.iter().position(|e| e.index == product_term.factor_a.index) {
                if let Some(b_flat_index) = b_flat_basis.iter().position(|e| e.index == product_term.factor_b.index) {
                    sorted_terms[result_flat_index][a_flat_index] = (
                        result_flat_basis[result_flat_index].scalar
                            * product_term.product.scalar
                            * a_flat_basis[a_flat_index].scalar
                            * product_term.factor_a.scalar
                            * b_flat_basis[b_flat_index].scalar
                            * product_term.factor_b.scalar,
                        b_flat_index,
                    );
                }
            }
        }
    }
    let mut body = Vec::new();
    let mut base_index = 0;
    for result_group in result_class.grouped_basis.iter() {
        let result_group_size = result_group.len();
        let mut expression = Expression {
            size: result_group_size,
            content: ExpressionContent::None,
            data_type_hint: None,
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





        // TODO refactor line




        // index_in_result_group -> a_flat_index -> (coefficient, b_flat_index)
        // (there is a built-in assumption that each a_flat_index is only incorporated once per index_in_result_group

        let result_terms_per_index_in_result_group = (0..result_group_size)
            .map(|index_in_result_group| &sorted_terms[base_index + index_in_result_group])
            .collect::<Vec<_>>();

        // Assume that all elements of this group have the same number of terms

        let qty_of_terms_per_result_group_element = result_terms_per_index_in_result_group[0].len();

        // a_flat_index -> index_in_group -> (coefficient, b_flat_index)
        // (so yeah... pretty much 99.9% sure I correctly understand transposed_terms now)

        let transposed_terms: Vec<Vec<(isize, usize)>> = (0..qty_of_terms_per_result_group_element)
            .map(|a_flat_index_in_result_group| result_terms_per_index_in_result_group
                .iter()
                .map(|terms_in_result_group| terms_in_result_group[a_flat_index_in_result_group])
                .collect())
            .collect();






        for (a_flat_index, coefficient_and_b_flat_index_per_index_in_result_group) in transposed_terms.enumerate() {
            if coefficient_and_b_flat_index_per_index_in_result_group.iter().all(|(factor, _)| *factor == 0) {
                continue;
            }
            let (a_group_index, a_group_sub_index) = parameter_a.multi_vector_class().index_in_group(a_flat_index);

            // All a_indices have the same value here.
            let a_indices = coefficient_and_b_flat_index_per_index_in_result_group.iter().map(|_| (a_group_index, a_group_sub_index)).collect::<Vec<_>>();


            let b_indices_per_index_in_result_group = coefficient_and_b_flat_index_per_index_in_result_group
                .iter()
                .map(|(_, b_flat_index)| parameter_b.multi_vector_class().index_in_group(*b_flat_index))
                .collect::<Vec<_>>();
            let non_zero_index = coefficient_and_b_flat_index_per_index_in_result_group.iter().position(|(factor, _index_pair)| *factor != 0).unwrap();
            let non_zero_b_group_index = b_indices_per_index_in_result_group[non_zero_index].0;
            let b_indices = coefficient_and_b_flat_index_per_index_in_result_group
                .iter()
                .enumerate()
                .map(|(index_in_result_group, (factor, _index_pair))| b_indices_per_index_in_result_group[if *factor == 0 { non_zero_index } else { index_in_result_group }])
                .collect::<Vec<_>>();
            let is_contractable = coefficient_and_b_flat_index_per_index_in_result_group.iter().enumerate().all(|(i, (factor, _))| *factor == 0 || last_term_coefficients[i] == 0)
                && (contraction_expr_a.content == ExpressionContent::None
                || contraction_expr_a.size == parameter_a.multi_vector_class().grouped_basis[a_group_index].len())
                && (contraction_expr_b.content == ExpressionContent::None
                || contraction_expr_b.size == parameter_b.multi_vector_class().grouped_basis[non_zero_b_group_index].len());
            if is_contractable && coefficient_and_b_flat_index_per_index_in_result_group.iter().any(|(factor, _)| *factor == 0) {
                if contraction_expr_a.content == ExpressionContent::None {
                    assert!(contraction_expr_b.content == ExpressionContent::None);
                    contraction_expr_a = Expression {
                        size: parameter_a.multi_vector_class().grouped_basis[a_group_index].len(),
                        content: ExpressionContent::Variable(parameter_a.data_type.clone(), parameter_a.name),
                        data_type_hint: None,
                    };
                    contraction_expr_b = Expression {
                        size: parameter_b.multi_vector_class().grouped_basis[non_zero_b_group_index].len(),
                        content: ExpressionContent::Variable(parameter_b.data_type.clone(), parameter_b.name),
                        data_type_hint: None,
                    };
                    contraction_expr_a_gather_indices = a_indices.iter().map(|(a_group_index, _)| (*a_group_index, 0)).collect();
                    contraction_expr_b_gather_indices = b_indices.iter().map(|(b_group_index, _)| (*b_group_index, 0)).collect();
                }
                for (i, (factor, _index_in_b)) in coefficient_and_b_flat_index_per_index_in_result_group.iter().enumerate() {
                    if *factor != 0 {
                        contraction_expr_a_gather_indices[i] = a_indices[i];
                        contraction_expr_b_gather_indices[i] = b_indices[i];
                        last_term_coefficients[i] = *factor;
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
                                            data_type_hint: None,
                                            content: ExpressionContent::Variable(parameter_a.name),
                                        }),
                                        a_indices,
                                    ),
                                }),
                                Box::new(Expression {
                                    size: result_group_size,
                                    data_type_hint: None,
                                    content: ExpressionContent::Multiply(
                                        Box::new(Expression {
                                            size: result_group_size,
                                            data_type_hint: None,
                                            content: ExpressionContent::Gather(
                                                Box::new(Expression {
                                                    size: parameter_b.multi_vector_class().grouped_basis[non_zero_b_group_index].len(),
                                                    data_type_hint: None,
                                                    content: ExpressionContent::Variable(parameter_b.name),
                                                }),
                                                b_indices,
                                            ),
                                        }),
                                        Box::new(Expression {
                                            size: result_group_size,
                                            data_type_hint: None,
                                            content: ExpressionContent::Constant(
                                                DataType::SimdVector(result_group_size),
                                                coefficient_and_b_flat_index_per_index_in_result_group.iter().map(|(factor, _)| *factor).collect::<Vec<_>>(),
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

        // TODO refactor line


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
                                        data_type_hint: None,
                                    }),
                                    Box::new(Expression {
                                        size: result_group_size,
                                        content: ExpressionContent::Gather(Box::new(contraction_expr_b), contraction_expr_b_gather_indices),
                                        data_type_hint: None,
                                    }),
                                ),
                                data_type_hint: None,
                            }),
                            Box::new(Expression {
                                size: result_group_size,
                                content: ExpressionContent::Constant(DataType::SimdVector(result_group_size), last_term_coefficients),
                                data_type_hint: None,
                            }),
                        ),
                        data_type_hint: None,
                    }),
                ),
                data_type_hint: None,
            };
        }

        // If this entire result_group has not been expressed yet...
        if expression.content == ExpressionContent::None {

            // ...then it is zero
            expression = Expression {
                size: result_group_size,
                content: ExpressionContent::Constant(DataType::SimdVector(result_group_size), (0..result_group_size).map(|_| 0).collect()),
                data_type_hint: None,
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
                data_type_hint: None,
            }),
        }],
    }
}
