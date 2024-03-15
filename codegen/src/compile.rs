use crate::algebra::basis_element::{BasisElement, BasisElementIndex};
use crate::algebra::GeometricAlgebraTrait;
use crate::ast::{GatherData, UsualGatherData};
use crate::{
    algebra::{Involution, MultiVectorClass, MultiVectorClassRegistry, Product},
    ast::{AstNode, DataType, Expression, ExpressionContent, Parameter},
};
use std::collections::BTreeMap;

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
            let is_all_consts = indices.iter().all(|it| match it {
                GatherData::Usual(_) => false,
                GatherData::RawZero => true,
            });
            if is_all_consts {
                let the_consts: Vec<_> = indices
                    .iter()
                    .map(|it| match it {
                        GatherData::RawZero => 0isize,
                        GatherData::Usual(_) => unreachable!(),
                    })
                    .collect();
                let size = the_consts.len();
                return Box::new(Expression {
                    size,
                    data_type_hint: None,
                    content: ExpressionContent::Constant(DataType::SimdVector(size), the_consts),
                });
            }

            let first_group_data = indices.iter().find_map(|it| match it {
                GatherData::Usual(u) => Some(u),
                _ => None,
            });
            if let Some(first_group_data) = first_group_data {
                inner_expression = simplify_and_legalize(inner_expression);

                let all_gathered_items_are_same = indices
                    .iter()
                    .all(|it| if let GatherData::Usual(u) = it { u == first_group_data } else { false });
                let gathering_in_same_group = indices.iter().all(|it| {
                    if let GatherData::Usual(u) = it {
                        u.group == first_group_data.group
                    } else {
                        false
                    }
                });
                let no_raw_consts = indices.iter().all(|it| if let GatherData::Usual(_) = it { true } else { false });
                let some_components_out_of_order =
                    indices
                        .iter()
                        .enumerate()
                        .any(|(i, it)| if let GatherData::Usual(u) = it { i != u.element } else { true });

                if all_gathered_items_are_same {
                    return Box::new(Expression {
                        size: expression.size,
                        content: ExpressionContent::Gather(inner_expression, vec![GatherData::Usual(first_group_data.clone())]),
                        data_type_hint: None,
                    });
                }

                if expression.size == indices.len() && gathering_in_same_group {
                    // We are gathering_in_same_group
                    let agreed_size = first_group_data.group_size;
                    if agreed_size == indices.len() {
                        inner_expression = Box::new(Expression {
                            size: expression.size,
                            content: ExpressionContent::Access(inner_expression, first_group_data.group),
                            data_type_hint: None,
                        });
                        if no_raw_consts && some_components_out_of_order && agreed_size > 1 {
                            return Box::new(Expression {
                                size: expression.size,
                                content: ExpressionContent::Swizzle(
                                    inner_expression,
                                    indices
                                        .iter()
                                        .map(|it| match it {
                                            GatherData::Usual(u) => u.element,
                                            _ => unreachable!(),
                                        })
                                        .collect(),
                                ),
                                data_type_hint: None,
                            });
                        }
                        return inner_expression;
                    }
                }
                return Box::new(Expression {
                    size: expression.size,
                    content: ExpressionContent::Gather(inner_expression, indices),
                    data_type_hint: None,
                });
            } else {
                Box::new(Expression {
                    size: expression.size,
                    content: ExpressionContent::None,
                    data_type_hint: None,
                })
            }
        }
        ExpressionContent::Constant(ref data_type, ref values) => {
            let first_value = values.first().unwrap();
            if values.iter().all(|value| value == first_value) {
                Box::new(Expression {
                    size: expression.size,
                    content: ExpressionContent::Constant(data_type.clone(), vec![*first_value]),
                    data_type_hint: Some(data_type.clone()),
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
                                data_type_hint: None,
                            });
                            return simplify_and_legalize(Box::new(Expression {
                                size: expression.size,
                                content: ExpressionContent::Subtract(a, b),
                                data_type_hint: None,
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
                let data_type_hint = if a.data_type_hint == b.data_type_hint {
                    a.data_type_hint.clone()
                } else {
                    None
                };
                Box::new(Expression {
                    size: expression.size,
                    content: ExpressionContent::Add(a, b),
                    data_type_hint,
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
                return b;
            }
            match b.content {
                ExpressionContent::None => return a,
                ExpressionContent::Constant(_, c) if c.iter().all(|c| *c == 1) => return a,
                ExpressionContent::Constant(_, c) if c.iter().all(|c| *c == 0) => {
                    return Box::new(Expression {
                        size: expression.size,
                        content: ExpressionContent::None,
                        data_type_hint: None,
                    })
                }
                _ => {}
            }

            match (&mut a.content, &mut b.content) {
                (ExpressionContent::Gather(_, gather_data), ExpressionContent::Constant(_, c))
                    if c.iter().all(|c| *c == 1 || *c == 0 || *c == -1) =>
                {
                    for (gather_data, c) in gather_data.iter_mut().zip(c) {
                        match *c {
                            0 => *gather_data = GatherData::RawZero,
                            1 => {}
                            -1 => {
                                if let GatherData::Usual(u) = gather_data {
                                    u.negate = !u.negate;
                                }
                            }
                            _ => unreachable!(),
                        }
                    }
                    return a;
                }
                _ => {}
            }

            let data_type_hint = if a.data_type_hint == b.data_type_hint {
                a.data_type_hint.clone()
            } else {
                None
            };
            return Box::new(Expression {
                size: expression.size,
                content: ExpressionContent::Multiply(a, b),
                data_type_hint,
            });
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
                data_type_hint: Some(DataType::SimdVector(size)),
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
                    data_type_hint: Some(DataType::MultiVector(self)),
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
            let mut a_group_index = None;
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
                    let coefficients =
                        out_element.coefficient * result_element.coefficient * in_element.coefficient * a_flat_basis[index_in_a].coefficient;
                    let (group, element) = parameter_a.multi_vector_class().index_in_group(index_in_a);
                    let group_size = parameter_a.multi_vector_class().grouped_basis[group].len();
                    if a_group_index.is_none() {
                        a_group_index = Some(group);
                    }
                    let negate = false;
                    (
                        coefficients,
                        GatherData::Usual(UsualGatherData {
                            negate,
                            group,
                            element,
                            group_size,
                        }),
                    )
                })
                .unzip();
            let a_group_index = a_group_index.unwrap();
            let expression = Expression {
                size,
                content: ExpressionContent::Multiply(
                    Box::new(Expression {
                        size,
                        content: ExpressionContent::Gather(
                            Box::new(Expression {
                                size: parameter_a.multi_vector_class().grouped_basis[a_group_index].len(),
                                content: ExpressionContent::Variable(parameter_a.name),
                                data_type_hint: None,
                            }),
                            a_indices,
                        ),
                        data_type_hint: None,
                    }),
                    Box::new(Expression {
                        size,
                        content: ExpressionContent::Constant(DataType::SimdVector(size), factors),
                        data_type_hint: Some(DataType::SimdVector(size)),
                    }),
                ),
                data_type_hint: Some(DataType::SimdVector(size)),
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
                    data_type_hint: Some(DataType::MultiVector(result_class)),
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
                                let group_size = parameter.multi_vector_class().grouped_basis[index_pair.0].len();
                                let negate = false;
                                let gd = GatherData::Usual(UsualGatherData {
                                    negate,
                                    group: index_pair.0,
                                    element: index_pair.1,
                                    group_size,
                                });
                                (result_element.coefficient * flat_basis[index_in_flat_basis].coefficient, gd)
                            } else {
                                (0, GatherData::RawZero)
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
                                        data_type_hint: None,
                                    }),
                                    terms.iter().map(|(_, index_pair)| index_pair).cloned().collect(),
                                ),
                                data_type_hint: None,
                            }),
                            Box::new(Expression {
                                size,
                                content: ExpressionContent::Constant(
                                    DataType::SimdVector(size),
                                    terms.iter().map(|(factor, _index_pair)| *factor).collect::<Vec<_>>(),
                                ),
                                data_type_hint: Some(DataType::SimdVector(size)),
                            }),
                        ),
                        data_type_hint: Some(DataType::SimdVector(size)),
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
                        data_type_hint: Some(DataType::SimdVector(size)),
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
                        data_type_hint: Some(DataType::MultiVector(result_class)),
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
                for pt in product_term.product.iter() {
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
            let mut viable_classes: Vec<_> = registry
                .classes
                .iter()
                .filter(|it| {
                    let sig = it.0.signature();
                    result_signature.iter().all(|it| sig.contains(it))
                })
                .collect();
            viable_classes.sort_by_key(|it| it.0.signature().len());
            result_class = viable_classes.first().map(|it| &it.0);
        }

        let result_class = match result_class {
            None => return AstNode::None,
            Some(rc) => rc,
        };

        let result_flat_basis = result_class.flat_basis();
        let mut terms_in_result: BTreeMap<usize, Vec<(isize, usize, usize)>> = BTreeMap::new();
        let stuff = product
            .terms
            .iter()
            .flat_map(|it| it.product.iter().map(|p| (it.factor_a.clone(), it.factor_b.clone(), p.clone())));
        for (factor_a, factor_b, product) in stuff {
            let a_position = a_flat_basis.iter().position(|e| e.index == factor_a.index);
            let b_position = b_flat_basis.iter().position(|e| e.index == factor_b.index);
            let result_position = result_flat_basis.iter().position(|e| e.index == product.index);
            let (a_flat_index, b_flat_index, result_flat_index) = match (a_position, b_position, result_position) {
                (Some(a), Some(b), Some(r)) => (a, b, r),
                _ => continue,
            };
            let coefficient = result_flat_basis[result_flat_index].coefficient
                * product.coefficient
                * a_flat_basis[a_flat_index].coefficient
                * factor_a.coefficient
                * b_flat_basis[b_flat_index].coefficient
                * factor_b.coefficient;
            terms_in_result
                .entry(result_flat_index)
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
                data_type_hint: None,
            };

            let mut terms_by_a: BTreeMap<(usize, usize), [Vec<(isize, usize, usize)>; 4]> = BTreeMap::new();
            for index_in_group in 0..result_group_size {
                let terms = terms_in_result.remove(&(base_index + index_in_group)).unwrap_or_default();
                for (coefficient, a_flat_index, b_flat_index) in terms {
                    let (a_group, a_element) = parameter_a.multi_vector_class().index_in_group(a_flat_index);
                    let (b_group, b_element) = parameter_b.multi_vector_class().index_in_group(b_flat_index);
                    terms_by_a
                        .entry((a_group, a_element))
                        .and_modify(|it| it[index_in_group].push((coefficient, b_group, b_element)))
                        .or_insert_with(|| {
                            let mut v = [vec![], vec![], vec![], vec![]];
                            v[index_in_group].push((coefficient, b_group, b_element));
                            v
                        });
                }
            }

            let mut new_terms_by_a = BTreeMap::new();

            let mut latest_entry = None;
            for ((a_group, a), [mut terms_0, mut terms_1, mut terms_2, mut terms_3]) in terms_by_a {
                if latest_entry.is_none() {
                    latest_entry = Some(((a_group, vec![a; result_group_size]), [terms_0, terms_1, terms_2, terms_3]));
                    continue;
                }
                let ((contract_a_group, mut contract_a), [mut contract_terms_0, mut contract_terms_1, mut contract_terms_2, mut contract_terms_3]) =
                    latest_entry.take().unwrap();

                let a_group_match = a_group == contract_a_group;
                let can_contract_on_0 = terms_0.iter().all(|it| it.0 == 0) || contract_terms_0.iter().all(|it| it.0 == 0);
                let can_contract_on_1 = terms_1.iter().all(|it| it.0 == 0) || contract_terms_1.iter().all(|it| it.0 == 0);
                let can_contract_on_2 = terms_2.iter().all(|it| it.0 == 0) || contract_terms_2.iter().all(|it| it.0 == 0);
                let can_contract_on_3 = terms_3.iter().all(|it| it.0 == 0) || contract_terms_3.iter().all(|it| it.0 == 0);

                let can_contract = a_group_match && can_contract_on_0 && can_contract_on_1 && can_contract_on_2 && can_contract_on_3;
                if can_contract {
                    if terms_0.iter().any(|it| it.0 != 0) && result_group_size > 0 {
                        contract_a[0] = a;
                    }
                    if terms_1.iter().any(|it| it.0 != 0) && result_group_size > 1 {
                        contract_a[1] = a;
                    }
                    if terms_2.iter().any(|it| it.0 != 0) && result_group_size > 2 {
                        contract_a[2] = a;
                    }
                    if terms_3.iter().any(|it| it.0 != 0) && result_group_size > 3 {
                        contract_a[3] = a;
                    }
                    contract_terms_0.append(&mut terms_0);
                    contract_terms_1.append(&mut terms_1);
                    contract_terms_2.append(&mut terms_2);
                    contract_terms_3.append(&mut terms_3);
                    latest_entry = Some((
                        (contract_a_group, contract_a),
                        [contract_terms_0, contract_terms_1, contract_terms_2, contract_terms_3],
                    ));
                } else {
                    new_terms_by_a.insert(
                        (contract_a_group, contract_a),
                        [contract_terms_0, contract_terms_1, contract_terms_2, contract_terms_3],
                    );
                    latest_entry = Some(((a_group, vec![a; result_group_size]), [terms_0, terms_1, terms_2, terms_3]));
                }
            }
            if let Some((key, val)) = latest_entry {
                new_terms_by_a.insert(key, val);
            }

            for ((a_group, a), [mut terms_0, mut terms_1, mut terms_2, mut terms_3]) in new_terms_by_a {
                let a_size = parameter_a.multi_vector_class().grouped_basis[a_group].len();
                let a_indices: Vec<_> = a
                    .iter()
                    .map(|a| {
                        let negate = false;
                        GatherData::Usual(UsualGatherData {
                            negate,
                            group: a_group,
                            element: *a,
                            group_size: a_size,
                        })
                    })
                    .collect();
                'inner: while !terms_0.is_empty() || !terms_1.is_empty() || !terms_2.is_empty() || !terms_3.is_empty() {
                    let mut b_indices = vec![];
                    let mut coefficients = vec![];

                    // Sort by group_b
                    terms_0.sort_by_key(|it| it.1);
                    terms_1.sort_by_key(|it| it.1);
                    terms_2.sort_by_key(|it| it.1);
                    terms_3.sort_by_key(|it| it.1);

                    let (c, b_group, b) = terms_0.pop().unwrap_or_else(|| (0, 0, 0));
                    let b_size = parameter_b.multi_vector_class().grouped_basis[b_group].len();
                    coefficients.push(c);
                    let negate = false;
                    let mut b_gather_data = GatherData::Usual(UsualGatherData {
                        negate,
                        group: b_group,
                        element: b,
                        group_size: b_size,
                    });
                    if c == 0 {
                        b_gather_data = GatherData::RawZero;
                    }
                    b_indices.push(b_gather_data);

                    if !terms_1.is_empty() {
                        assert!(result_group_size > 1);
                    }
                    if !terms_2.is_empty() {
                        assert!(result_group_size > 2);
                    }
                    if !terms_3.is_empty() {
                        assert!(result_group_size > 3);
                    }

                    let (c, b_group, b) = terms_1.pop().unwrap_or_else(|| (0, 0, 0));
                    if result_group_size > 1 {
                        let b_size = parameter_b.multi_vector_class().grouped_basis[b_group].len();
                        coefficients.push(c);
                        let negate = false;
                        let mut b_gather_data = GatherData::Usual(UsualGatherData {
                            negate,
                            group: b_group,
                            element: b,
                            group_size: b_size,
                        });
                        if c == 0 {
                            b_gather_data = GatherData::RawZero;
                        }
                        b_indices.push(b_gather_data);
                    }

                    let (c, b_group, b) = terms_2.pop().unwrap_or_else(|| (0, 0, 0));
                    if result_group_size > 2 {
                        let b_size = parameter_b.multi_vector_class().grouped_basis[b_group].len();
                        coefficients.push(c);
                        let negate = false;
                        let mut b_gather_data = GatherData::Usual(UsualGatherData {
                            negate,
                            group: b_group,
                            element: b,
                            group_size: b_size,
                        });
                        if c == 0 {
                            b_gather_data = GatherData::RawZero;
                        }
                        b_indices.push(b_gather_data);
                    }

                    let (c, b_group, b) = terms_3.pop().unwrap_or_else(|| (0, 0, 0));
                    if result_group_size > 3 {
                        let b_size = parameter_b.multi_vector_class().grouped_basis[b_group].len();
                        coefficients.push(c);
                        let negate = false;
                        let mut b_gather_data = GatherData::Usual(UsualGatherData {
                            negate,
                            group: b_group,
                            element: b,
                            group_size: b_size,
                        });
                        if c == 0 {
                            b_gather_data = GatherData::RawZero;
                        }
                        b_indices.push(b_gather_data);
                    }

                    if coefficients.iter().all(|it| *it == 0) {
                        continue 'inner;
                    }

                    // for (i, c) in coefficients.iter().enumerate() {
                    //     if *c == 0 && i < b_indices.len() {
                    //         b_indices[i] = GatherData::RawZero;
                    //     }
                    // }

                    let gather_a = Expression {
                        size: result_group_size,
                        data_type_hint: None,
                        content: ExpressionContent::Gather(
                            Box::new(Expression {
                                size: parameter_a.multi_vector_class().grouped_basis[a_group].len(),
                                data_type_hint: None,
                                content: ExpressionContent::Variable(parameter_a.name),
                            }),
                            a_indices.clone(),
                        ),
                    };

                    let mut gather_b = Expression {
                        size: result_group_size,
                        data_type_hint: None,
                        content: ExpressionContent::Gather(
                            Box::new(Expression {
                                size: result_group_size,
                                data_type_hint: None,
                                content: ExpressionContent::Variable(parameter_b.name),
                            }),
                            b_indices,
                        ),
                    };

                    if !coefficients.iter().all(|it| *it == 1) {
                        let const_coefficients = Expression {
                            size: result_group_size,
                            data_type_hint: None,
                            content: ExpressionContent::Constant(DataType::SimdVector(result_group_size), coefficients),
                        };
                        gather_b = Expression {
                            size: result_group_size,
                            data_type_hint: None,
                            content: ExpressionContent::Multiply(Box::new(gather_b), Box::new(const_coefficients)),
                        };
                    }

                    let mul = Expression {
                        size: result_group_size,
                        data_type_hint: None,
                        content: ExpressionContent::Multiply(Box::new(gather_a), Box::new(gather_b)),
                    };

                    let sum = Expression {
                        size: result_group_size,
                        data_type_hint: None,
                        content: ExpressionContent::Add(Box::new(expression), Box::new(mul)),
                    };

                    expression = sum;
                }
            }

            // If this entire result_group has not been expressed yet...
            if expression.content == ExpressionContent::None {
                // ...then it is zero
                expression = Expression {
                    size: result_group_size,
                    content: ExpressionContent::Constant(DataType::SimdVector(result_group_size), (0..result_group_size).map(|_| 0).collect()),
                    data_type_hint: Some(DataType::SimdVector(result_group_size)),
                };
            }

            // Push the expression for this result group
            let simplified = simplify_and_legalize(Box::new(expression));
            body.push((DataType::SimdVector(result_group_size), *simplified));

            // and move on to the next result_group
            base_index += result_group_size;
        }
        if body.is_empty() {
            return AstNode::None;
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
                    data_type_hint: Some(DataType::MultiVector(result_class)),
                }),
            }],
        }

        /* Example from rga3d

        impl WedgeDot<MultiVector> for MultiVector {
            type Output = MultiVector;

            fn wedge_dot(self, other: MultiVector) -> MultiVector {
                MultiVector {
                    groups: MultiVectorGroups {

                        g0:   Simd32x2::from(self.group0()[0]) * other.group0()
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

                        g1:   Simd32x4::from(self.group0()[0]) * other.group1()
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

                        g2:   Simd32x3::from(self.group0()[0]) * other.group2()
                            + Simd32x3::from(self.group0()[1]) * other.group3()
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

                        g3:   Simd32x3::from(self.group0()[0]) * other.group3()
                            + Simd32x3::from(self.group1()[0]) * Simd32x3::from([other.group4()[3], other.group1()[2], other.group1()[1]]) * Simd32x3::from([-1.0, -1.0, 1.0])
                            + Simd32x3::from(self.group1()[1]) * Simd32x3::from([other.group1()[2], other.group4()[3], other.group1()[0]]) * Simd32x3::from([1.0, -1.0, -1.0])
                            + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group1()[1], other.group1()[0], other.group4()[3]]) * Simd32x3::from([-1.0, 1.0, -1.0])
                            + Simd32x3::from(self.group3()[0]) * Simd32x3::from([other.group0()[0], other.group3()[2], other.group3()[1]]) * Simd32x3::from([1.0, 1.0, -1.0])
                            + Simd32x3::from(self.group3()[1]) * Simd32x3::from([other.group3()[2], other.group0()[0], other.group3()[0]]) * Simd32x3::from([-1.0, 1.0, 1.0])
                            + Simd32x3::from(self.group3()[2]) * Simd32x3::from([other.group3()[1], other.group3()[0], other.group0()[0]]) * Simd32x3::from([1.0, -1.0, 1.0])
                            - Simd32x3::from(self.group4()[3]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),

                        g4:   Simd32x4::from(self.group0()[0]) * other.group4()
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
            result: Parameter {
                name,
                data_type: geometric_product_result.data_type.clone(),
            },
            parameters: vec![parameter_a.clone()],
            body: vec![AstNode::ReturnStatement {
                expression: Box::new(Expression {
                    size: 1,
                    content: ExpressionContent::InvokeInstanceMethod(
                        parameter_a.data_type.clone(),
                        Box::new(Expression {
                            size: 1,
                            content: ExpressionContent::Variable(parameter_a.name),
                            data_type_hint: Some(parameter_a.data_type.clone()),
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
                                                    data_type_hint: Some(DataType::SimdVector(1)),
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
                                                                    data_type_hint: Some(parameter_a.data_type.clone()),
                                                                }),
                                                                weight_norm_result.name,
                                                                vec![],
                                                            ),
                                                            data_type_hint: Some(weight_norm_result.data_type.clone()),
                                                        }),
                                                        0,
                                                    ),
                                                    data_type_hint: None,
                                                }),
                                            ),
                                            data_type_hint: None,
                                        },
                                    )],
                                ),
                                data_type_hint: Some(parameter_b.data_type.clone()),
                            },
                        )],
                    ),
                    data_type_hint: Some(geometric_product_result.data_type.clone()),
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
                            data_type_hint: Some(parameter_a.data_type.clone()),
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
                                            data_type_hint: Some(parameter_b.data_type.clone()),
                                        },
                                    )],
                                ),
                                data_type_hint: Some(parameter_b.data_type.clone()),
                            },
                        )],
                    ),
                    data_type_hint: Some(geometric_product_result.data_type.clone()),
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
                                                                    data_type_hint: Some(parameter_a.data_type.clone()),
                                                                }),
                                                                magnitude_result.name,
                                                                vec![],
                                                            ),
                                                            data_type_hint: Some(magnitude_result.data_type.clone()),
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

    pub fn derive_grade<'a>(name: &'static str, parameter_a: &Parameter<'a>, grade: usize) -> AstNode<'a> {
        AstNode::TraitImplementation {
            result: Parameter {
                name,
                data_type: DataType::Integer,
            },
            parameters: vec![parameter_a.clone()],
            body: vec![AstNode::ReturnStatement {
                expression: Box::new(Expression {
                    size: 0,
                    data_type_hint: Some(DataType::Integer),
                    content: ExpressionContent::Constant(DataType::Integer, vec![grade as isize]),
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
                continue;
            }
        }
        result_signature.sort_unstable();
        if result_signature.is_empty() {
            return AstNode::None;
        }
        let mut param_a_signature = parameter_a.multi_vector_class().signature();
        param_a_signature.sort_unstable();
        if param_a_signature == result_signature {
            return single_expression_single_trait_impl(name, &parameter_a, variable(&parameter_a));
        }

        // Most objects have bulk and weight.
        // We'll try to find an exact match for the result class.
        // If there is no exact match, we'll try to find the closest match.
        // If nothing else, the starting class should always suffice.

        let mut result_class = registry.get(&result_signature);
        if result_class.is_none() && !result_signature.is_empty() {
            let mut viable_classes: Vec<_> = registry
                .classes
                .iter()
                .filter(|it| {
                    let sig = it.0.signature();
                    result_signature.iter().all(|it| sig.contains(it)) &&
                    // Bulk of Line could be represented as Translator with zero anti-scalar, but that is weird
                    sig.iter().all(|it| param_a_signature.contains(it))
                })
                .collect();
            viable_classes.sort_by_key(|it| it.0.signature().len());
            result_class = viable_classes.first().map(|it| &it.0);
        }
        let result_class = result_class.unwrap_or_else(|| parameter_a.multi_vector_class());

        let result_flat_basis = result_class.flat_basis();
        let mut base_index = 0;
        let mut body = Vec::new();
        for result_group in result_class.grouped_basis.iter() {
            let size = result_group.len();

            let mut a_group_index = None;
            let (factors, a_indices): (Vec<_>, Vec<_>) = (0..size)
                .map(|index_in_group| {
                    let result_element = &result_flat_basis[base_index + index_in_group];
                    let index_in_a = a_flat_basis.iter().position(|a_element| a_element == result_element).unwrap();
                    let result_element_is_projective = algebra.product(projective_basis, result_element).is_empty();
                    let scalar = if is_projective == result_element_is_projective { 1isize } else { 0isize };
                    let (group, element) = parameter_a.multi_vector_class().index_in_group(index_in_a);
                    let group_size = parameter_a.multi_vector_class().grouped_basis[group].len();
                    if a_group_index.is_none() {
                        a_group_index = Some(group);
                    }
                    let negate = false;
                    (
                        scalar,
                        GatherData::Usual(UsualGatherData {
                            negate,
                            group,
                            element,
                            group_size,
                        }),
                    )
                })
                .unzip();

            let a_group_index = a_group_index.unwrap();
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
            result: Parameter {
                name,
                data_type: DataType::MultiVector(result_class),
            },
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
    Expression {
        size: 1,
        content: ExpressionContent::Variable(param.name),
        data_type_hint: Some(param.data_type.clone()),
    }
}

pub fn single_expression_pair_trait_impl<'a>(
    name: &'static str,

    parameter_a: &Parameter<'a>,
    parameter_b: &Parameter<'a>,
    expression: Expression<'a>,
) -> AstNode<'a> {
    let data_type = match &expression.data_type_hint {
        Some(dt) => dt.clone(),
        _ => panic!("single_expression_pair_trait_impl for {name} requires data_type_hint on \"expression\" {expression:?}"),
    };
    AstNode::TraitImplementation {
        result: Parameter { name, data_type },
        parameters: vec![parameter_a.clone(), parameter_b.clone()],
        body: vec![AstNode::ReturnStatement {
            expression: Box::new(expression),
        }],
    }
}

pub fn single_expression_single_trait_impl<'a>(name: &'static str, parameter_a: &Parameter<'a>, expression: Expression<'a>) -> AstNode<'a> {
    let data_type = match &expression.data_type_hint {
        Some(dt) => dt.clone(),
        _ => panic!("single_expression_single_trait_impl for {name} requires data_type_hint on \"expression\" {expression:?}"),
    };
    AstNode::TraitImplementation {
        result: Parameter { name, data_type },
        parameters: vec![parameter_a.clone()],
        body: vec![AstNode::ReturnStatement {
            expression: Box::new(expression),
        }],
    }
}

pub fn single_expression_class_trait_impl<'a>(name: &'static str, mvc: &'a MultiVectorClass, expression: Expression<'a>) -> AstNode<'a> {
    AstNode::TraitImplementation {
        result: Parameter {
            name,
            data_type: DataType::MultiVector(mvc),
        },
        parameters: vec![],
        body: vec![AstNode::ReturnStatement {
            expression: Box::new(expression),
        }],
    }
}
