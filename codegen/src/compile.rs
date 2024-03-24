use crate::algebra::basis_element::{BasisElement, BasisElementIndex};
use crate::algebra::GeometricAlgebraTrait;
use crate::ast::{GatherData, UsualGatherData};
use crate::{
    algebra::{Involution, MultiVectorClass, MultiVectorClassRegistry, Product},
    ast::{AstNode, DataType, Expression, ExpressionContent, Parameter},
};
use std::collections::{BTreeMap, BTreeSet};
use crate::emit::Emitter;
use crate::impls::TraitImpls;

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
                let mut the_consts: Vec<_> = indices
                    .iter()
                    .map(|it| match it {
                        GatherData::RawZero => 0isize,
                        GatherData::Usual(_) => unreachable!(),
                    })
                    .collect();
                let size = the_consts.len();
                let first = the_consts.first();
                if let Some(first) = first {
                    if the_consts.iter().all(|it| *it == *first) {
                        the_consts = vec![*first];
                    }
                }
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

                let all_gathered_items_are_same = indices.iter().all(|it| if let GatherData::Usual(u) = it { u == first_group_data } else { false });
                let gathering_in_same_group = indices.iter().all(|it| if let GatherData::Usual(u) = it { u.group == first_group_data.group } else { false });
                let no_raw_consts = indices.iter().all(|it| if let GatherData::Usual(_) = it { true } else { false });
                let some_components_out_of_order = indices.iter().enumerate().any(|(i, it)| if let GatherData::Usual(u) = it { i != u.element } else { true });

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
                return b
            }
            if b.content == ExpressionContent::None {
                return a
            }
            if let ExpressionContent::Constant(_dt, coefficients) = &a.content {
                if coefficients.iter().all(|it| *it == 0) {
                    return b
                }
            }
            if let ExpressionContent::Constant(_dt, coefficients) = &b.content {
                if coefficients.iter().all(|it| *it == 0) {
                    return a
                }
            }
            let data_type_hint = if a.data_type_hint == b.data_type_hint { a.data_type_hint.clone() } else { None };
            return Box::new(Expression {
                size: expression.size,
                content: ExpressionContent::Add(a, b),
                data_type_hint,
            })
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
                return Box::new(Expression {
                    size: expression.size,
                    data_type_hint: b.data_type_hint.clone(),
                    content: ExpressionContent::Subtract(Box::new(constant), b),
                })
            }
            if b.content == ExpressionContent::None {
                return a
            }
            if let ExpressionContent::Constant(_dt, coefficients) = &b.content {
                if coefficients.iter().all(|it| *it == 0) {
                    return a
                }
            }

            return Box::new(Expression {
                size: expression.size,
                data_type_hint: a.data_type_hint.clone(),
                content: ExpressionContent::Subtract(a, b),
            })
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
            match &b.content {
                ExpressionContent::None => return a,
                ExpressionContent::Constant(_, c) if c.iter().all(|c| *c == 1) => return a,
                ExpressionContent::Constant(_, c) if c.iter().all(|c| *c == 0) => return b,
                _ => {}
            }

            match (&mut a.content, &mut b.content) {
                (ExpressionContent::Gather(_, gather_data), ExpressionContent::Constant(_, c)) if c.iter().all(|c| *c == 1 || *c == 0 || *c == -1) => {
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

            let data_type_hint = if a.data_type_hint == b.data_type_hint { a.data_type_hint.clone() } else { None };
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
                    result_group.iter().map(|element| if element.index == 0 { scalar_value } else { other_values }).collect(),
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
            class: self,
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
}

pub fn derive_involution<'a>(name: &'static str, involution: &Involution, parameter_a: &Parameter<'a>, registry: &'a MultiVectorClassRegistry, project: bool) -> AstNode<'a> {
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
    let mut result_candidates: Vec<_> = registry.classes.iter().filter_map(|it| {
        let rc_fb: Vec<_> = it.0.flat_basis().iter().map(|it| it.index).collect();
        if result_signature.iter().all(|it| rc_fb.contains(&it)) { Some(&it.0) } else { None }
    }).collect();
    result_candidates.sort_by(|a, b| a.flat_basis().len().cmp(&b.flat_basis().len()));

    let result_class = match result_candidates.first() {
        None => return AstNode::None,
        Some(rc) => *rc,
    };
    let result_flat_basis = result_class.flat_basis();
    let mut body = Vec::new();
    let mut base_index = 0;
    for result_group in result_class.grouped_basis.iter() {
        let size = result_group.len();
        let mut a_group_index = None;
        let mut factors = vec![];
        let mut a_indices = vec![];
        'for_index_in_group: for index_in_group in 0..size {
            let result_element = &result_flat_basis[base_index + index_in_group];
            let (in_element, out_element) = involution.terms.iter().find(|(_in, out)| out.index == result_element.index).unwrap();

            let index_in_a = a_flat_basis.iter().position(|a_element| a_element.index == in_element.index);
            let index_in_a = match index_in_a {
                None => {
                    factors.push(0);
                    a_indices.push(GatherData::RawZero);
                    continue 'for_index_in_group;
                },
                Some(index_in_a) => index_in_a,
            };
            let coefficient = out_element.coefficient * result_element.coefficient * in_element.coefficient * a_flat_basis[index_in_a].coefficient;
            let (group, element) = parameter_a.multi_vector_class().index_in_group(index_in_a);
            let group_size = parameter_a.multi_vector_class().grouped_basis[group].len();
            if a_group_index.is_none() {
                a_group_index = Some(group);
            }
            let negate = false;
            factors.push(coefficient);
            a_indices.push(GatherData::Usual(UsualGatherData {
                negate,
                group,
                element,
                group_size,
            }));
        }
        let expression = Expression {
            size,
            content: ExpressionContent::Multiply(
                Box::new(Expression {
                    size,
                    content: ExpressionContent::Gather(
                        Box::new(Expression {
                            size,
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
        class: parameter_a.multi_vector_class(),
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

pub fn element_wise<'a>(name: &'static str, parameter_a: &Parameter<'a>, parameter_b: &Parameter<'a>, registry: &'a MultiVectorClassRegistry) -> AstNode<'a> {
    let a_flat_basis = parameter_a.multi_vector_class().flat_basis();
    let b_flat_basis = parameter_b.multi_vector_class().flat_basis();
    let result_signature = a_flat_basis.iter().chain(b_flat_basis.iter()).cloned().collect::<std::collections::HashSet<_>>();
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
                            content: ExpressionContent::Constant(DataType::SimdVector(size), terms.iter().map(|(factor, _index_pair)| *factor).collect::<Vec<_>>()),
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
            class: parameter_a.multi_vector_class(),
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

pub fn derive_product<'a>(name: &'static str, product: &Product, parameter_a: &Parameter<'a>, parameter_b: &Parameter<'a>, registry: &'a MultiVectorClassRegistry) -> AstNode<'a> {

    let a_flat_basis = parameter_a.multi_vector_class().flat_basis();
    let b_flat_basis = parameter_b.multi_vector_class().flat_basis();
    let mut result_signature = BTreeSet::new();
    for product_term in product.terms.iter() {
        if a_flat_basis.iter().any(|e| e.index == product_term.factor_a.index) && b_flat_basis.iter().any(|e| e.index == product_term.factor_b.index) {
            for pt in product_term.product.iter() {
                if pt.coefficient != 0 {
                    result_signature.insert(pt.index);
                }
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
        Some(rc) => rc,
        None => return AstNode::None,
    };

    let result_flat_basis = result_class.flat_basis();
    let mut terms_in_result: BTreeMap<usize, Vec<(isize, usize, usize)>> = BTreeMap::new();
    let stuff = product.terms.iter().flat_map(|it| it.product.iter().map(|p| (it.factor_a.clone(), it.factor_b.clone(), p.clone())));
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
            let ((contract_a_group, mut contract_a), [mut contract_terms_0, mut contract_terms_1, mut contract_terms_2, mut contract_terms_3]) = latest_entry.take().unwrap();

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
                latest_entry = Some(((contract_a_group, contract_a), [contract_terms_0, contract_terms_1, contract_terms_2, contract_terms_3]));
            } else {
                new_terms_by_a.insert((contract_a_group, contract_a), [contract_terms_0, contract_terms_1, contract_terms_2, contract_terms_3]);
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
        class: parameter_a.multi_vector_class(),
        parameters: vec![parameter_a.clone(), parameter_b.clone()],
        body: vec![AstNode::ReturnStatement {
            expression: Box::new(Expression {
                size: 1,
                content: ExpressionContent::InvokeClassMethod(result_class, "Constructor", body),
                data_type_hint: Some(DataType::MultiVector(result_class)),
            }),
        }],
    }
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
        class: parameter_a.multi_vector_class(),
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

pub fn derive_scale<'a>(name: &'static str, geometric_product: &AstNode<'a>, parameter_a: &Parameter<'a>, parameter_b: &Parameter<'a>) -> AstNode<'a> {
    let geometric_product_result = result_of_trait!(geometric_product);
    AstNode::TraitImplementation {
        result: Parameter {
            name,
            data_type: geometric_product_result.data_type.clone(),
        },
        class: parameter_a.multi_vector_class(),
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

pub fn derive_signum<'a>(name: &'static str, geometric_product: &AstNode<'a>, magnitude: &AstNode<'a>, parameter_a: &Parameter<'a>) -> AstNode<'a> {
    let geometric_product_result = result_of_trait!(geometric_product);
    let magnitude_result = result_of_trait!(magnitude);
    AstNode::TraitImplementation {
        result: Parameter {
            name,
            data_type: geometric_product_result.data_type.clone(),
        },
        class: parameter_a.multi_vector_class(),
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
        class: parameter_a.multi_vector_class(),
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
        class: parameter_a.multi_vector_class(),
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
        class: parameter_a.multi_vector_class(),
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
        class: parameter_a.multi_vector_class(),
        parameters: vec![parameter_a.clone(), parameter_b.clone()],
        body: vec![AstNode::ReturnStatement {
            expression: if conversion.is_some() {
                Box::new(Expression {
                    size: 1,
                    data_type_hint: Some(conversion_result.data_type.clone()),
                    content: ExpressionContent::Conversion(geometric_product_2_result.multi_vector_class(), conversion_result.multi_vector_class(), product),
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
        class: parameter_a.multi_vector_class(),
        parameters: vec![],
        body: vec![AstNode::ReturnStatement {
            expression: Box::new(Expression {
                size: 0,
                data_type_hint: Some(DataType::Integer),
                content: ExpressionContent::Constant(DataType::Integer, vec![grade as isize]),
            }),
        }],
    }
}

pub fn derive_bulk_or_weight<'a>(
    name: &'static str,
    parameter_a: &Parameter<'a>,
    projective_basis: &BasisElement,
    is_projective: bool,
    flat_basis: Option<BasisElement>,
    is_flat: bool,
    registry: &'a MultiVectorClassRegistry,
) -> AstNode<'a> {
    let mut result_signature = Vec::new();
    for a_element in parameter_a.multi_vector_class().flat_basis().iter() {
        let basis_is_projective = projective_basis.index == (a_element.index & projective_basis.index);
        let basis_is_flat = if let Some(flat_basis) = &flat_basis {
            flat_basis.index == (a_element.index & flat_basis.index)
        } else { true };

        if is_projective == basis_is_projective && is_flat == basis_is_flat {
            result_signature.push(a_element.index);
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
                let index_in_a = parameter_a.multi_vector_class().flat_basis().iter().position(|a_element| a_element == result_element).unwrap();
                let scalar = if result_signature.contains(&result_element.index) { 1isize } else { 0isize };
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
        let mut expression = Expression {
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
        };
        if factors.iter().any(|it| *it != 1) {
            expression = Expression {
                size,
                data_type_hint: None,
                content: ExpressionContent::Multiply(
                    Box::new(expression),
                    Box::new(Expression {
                        size,
                        data_type_hint: Some(DataType::SimdVector(size)),
                        content: ExpressionContent::Constant(DataType::SimdVector(size), factors),
                    }),
                ),
            };
        }
        let mut expression = *simplify_and_legalize(Box::new(expression));
        if expression.content == ExpressionContent::None {
            expression = Expression {
                size,
                data_type_hint: Some(DataType::SimdVector(size)),
                content: ExpressionContent::Constant(DataType::SimdVector(size), vec![0; size]),
            };
        }
        body.push((DataType::SimdVector(size), expression));
        base_index += size;
    }
    AstNode::TraitImplementation {
        result: Parameter {
            name,
            data_type: DataType::MultiVector(result_class),
        },
        class: parameter_a.multi_vector_class(),
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

pub fn variable<'a>(param: &Parameter<'a>) -> Expression<'a> {
    Expression {
        size: 1,
        content: ExpressionContent::Variable(param.name),
        data_type_hint: Some(param.data_type.clone()),
    }
}

pub fn single_expression_pair_trait_impl<'a>(name: &'static str, parameter_a: &Parameter<'a>, parameter_b: &Parameter<'a>, expression: Expression<'a>) -> AstNode<'a> {
    let data_type = match &expression.data_type_hint {
        Some(dt) => dt.clone(),
        _ => panic!("single_expression_pair_trait_impl for {name} requires data_type_hint on \"expression\" {expression:?}"),
    };
    AstNode::TraitImplementation {
        result: Parameter { name, data_type },
        class: parameter_a.multi_vector_class(),
        parameters: vec![parameter_a.clone(), parameter_b.clone()],
        body: vec![AstNode::ReturnStatement { expression: Box::new(expression) }],
    }
}

pub fn single_expression_single_trait_impl<'a>(name: &'static str, parameter_a: &Parameter<'a>, expression: Expression<'a>) -> AstNode<'a> {
    let data_type = match &expression.data_type_hint {
        Some(dt) => dt.clone(),
        _ => panic!("single_expression_single_trait_impl for {name} requires data_type_hint on \"expression\" {expression:?}"),
    };
    AstNode::TraitImplementation {
        result: Parameter { name, data_type },
        class: parameter_a.multi_vector_class(),
        parameters: vec![parameter_a.clone()],
        body: vec![AstNode::ReturnStatement { expression: Box::new(expression) }],
    }
}

pub fn single_expression_class_trait_impl<'a>(name: &'static str, mvc: &'a MultiVectorClass, expression: Expression<'a>) -> AstNode<'a> {
    AstNode::TraitImplementation {
        result: Parameter {
            name,
            data_type: DataType::MultiVector(mvc),
        },
        class: mvc,
        parameters: vec![],
        body: vec![AstNode::ReturnStatement { expression: Box::new(expression) }],
    }
}

pub struct CodeGenerator<'r, GA> {
    pub algebra: GA,
    pub trait_impls: TraitImpls<'r>,
}

impl<'r, GA: GeometricAlgebraTrait> CodeGenerator<'r, GA> {
    pub fn new(algebra: GA) -> Self {
        CodeGenerator {
            algebra,
            trait_impls: TraitImpls::new(),
        }
    }

    /// Step 1: These items are somewhat universal across geometric algebras
    pub fn preamble_and_universal_traits<'s>(&'s mut self, registry: &'r MultiVectorClassRegistry) -> std::io::Result<()> {
        // Constants
        for param_a in registry.single_parameters() {
            let class_a = param_a.multi_vector_class();
            for name in &["Zero", "One"] {
                let ast_node = class_a.constant(name);
                if ast_node != AstNode::None {
                    self.trait_impls.add_class_impl(name, param_a.multi_vector_class(), ast_node);
                }
            }
        }

        // Uniform Grades
        for param_a in registry.single_parameters() {
            let class_a = param_a.multi_vector_class();

            let grade_unanimity = class_a
                .flat_basis()
                .iter()
                .map(|a| (a.grade(), true))
                .reduce(|(a_grade, unanimous), (b_grade, _)| (a_grade, a_grade == b_grade && unanimous));

            if let Some((grade, true)) = grade_unanimity {
                let anti_grade = (self.algebra.anti_scalar_element().grade() as isize - grade as isize).unsigned_abs();
                let grade_impl = derive_grade("Grade", &param_a, grade);
                self.trait_impls.add_class_impl("Grade", param_a.multi_vector_class(), grade_impl);

                let anti_grade_impl = derive_grade("AntiGrade", &param_a, anti_grade);
                self.trait_impls.add_class_impl("AntiGrade", param_a.multi_vector_class(), anti_grade_impl);
            }
        }

        // Involutions
        let involutions = Involution::involutions(&self.algebra);
        for param_a in registry.single_parameters() {
            for (name, involution, _) in involutions.iter() {
                let ast_node = derive_involution(name, involution, &param_a, registry, false);
                if ast_node != AstNode::None {
                    self.trait_impls.add_single_impl(name, param_a.clone(), ast_node);
                }
            }
        }

        // Square Root
        // Why not impl Sqrt for Magnitude? Well...
        //  - the choices of AstNode don't make it immediately easy. I mean it's possible, just annoying.
        //  - The question of "how" to implement Sqrt begs for other issues... Initial idea was to sqrt the scalar
        //    component and leave the antiscalar component. However this defies the symmetry of scalars and antiscalars.
        //    Additionally, you can look at the impl of Add for Magnitude and realize, that is not accurate (or biased,
        //    rather) to Scalars either. Magnitude + Magnitude is not the same result as Scalar + Scalar. This might
        //    seem strange or incorrect at first glance, but again.... symmetry with antiscalars implies it shouldn't
        //    be biased for scalars... and if we can component-wise "Add" Vectors and other MultiVectorClasses, then
        //    why not AntiScalars and Magnitudes? So the "impl Add for Magnitude" could very well be correct in its
        //    own special way, but in any case, this doesn't reeeeeaaallly make me comfortable enough to do a
        //    component-wise Square Root, because we don't see component-wise Square Roots in other multi component things.
        let scalar_like = vec!["Scalar", "AntiScalar"];
        for param_a in registry.single_parameters() {
            let class_name = param_a.data_type.data_class_name();
            if !scalar_like.contains(&class_name.as_str()) {
                continue;
            }
            let access = Expression {
                size: 1,
                data_type_hint: None,
                content: ExpressionContent::Access(Box::new(variable(&param_a)), 0),
            };
            let sqrt = Expression {
                size: 1,
                data_type_hint: None,
                content: ExpressionContent::SquareRoot(Box::new(access)),
            };
            let construct = Expression {
                size: 1,
                data_type_hint: Some(param_a.data_type.clone()),
                content: ExpressionContent::InvokeClassMethod(param_a.multi_vector_class(), "Constructor", vec![(DataType::SimdVector(1), sqrt)]),
            };
            let name = "Sqrt";
            let sqrt = single_expression_single_trait_impl(name, &param_a, construct);
            self.trait_impls.add_single_impl(name, param_a, sqrt);
        }

        // Into
        for (param_a, param_b) in registry.pair_parameters() {
            let class_a = param_a.multi_vector_class();
            let class_b = param_b.multi_vector_class();
            if class_a != class_b {
                let ast_node = derive_involution("Into", &Involution::projection(param_b.multi_vector_class()), &param_a, registry, true);
                if ast_node != AstNode::None {
                    self.trait_impls.add_pair_impl("Into", param_a.clone(), param_b.clone(), ast_node);
                }
            }
        }

        // Add, Subtract
        for (param_a, param_b) in registry.pair_parameters() {
            for name in &["Add", "Sub"] {
                let ast_node = element_wise(*name, &param_a, &param_b, registry);
                if ast_node != AstNode::None {
                    self.trait_impls.add_pair_impl(name, param_a.clone(), param_b.clone(), ast_node);
                }
            }
        }

        // Multiply, Divide
        for (param_a, param_b) in registry.pair_parameters() {
            if param_a.multi_vector_class() != param_b.multi_vector_class() {
                continue;
            }
            for name in &["Mul", "Div"] {
                let ast_node = element_wise(*name, &param_a, &param_b, registry);
                if ast_node != AstNode::None {
                    self.trait_impls.add_pair_impl(name, param_a.clone(), param_b.clone(), ast_node);
                }
            }
        }

        // Products from Geometric Algebra
        let products = Product::products(&self.algebra);
        for (param_a, param_b) in registry.pair_parameters() {
            for (name, product, _) in products.iter() {
                let ast_node = derive_product(name, product, &param_a, &param_b, registry);
                if ast_node != AstNode::None {
                    self.trait_impls.add_pair_impl(name, param_a.clone(), param_b.clone(), ast_node);
                }
            }
        }

        Ok(())
    }

    /// Step 2: Create some basic norms
    pub fn basic_norms<'s>(&'s mut self, registry: &'r MultiVectorClassRegistry) {
        for param_a in registry.single_parameters() {
            let name = "BulkNormSquared";
            let _: Option<()> = try {
                let dot = self.algebra.dialect().dot_product.first()?;
                let dot = self.trait_impls.get_pair_invocation(dot, variable(&param_a), variable(&param_a))?;
                let bulk_norm = single_expression_single_trait_impl(name, &param_a, dot);
                self.trait_impls.add_single_impl(name, param_a.clone(), bulk_norm);
            };
        }

        for param_a in registry.single_parameters() {
            let name = "WeightNormSquared";
            let _: Option<()> = try {
                let dot = self.algebra.dialect().anti_dot_product.first()?;
                let dot = self.trait_impls.get_pair_invocation(dot, variable(&param_a), variable(&param_a))?;
                let bulk_norm = single_expression_single_trait_impl(name, &param_a, dot);
                self.trait_impls.add_single_impl(name, param_a.clone(), bulk_norm);
            };
        }

        for param_a in registry.single_parameters() {
            let name = "BulkNorm";
            let _: Option<()> = try {
                let dot = self.algebra.dialect().dot_product.first()?;
                let dot = self.trait_impls.get_pair_invocation(dot, variable(&param_a), variable(&param_a))?;
                let m = self.trait_impls.get_single_invocation("Sqrt", dot)?;
                let bulk_norm = single_expression_single_trait_impl(name, &param_a, m);
                self.trait_impls.add_single_impl(name, param_a.clone(), bulk_norm);
            };
        }

        for param_a in registry.single_parameters() {
            let name = "WeightNorm";
            let _: Option<()> = try {
                let dot = self.algebra.dialect().anti_dot_product.first()?;
                let dot = self.trait_impls.get_pair_invocation(dot, variable(&param_a), variable(&param_a))?;
                let m = self.trait_impls.get_single_invocation("Sqrt", dot)?;
                let bulk_norm = single_expression_single_trait_impl(name, &param_a, m);
                self.trait_impls.add_single_impl(name, param_a.clone(), bulk_norm);
            };
        }

        for param_a in registry.single_parameters() {
            let name = "GeometricNorm";
            let _: Option<()> = try {
                let bn = self.trait_impls.get_single_invocation("BulkNorm", variable(&param_a))?;
                let wn = self.trait_impls.get_single_invocation("WeightNorm", variable(&param_a))?;
                let add = self.trait_impls.get_pair_invocation("Add", bn, wn)?;
                let gn = single_expression_single_trait_impl(name, &param_a, add);
                self.trait_impls.add_single_impl(name, param_a.clone(), gn);
            };
        }

        for param_a in registry.single_parameters() {
            let name = "UnitizedNormSquared";
            let _: Option<()> = try {
                let bn = self.trait_impls.get_single_invocation("BulkNormSquared", variable(&param_a))?;
                let wn = self.trait_impls.get_single_invocation("WeightNormSquared", variable(&param_a))?;
                let access_bn = Expression {
                    size: 1,
                    data_type_hint: Some(DataType::SimdVector(1)),
                    content: ExpressionContent::Access(Box::new(bn), 0),
                };
                let access_wn = Expression {
                    size: 1,
                    data_type_hint: Some(DataType::SimdVector(1)),
                    content: ExpressionContent::Access(Box::new(wn), 0),
                };
                let div = Expression {
                    size: 1,
                    data_type_hint: Some(DataType::SimdVector(1)),
                    content: ExpressionContent::Divide(Box::new(access_bn), Box::new(access_wn)),
                };
                let uns = single_expression_single_trait_impl(name, &param_a, div);
                self.trait_impls.add_single_impl(name, param_a.clone(), uns);
            };
        }

        for param_a in registry.single_parameters() {
            let name = "UnitizedNorm";
            let _: Option<()> = try {
                let uns = self.trait_impls.get_single_invocation("UnitizedNormSquared", variable(&param_a))?;
                let sqrt = Expression {
                    size: 1,
                    data_type_hint: Some(DataType::SimdVector(1)),
                    content: ExpressionContent::SquareRoot(Box::new(uns)),
                };
                let un = single_expression_single_trait_impl(name, &param_a, sqrt);
                self.trait_impls.add_single_impl(name, param_a.clone(), un);
            };
        }
    }

    /// Step 3: Create some fancy norms
    /// These items require some special insight per Geometric Algebra, for example if there are
    /// multiple/special projective dimensions with different meanings.
    pub fn fancy_norms(&mut self, _prefix: &'static str, _projective_basis: BasisElement, _registry: &'r MultiVectorClassRegistry) {

        // It is not elaborated very much, but one can infer/detect a few things from the CGA poster.
        // Flat objects have a Position Norm, but no Center Norm or Radius Norm.
        // Round objects have a Center Norm and Radius Norm, but no Position Norm.
        // Or at least that is the implication.
        // But closer inspection reveals what is clearly obvious, round objects also have a position norm.
        // The thing is, if you're familiar with the way flat objects are really round objects
        // with an infinite radius, then you can see how flat and round objects have the
        // same flat bulks. That is, they have the same definition for the position relative to the
        // origin. So a "Position Norm" is about the position with respect to the origin.
        // And a "Center Norm" is the center of the round object, which is not (necessarily) the
        // same as the component of the object closest to the origin. And same for radius.

        // And to elaborate further, and highlight the notation...
        // rga3d poster: https://projectivegeometricalgebra.org/projgeomalg.pdf
        // cga3d poster: https://projectivegeometricalgebra.org/confgeomalg.pdf
        // In the "Norms" section of rga3d you can see the double bars denoting absolute value,
        // one for bulk, one for norm, the "geometric" combination of both, and then
        // the little hat denoting unitization by projecting onto the plane where e4 = 1.
        // So the norms in the cga poster are apparently/obviously doing the same thing, but
        // with less elaboration. It is clear that the formulas are listing the unitized/projected
        // formulations, but you could decompose it to a bulk aspect, weight aspect, and/or geometric
        // combination if you wanted.

        // Hypothesis for how to implement...

        // Center Norm:
        // - unitize the object by its round weight
        // - take the center of the object (RoundPoint)
        // - split into round bulk and round weight (omits e5 element)
        // - take the square root of the (Anti)Dot of the Bulk or Weight for each part of the norm
        //
        // This looks like it would work for RoundPoint and Sphere,
        // but less confident in Dipole or Circle.

        // Radius Norm:
        // - there's an uncanny term... it seems to be from the "non-e4" element of the Container
        // - hmm yeah the Container seems promising... the e1234 and e3215 elements...
        // - hmmmmm

        // I'm probably overthinking this.
        // The RoundPoint and Sphere have construction formulas that include the radius.
        // And heck, so does the Dipole and Circle.
        // Obviously the radius norm is going to have to reverse the engineer that construction
        // to acquire the radius. Yesssss..... seems legit for RoundPoint....
        // But like.... really fricken complicated for Dipole/Circle....
        // ...unless I convert to Center anyway...


        todo!()
    }

    /// Step 4: Create some more stuff that depends on norms
    pub fn post_norm_universal_stuff<'s>(&'s mut self, registry: &'r MultiVectorClassRegistry) {
        // TODO tentatively not excluding Scale. Need to decide to keep after all, or fully delete
        // Scale
        // for (param_a, param_b) in registry.pair_parameters() {
        //     let name = "Scale";
        //     if param_b.multi_vector_class().grouped_basis != vec![vec![self.algebra.scalar_element()]] {
        //         continue
        //     }
        //     let _: Option<()> = try {
        //         let gp = self.algebra.dialect().geometric_product.first()?;
        //
        //         // These commented lines will implement "Scale" using Scalar
        //         // let gp = self.trait_impls.get_pair_invocation(gp, variable(&param_a), variable(&param_b))?;
        //         // let scale = single_expression_pair_trait_impl(name, &param_a, &param_b, gp);
        //
        //         // The following implementation will implement "Scale" using SimdVector(1) instead of a Scalar
        //         let gp = self.trait_impls.get_pair_impl(gp, &param_a, &param_b)?;
        //         let scale = derive_scale(name, gp, &param_a, &param_b);
        //
        //         emitter.emit(&scale).unwrap();
        //         self.trait_impls.add_pair_impl(name, param_a, param_b, scale);
        //     };
        // }

        // TODO not sure the use of Signum yet. Either keep, or fully delete
        // Signum
        // for (param_a, param_b) in registry.pair_parameters() {
        //     let name = "Signum";
        //     if param_b.multi_vector_class().grouped_basis != vec![vec![self.algebra.scalar_element()]] {
        //         continue
        //     }
        //     let _: Option<()> = try {
        //         let gp = self.algebra.dialect().geometric_product.first()?;
        //         let gp = self.trait_impls.get_pair_impl(gp, &param_a, &param_b)?;
        //         let magnitude = self.trait_impls.get_single_impl("Magnitude", &param_a)?;
        //         let signum = derive_signum("Signum", gp, magnitude, &param_a);
        //
        //         emitter.emit(&signum).unwrap();
        //         self.trait_impls.add_pair_impl(name, param_a, param_b, signum);
        //     };
        // }

        // TODO not sure the use of Inverse yet. Either keep, or fully delete
        // Inverse
        // for (param_a, param_b) in registry.pair_parameters() {
        //     if param_b.multi_vector_class().grouped_basis != vec![vec![self.algebra.scalar_element()]] {
        //         continue;
        //     }
        //     let _: Option<()> = try {
        //         let gp = self.algebra.dialect().geometric_product.first()?;
        //         let gp = self.trait_impls.get_pair_impl(gp, &param_a, &param_b)?;
        //         let squared_magnitude = self.trait_impls.get_single_impl("SquaredMagnitude", &param_a)?;
        //         let reversal = self.trait_impls.get_single_impl("Reversal", &param_a)?;
        //         let inverse = derive_inverse("Inverse", gp, squared_magnitude, reversal, &param_a);
        //         emitter.emit(&inverse).unwrap();
        //         self.trait_impls.add_single_impl("Inverse", param_a.clone(), inverse);
        //     };
        // }

        // Unitize
        for (param_a, param_b) in registry.pair_parameters() {
            if param_b.multi_vector_class().grouped_basis != vec![vec![self.algebra.scalar_element()]] {
                continue;
            }
            let name = "Unitize";
            let _: Option<()> = try {
                let gp = self.algebra.dialect().geometric_product.first()?;
                let gp = self.trait_impls.get_pair_impl(gp, &param_a, &param_b)?;
                let weight_norm = self.trait_impls.get_single_impl("WeightNorm", &param_a)?;
                let unitize = derive_unitize(name, gp, weight_norm, &param_a, &param_b);
                self.trait_impls.add_single_impl(name, param_a.clone(), unitize);
            };
        }

        // TODO Powi is a whole situation, figure it out
        // Powi
        // for param_a in registry.single_parameters() {
        //     let name = "Powi";
        //     let _: Option<()> = try {
        //         let gp = self.algebra.dialect().geometric_product.first()?;
        //         let (gp, gp_r) = self.trait_impls.get_pair_impl_and_result(gp, &param_a, &param_a)?;
        //         if gp_r.multi_vector_class() != param_a.multi_vector_class() {
        //             continue
        //         }
        //         let constant_one = self.trait_impls.get_class_impl("One", param_a.multi_vector_class())?;
        //         // TODo requiring inverse is causing us to get no Powi for Scalar
        //         let inverse = self.trait_impls.get_single_impl("Inverse", &param_a)?;
        //         let exponent = Parameter {
        //             name: "exponent",
        //             data_type: DataType::Integer,
        //         };
        //         let power_of_integer = derive_power_of_integer(
        //             name, gp, constant_one, inverse, &param_a, &exponent,
        //         );
        //         emitter.emit(&power_of_integer).unwrap();
        //         self.trait_impls.add_pair_impl(name, param_a, exponent, power_of_integer);
        //     };
        // }

        // TODO figure out the situation with GeometricQuotient
        // for (param_a, param_b) in registry.pair_parameters() {
        //     let name = "GeometricQuotient";
        //     let _: Option<()> = try {
        //         let gp = self.algebra.dialect().geometric_product.first()?;
        //         let gp = self.trait_impls.get_pair_impl(gp, &param_a, &param_b)?;
        //         let inverse = self.trait_impls.get_single_impl("Inverse", &param_b)?;
        //         let division = derive_division(name, gp, inverse, &param_a, &param_b);
        //         emitter.emit(&division).unwrap();
        //         self.trait_impls.add_pair_impl(name, param_a, param_b, division);
        //     };
        // }

        // for (param_a, param_b) in registry.pair_parameters() {
        //     let _: Option<()> = try {
        //         let gp_name = self.algebra.dialect().geometric_product.first()?;
        //         let (gp, gp_r) = self.trait_impls.get_pair_impl_and_result(gp_name, &param_a, &param_b)?;
        //         let reversal = self.trait_impls.get_single_impl("Reversal", &param_a)?;
        //         let (gp2, gp2_r) = self.trait_impls.get_pair_impl_and_result(gp_name, &gp_r, &param_a)?;
        //         let into = self.trait_impls.get_pair_impl("Into", &gp2_r, &param_b);
        //         let transformation = derive_sandwich_product(
        //             "Transformation", gp, gp2, reversal, into, &param_a, &param_b
        //         );
        //         emitter.emit(&transformation).unwrap();
        //         self.trait_impls.add_pair_impl("Transformation", param_a, param_b, transformation);
        //     };
        // }

        // let allowed_to_sandwich = ["Motor", "Translator", "Rotor", "Flector", "Point", "Plane"];
        let disallowed_to_be_sandwiched = ["Scalar", "AntiScalar", "Magnitude"];
        for (param_a, param_b) in registry.pair_parameters() {
            // if !allowed_to_sandwich.contains(&param_a.multi_vector_class().class_name.as_str()) {
            //     continue
            // }
            if disallowed_to_be_sandwiched.contains(&param_b.multi_vector_class().class_name.as_str()) {
                continue;
            }
            let _: Option<()> = try {
                let gap = self.algebra.dialect().geometric_anti_product.first()?;
                let (gp, gp_r) = self.trait_impls.get_pair_impl_and_result(gap, &param_a, &param_b)?;
                let (reversal, reversal_r) = self.trait_impls.get_single_impl_and_result("AntiReversal", &param_a)?;
                let (gp2, gp2_r) = self.trait_impls.get_pair_impl_and_result(gap, &gp_r, &reversal_r)?;

                let result_class = registry.get_preferring_superclass(param_b.multi_vector_class().signature().as_slice()).unwrap();
                let result_param = Parameter {
                    name: "other",
                    data_type: DataType::MultiVector(result_class),
                };
                let into = self.trait_impls.get_pair_impl("Into", &gp2_r, &result_param);

                let sandwich = derive_sandwich_product("Sandwich", gp, gp2, reversal, into, &param_a, &param_b);
                self.trait_impls.add_pair_impl("Sandwich", param_a, param_b, sandwich);
            };
        }

        for (param_a, param_b) in registry.pair_parameters() {
            // Invert (Inversion)
            // https://rigidgeometricalgebra.org/wiki/index.php?title=Inversion
            // The choice of what class should constitute a "Point" is somewhat contrived.
            // It will need extra consideration for CGA.
            if param_a.multi_vector_class().class_name != "Point" {
                continue;
            }
            let name = "Invert";
            let _: Option<()> = try {
                let unitize = self.trait_impls.get_single_invocation("Unitize", variable(&param_a))?;
                let sandwich = self.trait_impls.get_pair_invocation("Sandwich", unitize, variable(&param_b))?;
                let i = single_expression_pair_trait_impl(name, &param_a, &param_b, sandwich);
                self.trait_impls.add_pair_impl(name, param_a, param_b, i);
            };
        }

        for (param_a, param_b) in registry.pair_parameters() {
            // Reflect (Reflection)
            // https://rigidgeometricalgebra.org/wiki/index.php?title=Reflection
            // The choice of what class should constitute a "Plane" is somewhat contrived.
            // It will need extra consideration for CGA.
            if param_a.multi_vector_class().class_name != "Plane" {
                continue;
            }
            let name = "Reflect";
            let _: Option<()> = try {
                let unitize = self.trait_impls.get_single_invocation("Unitize", variable(&param_a))?;
                let sandwich = self.trait_impls.get_pair_invocation("Sandwich", unitize, variable(&param_b))?;
                let i = single_expression_pair_trait_impl(name, &param_a, &param_b, sandwich);
                self.trait_impls.add_pair_impl(name, param_a, param_b, i);
            };
        }

        // TODO find a way to generalize these hard coded basis elements
        let projective_basis = if self.algebra.algebra_name() == "rga3d" {
            Some(self.algebra.parse("e4"))
        } else if self.algebra.algebra_name() == "cga3d" {
            Some(self.algebra.parse("e4"))
        } else {
            None
        };
        let flat_basis = if self.algebra.algebra_name() == "cga3d" {
            Some(self.algebra.parse("e5"))
        } else {
            None
        };
        for param_a in registry.single_parameters() {
            let projective_basis = match projective_basis.clone() {
                None => continue,
                Some(pb) => pb,
            };

            let bulk = derive_bulk_or_weight("Bulk", &param_a, &projective_basis, false, flat_basis.clone(), true, registry);
            if bulk != AstNode::None {
                self.trait_impls.add_single_impl("Bulk", param_a.clone(), bulk);
            }

            let weight = derive_bulk_or_weight("Weight", &param_a, &projective_basis, true, flat_basis.clone(), true, registry);
            if weight != AstNode::None {
                self.trait_impls.add_single_impl("Weight", param_a.clone(), weight);
            }

            if self.algebra.algebra_name().contains("cga") {

                let round_bulk = derive_bulk_or_weight("RoundBulk", &param_a, &projective_basis, false, flat_basis.clone(), false, registry);
                if round_bulk != AstNode::None {
                    self.trait_impls.add_single_impl("RoundBulk", param_a.clone(), round_bulk);
                }

                let round_weight = derive_bulk_or_weight("RoundWeight", &param_a, &projective_basis, true, flat_basis.clone(), false, registry);
                if round_weight != AstNode::None {
                    self.trait_impls.add_single_impl("RoundWeight", param_a, round_weight);
                }
            }
        }

        let aspect_duals = [
            ("RightBulkDual", "Bulk", "RightComplement"),
            ("RightWeightDual", "Weight", "RightComplement"),
            ("LeftBulkDual", "Bulk", "LeftComplement"),
            ("LeftWeightDual", "Weight", "LeftComplement"),
            ("RightRoundBulkDual", "RoundBulk", "RightComplement"),
            ("RightRoundWeightDual", "RoundWeight", "RightComplement"),
            ("LeftRoundBulkDual", "RoundBulk", "LeftComplement"),
            ("LeftRoundWeightDual", "RoundWeight", "LeftComplement"),
        ];
        for (name, bulkOrWeight, complement) in aspect_duals.into_iter() {
            for param_a in registry.single_parameters() {
                let _: Option<()> = try {
                    let aspect = self.trait_impls.get_single_invocation(bulkOrWeight, variable(&param_a))?;
                    let comp = self.trait_impls.get_single_invocation(complement, aspect)?;
                    let the_impl = single_expression_single_trait_impl(name, &param_a, comp);
                    self.trait_impls.add_single_impl(name, param_a, the_impl);
                };
            }
        }

        // We can end up with some very strange expansions, contractions, and projections
        // if we don't manually exclude these at some point.
        let non_objects = ["Scalar", "AntiScalar", "Magnitude"];

        // In the future it might also not be a bad idea to restrict to objects of uniform grade.
        // However I'm not overly worried about that yet. Projecting to and from Flectors and Motors
        // may be weird at first glance, but maybe projecting to and from MultiVector isn't, and
        // so who knows.

        // I mean, look at these examples:
        // rotor.anti_project_orthogonally_onto(point) = motor
        // rotor.anti_project_orthogonally_onto(origin) = rotor
        // And rotors are not uniform grade. So that looks worth keeping to me.

        // Uniform grades aside, it can be weird to see stuff like "line.project_onto(line) = line"
        // or "origin.project_onto(plane_at_origin) = origin". I think it's kind of cute that these
        // implementations are generated, but they are somewhat superfluous. If you wanted to do
        // those kind of projections, I'm almost certain you'd rather just take one of the arguments
        // as the answer instead of waste CPU cycles on all the jumbling and juggling and products
        // and wasted floating point Simd math or whatever. I don't want to jump to conclusions
        // though, because maybe there is special effects on the weight when these happen, and
        // I shouldn't just assume that is useless/insignificant.

        let contraction_expansion_stuff = [
            ("BulkContraction", "RightBulkDual", "AntiWedge"),
            ("WeightContraction", "RightWeightDual", "AntiWedge"),
            ("BulkExpansion", "RightBulkDual", "Wedge"),
            ("WeightExpansion", "RightWeightDual", "Wedge"),
        ];
        for (name, dual, product) in contraction_expansion_stuff {
            for (param_a, param_b) in registry.pair_parameters() {
                let a_name = param_a.multi_vector_class().class_name.as_str();
                let b_name = param_b.multi_vector_class().class_name.as_str();
                if non_objects.contains(&a_name) || non_objects.contains(&b_name) {
                    continue;
                }
                let _: Option<()> = try {
                    let rbd = self.trait_impls.get_single_invocation(dual, variable(&param_b))?;
                    let aw = self.trait_impls.get_pair_invocation(product, variable(&param_a), rbd)?;
                    let bc = single_expression_pair_trait_impl(name, &param_a, &param_b, aw);
                    self.trait_impls.add_pair_impl(name, param_a, param_b, bc);
                };
            }
        }


        // TODO generated cga projections don't look extremely compelling yet.
        //  Probably missing a few prerequisites, or have a few minor misplacements.
        //  (e.g. more dual/complement mixups.)


        for (param_a, param_b) in registry.pair_parameters() {
            let name = "ProjectOrthogonallyOnto";
            let _: Option<()> = try {
                let we = self.trait_impls.get_pair_invocation("WeightExpansion", variable(&param_a), variable(&param_b))?;
                let anti_wedge = self.algebra.dialect().exterior_anti_product.first()?;
                let anti_wedge = self.trait_impls.get_pair_invocation(anti_wedge, variable(&param_b), we)?;
                let po = single_expression_pair_trait_impl(name, &param_a, &param_b, anti_wedge);
                self.trait_impls.add_pair_impl(name, param_a, param_b, po);
            };
        }
        for (param_a, param_b) in registry.pair_parameters() {
            let name = "AntiProjectOrthogonallyOnto";
            let _: Option<()> = try {
                let wc = self.trait_impls.get_pair_invocation("WeightContraction", variable(&param_a), variable(&param_b))?;
                let wedge = self.algebra.dialect().exterior_product.first()?;
                let wedge = self.trait_impls.get_pair_invocation(wedge, variable(&param_b), wc)?;
                let apo = single_expression_pair_trait_impl(name, &param_a, &param_b, wedge);
                self.trait_impls.add_pair_impl(name, param_a, param_b, apo);
            };
        }
        for (param_a, param_b) in registry.pair_parameters() {
            let name = "ProjectViaOriginOnto";
            let _: Option<()> = try {
                let be = self.trait_impls.get_pair_invocation("BulkExpansion", variable(&param_a), variable(&param_b))?;
                let anti_wedge = self.algebra.dialect().exterior_anti_product.first()?;
                let anti_wedge = self.trait_impls.get_pair_invocation(anti_wedge, variable(&param_b), be)?;
                let po = single_expression_pair_trait_impl(name, &param_a, &param_b, anti_wedge);
                self.trait_impls.add_pair_impl(name, param_a, param_b, po);
            };
        }
        // anti_project_via_horizon can be a little confusing... are these okay?
        //  line_at_origin.anti_project_via_horizon_onto(point_at_infinity) = line_at_origin
        //  plane.anti_project_via_horizon_onto(point_at_infinity) = plane
        //  ...
        //  I'm 90% sure it's okay though. I suspect what is happening is objects at the origin
        //  more or less rotate at the origin. (Might not be the exact same thing as a Rotor
        //  transformation if there are effects on the weight of the result.) Objects not at the
        //  origin presumably rotate in a similar fashion, although I'm not certain around
        //  which point. Heck... probably the origin again.
        // TODO play with this at runtime to get a better feel, and reach 100% certainty it's okay.
        for (param_a, param_b) in registry.pair_parameters() {
            let name = "AntiProjectViaHorizonOnto";
            let _: Option<()> = try {
                let bc = self.trait_impls.get_pair_invocation("BulkContraction", variable(&param_a), variable(&param_b))?;
                let wedge = self.algebra.dialect().exterior_product.first()?;
                let wedge = self.trait_impls.get_pair_invocation(wedge, variable(&param_b), bc)?;
                let apo = single_expression_pair_trait_impl(name, &param_a, &param_b, wedge);
                self.trait_impls.add_pair_impl(name, param_a, param_b, apo);
            };
        }

        /*
        To understand the CosineAngle operation, let's walk through a few examples
        See this chart for various stuff:
        https://projectivegeometricalgebra.org/projgeomalg.pdf
        And AntiWedge Product results:
        https://rigidgeometricalgebra.org/wiki/index.php?title=Exterior_products

        In all examples, lets assume the objects are unitized. That means the weight norm of each object is 1, and
        as you can see by the formula, the weight norm of the result is also 1. That is, the result will also be unitized.
        This lets us focus our attention on the bulk part, which must be where the cosine behavior takes place.

        Plane vs Plane
        Planes can be understood with a normal vector and "position" which is the distance from the origin.
        The normal vector has the projective components, and so is the weight. Which makes sense, since it is about
        orientation. So when the plane is unitized, the normal vector is magnitude 1. You can alternatively consider a
        non-unitized normal vector that actually reaches from the origin to the plane, and the positional element would be
        1 instead. However that would be the inverse of unitized, so to speak. Anyway.... looking at the chart you can see
        the weight dual of a plane is actually (the negative of) that point. So now we look at the results of an
        anti_wedge(plane, point), and see there are mostly zeros, and then some negative 1s. The negative 1 results
        counter the negative 1s of the weight dual. Then ultimately the factors that remain basically got dot-producted,
        with no basis interference in the result (it is just a scalar). So long story short, if you unitize the planes
        before seeking their angle, then the normal vectors are unit vectors, so you can just dot product those normal
        vectors to get the cosine of the angle between them.

        Point vs Point
        But how can it be? What does it even mean to take the angle between two points?? Well why not find out?
        The weight dual of a point is just its weight factor e321. However again we assume the points are unitized. So
        this factor is simply 1. Then we AntiWedge a whole point with e321. All the factors are 0 except antiwedge(e4, e321),
        for which the unit is scalar 1. So basically the result of CosineAngle on two unitized points is to say "hey
        both of their weights are 1 because they are unitized, so lets do Scalar(1*1) + AntiScalar(1*1) and that is your
        HomogeneousMagnitude result, in other words 1. And what is the angle such that cosine(angle)=1? Well 0 of course.
        So the angle between two points is zero.

        Line vs Line
        Similar to plane, you gotta really understand the bulk vs weight aspects of lines. The bulk is the distance from
        the origin, in other words the factors in the bulk correspond to the factors for a point/vector from origin to the
        line (at the point of the line closest to origin). Then the weight helps distinguish the direction of the line
        intersecting that point, but it is not arbitrary. In order to fulfill the geometric property it must be orthogonal
        to the bulk. Anyway if the weight is normalized, then it is just a unit vector to tell the direction. So...
        The weight dual of the line is the (negated) coefficients of the weight but the bases of the bulk. Now looking at
        anti_wedge(bivector, bivector) results.... Mostly 0s and -1s again. Each only works with its opposite. So this
        results in the weight of one line getting dot-producted with the weight of the other line. Fascinating.

        So for Plane vs Plane and Line vs Line you can see how we get the more specific angle formulas to the right
        in the big chart.
        */

        for (param_a, param_b) in registry.pair_parameters() {
            let name = "CosineAngle";
            let _: Option<()> = try {
                // Only allow angle between uniform Grade MultiVectorClasses.
                let _ = self.trait_impls.get_class_impl("Grade", param_a.multi_vector_class())?;
                let _ = self.trait_impls.get_class_impl("Grade", param_b.multi_vector_class())?;

                // We can return a Scalar and ignore the HomogeneousMagnitude fluff if we Unitize up front
                let a_unitize = self.trait_impls.get_single_invocation("Unitize", variable(&param_a))?;
                let b_unitize = self.trait_impls.get_single_invocation("Unitize", variable(&param_b))?;

                // The actual cosine part of the definition
                let wc = self.trait_impls.get_pair_invocation("WeightContraction", a_unitize, b_unitize)?;
                let bn = self.trait_impls.get_single_invocation("BulkNorm", wc)?;
                let raw_float = Expression {
                    size: 1,
                    data_type_hint: Some(DataType::SimdVector(1)),
                    content: ExpressionContent::Access(Box::new(bn), 0),
                };
                let cosine = single_expression_pair_trait_impl(name, &param_a, &param_b, raw_float);
                self.trait_impls.add_pair_impl(name, param_a, param_b, cosine);
            };
        }

        for (param_a, param_b) in registry.pair_parameters() {
            let name = "SineAngle";
            let _: Option<()> = try {
                let cos = self.trait_impls.get_pair_invocation("CosineAngle", variable(&param_a), variable(&param_b))?;
                let var_assign_cos = AstNode::VariableAssignment {
                    name: "cos",
                    data_type: Some(DataType::SimdVector(1)),
                    expression: Box::new(cos),
                };
                let var_cos = Box::new(Expression {
                    size: 1,
                    content: ExpressionContent::Variable("cos"),
                    data_type_hint: Some(DataType::SimdVector(1)),
                });
                let one = Box::new(Expression {
                    size: 1,
                    data_type_hint: Some(DataType::SimdVector(1)),
                    content: ExpressionContent::Constant(DataType::SimdVector(1), vec![1])
                });
                let cos_squared = Box::new(Expression {
                    size: 1,
                    data_type_hint: Some(DataType::SimdVector(1)),
                    content: ExpressionContent::Multiply(var_cos.clone(), var_cos),
                });
                let var_assign_cos_squared = AstNode::VariableAssignment {
                    name: "cos_squared",
                    data_type: Some(DataType::SimdVector(1)),
                    expression: cos_squared,
                };
                let var_cos_squared = Box::new(Expression {
                    size: 1,
                    content: ExpressionContent::Variable("cos_squared"),
                    data_type_hint: Some(DataType::SimdVector(1)),
                });
                let sub = Box::new(Expression {
                    size: 1,
                    data_type_hint: Some(DataType::SimdVector(1)),
                    content: ExpressionContent::Subtract(one, var_cos_squared)
                });
                let var_assign_sub = AstNode::VariableAssignment {
                    name: "sub",
                    data_type: Some(DataType::SimdVector(1)),
                    expression: sub,
                };
                let var_sub = Box::new(Expression {
                    size: 1,
                    content: ExpressionContent::Variable("sub"),
                    data_type_hint: Some(DataType::SimdVector(1)),
                });
                let sqrt = Box::new(Expression {
                    size: 1,
                    data_type_hint: Some(DataType::SimdVector(1)),
                    content: ExpressionContent::SquareRoot(var_sub)
                });
                let sine = AstNode::TraitImplementation {
                    result: Parameter { name, data_type: DataType::SimdVector(1), },
                    class: param_a.multi_vector_class(),
                    parameters: vec![param_a.clone(), param_b.clone()],
                    body: vec![
                        var_assign_cos,
                        var_assign_cos_squared,
                        var_assign_sub,
                        AstNode::ReturnStatement { expression: sqrt },
                    ],
                };
                self.trait_impls.add_pair_impl(name, param_a, param_b, sine);
            };
        }

        // for (param_a, param_b) in registry.pair_parameters() {
        //     let _: Option<()> = try {
        //
        //     };
        // }

        // Transflection?
        // https://rigidgeometricalgebra.org/wiki/index.php?title=Transflection
        // This is a sandwich operation of a special type of flector.
        // We're not really motivated to create an additional trait that is only valid on a data condition rather than a
        // typed representation. A better approach to this might be.... a "CanTransflect" trait or method
        // on Flectors that returns Option<Flector> which is just Some(self) if it fulfills both the geometric
        // property and the other flector requirement to be a transflection. In any case such methods do not seem
        // incredibly necessary at this time, at least not yet.

        // Commutators?
        // https://rigidgeometricalgebra.org/wiki/index.php?title=Commutators
        // Doesn't seem to have a first-order purpose, and we already have implementations of the stuff it is useful for.

        // Geometric Property?
        // https://rigidgeometricalgebra.org/wiki/index.php?title=Geometric_property
        // It would seem this amounts to a boolean/predicate. There could be an argument for making a trait out of this...
        // but under normal circumstances, it's kind of like.... if you violate the geometric property, that's kind of like
        // falling into an Option::None or Result::Err. So under "normal circumstances" and "the happy path", the operations
        // and stuff you are doing to your geometric objects should ideally not violate the geometric property to begin with,
        // and you probably have a bug, rather than wanting to know if the geometric property is satisfied for some
        // non-bug-detecting purpose. Anyway, it could be useful, and might even be somewhat ergonomic in Rust, but it would
        // be much more annoying in glsl or wgsl since branching is kind of discouraged on GPUs and they don't support
        // proper sum types (enums) like Rust (as far as I know anyway).

        // do_wgsl(algebra.algebra_name(), file_path);
    }

    /// Step 5: Attitude and its dependencies
    /// https://rigidgeometricalgebra.org/wiki/index.php?title=Attitude
    /// https://conformalgeometricalgebra.org/wiki/index.php?title=Attitude
    /// Note that e4 (article) = e3 (codegen) = Origin (MultivectorClass, One)
    pub fn attitude_and_dependencies<'s>(&'s mut self, horizon_class_name: &str, registry: &'r MultiVectorClassRegistry) {
        // Attitude
        for param_a in registry.single_parameters() {
            let name = "Attitude";
            let _: Option<()> = try {
                let horizon = &registry.classes.iter().find(|it| it.0.class_name == horizon_class_name)?.0;
                let one = self.trait_impls.get_class_invocation("One", horizon)?;
                let anti_wedge = self.algebra.dialect().exterior_anti_product.first()?;
                let anti_wedge = self.trait_impls.get_pair_invocation(anti_wedge, variable(&param_a), one)?;
                let attitude = single_expression_single_trait_impl(name, &param_a, anti_wedge);

                self.trait_impls.add_single_impl(name, param_a.clone(), attitude);
            };
        }

        for (param_a, param_b) in registry.pair_parameters() {
            // https://rigidgeometricalgebra.org/wiki/index.php?title=Euclidean_distance
            let name = "Distance";
            let _: Option<()> = try {
                let wedge_name = self.algebra.dialect().exterior_product.first()?;
                let bulk_wedge = self.trait_impls.get_pair_invocation(wedge_name, variable(&param_a), variable(&param_b))?;
                let bulk_attitude = self.trait_impls.get_single_invocation("Attitude", bulk_wedge)?;
                let weight_attitude = self.trait_impls.get_single_invocation("Attitude", variable(&param_b))?;
                let weight_wedge = self.trait_impls.get_pair_invocation(wedge_name, variable(&param_a), weight_attitude)?;
                let bulk_norm = self.trait_impls.get_single_invocation("BulkNorm", bulk_attitude)?;
                let weight_norm = self.trait_impls.get_single_invocation("WeightNorm", weight_wedge)?;
                let add = self.trait_impls.get_pair_invocation("Add", bulk_norm, weight_norm)?;
                let ed = single_expression_pair_trait_impl(name, &param_a, &param_b, add);
                self.trait_impls.add_pair_impl(name, param_a, param_b, ed);
            };
        }
    }

    pub fn round_features<'s>(&'s mut self, flat_basis: BasisElement, registry: &'r MultiVectorClassRegistry) {
        for param_a in registry.single_parameters() {
            let is_all_flat = param_a.multi_vector_class().flat_basis().iter().all(|it| {
                flat_basis.index == (flat_basis.index & it.index)
            });
            if is_all_flat {
                continue;
            }

            // The object is round

            let mut e5_candidates: Vec<_> = registry.classes.iter().filter_map(|it| {
                if it.0.flat_basis().contains(&flat_basis) { Some(&it.0) } else { None }
            }).collect();
            e5_candidates.sort_by(|a, b| a.flat_basis().len().cmp(&b.flat_basis().len()));
            let mut construct_infinity = None;
            if let Some(mvc) = e5_candidates.first() {
                let mut body = vec![];
                for group in &mvc.grouped_basis {
                    let mut elements = vec![];
                    for element in group {
                        let v = if *element == flat_basis { 1 } else { 0 };
                        elements.push(v);
                    }
                    let e = Expression {
                        size: group.len(),
                        data_type_hint: None,
                        content: ExpressionContent::Constant(DataType::SimdVector(group.len()), elements),
                    };
                    body.push((DataType::SimdVector(group.len()), e));
                }
                construct_infinity = Some(Expression {
                    size: 1,
                    data_type_hint: Some(DataType::MultiVector(mvc)),
                    content: ExpressionContent::InvokeClassMethod(mvc, "Constructor", body),
                });
            }

            // TODO a lot of these have "simpler" more direct implementations per object
            //  You can see on the tables on the wiki pages
            //  It is nice to have old reliable formulas that always work
            //  but it might also be nice to (eventually) write out most direct formulations
            //  instead, since they'll definitely use less CPU power. The same could possibly
            //  be said for almost all other trait impls. So maybe the right solution is
            //  an alternate version of "simplify_and_legalize" that inlines trait
            //  invocations.

            let name = "Carrier";
            let _: Option<()> = try {
                let construct = construct_infinity.clone()?;
                let wedge_name = self.algebra.dialect().exterior_product.first()?;
                let wedge = self.trait_impls.get_pair_invocation(wedge_name, variable(&param_a), construct)?;
                let carrier = single_expression_single_trait_impl(name, &param_a, wedge);
                self.trait_impls.add_single_impl(name, param_a.clone(), carrier)
            };

            let name = "CoCarrier";
            let _: Option<()> = try {
                let construct = construct_infinity.clone()?;
                let wedge_name = self.algebra.dialect().exterior_product.first()?;
                let right_weight_dual = self.trait_impls.get_single_invocation("RightRoundWeightDual", variable(&param_a))?;
                let wedge = self.trait_impls.get_pair_invocation(wedge_name, right_weight_dual, construct)?;
                let carrier = single_expression_single_trait_impl(name, &param_a, wedge);
                self.trait_impls.add_single_impl(name, param_a.clone(), carrier)
            };

            let name = "Center";
            let _: Option<()> = try {
                let anti_wedge_name = self.algebra.dialect().exterior_anti_product.first()?;
                let ccr = self.trait_impls.get_single_invocation("CoCarrier", variable(&param_a))?;
                let anti_wedge = self.trait_impls.get_pair_invocation(anti_wedge_name, ccr, variable(&param_a))?;
                let center = single_expression_single_trait_impl(name, &param_a, anti_wedge);
                self.trait_impls.add_single_impl(name, param_a.clone(), center)
            };

            let name = "Container";
            let _: Option<()> = try {
                let wedge_name = self.algebra.dialect().exterior_product.first()?;
                let car = self.trait_impls.get_single_invocation("Carrier", variable(&param_a))?;
                let weight_dual = self.trait_impls.get_single_invocation("RightWeightDual", car)?;
                let anti_wedge = self.trait_impls.get_pair_invocation(wedge_name, variable(&param_a), weight_dual)?;
                let container = single_expression_single_trait_impl(name, &param_a, anti_wedge);
                self.trait_impls.add_single_impl(name, param_a.clone(), container)
            };
        }

        for param_a in registry.single_parameters() {
            let is_all_flat = param_a.multi_vector_class().flat_basis().iter().all(|it| {
                flat_basis.index == (flat_basis.index & it.index)
            });
            if is_all_flat {
                continue;
            }

            // The object is round

            let name = "Partner";
            let _: Option<()> = try {
                // let mut debug = false;
                // if param_a.multi_vector_class().class_name == "RoundPoint" {
                //     debug = true;
                //     eprintln!("Implementing partner for RoundPoint");
                // }
                let anti_wedge_name = self.algebra.dialect().exterior_anti_product.first()?;
                let car = self.trait_impls.get_single_invocation("Carrier", variable(&param_a))?;
                // if debug {
                //     eprintln!("Found Carrier for RoundPoint");
                // }
                let bulk_dual = self.trait_impls.get_single_invocation("RightBulkDual", variable(&param_a))?;
                // if debug {
                //     eprintln!("Found RightBulkDual for RoundPoint");
                // }
                let container = self.trait_impls.get_single_invocation("Container", bulk_dual)?;
                // if debug {
                //     eprintln!("Found Container");
                // }
                let neg = self.trait_impls.get_single_invocation("Neg", container)?;
                // if debug {
                //     eprintln!("Found Neg");
                // }
                let anti_wedge = self.trait_impls.get_pair_invocation(anti_wedge_name, neg, car)?;
                // if debug {
                //     eprintln!("Found AntiWedge");
                // }
                let partner = single_expression_single_trait_impl(name, &param_a, anti_wedge);
                self.trait_impls.add_single_impl(name, param_a.clone(), partner)
            };
        }
    }

    /// Datatype definitions, and implementations of external traits
    pub fn emit_datatypes_and_external_traits(&mut self, registry: &'r MultiVectorClassRegistry, emitter: &mut Emitter<std::fs::File>) -> std::io::Result<()> {
        // Preamble
        // emitter.emit(&AstNode::Preamble)?;

        // Class Definitions
        for (class, _) in registry.classes.iter() {
            if class.class_name == "Origin" {
                emitter.emit(&AstNode::TypeAlias("PointAtOrigin".to_string(), "Origin".to_string()))?;
            }
            if class.class_name == "Horizon" {
                emitter.emit(&AstNode::TypeAlias("PlaneAtInfinity".to_string(), "Horizon".to_string()))?;
            }
            emitter.emit(&AstNode::ClassDefinition { class })?;
        }

        // External Traits
        let trait_names = ["Zero", "One", "Neg", "Into", "Add", "Sub", "Mul", "Div"];
        self.emit_exact_name_match_trait_impls(&trait_names, emitter)?;
        Ok(())
    }

    pub fn emit_component_wise_aspects(&mut self, emitter: &mut Emitter<std::fs::File>) -> std::io::Result<()> {
        // Bulk, Weight, RoundBulk, RoundWeight

        emitter.emit(&AstNode::TraitDefinition {
            name: "Bulk".to_string(),
            params: 1,
            docs: "
            The Bulk of an object usually describes the object's relationship with the origin.
            An object with a Bulk of zero contains the origin.
            http://rigidgeometricalgebra.org/wiki/index.php?title=Bulk_and_weight
        "
            .to_string(),
        })?;

        emitter.emit(&AstNode::TraitDefinition {
            name: "Weight".to_string(),
            params: 1,
            docs: "
            The Weight of an object usually describes the object's attitude and orientation.
            An object with zero weight is contained by the horizon.
            Also known as the attitude operator.
            http://rigidgeometricalgebra.org/wiki/index.php?title=Bulk_and_weight
        "
            .to_string(),
        })?;

        if self.algebra.algebra_name().contains("cga") {
            emitter.emit(&AstNode::TraitDefinition {
                name: "RoundBulk".to_string(),
                params: 1,
                docs: "
                Round Bulk is a special type of bulk in CGA
                https://conformalgeometricalgebra.com/wiki/index.php?title=Main_Page
            "
                .to_string(),
            })?;

            emitter.emit(&AstNode::TraitDefinition {
                name: "RoundWeight".to_string(),
                params: 1,
                docs: "
                Round Weight is a special type of weight in CGA
                https://conformalgeometricalgebra.com/wiki/index.php?title=Main_Page
            "
                .to_string(),
            })?;
        }

        let trait_names = ["Bulk", "Weight", "RoundBulk", "RoundWeight"];
        self.emit_exact_name_match_trait_impls(&trait_names, emitter)?;
        Ok(())
    }

    pub fn emit_unitize(&mut self, emitter: &mut Emitter<std::fs::File>) -> std::io::Result<()> {
        // Unitize

        emitter.emit(&AstNode::TraitDefinition {
            name: "Unitize".to_string(),
            params: 1,
            docs: "
            Unitization
            https://rigidgeometricalgebra.org/wiki/index.php?title=Unitization
        "
            .to_string(),
        })?;

        let trait_names = ["Unitize"];
        self.emit_exact_name_match_trait_impls(&trait_names, emitter)?;
        Ok(())
    }

    pub fn emit_geometric_products(&mut self, emitter: &mut Emitter<std::fs::File>) -> std::io::Result<()> {
        let products = Product::products(&self.algebra);

        let mut trait_names = BTreeSet::new();
        for n in &self.algebra.dialect().geometric_product {
            trait_names.insert(n.to_string());
        }
        for n in &self.algebra.dialect().geometric_anti_product {
            trait_names.insert(n.to_string());
        }
        for (name, _, docs) in &products {
            if trait_names.contains(*name) {
                emitter.emit(&AstNode::TraitDefinition {
                    name: name.to_string(),
                    params: 2,
                    docs: docs.to_string(),
                })?;
            }
        }

        let trait_names2: Vec<_> = trait_names.iter().map(|it| it.as_str()).collect();
        self.emit_exact_name_match_trait_impls(trait_names2.as_slice(), emitter)?;
        Ok(())
    }
    pub fn emit_exterior_products(&mut self, emitter: &mut Emitter<std::fs::File>) -> std::io::Result<()> {
        let products = Product::products(&self.algebra);

        let mut trait_names = BTreeSet::new();
        for n in &self.algebra.dialect().exterior_product {
            trait_names.insert(n.to_string());
        }
        for n in &self.algebra.dialect().exterior_anti_product {
            trait_names.insert(n.to_string());
        }
        for (name, _, docs) in &products {
            if trait_names.contains(*name) {
                emitter.emit(&AstNode::TraitDefinition {
                    name: name.to_string(),
                    params: 2,
                    docs: docs.to_string(),
                })?;
            }
        }

        let trait_names2: Vec<_> = trait_names.iter().map(|it| it.as_str()).collect();
        self.emit_exact_name_match_trait_impls(trait_names2.as_slice(), emitter)?;
        Ok(())
    }
    pub fn emit_dot_products(&mut self, emitter: &mut Emitter<std::fs::File>) -> std::io::Result<()> {
        let products = Product::products(&self.algebra);

        let mut trait_names = BTreeSet::new();
        for n in &self.algebra.dialect().dot_product {
            trait_names.insert(n.to_string());
        }
        for n in &self.algebra.dialect().anti_dot_product {
            trait_names.insert(n.to_string());
        }
        for (name, _, docs) in &products {
            if trait_names.contains(*name) {
                emitter.emit(&AstNode::TraitDefinition {
                    name: name.to_string(),
                    params: 2,
                    docs: docs.to_string(),
                })?;
            }
        }

        let trait_names2: Vec<_> = trait_names.iter().map(|it| it.as_str()).collect();
        self.emit_exact_name_match_trait_impls(trait_names2.as_slice(), emitter)?;
        Ok(())
    }

    pub fn emit_isometries(&mut self, emitter: &mut Emitter<std::fs::File>) -> std::io::Result<()> {
        emitter.emit(&AstNode::TraitDefinition {
            name: "Sandwich".to_string(),
            params: 2,
            docs: "
            self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())

            Also called sandwich product
            See article \"Projective Geometric Algebra Done Right\"
            https://rigidgeometricalgebra.org/wiki/index.php?title=Projective_Geometric_Algebra_Done_Right
        "
            .to_string(),
        })?;

        emitter.emit(&AstNode::TraitDefinition {
            name: "Invert".to_string(),
            params: 2,
            docs: "
            Invert (Inversion)
            An improper isometry that performs an inversion through a point.
            Be careful not to confuse with `Inverse`, which raises a number to the power of `-1.0`.
            https://rigidgeometricalgebra.org/wiki/index.php?title=Inversion
        "
            .to_string(),
        })?;

        emitter.emit(&AstNode::TraitDefinition {
            name: "Reflect".to_string(),
            params: 2,
            docs: "
            Reflection
            https://rigidgeometricalgebra.org/wiki/index.php?title=Reflection
        "
            .to_string(),
        })?;

        self.emit_exact_name_match_trait_impls(&["Sandwich"], emitter)?;
        self.emit_exact_name_match_trait_impls(&["Invert", "Reflect"], emitter)?;
        Ok(())
    }

    pub fn emit_involutions_and_duals(&mut self, emitter: &mut Emitter<std::fs::File>) -> std::io::Result<()> {
        let external_trait_names = vec!["Neg"];
        let mut trait_names = BTreeSet::new();
        let involutions = Involution::involutions(&self.algebra);
        for (name, _, docs) in involutions.iter() {
            if !external_trait_names.contains(name) {
                let name = name.to_string();
                trait_names.insert(name.clone());
                emitter.emit(&AstNode::TraitDefinition {
                    name,
                    params: 1,
                    docs: docs.to_string(),
                })?;
            }
        }

        let trait_names: Vec<_> = trait_names.iter().map(|it| it.as_str()).collect();
        self.emit_exact_name_match_trait_impls(trait_names.as_slice(), emitter)?;
        Ok(())
    }

    pub fn emit_aspect_duals(&mut self, emitter: &mut Emitter<std::fs::File>) -> std::io::Result<()> {
        let mut trait_names = BTreeSet::new();
        trait_names.insert("RightBulkDual".to_string());
        trait_names.insert("RightWeightDual".to_string());
        trait_names.insert("LeftBulkDual".to_string());
        trait_names.insert("LeftWeightDual".to_string());
        let is_cga = self.algebra.algebra_name().contains("cga");
        if is_cga {
            trait_names.insert("RightRoundBulkDual".to_string());
            trait_names.insert("RightRoundWeightDual".to_string());
            trait_names.insert("LeftRoundBulkDual".to_string());
            trait_names.insert("LeftRoundWeightDual".to_string());
        }

        emitter.emit(&AstNode::TraitDefinition {
            name: "RightBulkDual".to_string(),
            params: 1,
            docs: "
            Right Bulk Dual
            https://projectivegeometricalgebra.org/projgeomalg.pdf
            ".to_string(),
        })?;
        emitter.emit(&AstNode::TraitDefinition {
            name: "RightWeightDual".to_string(),
            params: 1,
            docs: "
            Right Weight Dual
            https://projectivegeometricalgebra.org/projgeomalg.pdf
            ".to_string(),
        })?;
        emitter.emit(&AstNode::TraitDefinition {
            name: "LeftBulkDual".to_string(),
            params: 1,
            docs: "
            Left Bulk Dual
            https://projectivegeometricalgebra.org/projgeomalg.pdf
            ".to_string(),
        })?;
        emitter.emit(&AstNode::TraitDefinition {
            name: "LeftWeightDual".to_string(),
            params: 1,
            docs: "
            Left Weight Dual
            https://projectivegeometricalgebra.org/projgeomalg.pdf
            ".to_string(),
        })?;

        if is_cga {
            emitter.emit(&AstNode::TraitDefinition {
                name: "RightRoundBulkDual".to_string(),
                params: 1,
                docs: "
                Right Round Bulk Dual
                https://projectivegeometricalgebra.org/projgeomalg.pdf
                https://projectivegeometricalgebra.org/confgeomalg.pdf
                ".to_string(),
            })?;
            emitter.emit(&AstNode::TraitDefinition {
                name: "RightRoundWeightDual".to_string(),
                params: 1,
                docs: "
                Right Round Weight Dual. Needed to implement CoCarriers.
                https://projectivegeometricalgebra.org/projgeomalg.pdf
                https://projectivegeometricalgebra.org/confgeomalg.pdf
                ".to_string(),
            })?;
            emitter.emit(&AstNode::TraitDefinition {
                name: "LeftRoundBulkDual".to_string(),
                params: 1,
                docs: "
                Left Round Bulk Dual
                https://projectivegeometricalgebra.org/projgeomalg.pdf
                https://projectivegeometricalgebra.org/confgeomalg.pdf
                ".to_string(),
            })?;
            emitter.emit(&AstNode::TraitDefinition {
                name: "LeftRoundWeightDual".to_string(),
                params: 1,
                docs: "
                Left Round Weight Dual
                https://projectivegeometricalgebra.org/projgeomalg.pdf
                https://projectivegeometricalgebra.org/confgeomalg.pdf
                ".to_string(),
            })?;
        }

        let trait_names: Vec<_> = trait_names.iter().map(|it| it.as_str()).collect();
        self.emit_exact_name_match_trait_impls(trait_names.as_slice(), emitter)?;
        Ok(())
    }

    pub fn emit_contractions(&mut self, emitter: &mut Emitter<std::fs::File>) -> std::io::Result<()> {
        let trait_names = ["BulkContraction", "WeightContraction"];
        emitter.emit(&AstNode::TraitDefinition {
            name: "BulkContraction".to_string(),
            params: 2,
            docs: "
            Bulk Contraction (Interior Product)
            https://rigidgeometricalgebra.org/wiki/index.php?title=Interior_products
        "
            .to_string(),
        })?;
        emitter.emit(&AstNode::TraitDefinition {
            name: "WeightContraction".to_string(),
            params: 2,
            docs: "
            Weight Contraction (Interior Product)
            https://rigidgeometricalgebra.org/wiki/index.php?title=Interior_products
        "
            .to_string(),
        })?;
        self.emit_exact_name_match_trait_impls(&trait_names, emitter)?;
        Ok(())
    }

    pub fn emit_expansions(&mut self, emitter: &mut Emitter<std::fs::File>) -> std::io::Result<()> {
        let trait_names = ["BulkExpansion", "WeightExpansion"];
        emitter.emit(&AstNode::TraitDefinition {
            name: "BulkExpansion".to_string(),
            params: 2,
            docs: "
            Bulk Expansion (Interior Product)
            https://rigidgeometricalgebra.org/wiki/index.php?title=Interior_products
        "
            .to_string(),
        })?;
        emitter.emit(&AstNode::TraitDefinition {
            name: "WeightExpansion".to_string(),
            params: 2,
            docs: "
            Weight Expansion (Interior Product)
            https://rigidgeometricalgebra.org/wiki/index.php?title=Interior_products
        "
            .to_string(),
        })?;
        self.emit_exact_name_match_trait_impls(&trait_names, emitter)?;
        Ok(())
    }

    pub fn emit_norms(&mut self, emitter: &mut Emitter<std::fs::File>) -> std::io::Result<()> {
        // bulk norm, weight norm, position norm, center norm, radius norm, squared variants
        let mut trait_names = BTreeSet::new();
        for ((name, _), _) in &self.trait_impls.single_args {
            if name.contains("Norm") {
                trait_names.insert(name.to_string());
            }
        }
        for name in &trait_names {
            // Even though CGA has its own unique norms, there is not a page dedicated to them on the CGA wiki yet.
            let docs = format!(
                "
                {name}
                https://rigidgeometricalgebra.org/wiki/index.php?title=Geometric_norm
            "
            );
            emitter.emit(&AstNode::TraitDefinition {
                name: name.clone(),
                params: 1,
                docs,
            })?;
        }
        for ((name, _), ast) in &self.trait_impls.single_args {
            if trait_names.contains(name) && name.as_str() != "GeometricNorm" && !name.contains("Unitized") {
                emitter.emit(ast)?;
            }
        }
        self.emit_exact_name_match_trait_impls(&["GeometricNorm"], emitter)?;
        self.emit_exact_name_match_trait_impls(&["UnitizedNormSquared"], emitter)?;
        self.emit_exact_name_match_trait_impls(&["UnitizedNorm"], emitter)?;
        Ok(())
    }

    pub fn emit_characteristic_features(&mut self, emitter: &mut Emitter<std::fs::File>) -> std::io::Result<()> {
        // Sqrt, Grade, AntiGrade, Attitude, Carrier, CoCarrier, Container, Center, Partner

        emitter.emit(&AstNode::TraitDefinition {
            name: "Sqrt".to_string(),
            params: 1,
            docs: "
            Square Root
            ".to_string(),
        })?;

        emitter.emit(&AstNode::TraitDefinition {
            name: "Grade".to_string(),
            params: 0,
            docs: "
            Grade
            https://rigidgeometricalgebra.org/wiki/index.php?title=Grade_and_antigrade
            ".to_string(),
        })?;

        emitter.emit(&AstNode::TraitDefinition {
            name: "AntiGrade".to_string(),
            params: 0,
            docs: "
            Anti-Grade
            https://rigidgeometricalgebra.org/wiki/index.php?title=Grade_and_antigrade
            ".to_string(),
        })?;

        emitter.emit(&AstNode::TraitDefinition {
            name: "Attitude".to_string(),
            params: 1,
            docs: "
            Attitude
            https://rigidgeometricalgebra.org/wiki/index.php?title=Attitude
            ".to_string(),
        })?;

        if self.algebra.algebra_name().contains("cga") {
            emitter.emit(&AstNode::TraitDefinition {
                name: "Carrier".to_string(),
                params: 1,
                docs: "
                Carrier
                The Carrier of a round object is the lowest dimensional flat object that contains it.
                https://conformalgeometricalgebra.org/wiki/index.php?title=Carriers
                ".to_string(),
            })?;

            emitter.emit(&AstNode::TraitDefinition {
                name: "CoCarrier".to_string(),
                params: 1,
                docs: "
                CoCarrier
                The CoCarrier of a round object is the Carrier of its antidual.
                https://conformalgeometricalgebra.org/wiki/index.php?title=Carriers
                ".to_string(),
            })?;

            emitter.emit(&AstNode::TraitDefinition {
                name: "Container".to_string(),
                params: 1,
                docs: "
                Container
                The Container of a round object is the smallest Sphere that contains it.
                https://conformalgeometricalgebra.org/wiki/index.php?title=Containers
                ".to_string(),
            })?;

            emitter.emit(&AstNode::TraitDefinition {
                name: "Center".to_string(),
                params: 1,
                docs: "
                Center
                The Center of a round object is the RoundPoint having the same center and radius.
                https://conformalgeometricalgebra.org/wiki/index.php?title=Centers
                ".to_string(),
            })?;

            emitter.emit(&AstNode::TraitDefinition {
                name: "Partner".to_string(),
                params: 1,
                docs: "
                Partner
                The Partner of a round object is the round object having the same center, same carrier,
                and same absolute size, but having a squared radius of the opposite sign.
                The dot product between a round object and its partner is always zero. They are orthogonal.
                https://conformalgeometricalgebra.org/wiki/index.php?title=Partners
                ".to_string(),
            })?;
        }

        let trait_names = ["Sqrt", "Grade", "AntiGrade", "Attitude", "Carrier", "CoCarrier"];
        self.emit_exact_name_match_trait_impls(&trait_names, emitter)?;
        let trait_names = ["Container", "Center"];
        self.emit_exact_name_match_trait_impls(&trait_names, emitter)?;
        let trait_names = ["Partner"];
        self.emit_exact_name_match_trait_impls(&trait_names, emitter)?;
        Ok(())
    }

    pub fn emit_projections_and_stuff(&mut self, emitter: &mut Emitter<std::fs::File>) -> std::io::Result<()> {
        emitter.emit(&AstNode::TraitDefinition {
            name: "ProjectOrthogonallyOnto".to_string(),
            params: 2,
            docs: "
            Orthogonal Projection
            Typically involves bringing a lower dimensional object to a higher dimensional object
            https://rigidgeometricalgebra.org/wiki/index.php?title=Projections
            https://projectivegeometricalgebra.org/projgeomalg.pdf
        "
            .to_string()
            .to_string(),
        })?;
        emitter.emit(&AstNode::TraitDefinition {
            name: "AntiProjectOrthogonallyOnto".to_string(),
            params: 2,
            docs: "
            Orthogonal AntiProjection
            Typically involves bringing a higher dimensional object to a lower dimensional object.
            https://rigidgeometricalgebra.org/wiki/index.php?title=Projections
            https://projectivegeometricalgebra.org/projgeomalg.pdf
        "
            .to_string(),
        })?;
        emitter.emit(&AstNode::TraitDefinition {
            name: "ProjectViaOriginOnto".to_string(),
            params: 2,
            docs: "
            Central (to origin) Projection
            https://rigidgeometricalgebra.org/wiki/index.php?title=Projections
            https://projectivegeometricalgebra.org/projgeomalg.pdf
        "
            .to_string(),
        })?;
        emitter.emit(&AstNode::TraitDefinition {
            name: "AntiProjectViaHorizonOnto".to_string(),
            params: 2,
            docs: "
            Outward (to horizon) AntiProjection
            https://rigidgeometricalgebra.org/wiki/index.php?title=Projections
        "
            .to_string(),
        })?;

        let trait_names = ["ProjectOrthogonallyOnto", "ProjectOrthogonallyOnto", "ProjectViaOriginOnto", "AntiProjectViaHorizonOnto"];
        self.emit_exact_name_match_trait_impls(&trait_names, emitter)?;
        Ok(())
    }

    pub fn emit_metric_operations(&mut self, emitter: &mut Emitter<std::fs::File>) -> std::io::Result<()> {
        emitter.emit(&AstNode::TraitDefinition {
            name: "Distance".to_string(),
            params: 2,
            docs: "
            Euclidean distance between objects
            https://rigidgeometricalgebra.org/wiki/index.php?title=Euclidean_distance
            distance(a,b) = bulk_norm(attitude(a wedge b)) + weight_norm(a wedge attitude(b))
            where attitude(c) = c anti_wedge complement(e4) where e4 is the projective dimension
        "
            .to_string(),
        })?;
        emitter.emit(&AstNode::TraitDefinition {
            name: "CosineAngle".to_string(),
            params: 2,
            docs: "
            The cosine of the angle between two objects.
            https://projectivegeometricalgebra.org/projgeomalg.pdf
        "
            .to_string(),
        })?;
        emitter.emit(&AstNode::TraitDefinition {
            name: "SineAngle".to_string(),
            params: 2,
            docs: "
            The sine of the angle between two objects.
            https://projectivegeometricalgebra.org/projgeomalg.pdf
        "
            .to_string(),
        })?;

        let trait_names = ["Distance", "CosineAngle", "SineAngle"];
        self.emit_exact_name_match_trait_impls(&trait_names, emitter)?;
        Ok(())
    }

    fn emit_exact_name_match_trait_impls(&mut self, trait_names: &[&str], emitter: &mut Emitter<std::fs::File>) -> std::io::Result<()> {
        for ((name, _), ast_node) in &self.trait_impls.class_level {
            if trait_names.contains(&name.as_str()) {
                emitter.emit(ast_node)?;
            }
        }
        for ((name, _), ast_node) in &self.trait_impls.single_args {
            if trait_names.contains(&name.as_str()) {
                emitter.emit(ast_node)?;
            }
        }
        for ((name, _, _), ast_node) in &self.trait_impls.pair_args {
            if trait_names.contains(&name.as_str()) {
                emitter.emit(ast_node)?;
            }
        }
        Ok(())
    }
}
