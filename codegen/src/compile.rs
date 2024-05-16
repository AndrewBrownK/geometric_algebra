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
        ExpressionContent::Divide(mut a, mut b) => {
            a = simplify_and_legalize(a);
            b = simplify_and_legalize(b);
            let data_type_hint = if a.data_type_hint == b.data_type_hint { a.data_type_hint.clone() } else { None };
            return Box::new(Expression {
                size: expression.size,
                content: ExpressionContent::Divide(a, b),
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
            "Unit" => (1, 1),
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
            let indexes_match = in_element.index == a_element.index;
            let in_is_nonzero = in_element.coefficient != 0;
            let out_is_nonzero = out_element.coefficient != 0;
            if indexes_match && in_is_nonzero && out_is_nonzero {
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

    let result_class = match registry.get_at_least(result_signature.as_slice()) {
        None => return AstNode::None,
        Some(rc) => rc,
    };
    let result_flat_basis = result_class.flat_basis();
    let mut body = Vec::new();
    let mut base_index = 0;
    let mut all_zeroes = true;
    let mut is_identity = result_class == parameter_a.multi_vector_class();
    for (result_group_index, result_group) in result_class.grouped_basis.iter().enumerate() {
        let size = result_group.len();
        let mut a_group_index = None;
        let mut factors = vec![];
        let mut a_indices = vec![];
        'for_index_in_group: for index_in_group in 0..size {
            let result_element = &result_flat_basis[base_index + index_in_group];
            let (in_element, out_element) = match involution.terms.iter().find(|(_in, out)| out.index == result_element.index) {
                Some((_in, out)) => (_in.clone(), out.clone()),
                None => (BasisElement::zero(), BasisElement::zero())
            };
            if in_element.coefficient == 0 || out_element.coefficient == 0 {
                factors.push(0);
                is_identity = false;
                a_indices.push(GatherData::RawZero);
                continue 'for_index_in_group;
            }

            let index_in_a = a_flat_basis.iter().position(|a_element| a_element.index == in_element.index);
            let index_in_a = match index_in_a {
                None => {
                    factors.push(0);
                    is_identity = false;
                    a_indices.push(GatherData::RawZero);
                    continue 'for_index_in_group;
                },
                Some(index_in_a) => index_in_a,
            };
            all_zeroes = false;
            let coefficient = out_element.coefficient * result_element.coefficient * in_element.coefficient * a_flat_basis[index_in_a].coefficient;
            let (group, element) = parameter_a.multi_vector_class().index_in_group(index_in_a);
            let group_size = parameter_a.multi_vector_class().grouped_basis[group].len();
            if a_group_index.is_none() {
                a_group_index = Some(group);
            }
            let negate = false;
            factors.push(coefficient);
            if coefficient != 1 {
                is_identity = false;
            }
            if group != result_group_index || element != a_indices.len() {
                is_identity = false;
            }
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
    if all_zeroes {
        return AstNode::None;
    }
    let return_expr =
        if is_identity {
            variable(&parameter_a)
        } else {
            Expression {
                size: 1,
                content: ExpressionContent::InvokeClassMethod(result_class, "Constructor", body),
                data_type_hint: Some(DataType::MultiVector(result_class)),
            }
        };
    AstNode::TraitImplementation {
        result: Parameter {
            name,
            data_type: DataType::MultiVector(result_class),
        },
        class: parameter_a.multi_vector_class(),
        parameters: vec![parameter_a.clone()],
        body: vec![AstNode::ReturnStatement { expression: Box::new(return_expr), }],
    }
}

pub fn element_wise<'a>(name: &'static str, parameter_a: &Parameter<'a>, parameter_b: &Parameter<'a>, registry: &'a MultiVectorClassRegistry) -> AstNode<'a> {
    let a_flat_basis = parameter_a.multi_vector_class().flat_basis();
    let b_flat_basis = parameter_b.multi_vector_class().flat_basis();
    let result_signature = a_flat_basis.iter().chain(b_flat_basis.iter()).cloned().collect::<std::collections::HashSet<_>>();
    let mut result_signature = result_signature.into_iter().map(|element| element.index).collect::<Vec<_>>();
    result_signature.sort_unstable();

    let result_class = match registry.get_at_least(&result_signature) {
        None => return AstNode::None,
        Some(rc) => rc
    };

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

    let mut result_class = registry.get_at_least(&result_signature);

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

pub fn derive_bulk_or_weight<'a, GA: GeometricAlgebraTrait>(
    algebra: &GA,
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

    let mut result_class = registry.get_at_least(&result_signature);
    if algebra.algebra_name().contains("min") {
        result_class = Some(parameter_a.multi_vector_class());
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
                let index_in_a = parameter_a.multi_vector_class().flat_basis().iter().position(|a_element| a_element == result_element);
                let index_in_a = match index_in_a {
                    Some(iia) => iia,
                    None => return (0, GatherData::RawZero),
                };
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
            for name in &["Zero", "One", "Unit"] {
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
        // See also DualNum
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
            let mut name = "Sqrt";
            if class_name == "AntiScalar" {
                name = "AntiSqrt";
            }
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

        // Inverse
        for param_a in registry.single_parameters() {
            let name = "Inverse";
            let _: Option<()> = try {
                let dot = self.algebra.dialect().dot_product.first()?;
                let dot = self.trait_impls.get_pair_invocation(dot, variable(&param_a), variable(&param_a))?;
                let scalar_type = registry.classes.iter().find(|it| it.class_name == "Scalar")?;
                let one = self.trait_impls.get_class_invocation("Unit", scalar_type)?;
                let inverse_norm_squared = self.trait_impls.get_pair_invocation("Div", one, dot)?;
                let product = self.algebra.dialect().geometric_product.first()?;
                let expr = self.trait_impls.get_pair_invocation(product, variable(&param_a), inverse_norm_squared)?;
                let the_impl = single_expression_single_trait_impl(name, &param_a, expr);
                self.trait_impls.add_single_impl(name, param_a, the_impl);
            };
        }

        // Anti-Inverse
        for param_a in registry.single_parameters() {
            let name = "AntiInverse";
            let _: Option<()> = try {
                let dot = self.algebra.dialect().anti_dot_product.first()?;
                let dot = self.trait_impls.get_pair_invocation(dot, variable(&param_a), variable(&param_a))?;
                let scalar_type = registry.classes.iter().find(|it| it.class_name == "AntiScalar")?;
                let one = self.trait_impls.get_class_invocation("Unit", scalar_type)?;
                let inverse_norm_squared = self.trait_impls.get_pair_invocation("Div", one, dot)?;
                let product = self.algebra.dialect().geometric_anti_product.first()?;
                let expr = self.trait_impls.get_pair_invocation(product, variable(&param_a), inverse_norm_squared)?;
                let the_impl = single_expression_single_trait_impl(name, &param_a, expr);
                self.trait_impls.add_single_impl(name, param_a, the_impl);
            };
        }

        // TODO study the output types of quotients more closely,
        //  I'm seeing a lot of non-specific "MultiVector" outputs.
        for (param_a, param_b) in registry.pair_parameters() {
            let name = "GeometricQuotient";
            let _: Option<()> = try {
                let inverse_b = self.trait_impls.get_single_invocation("Inverse", variable(&param_b))?;
                let product = self.algebra.dialect().geometric_product.first()?;
                let product = self.trait_impls.get_pair_invocation(product, variable(&param_a), inverse_b)?;
                let the_impl = single_expression_pair_trait_impl(name, &param_a, &param_b, product);
                self.trait_impls.add_pair_impl(name, param_a, param_b, the_impl);
            };
        }

        for (param_a, param_b) in registry.pair_parameters() {
            let name = "GeometricAntiQuotient";
            let _: Option<()> = try {
                let inverse_b = self.trait_impls.get_single_invocation("AntiInverse", variable(&param_b))?;
                let product = self.algebra.dialect().geometric_anti_product.first()?;
                let product = self.trait_impls.get_pair_invocation(product, variable(&param_a), inverse_b)?;
                let the_impl = single_expression_pair_trait_impl(name, &param_a, &param_b, product);
                self.trait_impls.add_pair_impl(name, param_a, param_b, the_impl);
            };
        }

        Ok(())
    }

    pub fn dual_num_stuff<'s>(&'s mut self, registry: &'r MultiVectorClassRegistry) -> std::io::Result<()> {
        let dual_num_param = registry.single_parameters().find(|it| it.multi_vector_class().class_name == "DualNum");
        let scalar_param = registry.single_parameters().find(|it| it.multi_vector_class().class_name == "Scalar");
        let anti_scalar_param = registry.single_parameters().find(|it| it.multi_vector_class().class_name == "AntiScalar");
        let (dual_num_param, scalar_param, anti_scalar_param) = match (dual_num_param, scalar_param, anti_scalar_param) {
            (Some(dnp), Some(sp), Some(asp)) => (dnp, sp, asp),
            _ => return Ok(()),
        };

        let float_expression = |a: ExpressionContent<'r>| {
            Box::new(Expression {
                size: 1,
                data_type_hint: Some(DataType::SimdVector(1)),
                content: a,
            })
        };
        let var_param_a = Box::new(variable(&dual_num_param));
        let assign_float_var = |v: &'static str, a: ExpressionContent<'r>| {
            return AstNode::VariableAssignment {
                name: v,
                data_type: Some(DataType::SimdVector(1)),
                expression: float_expression(a)
            }
        };
        let assign_var_s = assign_float_var(
            "s",
            ExpressionContent::Gather(
                var_param_a.clone(),
                vec![GatherData::Usual(UsualGatherData {
                    negate: false,
                    group: 0,
                    element: 0,
                    group_size: 2,
                })]
            ),
        );
        let assign_var_t = assign_float_var(
            "t",
            ExpressionContent::Gather(
                var_param_a.clone(),
                vec![GatherData::Usual(UsualGatherData {
                    negate: false,
                    group: 0,
                    element: 1,
                    group_size: 2,
                })]
            ),
        );

        let access_scalar = ExpressionContent::Access(Box::new(variable(&scalar_param)), 0);
        let access_anti_scalar = ExpressionContent::Access(Box::new(variable(&anti_scalar_param)), 0);

        let mut construct_scalar = |a: ExpressionContent<'r>| {
            return Expression {
                size: 1,
                data_type_hint: Some(scalar_param.data_type.clone()),
                content: ExpressionContent::InvokeClassMethod(
                    scalar_param.multi_vector_class(), "Constructor", vec![
                        (DataType::SimdVector(1), Expression {
                            size: 1,
                            data_type_hint: Some(DataType::SimdVector(1)),
                            content: ExpressionContent::ConstructVec(DataType::SimdVector(1), vec![
                                Expression {
                                    size: 1,
                                    data_type_hint: Some(DataType::SimdVector(1)),
                                    content: a,
                                },
                            ]),
                        }),
                    ]),
            };
        };

        let mut construct_anti_scalar = |a: ExpressionContent<'r>| {
            return Expression {
                size: 1,
                data_type_hint: Some(anti_scalar_param.data_type.clone()),
                content: ExpressionContent::InvokeClassMethod(
                    anti_scalar_param.multi_vector_class(), "Constructor", vec![
                        (DataType::SimdVector(1), Expression {
                            size: 1,
                            data_type_hint: Some(DataType::SimdVector(1)),
                            content: ExpressionContent::ConstructVec(DataType::SimdVector(1), vec![
                                Expression {
                                    size: 1,
                                    data_type_hint: Some(DataType::SimdVector(1)),
                                    content: a,
                                },
                            ]),
                        }),
                    ]),
            };
        };

        let mut construct_dual_num = |a: ExpressionContent<'r>, b: ExpressionContent<'r>| {
            return Expression {
                size: 1,
                data_type_hint: Some(dual_num_param.data_type.clone()),
                content: ExpressionContent::InvokeClassMethod(
                    dual_num_param.multi_vector_class(), "Constructor", vec![
                        (DataType::SimdVector(2), Expression {
                            size: 2,
                            data_type_hint: Some(DataType::SimdVector(2)),
                            content: ExpressionContent::ConstructVec(DataType::SimdVector(2), vec![
                                Expression {
                                    size: 1,
                                    data_type_hint: Some(DataType::SimdVector(1)),
                                    content: a,
                                },
                                Expression {
                                    size: 1,
                                    data_type_hint: Some(DataType::SimdVector(1)),
                                    content: b,
                                },
                            ]),
                        }),
                    ]),
            };
        };

        let mut return_scalar = |a: ExpressionContent<'r>| {
            return AstNode::ReturnStatement {
                expression: Box::new(construct_scalar(a)),
            };
        };
        let mut return_anti_scalar = |a: ExpressionContent<'r>| {
            return AstNode::ReturnStatement {
                expression: Box::new(construct_anti_scalar(a)),
            };
        };
        let mut return_dual_num = |a: ExpressionContent<'r>, b: ExpressionContent<'r>| {
            return AstNode::ReturnStatement {
                expression: Box::new(construct_dual_num(a, b)),
            };
        };

        // this trait impl is only for single argument traits (not power-of-n)
        let mut dual_num_trait_impl = |name: &'static str, more_vars: Vec<AstNode<'r>>, a: ExpressionContent<'r>, b: ExpressionContent<'r>| {
            let mut body = vec![
                assign_var_s.clone(),
                assign_var_t.clone(),
            ];
            for more_var in more_vars {
                body.push(more_var);
            }
            body.push(return_dual_num(a, b));
            let the_impl = AstNode::TraitImplementation {
                result: Parameter { name, data_type: DataType::MultiVector(dual_num_param.multi_vector_class()) },
                class: dual_num_param.multi_vector_class(),
                parameters: vec![dual_num_param.clone()],
                body,
            };
            self.trait_impls.add_single_impl(name, dual_num_param.clone(), the_impl);
        };

        let multiply = |a: ExpressionContent<'r>, b: ExpressionContent<'r>| {
            return ExpressionContent::Multiply(float_expression(a), float_expression(b));
        };

        let add = |a: ExpressionContent<'r>, b: ExpressionContent<'r>| {
            return ExpressionContent::Add(float_expression(a), float_expression(b));
        };

        let sub = |a: ExpressionContent<'r>, b: ExpressionContent<'r>| {
            return ExpressionContent::Subtract(float_expression(a), float_expression(b));
        };

        let divide = |a: ExpressionContent<'r>, b: ExpressionContent<'r>| {
            return ExpressionContent::Divide(float_expression(a), float_expression(b));
        };

        let reciprocal = |a: ExpressionContent<'r>| {
            return divide(ExpressionContent::Constant(DataType::SimdVector(1), vec![1]), a);
        };
        let mul_2 = |a: ExpressionContent<'r>| {
            return multiply(ExpressionContent::Constant(DataType::SimdVector(1), vec![2]), a);
        };
        let one_plus = |a: ExpressionContent<'r>| {
            return add(ExpressionContent::Constant(DataType::SimdVector(1), vec![1]), a);
        };
        let one_minus = |a: ExpressionContent<'r>| {
            return sub(ExpressionContent::Constant(DataType::SimdVector(1), vec![1]), a);
        };
        let minus_one = |a: ExpressionContent<'r>| {
            return sub(a, ExpressionContent::Constant(DataType::SimdVector(1), vec![1]));
        };
        let negate = |a: ExpressionContent<'r>| {
            return multiply(ExpressionContent::Constant(DataType::SimdVector(1), vec![-1]), a);
        };
        let sqrt = |a: ExpressionContent<'r>| {
            return ExpressionContent::SquareRoot(float_expression(a));
        };
        let exp = |a: ExpressionContent<'r>| {
            return ExpressionContent::Exp(float_expression(a));
        };
        let sin = |a: ExpressionContent<'r>| {
            return ExpressionContent::Sin(float_expression(a));
        };
        let cos = |a: ExpressionContent<'r>| {
            return ExpressionContent::Cos(float_expression(a));
        };
        let tan = |a: ExpressionContent<'r>| {
            return ExpressionContent::Tan(float_expression(a));
        };
        let sinh = |a: ExpressionContent<'r>| {
            return ExpressionContent::Sinh(float_expression(a));
        };
        let cosh = |a: ExpressionContent<'r>| {
            return ExpressionContent::Cosh(float_expression(a));
        };
        let tanh = |a: ExpressionContent<'r>| {
            return ExpressionContent::Tanh(float_expression(a));
        };
        let pow = |a: ExpressionContent<'r>, power: ExpressionContent<'r>| {
            return ExpressionContent::Pow(float_expression(a), float_expression(power));
        };

        let var_s = ExpressionContent::Variable("s");
        let var_t = ExpressionContent::Variable("t");
        let var_sqrt_s = ExpressionContent::Variable("sqrt_s");
        let var_sqrt_t = ExpressionContent::Variable("sqrt_t");
        let var_exp_s = ExpressionContent::Variable("exp_s");
        let var_exp_t = ExpressionContent::Variable("exp_t");
        let var_tan_s = ExpressionContent::Variable("tan_s");
        let var_tan_t = ExpressionContent::Variable("tan_t");
        let var_tanh_s = ExpressionContent::Variable("tanh_s");
        let var_tanh_t = ExpressionContent::Variable("tanh_t");

        // I don't like including "Square" and "AntiSquare" because you can just
        // write a.wedge_dot(a) or a.anti_wedge_dot(a) instead, and I really don't think
        // that is worth an entirely new trait. This is especially the case considering you
        // could square literally any other geometric object as well (not just Scalars,
        // AntiScalars, and DualNums), and then it gets incredibly tedious and bloated to
        // include all impls for other objects too.

        // dual_num_trait_impl(
        //     "Square",
        //     multiply(var_s.clone(), var_s.clone()),
        //     mul_2(multiply(var_s.clone(), var_t.clone())),
        // );
        // dual_num_trait_impl(
        //     "AntiSquare",
        //     mul_2(multiply(var_s.clone(), var_t.clone())),
        //     multiply(var_t.clone(), var_t.clone()),
        // );

        dual_num_trait_impl(
            "Inverse",
            vec![],
            reciprocal(var_s.clone()),
            divide(
                negate(var_t.clone()),
                multiply(var_s.clone(), var_s.clone())
            ),
        );
        dual_num_trait_impl(
            "AntiInverse",
            vec![],
            divide(
                negate(var_s.clone()),
                multiply(var_t.clone(), var_t.clone())
            ),
            reciprocal(var_t.clone()),
        );
        dual_num_trait_impl(
            "Sqrt",
            vec![assign_float_var("sqrt_s", sqrt(var_s.clone()))],
            var_sqrt_s.clone(),
            divide(var_t.clone(), mul_2(var_sqrt_s.clone())),
        );
        dual_num_trait_impl(
            "AntiSqrt",
            vec![assign_float_var("sqrt_t", sqrt(var_t.clone()))],
            divide(var_s.clone(), mul_2(var_sqrt_t.clone())),
            var_sqrt_t.clone(),
        );
        dual_num_trait_impl(
            "InverseSqrt",
            vec![assign_float_var("sqrt_s", sqrt(var_s.clone()))],
            reciprocal(var_sqrt_s.clone()),
            divide(negate(var_t.clone()), mul_2(multiply(var_s.clone(), var_sqrt_s.clone()))),
        );
        dual_num_trait_impl(
            "AntiInverseSqrt",
            vec![assign_float_var("sqrt_t", sqrt(var_t.clone()))],
            divide(negate(var_s.clone()), mul_2(multiply(var_t.clone(), var_sqrt_t.clone()))),
            reciprocal(var_sqrt_t.clone()),
        );
        dual_num_trait_impl(
            "Exp",
            vec![assign_float_var("exp_s", exp(var_s.clone()))],
            var_exp_s.clone(),
            multiply(var_t.clone(), var_exp_s.clone()),
        );
        dual_num_trait_impl(
            "AntiExp",
            vec![assign_float_var("exp_t", exp(var_t.clone()))],
            multiply(var_s.clone(), var_exp_t.clone()),
            var_exp_t.clone(),
        );
        dual_num_trait_impl(
            "Sin",
            vec![],
            sin(var_s.clone()),
            multiply(var_t.clone(), cos(var_s.clone())),
        );
        dual_num_trait_impl(
            "AntiSin",
            vec![],
            multiply(var_s.clone(), cos(var_t.clone())),
            sin(var_t.clone()),
        );
        dual_num_trait_impl(
            "Cos",
            vec![],
            cos(var_s.clone()),
            negate(multiply(var_t.clone(), sin(var_s.clone()))),
        );
        dual_num_trait_impl(
            "AntiCos",
            vec![],
            negate(multiply(var_s.clone(), sin(var_t.clone()))),
            cos(var_t.clone()),
        );
        dual_num_trait_impl(
            "Tan",
            vec![assign_float_var("tan_s", tan(var_s.clone()))],
            var_tan_s.clone(),
            multiply(var_t.clone(), one_plus(multiply(var_tan_s.clone(), var_tan_s.clone()))),
        );
        dual_num_trait_impl(
            "AntiTan",
            vec![assign_float_var("tan_t", tan(var_t.clone()))],
            multiply(var_s.clone(), one_plus(multiply(var_tan_t.clone(), var_tan_t.clone()))),
            var_tan_t.clone(),
        );
        dual_num_trait_impl(
            "Sinh",
            vec![],
            sinh(var_s.clone()),
            multiply(var_t.clone(), cosh(var_s.clone())),
        );
        dual_num_trait_impl(
            "AntiSinh",
            vec![],
            multiply(var_s.clone(), cosh(var_t.clone())),
            sinh(var_t.clone()),
        );
        dual_num_trait_impl(
            "Cosh",
            vec![],
            cosh(var_s.clone()),
            multiply(var_t.clone(), sinh(var_s.clone())),
        );
        dual_num_trait_impl(
            "AntiCosh",
            vec![],
            multiply(var_s.clone(), sinh(var_t.clone())),
            cosh(var_t.clone()),
        );
        dual_num_trait_impl(
            "Tanh",
            vec![assign_float_var("tanh_s", tanh(var_s.clone()))],
            var_tanh_s.clone(),
            multiply(var_t.clone(), one_minus(multiply(var_tanh_s.clone(), var_tanh_s.clone()))),
        );
        dual_num_trait_impl(
            "AntiTanh",
            vec![assign_float_var("tanh_t", tanh(var_t.clone()))],
            multiply(var_s.clone(), one_minus(multiply(var_tanh_t.clone(), var_tanh_t.clone()))),
            var_tanh_t.clone(),
        );

        let var_other = ExpressionContent::Variable("other");
        self.trait_impls.add_single_impl("Pow", dual_num_param.clone(), AstNode::TraitImplementation {
            result: Parameter { name: "Pow", data_type: DataType::MultiVector(dual_num_param.multi_vector_class()) },
            class: dual_num_param.multi_vector_class(),
            parameters: vec![dual_num_param.clone(), Parameter {
                name: "other",
                data_type: DataType::SimdVector(1),
            }],
            body: vec![
                assign_var_s.clone(),
                assign_var_t.clone(),
                return_dual_num(
                    pow(var_s.clone(), var_other.clone()),
                    multiply(var_other.clone(), multiply(pow(var_s.clone(), minus_one(var_other.clone())), var_t.clone()))
                )
            ],
        });
        self.trait_impls.add_single_impl("AntiPow", dual_num_param.clone(), AstNode::TraitImplementation {
            result: Parameter { name: "AntiPow", data_type: DataType::MultiVector(dual_num_param.multi_vector_class()) },
            class: dual_num_param.multi_vector_class(),
            parameters: vec![dual_num_param.clone(), Parameter {
                name: "other",
                data_type: DataType::SimdVector(1),
            }],
            body: vec![
                assign_var_s.clone(),
                assign_var_t.clone(),
                return_dual_num(
                    multiply(var_other.clone(), multiply(pow(var_t.clone(), minus_one(var_other.clone())), var_s.clone())),
                    pow(var_t.clone(), var_other.clone()),
                )
            ],
        });


        let mut scalar_trait_impl = |name: &'static str, a: ExpressionContent<'r>| {
            let the_impl = AstNode::TraitImplementation {
                result: Parameter { name, data_type: DataType::MultiVector(scalar_param.multi_vector_class()) },
                class: scalar_param.multi_vector_class(),
                parameters: vec![scalar_param.clone()],
                body: vec![return_scalar(a)],
            };
            self.trait_impls.add_single_impl(name, scalar_param.clone(), the_impl);
        };
        scalar_trait_impl("InverseSqrt", reciprocal(sqrt(access_scalar.clone())));
        scalar_trait_impl("Exp", exp(access_scalar.clone()));
        scalar_trait_impl("Sine", sin(access_scalar.clone()));
        scalar_trait_impl("Cosine", cos(access_scalar.clone()));
        scalar_trait_impl("Tangent", tan(access_scalar.clone()));
        scalar_trait_impl("Sinh", sinh(access_scalar.clone()));
        scalar_trait_impl("Cosh", cosh(access_scalar.clone()));
        scalar_trait_impl("Tanh", tanh(access_scalar.clone()));
        self.trait_impls.add_single_impl("Pow", scalar_param.clone(), AstNode::TraitImplementation {
            result: Parameter { name: "Pow", data_type: DataType::MultiVector(scalar_param.multi_vector_class()) },
            class: scalar_param.multi_vector_class(),
            parameters: vec![scalar_param.clone(), Parameter {
                name: "other",
                data_type: DataType::SimdVector(1),
            }],
            body: vec![return_scalar(pow(access_scalar.clone(), var_other.clone()))],
        });


        let mut anti_scalar_trait_impl = |name: &'static str, a: ExpressionContent<'r>| {
            let the_impl = AstNode::TraitImplementation {
                result: Parameter { name, data_type: DataType::MultiVector(anti_scalar_param.multi_vector_class()) },
                class: anti_scalar_param.multi_vector_class(),
                parameters: vec![anti_scalar_param.clone()],
                body: vec![return_anti_scalar(a)],
            };
            self.trait_impls.add_single_impl(name, anti_scalar_param.clone(), the_impl);
        };
        anti_scalar_trait_impl("AntiInverseSqrt", reciprocal(sqrt(access_anti_scalar.clone())));
        anti_scalar_trait_impl("AntiExp", exp(access_anti_scalar.clone()));
        anti_scalar_trait_impl("AntiSine", sin(access_anti_scalar.clone()));
        anti_scalar_trait_impl("AntiCosine", cos(access_anti_scalar.clone()));
        anti_scalar_trait_impl("AntiTangent", tan(access_anti_scalar.clone()));
        anti_scalar_trait_impl("AntiSinh", sinh(access_anti_scalar.clone()));
        anti_scalar_trait_impl("AntiCosh", cosh(access_anti_scalar.clone()));
        anti_scalar_trait_impl("AntiTanh", tanh(access_anti_scalar.clone()));
        self.trait_impls.add_single_impl("AntiPow", anti_scalar_param.clone(), AstNode::TraitImplementation {
            result: Parameter { name: "AntiPow", data_type: DataType::MultiVector(anti_scalar_param.multi_vector_class()) },
            class: anti_scalar_param.multi_vector_class(),
            parameters: vec![anti_scalar_param.clone(), Parameter {
                name: "other",
                data_type: DataType::SimdVector(1),
            }],
            body: vec![return_anti_scalar(pow(access_anti_scalar.clone(), var_other.clone()))],
        });

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
                let anti_dot = self.algebra.dialect().anti_dot_product.first()?;
                let anti_dot = self.trait_impls.get_pair_invocation(anti_dot, variable(&param_a), variable(&param_a))?;
                let m = self.trait_impls.get_single_invocation("AntiSqrt", anti_dot)?;
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
    pub fn fancy_norms(&mut self, registry: &'r MultiVectorClassRegistry) {

        // TODO here
        // TODO hardly any of these are getting emitted

        // TODO see page 197 of the book (and onward) for formulas to check against



        for param_a in registry.single_parameters() {

            // TODO I broke things in cga3d_min, so I should do this next.

            // TODO according to page 204-205, the conformal conjugate might be useful in defining
            //  the distance between the origin and the center position

            let center = match self.trait_impls.get_single_invocation("Center", variable(&param_a)) {
                None => continue,
                Some(c) => c
            };
            let d = self.algebra.represented_dimensions();
            let projective_basis = BasisElement::from_index(1 << d as BasisElementIndex);

            let center_bulk_norm_squared = "CenterBulkNormSquared";
            let center_bulk_norm = "CenterBulkNorm";
            let center_weight_norm_squared = "CenterWeightNormSquared";
            let center_weight_norm = "CenterWeightNorm";
            let center_geometric_norm = "CenterGeometricNorm";
            let center_unitized_norm_squared = "CenterUnitizedNormSquared";
            let center_unitized_norm = "CenterUnitizedNorm";

            let radius_bulk_norm_squared = "RadiusBulkNormSquared";
            let radius_bulk_norm = "RadiusBulkNorm";
            let radius_weight_norm_squared = "RadiusWeightNormSquared";
            let radius_weight_norm = "RadiusWeightNorm";
            let radius_geometric_norm = "RadiusGeometricNorm";
            let radius_unitized_norm_squared = "RadiusUnitizedNormSquared";
            let radius_unitized_norm = "RadiusUnitizedNorm";

            let _: Option<()> = try {
                let center = self.trait_impls.get_single_invocation("Center", variable(&param_a))?;
                let center_data_type = center.data_type_hint.clone()?;
                let center_mv = match center_data_type {
                    DataType::MultiVector(mv) => mv,
                    _ => continue,
                };
                if center_mv.class_name != "RoundPoint" {
                    continue
                }


                // Center Bulk Norm Squared

                let round_bulk = self.trait_impls.get_single_invocation("RoundBulk", center.clone())?;
                let round_bulk_datatype = round_bulk.data_type_hint.clone()?;
                let var_round_bulk = Expression {
                    size: round_bulk.size.clone(),
                    data_type_hint: Some(round_bulk_datatype.clone()),
                    content: ExpressionContent::Variable("round_bulk"),
                };
                let assign_round_bulk = AstNode::VariableAssignment {
                    name: "round_bulk",
                    data_type: Some(round_bulk_datatype),
                    expression: Box::new(round_bulk),
                };
                let dot = self.algebra.dialect().dot_product.first()?;
                let rb_dot_rb = self.trait_impls.get_pair_invocation(dot, var_round_bulk.clone(), var_round_bulk)?;
                let dot_datatype = rb_dot_rb.data_type_hint.clone()?;

                let the_return = AstNode::ReturnStatement {
                    expression: Box::new(rb_dot_rb.clone()),
                };
                let the_impl = AstNode::TraitImplementation {
                    result: Parameter { name: center_bulk_norm_squared, data_type: dot_datatype.clone() },
                    class: param_a.multi_vector_class(),
                    parameters: vec![param_a.clone()],
                    body: vec![assign_round_bulk.clone(), the_return],
                };
                self.trait_impls.add_single_impl(center_bulk_norm_squared, param_a.clone(), the_impl);


                // Center Bulk Norm

                let bns = self.trait_impls.get_single_invocation(center_bulk_norm_squared, variable(&param_a))?;
                let sqrt = self.trait_impls.get_single_invocation("Sqrt", bns)?;
                let the_impl = single_expression_single_trait_impl(center_bulk_norm, &param_a, sqrt);
                self.trait_impls.add_single_impl(center_bulk_norm, param_a.clone(), the_impl);


                // Center Weight Norm Squared

                let round_weight = self.trait_impls.get_single_invocation("RoundWeight", center.clone())?;
                let round_weight_datatype = round_weight.data_type_hint.clone()?;
                let var_round_weight = Expression {
                    size: round_weight.size.clone(),
                    data_type_hint: Some(round_weight_datatype.clone()),
                    content: ExpressionContent::Variable("round_weight"),
                };
                let assign_round_weight = AstNode::VariableAssignment {
                    name: "round_weight",
                    data_type: Some(round_weight_datatype),
                    expression: Box::new(round_weight),
                };
                let anti_dot = self.algebra.dialect().anti_dot_product.first()?;
                // TODO cannot find Origin anti_dot Origin
                // TODO IT'S TRUE.... ORIGIN ANTIDOT ORIGIN IS ZERO....
                //  So I need to reformulate these round features entirely...
                let rw_anti_dot_rw = self.trait_impls.get_pair_invocation(anti_dot, var_round_weight.clone(), var_round_weight)?;
                let anti_dot_datatype = rw_anti_dot_rw.data_type_hint.clone()?;

                let the_return = AstNode::ReturnStatement {
                    expression: Box::new(rw_anti_dot_rw.clone()),
                };
                let the_impl = AstNode::TraitImplementation {
                    result: Parameter { name: center_weight_norm_squared, data_type: anti_dot_datatype.clone() },
                    class: param_a.multi_vector_class(),
                    parameters: vec![param_a.clone()],
                    body: vec![assign_round_weight.clone(), the_return],
                };
                self.trait_impls.add_single_impl(center_weight_norm_squared, param_a.clone(), the_impl);


                // Center Weight Norm

                let bns = self.trait_impls.get_single_invocation(center_weight_norm_squared, variable(&param_a))?;
                let sqrt = self.trait_impls.get_single_invocation("AntiSqrt", bns)?;
                let the_impl = single_expression_single_trait_impl(center_weight_norm, &param_a, sqrt);
                self.trait_impls.add_single_impl(center_weight_norm, param_a.clone(), the_impl);


                // Center Geometric Norm

                let bn = self.trait_impls.get_single_invocation(center_bulk_norm, variable(&param_a))?;
                let wn = self.trait_impls.get_single_invocation(center_weight_norm, variable(&param_a))?;
                let add = self.trait_impls.get_pair_invocation("Add", bn, wn)?;
                let the_impl = single_expression_single_trait_impl(center_geometric_norm, &param_a, add);
                self.trait_impls.add_single_impl(center_geometric_norm, param_a.clone(), the_impl);


                // Center Unitized Norm Squared

                let bn = self.trait_impls.get_single_invocation(center_bulk_norm_squared, variable(&param_a))?;
                let wn = self.trait_impls.get_single_invocation(center_weight_norm_squared, variable(&param_a))?;
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
                let the_impl = single_expression_single_trait_impl(center_unitized_norm_squared, &param_a, div);
                self.trait_impls.add_single_impl(center_unitized_norm_squared, param_a.clone(), the_impl);


                // Center Unitized Norm

                let uns = self.trait_impls.get_single_invocation(center_unitized_norm_squared, variable(&param_a))?;
                let sqrt = Expression {
                    size: 1,
                    data_type_hint: Some(DataType::SimdVector(1)),
                    content: ExpressionContent::SquareRoot(Box::new(uns)),
                };
                let un = single_expression_single_trait_impl(center_unitized_norm, &param_a, sqrt);
                self.trait_impls.add_single_impl(center_unitized_norm, param_a.clone(), un);


                // Radius Bulk Norm Squared


                let center_data_type = center.data_type_hint.clone()?;
                let var_center = Expression {
                    size: center.size.clone(),
                    data_type_hint: Some(center_data_type.clone()),
                    content: ExpressionContent::Variable("center"),
                };
                let assign_center = AstNode::VariableAssignment {
                    name: "center",
                    data_type: Some(center_data_type.clone()),
                    expression: Box::new(center.clone())
                };

                let round_bulk = self.trait_impls.get_single_invocation("RoundBulk", center.clone())?;
                let round_bulk_datatype = round_bulk.data_type_hint.clone()?;
                let var_round_bulk = Expression {
                    size: round_bulk.size.clone(),
                    data_type_hint: Some(round_bulk_datatype.clone()),
                    content: ExpressionContent::Variable("round_bulk"),
                };
                let assign_round_bulk = AstNode::VariableAssignment {
                    name: "round_bulk",
                    data_type: Some(round_bulk_datatype),
                    expression: Box::new(round_bulk),
                };
                let dot = self.algebra.dialect().dot_product.first()?;
                let rb_dot_rb = self.trait_impls.get_pair_invocation(dot, var_round_bulk.clone(), var_round_bulk)?;
                let dot_data_type = rb_dot_rb.data_type_hint.clone()?;
                // formula calls for "2 * aw * au"
                // but when we invoke "center", the e5 component becomes "aw * au" instead of just "au"
                // so by taking the "flat bulk" of the center result, we get the coefficient we need
                // Just have to multiply it by 2. Or add it with itself.
                // Ah... except the flat bulk is not a scalar... it is an e5... hmmm....
                // Okay so yeah I have to manually dig into the insides of the thing either way.
                let flat_bulk = self.trait_impls.get_single_invocation("Bulk", var_center.clone())?;
                let scalar_mv = match rb_dot_rb.data_type_hint {
                    Some(DataType::MultiVector(mv)) => mv,
                    _ => None?
                };
                let e5_element = Expression {
                    size: 1,
                    data_type_hint: Some(DataType::SimdVector(1)),
                    content: ExpressionContent::Access(Box::new(flat_bulk), 0)
                };
                let two = Expression {
                    size: 1,
                    data_type_hint: Some(DataType::SimdVector(1)),
                    content: ExpressionContent::Constant(DataType::SimdVector(1), vec![2])
                };
                let two_aw_au = Expression {
                    size: 1,
                    data_type_hint: Some(DataType::SimdVector(1)),
                    content: ExpressionContent::Multiply(Box::new(two), Box::new(e5_element)),
                };
                let two_aw_au_scalar = Expression {
                    size: 1,
                    data_type_hint: Some(dot_data_type.clone()),
                    content: ExpressionContent::InvokeClassMethod(scalar_mv, "Constructor", vec![(DataType::SimdVector(1), two_aw_au)]),
                };
                let var_taas = Expression {
                    size: 1,
                    data_type_hint: Some(dot_data_type.clone()),
                    content: ExpressionContent::Variable("two_aw_au"),
                };
                let assign_taas = AstNode::VariableAssignment {
                    name: "two_aw_au",
                    data_type: Some(dot_data_type.clone()),
                    expression: Box::new(two_aw_au_scalar),
                };
                let sub = self.trait_impls.get_pair_invocation("Sub", var_taas, rb_dot_rb)?;

                let the_return = AstNode::ReturnStatement {
                    expression: Box::new(sub.clone()),
                };
                let the_impl = AstNode::TraitImplementation {
                    result: Parameter { name: radius_bulk_norm_squared, data_type: DataType::MultiVector(scalar_mv) },
                    class: param_a.multi_vector_class(),
                    parameters: vec![param_a.clone()],
                    body: vec![assign_center.clone(), assign_round_bulk, assign_taas, the_return],
                };
                self.trait_impls.add_single_impl(radius_bulk_norm_squared, param_a.clone(), the_impl);


                // Radius Bulk Norm

                let bns = self.trait_impls.get_single_invocation(radius_bulk_norm_squared, variable(&param_a))?;
                let sqrt = self.trait_impls.get_single_invocation("Sqrt", bns)?;
                let the_impl = single_expression_single_trait_impl(radius_bulk_norm, &param_a, sqrt);
                self.trait_impls.add_single_impl(radius_bulk_norm, param_a.clone(), the_impl);


                // Radius Weight Norm Squared

                let round_weight = self.trait_impls.get_single_invocation("RoundWeight", center.clone())?;
                let round_weight_datatype = round_weight.data_type_hint.clone()?;
                let var_round_weight = Expression {
                    size: round_weight.size.clone(),
                    data_type_hint: Some(round_weight_datatype.clone()),
                    content: ExpressionContent::Variable("round_weight"),
                };
                let assign_round_weight = AstNode::VariableAssignment {
                    name: "round_weight",
                    data_type: Some(round_weight_datatype),
                    expression: Box::new(round_weight),
                };
                let anti_dot = self.algebra.dialect().anti_dot_product.first()?;
                let rw_anti_dot_rw = self.trait_impls.get_pair_invocation(anti_dot, var_round_weight.clone(), var_round_weight)?;
                let anti_dot_datatype = rw_anti_dot_rw.data_type_hint.clone()?;

                let the_return = AstNode::ReturnStatement {
                    expression: Box::new(rw_anti_dot_rw.clone()),
                };
                let the_impl = AstNode::TraitImplementation {
                    result: Parameter { name: radius_weight_norm_squared, data_type: anti_dot_datatype.clone() },
                    class: param_a.multi_vector_class(),
                    parameters: vec![param_a.clone()],
                    body: vec![assign_round_weight.clone(), the_return],
                };
                self.trait_impls.add_single_impl(radius_weight_norm_squared, param_a.clone(), the_impl);


                // Radius Weight Norm

                let wns = self.trait_impls.get_single_invocation(radius_weight_norm_squared, variable(&param_a))?;
                let sqrt = self.trait_impls.get_single_invocation("AntiSqrt", wns)?;
                let the_impl = single_expression_single_trait_impl(radius_weight_norm, &param_a, sqrt);
                self.trait_impls.add_single_impl(radius_weight_norm, param_a.clone(), the_impl);


                // Radius Geometric Norm

                let bn = self.trait_impls.get_single_invocation(radius_bulk_norm, variable(&param_a))?;
                let wn = self.trait_impls.get_single_invocation(radius_weight_norm, variable(&param_a))?;
                let add = self.trait_impls.get_pair_invocation("Add", bn, wn)?;
                let the_impl = single_expression_single_trait_impl(radius_geometric_norm, &param_a, add);
                self.trait_impls.add_single_impl(radius_geometric_norm, param_a.clone(), the_impl);


                // Radius Unitized Norm Squared

                let bn = self.trait_impls.get_single_invocation(radius_bulk_norm_squared, variable(&param_a))?;
                let wn = self.trait_impls.get_single_invocation(radius_weight_norm_squared, variable(&param_a))?;
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
                let the_impl = single_expression_single_trait_impl(radius_unitized_norm_squared, &param_a, div);
                self.trait_impls.add_single_impl(radius_unitized_norm_squared, param_a.clone(), the_impl);


                // Radius Unitized Norm

                let uns = self.trait_impls.get_single_invocation(radius_unitized_norm_squared, variable(&param_a))?;
                let sqrt = Expression {
                    size: 1,
                    data_type_hint: Some(DataType::SimdVector(1)),
                    content: ExpressionContent::SquareRoot(Box::new(uns)),
                };
                let un = single_expression_single_trait_impl(radius_unitized_norm, &param_a, sqrt);
                self.trait_impls.add_single_impl(radius_unitized_norm, param_a.clone(), un);
            };
        }
    }

    /// Step 4: Create some more stuff that depends on norms
    pub fn post_norm_universal_stuff<'s>(
        &'s mut self, registry: &'r MultiVectorClassRegistry,
        sandwich_outputs: &BTreeMap<(&str, &str), &str>,
    ) {

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

        // TODO reciprocal sandwiches (rotation, translation, reflection)
        //  https://rigidgeometricalgebra.org/wiki/index.php?title=Reciprocal_rotation
        //  https://rigidgeometricalgebra.org/wiki/index.php?title=Reciprocal_translation
        //  https://rigidgeometricalgebra.org/wiki/index.php?title=Reciprocal_reflection
        // let allowed_to_sandwich = ["Motor", "Translator", "Rotor", "Flector", "Point", "Plane"];
        let disallowed_to_be_sandwiched = ["Scalar", "AntiScalar", "DualNum"];
        for (param_a, param_b) in registry.pair_parameters() {
            // if !allowed_to_sandwich.contains(&param_a.multi_vector_class().class_name.as_str()) {
            //     continue
            // }
            let class_a = param_a.multi_vector_class();
            let class_b = param_b.multi_vector_class();
            let class_b_name = class_b.class_name.as_str();
            let class_a_name = class_a.class_name.as_str();
            if disallowed_to_be_sandwiched.contains(&class_b_name) {
                continue;
            }
            let _: Option<()> = try {
                let gap = self.algebra.dialect().geometric_anti_product.first()?;
                let (gp, gp_r) = self.trait_impls.get_pair_impl_and_result(gap, &param_a, &param_b)?;
                let (reversal, reversal_r) = self.trait_impls.get_single_impl_and_result("AntiReversal", &param_a)?;
                let (gp2, gp2_r) = self.trait_impls.get_pair_impl_and_result(gap, &gp_r, &reversal_r)?;

                let result_class = match sandwich_outputs.get(&(class_a_name, class_b_name)) {
                    Some(rc) => registry.classes.iter().find(|it| it.class_name.as_str() == *rc).unwrap_or_else(|| class_b),
                    None => class_b,
                };
                let result_param = Parameter {
                    name: "other",
                    data_type: DataType::MultiVector(result_class),
                };
                let into = self.trait_impls.get_pair_impl("Into", &gp2_r, &result_param);

                let sandwich = derive_sandwich_product("Sandwich", gp, gp2, reversal, into, &param_a, &param_b);
                self.trait_impls.add_pair_impl("Sandwich", param_a, param_b, sandwich);
            };
        }

        // TODO "Sphere Inversion" is a special type of sandwich using spheres (generalizing
        //  reflection across planes, and/or motors). When the spheres are concentric, this
        //  is called dilation. See pages 239 and 244
        // TODO then there is "Circle rotation" which is like instead of rotating around a line
        //  (or reflecting across 2 intersecting planes, rather) you can rotate around a circle
        //  (or spherical invert across 2 intersecting spheres, rather) and get a magnetic coil
        //  field looking transformation. (page 242, 243)
        //
        for (param_a, param_b) in registry.pair_parameters() {
            // Point Inversion
            // https://rigidgeometricalgebra.org/wiki/index.php?title=Inversion
            let n = param_a.multi_vector_class().class_name.as_str();
            if n != "Point" && n != "FlatPoint" {
                continue;
            }
            let name = "PointInversion";
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

        for (param_a, param_b) in registry.pair_parameters() {
            // Transflection
            // https://rigidgeometricalgebra.org/wiki/index.php?title=Transflection
            if param_a.multi_vector_class().class_name != "Transflector" {
                continue;
            }
            let name = "Transflect";
            let _: Option<()> = try {
                let sandwich = self.trait_impls.get_pair_invocation("Sandwich", variable(&param_a), variable(&param_b))?;
                let i = single_expression_pair_trait_impl(name, &param_a, &param_b, sandwich);
                self.trait_impls.add_pair_impl(name, param_a, param_b, i);
            };
        }

        for (param_a, param_b) in registry.pair_parameters() {
            // Translation
            // https://rigidgeometricalgebra.org/wiki/index.php?title=Translation
            if param_a.multi_vector_class().class_name != "Translator" {
                continue;
            }
            let name = "Translate";
            let _: Option<()> = try {
                let sandwich = self.trait_impls.get_pair_invocation("Sandwich", variable(&param_a), variable(&param_b))?;
                let i = single_expression_pair_trait_impl(name, &param_a, &param_b, sandwich);
                self.trait_impls.add_pair_impl(name, param_a, param_b, i);
            };
        }

        for (param_a, param_b) in registry.pair_parameters() {
            // Rotation
            // https://rigidgeometricalgebra.org/wiki/index.php?title=Rotation
            if param_a.multi_vector_class().class_name != "Rotor" {
                continue;
            }
            let name = "Rotate";
            let _: Option<()> = try {
                let sandwich = self.trait_impls.get_pair_invocation("Sandwich", variable(&param_a), variable(&param_b))?;
                let i = single_expression_pair_trait_impl(name, &param_a, &param_b, sandwich);
                self.trait_impls.add_pair_impl(name, param_a, param_b, i);
            };
        }

        let projective_basis = {
            let e4_idx = self.algebra.represented_dimensions() + 1;
            self.algebra.parse(format!("e{e4_idx}").as_str())
        };
        let flat_basis = if self.algebra.algebra_name().contains("cga") {
            let e5_idx = self.algebra.represented_dimensions() + 2;
            Some(self.algebra.parse(format!("e{e5_idx}").as_str()))
        } else {
            None
        };
        for param_a in registry.single_parameters() {
            let bulk = derive_bulk_or_weight(&self.algebra, "Bulk", &param_a, &projective_basis, false, flat_basis.clone(), true, registry);
            if bulk != AstNode::None {
                self.trait_impls.add_single_impl("Bulk", param_a.clone(), bulk);
            }

            let weight = derive_bulk_or_weight(&self.algebra, "Weight", &param_a, &projective_basis, true, flat_basis.clone(), true, registry);
            if weight != AstNode::None {
                self.trait_impls.add_single_impl("Weight", param_a.clone(), weight);
            }

            if self.algebra.algebra_name().contains("cga") {

                let round_bulk = derive_bulk_or_weight(&self.algebra, "RoundBulk", &param_a, &projective_basis, false, flat_basis.clone(), false, registry);
                if round_bulk != AstNode::None {
                    self.trait_impls.add_single_impl("RoundBulk", param_a.clone(), round_bulk);
                }

                let round_weight = derive_bulk_or_weight(&self.algebra, "RoundWeight", &param_a, &projective_basis, true, flat_basis.clone(), false, registry);
                if round_weight != AstNode::None {
                    self.trait_impls.add_single_impl("RoundWeight", param_a, round_weight);
                }
            }
        }

        // This has an almost annoying amount of nuance, but it really helps cut down on junk
        // trait definitions. See thread: https://twitter.com/EricLengyel/status/1775663934424654262
        let is_cga = self.algebra.algebra_name().contains("cga");
        let multiple_complements = self.algebra.has_multiple_complements();
        let aspects = ["Bulk", "Weight"];
        let complements = if multiple_complements { vec!["Right", "Left"] } else { vec![""] };
        let round_aspects = if is_cga { vec!["", "Round"] } else { vec![""] };
        let static_names = [
            "BulkDual",
            "RightBulkDual",
            "LeftBulkDual",
            "RoundBulkDual",
            "RightRoundBulkDual",
            "LeftRoundBulkDual",
            "WeightDual",
            "RightWeightDual",
            "LeftWeightDual",
            "RoundWeightDual",
            "RightRoundWeightDual",
            "LeftRoundWeightDual",
        ];

        for round_aspect in round_aspects {
            for leftOrRight in complements.iter() {
                for bulkOrWeight in aspects.iter() {
                    let name = format!("{leftOrRight}{round_aspect}{bulkOrWeight}Dual");
                    let static_name = static_names.iter().find(|it| **it == name.as_str())
                        .expect("You need to manually write out static names for this, it's not worth over-engineering the lifetimes here");
                    let complement = format!("{leftOrRight}Complement");
                    for param_a in registry.single_parameters() {
                        let _: Option<()> = try {
                            let aspect = self.trait_impls.get_single_invocation(bulkOrWeight, variable(&param_a))?;
                            let comp = self.trait_impls.get_single_invocation(complement.as_str(), aspect)?;
                            let the_impl = single_expression_single_trait_impl(static_name, &param_a, comp);
                            self.trait_impls.add_single_impl(static_name, param_a, the_impl);
                        };
                    }
                }
            }
        }

        // We can end up with some very strange expansions, contractions, and projections
        // if we don't manually exclude these at some point.
        let non_objects = ["Scalar", "AntiScalar", "DualNum"];


        // See book for lots of elaboration on expansion/contraction
        // (but no mention of contraction in CGA for some reason?)
        // In particular, you could construct left vs right variants of these interior products,
        // but we don't really need them.
        let contraction_expansion_stuff =
            if self.algebra.is_degenerate() {
                vec![
                    ("BulkContraction", "Dual", "AntiWedge"),
                    ("WeightContraction", "AntiDual", "AntiWedge"),
                    ("BulkExpansion", "Dual", "Wedge"),
                    ("WeightExpansion", "AntiDual", "Wedge"),
                ]
            } else {
                // Question... how do we know to use Dual or AntiDual in these?
                // Well both Dual and AntiDual are defined in terms of RightComplement.
                // But as mentioned in previous comment and chapter 2.13 of book (page 85),
                // we are only going to bother with right interior products and ignore left
                // interior products. So self.algebra.has_multiple_complements() is not a concern.
                // So any other differences of concern between Dual and AntiDual?
                // Well in a degenerate metric, one corresponds to Bulk, and other corresponds
                // to bulk. But we already checked for that above.
                // We split up bulk/weight variants of contractions/expansions because
                // Dual is (Right)BulkDual, and AntiDual is (Right)WeightDual (regardless of
                // Left or Right shenanigans).
                // So... all that said... if the metric is not degenerate, then Dual and AntiDual
                // are practically the same, just negatives of one another. So we could use either
                // Dual or AntiDual here in the non-degenerate side.
                // So pertaining to Expansion, AntiDual is chosen to "appear consistent with RGA
                // weight expansion" apparently. But there's no mention of contraction, so who
                // knows (or cares?) for that.
                vec![
                    ("Contraction", "AntiDual", "AntiWedge"),
                    ("Expansion", "AntiDual", "Wedge"),
                ]
            };
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


        for (param_a, param_b) in registry.pair_parameters() {
            let a_name = param_a.multi_vector_class().class_name.as_str();
            let b_name = param_b.multi_vector_class().class_name.as_str();
            if non_objects.contains(&a_name) || non_objects.contains(&b_name) {
                continue;
            }
            let name = "ProjectOrthogonallyOnto";
            let _: Option<()> = try {
                let wedge = self.algebra.dialect().exterior_product.first()?;
                let anti_wedge = self.algebra.dialect().exterior_anti_product.first()?;
                let anti_dual = self.trait_impls.get_single_invocation("AntiDual", variable(&param_b))?;
                let we = self.trait_impls.get_pair_invocation(wedge, variable(&param_a), anti_dual)?;
                let anti_wedge = self.trait_impls.get_pair_invocation(anti_wedge, variable(&param_b), we)?;
                let po = single_expression_pair_trait_impl(name, &param_a, &param_b, anti_wedge);
                self.trait_impls.add_pair_impl(name, param_a, param_b, po);
            };
        }
        for (param_a, param_b) in registry.pair_parameters() {
            let a_name = param_a.multi_vector_class().class_name.as_str();
            let b_name = param_b.multi_vector_class().class_name.as_str();
            if non_objects.contains(&a_name) || non_objects.contains(&b_name) {
                continue;
            }
            let name = "AntiProjectOrthogonallyOnto";
            let _: Option<()> = try {
                let wedge = self.algebra.dialect().exterior_product.first()?;
                let anti_wedge = self.algebra.dialect().exterior_anti_product.first()?;
                let anti_dual = self.trait_impls.get_single_invocation("AntiDual", variable(&param_b))?;
                let wc = self.trait_impls.get_pair_invocation(anti_wedge, variable(&param_a), anti_dual)?;
                let wedge = self.trait_impls.get_pair_invocation(wedge, variable(&param_b), wc)?;
                let apo = single_expression_pair_trait_impl(name, &param_a, &param_b, wedge);
                self.trait_impls.add_pair_impl(name, param_a, param_b, apo);
            };
        }
        for (param_a, param_b) in registry.pair_parameters() {
            let a_name = param_a.multi_vector_class().class_name.as_str();
            let b_name = param_b.multi_vector_class().class_name.as_str();
            if non_objects.contains(&a_name) || non_objects.contains(&b_name) {
                continue;
            }
            let name = "ProjectViaOriginOnto";
            let _: Option<()> = try {
                let wedge = self.algebra.dialect().exterior_product.first()?;
                let anti_wedge = self.algebra.dialect().exterior_anti_product.first()?;
                let dual = self.trait_impls.get_single_invocation("Dual", variable(&param_b))?;
                let be = self.trait_impls.get_pair_invocation(wedge, variable(&param_a), dual)?;
                let anti_wedge = self.trait_impls.get_pair_invocation(anti_wedge, variable(&param_b), be)?;
                let po = single_expression_pair_trait_impl(name, &param_a, &param_b, anti_wedge);
                self.trait_impls.add_pair_impl(name, param_a, param_b, po);
            };
        }
        // anti_project_via_horizon might seem weird at first...
        //  line_at_origin.anti_project_via_horizon_onto(point_at_infinity) = line_at_origin
        //  plane.anti_project_via_horizon_onto(point_at_infinity) = plane
        //  ...
        //  but it is confirmed on page 104 of the book.
        //  "The utility of central antiprojection is questionable, but it is included for
        //  completeness. It tends to reorient the object `a` being antiprojected so that it
        //  contains the object `b` instead of moving it in a direction perpendicular to `b`."
        for (param_a, param_b) in registry.pair_parameters() {
            let a_name = param_a.multi_vector_class().class_name.as_str();
            let b_name = param_b.multi_vector_class().class_name.as_str();
            if non_objects.contains(&a_name) || non_objects.contains(&b_name) {
                continue;
            }
            let name = "AntiProjectViaHorizonOnto";
            let _: Option<()> = try {
                let wedge = self.algebra.dialect().exterior_product.first()?;
                let anti_wedge = self.algebra.dialect().exterior_anti_product.first()?;
                let dual = self.trait_impls.get_single_invocation("Dual", variable(&param_b))?;
                let bc = self.trait_impls.get_pair_invocation(anti_wedge, variable(&param_a), dual)?;
                let wedge = self.trait_impls.get_pair_invocation(wedge, variable(&param_b), bc)?;
                let apo = single_expression_pair_trait_impl(name, &param_a, &param_b, wedge);
                self.trait_impls.add_pair_impl(name, param_a, param_b, apo);
            };
        }

        // TODO figure out what the heck these rejections are

        for (param_a, param_b) in registry.pair_parameters() {
            let a_name = param_a.multi_vector_class().class_name.as_str();
            let b_name = param_b.multi_vector_class().class_name.as_str();
            if non_objects.contains(&a_name) || non_objects.contains(&b_name) {
                continue;
            }
            let name = "RejectOrthogonallyFrom";
            let _: Option<()> = try {
                let wedge = self.algebra.dialect().exterior_product.first()?;
                let anti_wedge = self.algebra.dialect().exterior_anti_product.first()?;
                let anti_dual = self.trait_impls.get_single_invocation("AntiDual", variable(&param_b))?;
                let anti_wedge = self.trait_impls.get_pair_invocation(anti_wedge, variable(&param_a), variable(&param_b))?;
                let we = self.trait_impls.get_pair_invocation(wedge, anti_wedge, anti_dual)?;
                let po = single_expression_pair_trait_impl(name, &param_a, &param_b, we);
                self.trait_impls.add_pair_impl(name, param_a, param_b, po);
            };
        }

        for (param_a, param_b) in registry.pair_parameters() {
            let a_name = param_a.multi_vector_class().class_name.as_str();
            let b_name = param_b.multi_vector_class().class_name.as_str();
            if non_objects.contains(&a_name) || non_objects.contains(&b_name) {
                continue;
            }
            let name = "AntiRejectOrthogonallyFrom";
            let _: Option<()> = try {
                let wedge = self.algebra.dialect().exterior_product.first()?;
                let anti_wedge = self.algebra.dialect().exterior_anti_product.first()?;
                let anti_dual = self.trait_impls.get_single_invocation("AntiDual", variable(&param_b))?;
                let wedge = self.trait_impls.get_pair_invocation(wedge, variable(&param_a), variable(&param_b))?;
                let wc = self.trait_impls.get_pair_invocation(anti_wedge, wedge, anti_dual)?;
                let apo = single_expression_pair_trait_impl(name, &param_a, &param_b, wc);
                self.trait_impls.add_pair_impl(name, param_a, param_b, apo);
            };
        }

        for (param_a, param_b) in registry.pair_parameters() {
            let a_name = param_a.multi_vector_class().class_name.as_str();
            let b_name = param_b.multi_vector_class().class_name.as_str();
            if non_objects.contains(&a_name) || non_objects.contains(&b_name) {
                continue;
            }
            let name = "RejectViaOriginFrom";
            let _: Option<()> = try {
                let wedge = self.algebra.dialect().exterior_product.first()?;
                let anti_wedge = self.algebra.dialect().exterior_anti_product.first()?;
                let dual = self.trait_impls.get_single_invocation("Dual", variable(&param_b))?;
                let anti_wedge = self.trait_impls.get_pair_invocation(anti_wedge, variable(&param_a), variable(&param_b))?;
                let be = self.trait_impls.get_pair_invocation(wedge, anti_wedge, dual)?;
                let po = single_expression_pair_trait_impl(name, &param_a, &param_b, be);
                self.trait_impls.add_pair_impl(name, param_a, param_b, po);
            };
        }

        for (param_a, param_b) in registry.pair_parameters() {
            let a_name = param_a.multi_vector_class().class_name.as_str();
            let b_name = param_b.multi_vector_class().class_name.as_str();
            if non_objects.contains(&a_name) || non_objects.contains(&b_name) {
                continue;
            }
            let name = "AntiRejectViaHorizonFrom";
            let _: Option<()> = try {
                let wedge = self.algebra.dialect().exterior_product.first()?;
                let anti_wedge = self.algebra.dialect().exterior_anti_product.first()?;
                let dual = self.trait_impls.get_single_invocation("Dual", variable(&param_b))?;
                let wedge = self.trait_impls.get_pair_invocation(wedge, variable(&param_a), variable(&param_b))?;
                let bc = self.trait_impls.get_pair_invocation(anti_wedge, wedge, dual)?;
                let apo = single_expression_pair_trait_impl(name, &param_a, &param_b, bc);
                self.trait_impls.add_pair_impl(name, param_a, param_b, apo);
            };
        }

        let origin_idx = (1 as BasisElementIndex) << self.algebra.represented_dimensions();
        let origin = registry.get_at_least(&[origin_idx]);
        let origin = match origin { None => None, Some(origin) => {
            if origin.flat_basis().len() == 1 {
                self.trait_impls.get_class_invocation("Unit", origin)
            } else {
                let mut body = vec![];
                for result_group in origin.grouped_basis.iter() {
                    let size = result_group.len();
                    let expression = Expression {
                        size,
                        content: ExpressionContent::Constant(
                            DataType::SimdVector(size),
                            result_group.iter().map(|element| if element.index == origin_idx { 1 } else { 0 }).collect(),
                        ),
                        data_type_hint: Some(DataType::SimdVector(size)),
                    };
                    body.push((DataType::SimdVector(size), *simplify_and_legalize(Box::new(expression))));
                }
                Some(Expression {
                    size: 1,
                    data_type_hint: Some(DataType::MultiVector(origin)),
                    content: ExpressionContent::InvokeClassMethod(origin, "Constructor", body),
                })
            }
        }};
        for param_a in registry.single_parameters() {
            let name = "Support";
            let _: Option<()> = try {
                let origin = origin.clone()?;
                let ad = self.trait_impls.get_single_invocation("AntiDual", variable(&param_a))?;
                let wedge = self.algebra.dialect().exterior_product.first()?;
                let wedge = self.trait_impls.get_pair_invocation(wedge, origin, ad)?;
                let anti_wedge = self.algebra.dialect().exterior_anti_product.first()?;
                let anti_wedge = self.trait_impls.get_pair_invocation(anti_wedge, variable(&param_a), wedge)?;
                let the_impl = single_expression_single_trait_impl(name, &param_a, anti_wedge);
                self.trait_impls.add_single_impl(name, param_a, the_impl);
            };
        }

        for param_a in registry.single_parameters() {
            let name = "AntiSupport";
            let _: Option<()> = try {
                let origin = origin.clone()?;
                let origin_right_complement = self.trait_impls.get_single_invocation("RightComplement", origin.clone());
                let origin_complement = self.trait_impls.get_single_invocation("Complement", origin);
                let origin_complement = origin_right_complement.or(origin_complement)?;
                let dual = self.trait_impls.get_single_invocation("Dual", variable(&param_a))?;
                let anti_wedge = self.algebra.dialect().exterior_anti_product.first()?;
                let anti_wedge = self.trait_impls.get_pair_invocation(anti_wedge, origin_complement, dual)?;
                let wedge = self.algebra.dialect().exterior_product.first()?;
                let wedge = self.trait_impls.get_pair_invocation(wedge, variable(&param_a), anti_wedge)?;
                let the_impl = single_expression_single_trait_impl(name, &param_a, wedge);
                self.trait_impls.add_single_impl(name, param_a, the_impl);
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
                let a_grade = param_a.multi_vector_class().flat_basis().first()?.grade();
                let b_grade = param_b.multi_vector_class().flat_basis().first()?.grade();
                let n = self.algebra.anti_scalar_element().grade();
                if a_grade == 0 || b_grade == 0 {
                    continue
                }
                if a_grade == n || b_grade == n {
                    continue
                }
                let same_grade = a_grade == b_grade;

                let anti_dual_b = self.trait_impls.get_single_invocation("AntiDual", variable(&param_b))?;
                let anti_wedge = self.algebra.dialect().exterior_anti_product.first()?;
                let mut anti_wedge = self.trait_impls.get_pair_invocation(anti_wedge, variable(&param_a), anti_dual_b)?;
                if !same_grade {
                    anti_wedge = self.trait_impls.get_single_invocation("BulkNorm", anti_wedge)?;
                }

                let wn_a = self.trait_impls.get_single_invocation("WeightNorm", variable(&param_a))?;
                let wn_b = self.trait_impls.get_single_invocation("WeightNorm", variable(&param_b))?;
                let anti_product = self.algebra.dialect().geometric_anti_product.first()?;
                let wn = self.trait_impls.get_pair_invocation(anti_product, wn_a, wn_b)?;

                let plus = self.trait_impls.get_pair_invocation("Add", anti_wedge, wn)?;

                let cosine = single_expression_pair_trait_impl(name, &param_a, &param_b, plus);
                self.trait_impls.add_pair_impl(name, param_a, param_b, cosine);
            };
        }

        for (param_a, param_b) in registry.pair_parameters() {
            let name = "SineAngle";
            let _: Option<()> = try {
                let cos = self.trait_impls.get_pair_invocation("CosineAngle", variable(&param_a), variable(&param_b))?;
                let dual_num = match cos.data_type_hint {
                    Some(DataType::MultiVector(dn)) => dn,
                    _ => None?,
                };
                let var_cos = Expression {
                    size: 1,
                    content: ExpressionContent::Variable("cos"),
                    data_type_hint: cos.data_type_hint.clone(),
                };
                let var_assign_cos = AstNode::VariableAssignment {
                    name: "cos",
                    data_type: cos.data_type_hint.clone(),
                    expression: Box::new(cos),
                };
                let one = self.trait_impls.get_class_invocation("Unit", dual_num)?;
                let product = self.algebra.dialect().geometric_product.first()?;
                let cos_squared = self.trait_impls.get_pair_invocation(product, var_cos.clone(), var_cos)?;
                let var_cos_squared = Expression {
                    size: 1,
                    content: ExpressionContent::Variable("cos_squared"),
                    data_type_hint: cos_squared.data_type_hint.clone(),
                };
                let var_assign_cos_squared = AstNode::VariableAssignment {
                    name: "cos_squared",
                    data_type: cos_squared.data_type_hint.clone(),
                    expression: Box::new(cos_squared),
                };
                let sub = self.trait_impls.get_pair_invocation("Sub", one, var_cos_squared)?;
                let var_sub = Expression {
                    size: 1,
                    content: ExpressionContent::Variable("sub"),
                    data_type_hint: sub.data_type_hint.clone(),
                };
                let var_assign_sub = AstNode::VariableAssignment {
                    name: "sub",
                    data_type: sub.data_type_hint.clone(),
                    expression: Box::new(sub),
                };
                let sqrt = self.trait_impls.get_single_invocation("Sqrt", var_sub)?;
                let sqrt_type = sqrt.data_type_hint.clone()?;
                let sine = AstNode::TraitImplementation {
                    result: Parameter { name, data_type: sqrt_type, },
                    class: param_a.multi_vector_class(),
                    parameters: vec![param_a.clone(), param_b.clone()],
                    body: vec![
                        var_assign_cos,
                        var_assign_cos_squared,
                        var_assign_sub,
                        AstNode::ReturnStatement { expression: Box::new(sqrt) },
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
    pub fn attitude_and_dependencies<'s>(&'s mut self, registry: &'r MultiVectorClassRegistry) {
        let d = self.algebra.represented_dimensions();
        let origin = BasisElement::from_index((1 as BasisElementIndex) << d);
        let horizon = self.algebra.right_complement(&origin);


        // Attitude
        for param_a in registry.single_parameters() {
            let name = "Attitude";
            let _: Option<()> = try {
                let horizon_or_plane = registry.get_at_least(&[horizon.index])?;
                let unit = if horizon_or_plane.flat_basis().len() == 1 {
                    self.trait_impls.get_class_invocation("Unit", horizon_or_plane)?
                } else {
                    let mut body = vec![];
                    for result_group in horizon_or_plane.grouped_basis.iter() {
                        let size = result_group.len();
                        let expression = Expression {
                            size,
                            content: ExpressionContent::Constant(
                                DataType::SimdVector(size),
                                result_group.iter().map(|element| if element.index == horizon.index { 1 } else { 0 }).collect(),
                            ),
                            data_type_hint: Some(DataType::SimdVector(size)),
                        };
                        body.push((DataType::SimdVector(size), *simplify_and_legalize(Box::new(expression))));
                    }
                    Expression {
                        size: 1,
                        data_type_hint: Some(DataType::MultiVector(horizon_or_plane)),
                        content: ExpressionContent::InvokeClassMethod(horizon_or_plane, "Constructor", body),
                    }
                };
                let anti_wedge = self.algebra.dialect().exterior_anti_product.first()?;
                let anti_wedge = self.trait_impls.get_pair_invocation(anti_wedge, variable(&param_a), unit)?;
                let attitude = single_expression_single_trait_impl(name, &param_a, anti_wedge);

                self.trait_impls.add_single_impl(name, param_a.clone(), attitude);
            };
        }

        for (param_a, param_b) in registry.pair_parameters() {
            // https://rigidgeometricalgebra.org/wiki/index.php?title=Euclidean_distance

            // No distance formula in CGA yet
            // https://twitter.com/EricLengyel/status/1786624195402813549
            if self.algebra.algebra_name().starts_with("cga") { continue }

            let name = "Distance";
            let _: Option<()> = try {
                let _ = self.trait_impls.get_class_invocation("Grade", param_a.multi_vector_class())?;
                let _ = self.trait_impls.get_class_invocation("Grade", param_b.multi_vector_class())?;
                let grade_a = param_a.multi_vector_class().flat_basis().first()?.grade();
                let grade_b = param_b.multi_vector_class().flat_basis().first()?.grade();
                let anti_scalar_grade = self.algebra.anti_scalar_element().grade();
                if grade_a == 0 || grade_b == 0 {
                    continue;
                }
                if grade_a == anti_scalar_grade || grade_b == anti_scalar_grade {
                    continue;
                }

                let wedge_name = self.algebra.dialect().exterior_product.first()?;
                let anti_wedge_name = self.algebra.dialect().exterior_anti_product.first()?;
                let bulk_term = if grade_a + grade_b == anti_scalar_grade {
                    self.trait_impls.get_pair_invocation(anti_wedge_name,  variable(&param_a), variable(&param_b))?
                } else {
                    let bulk_wedge = self.trait_impls.get_pair_invocation(wedge_name, variable(&param_a), variable(&param_b))?;
                    let bulk_attitude = self.trait_impls.get_single_invocation("Attitude", bulk_wedge)?;
                    self.trait_impls.get_single_invocation("BulkNorm", bulk_attitude)?
                };

                let weight_attitude = self.trait_impls.get_single_invocation("Attitude", variable(&param_b))?;
                let weight_wedge = self.trait_impls.get_pair_invocation(wedge_name, variable(&param_a), weight_attitude)?;


                let weight_norm = self.trait_impls.get_single_invocation("WeightNorm", weight_wedge)?;

                let add = self.trait_impls.get_pair_invocation("Add", bulk_term, weight_norm)?;
                let ed = single_expression_pair_trait_impl(name, &param_a, &param_b, add);
                self.trait_impls.add_pair_impl(name, param_a, param_b, ed);
            };
        }

        //TODO speculation on distance formula for CGA
        // It seems very difficult
        // The pattern used in RGA doesn't seem to translate well...
        // that being bulk_norm(att(wedge(a,b)))/bulk_norm(wedge(att(a), att(b)))
        // It just gets kind of incomprehensible when trying to consider that with circles or whatever
        // So..... lets try new ideas. Here are a few starting ingredients..
        // - It is tempting to look at the carrier geometry and be tempted to use it. But I don't
        //   think it can be used directly, because it is too extensive on round objects.
        // - However there IS a generalized pattern that might be useful.... take the container,
        //   and take the carrier, and meet them. That is always the object itself. The container
        //   is always a sphere. And it should be easy enough to make a formula for the distance
        //   between spheres. That makes a first aproximation of distance. So if you can somehow
        //   factor in the meet with a carrier too, the true distance between objects can't be
        //   far off.
        // - I want to assume (but haven't tested yet) that RoundPoints with zero radius accurately
        //   obey euclidean distance already. Subtract, dot, square root. If this turns out to be
        //   true, then it might also be true for RoundPoints with a radius as well. After such
        //   is proven, it might be possible to get the distance of FlatPoints by converting them
        //   into RoundPoints first. (More on that in a bit.) However the downside is, extending
        //   this train of thought to dipoles is less encouraging, because it seems weird to create
        //   two RoundPoints and then check all combinations of distance and return the min or
        //   whatever. It might work for Dipoles, but really starts falling apart if we try to
        //   carry the pattern to circles, or ask ourselves about signed distances.
        // - Obviously (or maybe not so obvious to outside readers) I want to take SpacialCurvature
        //   as a parameter for distance formulas. It was my intention to use dynamic
        //   SpacialCurvature all along, but I think it's kind of uncanny and interesting that
        //   the rga3d distance formula using attitudes has an implicit usage of a flat Horizon.
        //   (Horizon is the flat variant of SpacialCurvature.) I think a great place to start here
        //   is converting FlatPoints into RoundPoints (with zero radius) using SpacialCurvature,
        //   where the resulting RoundPoint gives accurate distances.

        //TODO so.... let's start digging.
        // - How do we convert a FlatPoint into a RoundPoint (zero radius) using a Horizon or
        //   SpacialCurvature? It is an intuitive first step to perform a meet on the FlatPoint and
        //   Horizon, but this only returns an Infinity (an e5). This might be a non-starter, or it
        //   might just be incomplete. NOTABLY... A FlatPoint meeting a SphereWeight is a
        //   RoundPointOnOrigin, which is a full RoundPoint except the e5 element. So if we add these
        //   together, we get a full RoundPoint... and wouldn't you know it, SphereWeight + Horizon
        //   = SpacialCurvature anyway. Well... that's sure cute. But we need to be able to get
        //   a full and proper RoundPoint (zero radius) even when all we have is a Horizon.

    }

    pub fn round_features<'s>(&'s mut self, registry: &'r MultiVectorClassRegistry) {

        let d = self.algebra.represented_dimensions();
        let infinity_idx = (1 as BasisElementIndex) << (d + 1);
        let infinity = registry.get_at_least(&[infinity_idx]);
        let one_infinity = match infinity { None => None, Some(infinity) => {
            if infinity.flat_basis().len() == 1 {
                self.trait_impls.get_class_invocation("Unit", infinity)
            } else {
                let mut body = vec![];
                for result_group in infinity.grouped_basis.iter() {
                    let size = result_group.len();
                    let expression = Expression {
                        size,
                        content: ExpressionContent::Constant(
                            DataType::SimdVector(size),
                            result_group.iter().map(|element| if element.index == infinity_idx { 1 } else { 0 }).collect(),
                        ),
                        data_type_hint: Some(DataType::SimdVector(size)),
                    };
                    body.push((DataType::SimdVector(size), *simplify_and_legalize(Box::new(expression))));
                }
                Some(Expression {
                    size: 1,
                    data_type_hint: Some(DataType::MultiVector(infinity)),
                    content: ExpressionContent::InvokeClassMethod(infinity, "Constructor", body),
                })
            }
        }};
        let one_infinity = match one_infinity {
            None => return,
            Some(it) => it
        };

        for param_a in registry.single_parameters() {
            let is_all_flat = param_a.multi_vector_class().flat_basis().iter().all(|it| {
                infinity_idx == (infinity_idx & it.index)
            });
            if is_all_flat {
                continue;
            }

            // The object is round



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
                let wedge_name = self.algebra.dialect().exterior_product.first()?;
                let wedge = self.trait_impls.get_pair_invocation(wedge_name, variable(&param_a), one_infinity.clone())?;
                let carrier = single_expression_single_trait_impl(name, &param_a, wedge);
                self.trait_impls.add_single_impl(name, param_a.clone(), carrier)
            };

            let name = "CoCarrier";
            let _: Option<()> = try {
                let wedge_name = self.algebra.dialect().exterior_product.first()?;
                let anti_dual = self.trait_impls.get_single_invocation("AntiDual", variable(&param_a))?;
                let wedge = self.trait_impls.get_pair_invocation(wedge_name, anti_dual, one_infinity.clone())?;
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
                let anti_dual = self.trait_impls.get_single_invocation("AntiDual", car)?;
                let anti_wedge = self.trait_impls.get_pair_invocation(wedge_name, variable(&param_a), anti_dual)?;
                let container = single_expression_single_trait_impl(name, &param_a, anti_wedge);
                self.trait_impls.add_single_impl(name, param_a.clone(), container)
            };
        }

        for param_a in registry.single_parameters() {
            let is_all_flat = param_a.multi_vector_class().flat_basis().iter().all(|it| {
                infinity_idx == (infinity_idx & it.index)
            });
            if is_all_flat {
                continue;
            }

            // The object is round

            let name = "Partner";
            let _: Option<()> = try {
                let anti_wedge_name = self.algebra.dialect().exterior_anti_product.first()?;
                let car = self.trait_impls.get_single_invocation("Carrier", variable(&param_a))?;
                let dual = self.trait_impls.get_single_invocation("Dual", variable(&param_a))?;
                let container = self.trait_impls.get_single_invocation("Container", dual)?;
                let neg = self.trait_impls.get_single_invocation("Neg", container)?;
                let anti_wedge = self.trait_impls.get_pair_invocation(anti_wedge_name, neg, car)?;
                let partner = single_expression_single_trait_impl(name, &param_a, anti_wedge);
                self.trait_impls.add_single_impl(name, param_a.clone(), partner)
            };
        }
    }

    /// Datatype definitions, and implementations of external traits
    pub fn emit_datatypes_and_external_traits(&mut self, registry: &'r MultiVectorClassRegistry, emitter: &mut Emitter<std::fs::File>) -> std::io::Result<()> {
        // Preamble
        // emitter.emit(&AstNode::Preamble)?;

        // TODO give each class it's own module because shit's getting crazy at a 56k line lib.rs for cga

        // Class Definitions
        for class in registry.classes.iter() {
            if class.class_name == "Origin" && self.algebra.algebra_name().contains("rga") {
                emitter.emit(&AstNode::TypeAlias("PointAtOrigin".to_string(), "Origin".to_string()))?;
            }
            if class.class_name == "PointAtInfinity" {
                emitter.emit(&AstNode::TypeAlias("Direction".to_string(), "PointAtInfinity".to_string()))?;
            }
            if class.class_name == "Horizon" {
                emitter.emit(&AstNode::TypeAlias("PlaneAtInfinity".to_string(), "Horizon".to_string()))?;
            }
            if class.class_name == "DualNum" {
                emitter.emit(&AstNode::TypeAlias("Magnitude".to_string(), "DualNum".to_string()))?;
            }
            emitter.emit(&AstNode::ClassDefinition { class })?;
        }

        // External Traits
        let trait_names = ["Zero", "One", "Unit", "Neg", "Into", "Add", "Sub", "Mul", "Div"];
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

        // TODO rename so that in cga RoundBulk is called Bulk
        //  and FlatBulk is the qualified trait, and
        //  obviously same for Weight
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
            name: "PointInversion".to_string(),
            params: 2,
            docs: "
            Point Inversion
            An improper isometry that performs an inversion through a point.
            Points may pass as specialized as Flectors, so in other words, this is a specialized Flector sandwich.
            https://rigidgeometricalgebra.org/wiki/index.php?title=Inversion

            Be careful not to confuse with `Inverse`, which raises a number to the power of `-1.0`.
        "
            .to_string(),
        })?;

        emitter.emit(&AstNode::TraitDefinition {
            name: "Reflect".to_string(),
            params: 2,
            docs: "
            Reflection
            An improper isometry that performs reflection across a plane.
            Planes may pass as specialized Flectors, so in other words, this is a specialized Flector sandwich.
            https://rigidgeometricalgebra.org/wiki/index.php?title=Reflection
        "
            .to_string(),
        })?;

        emitter.emit(&AstNode::TraitDefinition {
            name: "Transflect".to_string(),
            params: 2,
            docs: "
            Transflection
            An improper isometry that performs a reflection and translation.
            Transflectors are specialized Flectors, so in other words, this is a specialized Flector sandwich.
            https://rigidgeometricalgebra.org/wiki/index.php?title=Transflection
        "
                .to_string(),
        })?;

        emitter.emit(&AstNode::TraitDefinition {
            name: "Translate".to_string(),
            params: 2,
            docs: "
            Translate
            A proper isometry that performs translation.
            Translators are specialized Motors, so in other words, this is a specialized Motor sandwich.
            https://rigidgeometricalgebra.org/wiki/index.php?title=Translation
        "
                .to_string(),
        })?;

        emitter.emit(&AstNode::TraitDefinition {
            name: "Rotate".to_string(),
            params: 2,
            docs: "
            Rotate
            A proper isometry that performs rotation.
            Rotors are specialized Motors, so in other words, this is a specialized Motor sandwich.
            https://rigidgeometricalgebra.org/wiki/index.php?title=Rotation
        "
                .to_string(),
        })?;

        self.emit_exact_name_match_trait_impls(&["Sandwich"], emitter)?;
        self.emit_exact_name_match_trait_impls(&["PointInversion", "Reflect", "Transflect", "Translate", "Rotate"], emitter)?;
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
        // This has an almost annoying amount of nuance, but it really helps cut down on junk
        // trait definitions. See thread: https://twitter.com/EricLengyel/status/1775663934424654262
        let is_degenerate = self.algebra.is_degenerate();
        let is_cga = self.algebra.algebra_name().contains("cga");
        let multiple_complements = self.algebra.has_multiple_complements();
        let aspects = ["Bulk", "Weight"];
        let complements = if multiple_complements { vec!["Right", "Left"] } else { vec![""] };
        let round_aspects = if is_cga { vec!["", "Round"] } else { vec![""] };

        let mut trait_names = BTreeSet::new();
        for round_aspect in round_aspects {
            for leftOrRight in complements.iter() {
                for bulkOrWeight in aspects.iter() {
                    let name = format!("{leftOrRight}{round_aspect}{bulkOrWeight}Dual");
                    let explain = "Get the complement of an aspect of an object.";
                    let pro_tip = if is_degenerate && round_aspect.is_empty() {
                        let (rbd, rwd, comp) = match *leftOrRight {
                            "Right" => ("RightBulkDual", "RightWeightDual", "RightComplement"),
                            "" => ("BulkDual", "WeightDual", "Complement"),
                            _ => ("", "", ""),
                        };
                        if rbd.is_empty() {
                            "".to_string()
                        } else {
                            format!("The metric of this algebra is degenerate. One of the side \
                                effects of this is that \n invoking the `Dual` operation erases the \
                                Weight (prior to invoking `{comp}`), and \n invoking the `AntiDual` \
                                operation erases the Bulk (prior to invoking `{comp}`). It is for \n\
                                this reason that (in this algebra) `{rbd}` = `Dual` and `{rwd}` = \
                                `AntiDual`.\n\n")
                        }
                    } else {
                        "".to_string()
                    };
                    let links = if is_cga {
                        "https://projectivegeometricalgebra.org/projgeomalg.pdf
                        https://projectivegeometricalgebra.org/confgeomalg.pdf"
                    } else {
                        "https://projectivegeometricalgebra.org/projgeomalg.pdf"
                    };

                    let docs = format!("{name}\n{explain}\n\n{pro_tip}{links}");

                    trait_names.insert(name.clone());
                    emitter.emit(&AstNode::TraitDefinition {
                        name,
                        params: 1,
                        docs,
                    })?;
                }
            }
        }

        let trait_names: Vec<_> = trait_names.iter().map(|it| it.as_str()).collect();
        self.emit_exact_name_match_trait_impls(trait_names.as_slice(), emitter)?;
        Ok(())
    }

    pub fn emit_contractions(&mut self, emitter: &mut Emitter<std::fs::File>) -> std::io::Result<()> {
        if self.algebra.is_degenerate() {
            emitter.emit(&AstNode::TraitDefinition {
                name: "BulkContraction".to_string(),
                params: 2,
                docs: "
                Bulk Contraction (Interior Product)
                https://rigidgeometricalgebra.org/wiki/index.php?title=Interior_products
                ".to_string(),
            })?;
            emitter.emit(&AstNode::TraitDefinition {
                name: "WeightContraction".to_string(),
                params: 2,
                docs: "
                Weight Contraction (Interior Product)
                https://rigidgeometricalgebra.org/wiki/index.php?title=Interior_products
                ".to_string(),
            })?;
        } else {
            emitter.emit(&AstNode::TraitDefinition {
                name: "Contraction".to_string(),
                params: 2,
                docs: "
                Contraction (Interior Product)
                https://rigidgeometricalgebra.org/wiki/index.php?title=Interior_products
                ".to_string(),
            })?;
        }
        let trait_names = ["BulkContraction", "WeightContraction", "Contraction"];
        self.emit_exact_name_match_trait_impls(&trait_names, emitter)?;
        Ok(())
    }

    pub fn emit_expansions(&mut self, emitter: &mut Emitter<std::fs::File>) -> std::io::Result<()> {
        if self.algebra.is_degenerate() {
            emitter.emit(&AstNode::TraitDefinition {
                name: "BulkExpansion".to_string(),
                params: 2,
                docs: "
                Bulk Expansion (Interior Product)
                https://rigidgeometricalgebra.org/wiki/index.php?title=Interior_products
                ".to_string(),
            })?;
            emitter.emit(&AstNode::TraitDefinition {
                name: "WeightExpansion".to_string(),
                params: 2,
                docs: "
                Weight Expansion (Interior Product)
                https://rigidgeometricalgebra.org/wiki/index.php?title=Interior_products
                ".to_string(),
            })?;
        } else {
            emitter.emit(&AstNode::TraitDefinition {
                name: "Expansion".to_string(),
                params: 2,
                docs: "
                Expansion (Interior Product)
                https://rigidgeometricalgebra.org/wiki/index.php?title=Interior_products
                ".to_string(),
            })?;
        }
        let trait_names = ["BulkExpansion", "WeightExpansion", "Expansion"];
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

        self.emit_exact_name_match_trait_impls(&["BulkNormSquared"], emitter)?;
        self.emit_exact_name_match_trait_impls(&["BulkNorm"], emitter)?;
        self.emit_exact_name_match_trait_impls(&["WeightNormSquared"], emitter)?;
        self.emit_exact_name_match_trait_impls(&["WeightNorm"], emitter)?;
        self.emit_exact_name_match_trait_impls(&["GeometricNorm"], emitter)?;
        self.emit_exact_name_match_trait_impls(&["UnitizedNormSquared"], emitter)?;
        self.emit_exact_name_match_trait_impls(&["UnitizedNorm"], emitter)?;

        self.emit_exact_name_match_trait_impls(&["CenterBulkNormSquared"], emitter)?;
        self.emit_exact_name_match_trait_impls(&["CenterBulkNorm"], emitter)?;
        self.emit_exact_name_match_trait_impls(&["CenterWeightNormSquared"], emitter)?;
        self.emit_exact_name_match_trait_impls(&["CenterWeightNorm"], emitter)?;
        self.emit_exact_name_match_trait_impls(&["CenterGeometricNorm"], emitter)?;
        self.emit_exact_name_match_trait_impls(&["CenterUnitizedNormSquared"], emitter)?;
        self.emit_exact_name_match_trait_impls(&["CenterUnitizedNorm"], emitter)?;

        self.emit_exact_name_match_trait_impls(&["RadiusBulkNormSquared"], emitter)?;
        self.emit_exact_name_match_trait_impls(&["RadiusBulkNorm"], emitter)?;
        self.emit_exact_name_match_trait_impls(&["RadiusWeightNormSquared"], emitter)?;
        self.emit_exact_name_match_trait_impls(&["RadiusWeightNorm"], emitter)?;
        self.emit_exact_name_match_trait_impls(&["RadiusGeometricNorm"], emitter)?;
        self.emit_exact_name_match_trait_impls(&["RadiusUnitizedNormSquared"], emitter)?;
        self.emit_exact_name_match_trait_impls(&["RadiusUnitizedNorm"], emitter)?;

        Ok(())
    }

    pub fn emit_characteristic_features(&mut self, emitter: &mut Emitter<std::fs::File>) -> std::io::Result<()> {
        // Square, AntiSquare
        // Inverse, AntiInverse
        // Sqrt, AntiSqrt,
        // InverseSqrt, AntiInverseSqrt
        // Pow, AntiPow, Exp, AntiExp,
        // Sine, AntiSine, Cosine, AntiCosine, Tangent, AntiTangent,
        // Sinh, AntiSinh, Cosh, AntiCosh, Tanh, AntiTanh
        // Grade, AntiGrade, Attitude,
        // Carrier, CoCarrier, Container, Center, Partner

        // I don't like including "Square" and "AntiSquare" because you can just
        // write a.wedge_dot(a) or a.anti_wedge_dot(a) instead, and I really don't think
        // that is worth an entirely new trait. This is especially the case considering you
        // could square literally any other geometric object as well (not just Scalars,
        // AntiScalars, and DualNums), and then it gets incredibly tedious and bloated to
        // include all impls for other objects too.

        // emitter.emit(&AstNode::TraitDefinition {
        //     name: "Square".to_string(),
        //     params: 1,
        //     docs: "
        //     Square (with respect to geometric product)
        //     ".to_string(),
        // })?;
        //
        // emitter.emit(&AstNode::TraitDefinition {
        //     name: "AntiSquare".to_string(),
        //     params: 1,
        //     docs: "
        //     Anti Square (with respect to geometric anti-product)
        //     ".to_string(),
        // })?;

        emitter.emit(&AstNode::TraitDefinition {
            name: "Inverse".to_string(),
            params: 1,
            docs: "
            Inverse, as in `x^-1` (with respect to geometric product).
            Useful to define the geometric quotient.
            Not to be confused with the \"Point Inversion\" or \"Sphere Inversion\" operations.
            ".to_string(),
        })?;

        emitter.emit(&AstNode::TraitDefinition {
            name: "AntiInverse".to_string(),
            params: 1,
            docs: "
            Anti Inverse, as in `x^-1` (with respect to geometric anti-product).
            Useful to define the geometric anti-quotient.
            Not to be confused with the \"Point Inversion\" or \"Sphere Inversion\" operations.
            ".to_string(),
        })?;

        emitter.emit(&AstNode::TraitDefinition {
            name: "Sqrt".to_string(),
            params: 1,
            docs: "
            Square Root (with respect to geometric product)
            ".to_string(),
        })?;

        emitter.emit(&AstNode::TraitDefinition {
            name: "AntiSqrt".to_string(),
            params: 1,
            docs: "
            Anti Square Root (with respect to geometric anti-product)
            ".to_string(),
        })?;

        emitter.emit(&AstNode::TraitDefinition {
            name: "InverseSqrt".to_string(),
            params: 1,
            docs: "
            Inverse Square Root (with respect to geometric product)
            ".to_string(),
        })?;

        emitter.emit(&AstNode::TraitDefinition {
            name: "AntiInverseSqrt".to_string(),
            params: 1,
            docs: "
            Anti Inverse Square Root (with respect to geometric anti-product)
            ".to_string(),
        })?;

        emitter.emit(&AstNode::TraitDefinition {
            name: "Pow".to_string(),
            params: 2,
            docs: "
            Self to the power of other (with respect to geometric product)
            ".to_string(),
        })?;

        emitter.emit(&AstNode::TraitDefinition {
            name: "AntiPow".to_string(),
            params: 2,
            docs: "
            Self to the power of other (with respect to geometric anti-product)
            ".to_string(),
        })?;

        emitter.emit(&AstNode::TraitDefinition {
            name: "Exp".to_string(),
            params: 1,
            docs: "
            Natural Exponentiation (with respect to geometric product)
            ".to_string(),
        })?;

        emitter.emit(&AstNode::TraitDefinition {
            name: "AntiExp".to_string(),
            params: 1,
            docs: "
            Anti Natural Exponentiation (with respect to geometric anti-product)
            ".to_string(),
        })?;

        emitter.emit(&AstNode::TraitDefinition {
            name: "Sin".to_string(),
            params: 1,
            docs: "
            Sine (with respect to geometric product)
            ".to_string(),
        })?;

        emitter.emit(&AstNode::TraitDefinition {
            name: "AntiSin".to_string(),
            params: 1,
            docs: "
            Anti Sine (with respect to geometric anti-product)
            Be careful not to confuse with \"asin\" aka \"arcsin\" aka \"inverse sine\".
            ".to_string(),
        })?;

        emitter.emit(&AstNode::TraitDefinition {
            name: "Cos".to_string(),
            params: 1,
            docs: "
            Cosine (with respect to geometric product)
            ".to_string(),
        })?;

        emitter.emit(&AstNode::TraitDefinition {
            name: "AntiCos".to_string(),
            params: 1,
            docs: "
            Anti Cosine (with respect to geometric anti-product)
            Be careful not to confuse with \"acos\" aka \"arccos\" aka \"inverse cosine\".
            ".to_string(),
        })?;

        emitter.emit(&AstNode::TraitDefinition {
            name: "Tan".to_string(),
            params: 1,
            docs: "
            Tangent (with respect to geometric product)
            ".to_string(),
        })?;

        emitter.emit(&AstNode::TraitDefinition {
            name: "AntiTan".to_string(),
            params: 1,
            docs: "
            Anti Tangent (with respect to geometric anti-product)
            Be careful not to confuse with \"atan\" aka \"arctan\" aka \"inverse tangent\".
            ".to_string(),
        })?;

        emitter.emit(&AstNode::TraitDefinition {
            name: "Sinh".to_string(),
            params: 1,
            docs: "
            Hyperbolic Sine (with respect to geometric product)
            ".to_string(),
        })?;

        emitter.emit(&AstNode::TraitDefinition {
            name: "AntiSinh".to_string(),
            params: 1,
            docs: "
            Anti Hyperbolic Sine (with respect to geometric anti-product)
            Be careful not to confuse with \"asinh\" aka \"arcsinh\" aka \"inverse hyperbolic sine\".
            ".to_string(),
        })?;

        emitter.emit(&AstNode::TraitDefinition {
            name: "Cosh".to_string(),
            params: 1,
            docs: "
            Hyperbolic Cosine (with respect to geometric product)
            ".to_string(),
        })?;

        emitter.emit(&AstNode::TraitDefinition {
            name: "AntiCosh".to_string(),
            params: 1,
            docs: "
            Anti Hyperbolic Cosine (with respect to geometric anti-product)
            Be careful not to confuse with \"acosh\" aka \"arccosh\" aka \"inverse hyperbolic cosine\".
            ".to_string(),
        })?;

        emitter.emit(&AstNode::TraitDefinition {
            name: "Tanh".to_string(),
            params: 1,
            docs: "
            Hyperbolic Tangent (with respect to geometric product)
            ".to_string(),
        })?;

        emitter.emit(&AstNode::TraitDefinition {
            name: "AntiTanh".to_string(),
            params: 1,
            docs: "
            Anti Hyperbolic Tangent (with respect to geometric anti-product)
            Be careful not to confuse with \"atanh\" aka \"arctanh\" aka \"inverse hyperbolic tangent\".
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

        let trait_names = ["Sqrt", "AntiSqrt", "Grade", "AntiGrade", "Attitude", "Carrier", "CoCarrier"];
        self.emit_exact_name_match_trait_impls(&trait_names, emitter)?;
        let trait_names = ["Container", "Center"];
        self.emit_exact_name_match_trait_impls(&trait_names, emitter)?;
        let trait_names = ["Partner"];
        self.emit_exact_name_match_trait_impls(&trait_names, emitter)?;
        let trait_names = ["Inverse", "AntiInverse"];
        self.emit_exact_name_match_trait_impls(&trait_names, emitter)?;

        let trait_names = [
            "Pow", "AntiPow",
            /*"Square", "AntiSquare",*/ /*"Inverse", "AntiInverse",*/
            /*"Sqrt", "AntiSqrt",*/ "InverseSqrt", "AntiInverseSqrt",
            "Exp", "AntiExp", "Sin", "AntiSin",
            "Cos", "AntiCos", "Tan", "AntiTan",
            "Sinh", "AntiSinh", "Cosh", "AntiCosh",
            "Tanh", "AntiTanh"
        ];
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

        let trait_names = ["ProjectOrthogonallyOnto", "AntiProjectOrthogonallyOnto", "ProjectViaOriginOnto", "AntiProjectViaHorizonOnto"];
        self.emit_exact_name_match_trait_impls(&trait_names, emitter)?;
        Ok(())
    }

    pub fn emit_rejections_and_stuff(&mut self, emitter: &mut Emitter<std::fs::File>) -> std::io::Result<()> {

        // TODO refine the conjugates in the names once you have an idea what these actually look like

        emitter.emit(&AstNode::TraitDefinition {
            name: "RejectOrthogonallyFrom".to_string(),
            params: 2,
            docs: "
            Orthogonal Rejection
            Rejection and Projection are counterparts to one another.
            This is the counterpart to `ProjectOrthogonallyOnto`.
        "
                .to_string()
                .to_string(),
        })?;
        emitter.emit(&AstNode::TraitDefinition {
            name: "AntiRejectOrthogonallyFrom".to_string(),
            params: 2,
            docs: "
            Orthogonal AntiRejection
            Rejection and Projection are counterparts to one another.
            This is the counterpart to `AntiProjectOrthogonallyOnto`.
        "
                .to_string(),
        })?;
        emitter.emit(&AstNode::TraitDefinition {
            name: "RejectViaOriginFrom".to_string(),
            params: 2,
            docs: "
            Central (from origin) Rejection
            Rejection and Projection are counterparts to one another.
            This is the counterpart to `ProjectViaOriginOnto`.
        "
                .to_string(),
        })?;
        emitter.emit(&AstNode::TraitDefinition {
            name: "AntiRejectViaHorizonFrom".to_string(),
            params: 2,
            docs: "
            Outward (from horizon) AntiRejection
            Rejection and Projection are counterparts to one another.
            This is the counterpart to `AntiProjectViaHorizonOnto`.
        "
                .to_string(),
        })?;

        let trait_names = ["RejectOrthogonallyFrom", "AntiRejectOrthogonallyFrom", "RejectViaOriginFrom", "AntiRejectViaHorizonFrom"];
        self.emit_exact_name_match_trait_impls(&trait_names, emitter)?;
        Ok(())
    }

    pub fn emit_supports(&mut self, emitter: &mut Emitter<std::fs::File>) -> std::io::Result<()> {
        emitter.emit(&AstNode::TraitDefinition {
            name: "Support".to_string(),
            params: 1,
            docs: "
            Support
            The support is the point enclosed by the object closest to the origin.
        "
                .to_string(),
        })?;
        emitter.emit(&AstNode::TraitDefinition {
            name: "AntiSupport".to_string(),
            params: 1,
            docs: "
            AntiSupport
            The anti-support is the anti-vector furthest from the origin that encloses the object.
        "
                .to_string(),
        })?;

        let trait_names = ["Support", "AntiSupport"];
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

    pub fn emit_quotients(&mut self, emitter: &mut Emitter<std::fs::File>) -> std::io::Result<()> {
        emitter.emit(&AstNode::TraitDefinition {
            name: "GeometricQuotient".to_string(),
            params: 2,
            docs: "
            The Geometric Quotient between `a` and `b` is the geometric product between `a` and `b^-1` (the inverse of `b`).
            See also \"Inverse\".
        "
                .to_string(),
        })?;
        emitter.emit(&AstNode::TraitDefinition {
            name: "GeometricAntiQuotient".to_string(),
            params: 2,
            docs: "
            The Geometric AntiQuotient between `a` and `b` is the geometric anti-product between `a` and the anti-inverse of `b`.
            See also \"AntiInverse\".
        "
                .to_string(),
        })?;

        let trait_names = ["GeometricQuotient", "GeometricAntiQuotient"];
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
