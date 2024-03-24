use std::collections::BTreeMap;
use crate::algebra::MultiVectorClass;
use crate::ast::{AstNode, Expression, ExpressionContent, Parameter};
use crate::result_of_trait;

pub struct TraitImpls<'a> {
    pub class_level: BTreeMap<(String, String), AstNode<'a>>,
    pub single_args: BTreeMap<(String, String), AstNode<'a>>,
    pub pair_args: BTreeMap<(String, String, String), AstNode<'a>>,
    // TODO inlined_single_args, inlined_pair_args
}

impl<'a> TraitImpls<'a> {
    pub fn new() -> Self {
        TraitImpls {
            class_level: BTreeMap::new(),
            single_args: BTreeMap::new(),
            pair_args: BTreeMap::new(),
        }
    }

    pub fn add_pair_impl(&mut self, name: &str, parameter_a: Parameter<'a>, parameter_b: Parameter<'a>, the_impl: AstNode<'a>) {
        let a_name = parameter_a.data_type.data_class_name();
        let b_name = parameter_b.data_type.data_class_name();
        self.pair_args.insert((name.to_string(), a_name.to_string(), b_name.to_string()), the_impl);
    }

    pub fn add_single_impl(&mut self, name: &str, parameter_a: Parameter<'a>, the_impl: AstNode<'a>) {
        let a_name = parameter_a.data_type.data_class_name();
        self.single_args.insert((name.to_string(), a_name.to_string()), the_impl);
    }

    pub fn add_class_impl(&mut self, name: &str, class_a: &'a MultiVectorClass, the_impl: AstNode<'a>) {
        let a_name = class_a.class_name.clone();
        self.class_level.insert((name.to_string(), a_name.to_string()), the_impl);
    }

    pub fn get_pair_impl(&self, name: &str, parameter_a: &Parameter<'a>, parameter_b: &Parameter<'a>) -> Option<&AstNode<'a>> {
        let a_name = parameter_a.data_type.data_class_name();
        let b_name = parameter_b.data_type.data_class_name();
        return self.pair_args.get(&(name.to_string(), a_name, b_name));
    }

    pub fn get_single_impl(&self, name: &str, parameter_a: &Parameter<'a>) -> Option<&AstNode<'a>> {
        let a_name = parameter_a.data_type.data_class_name();
        return self.single_args.get(&(name.to_string(), a_name));
    }

    pub fn get_class_impl(&self, name: &str, class_a: &'a MultiVectorClass) -> Option<&AstNode<'a>> {
        let a_name = class_a.class_name.clone();
        return self.class_level.get(&(name.to_string(), a_name.to_string()));
    }

    pub fn get_pair_impl_and_result(&self, name: &str, parameter_a: &Parameter<'a>, parameter_b: &Parameter<'a>) -> Option<(&AstNode<'a>, &Parameter<'a>)> {
        let a_name = parameter_a.data_type.data_class_name();
        let b_name = parameter_b.data_type.data_class_name();
        let the_impl = self.pair_args.get(&(name.to_string(), a_name, b_name))?;
        let result = result_of_trait!(the_impl);
        return Some((the_impl, result));
    }

    pub fn get_single_impl_and_result(&self, name: &str, parameter_a: &Parameter<'a>) -> Option<(&AstNode<'a>, &Parameter<'a>)> {
        let a_name = parameter_a.data_type.data_class_name();
        let the_impl = self.single_args.get(&(name.to_string(), a_name))?;
        let result = result_of_trait!(the_impl);
        return Some((the_impl, result));
    }

    pub fn get_class_impl_and_result(&self, name: &str, class_a: &'a MultiVectorClass) -> Option<(&AstNode<'a>, &Parameter<'a>)> {
        let a_name = class_a.class_name.clone();
        let the_impl = self.class_level.get(&(name.to_string(), a_name))?;
        let result = result_of_trait!(the_impl);
        return Some((the_impl, result));
    }

    pub fn get_pair_invocation(&self, name: &str, a: Expression<'a>, b: Expression<'a>) -> Option<Expression<'a>> {
        let datatype_a = match &a.data_type_hint {
            Some(dt) => dt.clone(),
            _ => panic!("TraitImpls.get_pair_invocation for {name} requires MultiVectorClass data_type_hints on \"a\" {a:?}"),
        };
        let datatype_b = match &b.data_type_hint {
            Some(dt) => dt.clone(),
            _ => panic!("TraitImpls.get_pair_invocation for {name} requires MultiVectorClass data_type_hints on \"b\" {b:?}"),
        };
        let a_name = datatype_a.data_class_name();
        let b_name = datatype_b.data_class_name();
        let the_impl = self.pair_args.get(&(name.to_string(), a_name, b_name))?;
        let result = result_of_trait!(the_impl);

        // InvokeInstanceMethod:
        // - Class implementing trait
        // - Inner expression
        // - Method name
        // - Arguments

        Some(Expression {
            size: 1,
            data_type_hint: Some(result.data_type.clone()),
            content: ExpressionContent::InvokeInstanceMethod(datatype_a, Box::new(a), result.name, vec![(datatype_b, b)]),
        })
    }

    pub fn get_single_invocation(&self, name: &str, a: Expression<'a>) -> Option<Expression<'a>> {
        let datatype_a = match &a.data_type_hint {
            Some(dt) => dt.clone(),
            _ => panic!("TraitImpls.get_single_invocation for {name} requires MultiVectorClass data_type_hints on \"a\" {a:?}"),
        };
        let a_name = datatype_a.data_class_name();
        let the_impl = self.single_args.get(&(name.to_string(), a_name))?;
        let result = result_of_trait!(the_impl);

        // InvokeInstanceMethod:
        // - Class implementing trait
        // - Inner expression
        // - Method name
        // - Arguments

        Some(Expression {
            size: 1,
            data_type_hint: Some(result.data_type.clone()),
            content: ExpressionContent::InvokeInstanceMethod(datatype_a, Box::new(a), result.name, vec![]),
        })
    }

    pub fn get_class_invocation(&self, name: &'static str, class_a: &'a MultiVectorClass) -> Option<Expression<'a>> {
        let a_name = class_a.class_name.clone();
        let the_impl = self.class_level.get(&(name.to_string(), a_name))?;
        let result = result_of_trait!(the_impl);
        Some(Expression {
            size: 1,
            data_type_hint: Some(result.data_type.clone()),
            content: ExpressionContent::InvokeClassMethod(class_a, name, vec![]),
        })
    }
}
