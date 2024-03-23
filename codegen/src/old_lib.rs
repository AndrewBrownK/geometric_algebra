use std::collections::{BTreeMap, BTreeSet};
use std::io::Read;
use std::path::PathBuf;

use naga;
use naga::front::glsl::Error;
use naga::valid::{Capabilities, ValidationFlags};
use naga::ShaderStage;

use crate::algebra::basis_element::BasisElement;

use crate::algebra::GeometricAlgebraTrait;
use crate::ast::{Expression, ExpressionContent};
use crate::compile::{single_expression_pair_trait_impl, single_expression_single_trait_impl, variable};
use crate::{
    algebra::{Involution, MultiVectorClass, MultiVectorClassRegistry, Product},
    ast::{AstNode, DataType, Parameter},
    emit::Emitter,
    result_of_trait,
};

// TODO this file is renamed "lib.rs" since we switched to "main.rs"
//  So organize all the stuff in here, give them proper homes, then delete or rename "old_lib.rs"

pub fn read_multi_vector_from_str<GA: GeometricAlgebraTrait>(multi_vector_descriptor: &str, algebra: &GA) -> (MultiVectorClass, Option<String>) {
    let mut multi_vector_descriptor_iter = multi_vector_descriptor.split(':');
    let mut class_name = multi_vector_descriptor_iter.next().unwrap().to_owned();
    let mut superclass_name = None;
    if class_name.contains('/') {
        let mut split = class_name.split('/').map(|it| it.to_string());
        superclass_name = split.next();
        class_name = split.next().unwrap();
    }
    let mvc = MultiVectorClass {
        class_name,
        grouped_basis: multi_vector_descriptor_iter
            .next()
            .unwrap()
            .split('|')
            .map(|group_descriptor| group_descriptor.split(',').map(|element_name| algebra.parse(element_name)).collect::<Vec<_>>())
            .collect::<Vec<_>>(),
    };
    (mvc, superclass_name)
}

struct TraitImpls<'a> {
    class_level: BTreeMap<(String, String), AstNode<'a>>,
    single_args: BTreeMap<(String, String), AstNode<'a>>,
    pair_args: BTreeMap<(String, String, String), AstNode<'a>>,
}

impl<'a> TraitImpls<'a> {
    fn new() -> Self {
        TraitImpls {
            class_level: BTreeMap::new(),
            single_args: BTreeMap::new(),
            pair_args: BTreeMap::new(),
        }
    }

    fn add_pair_impl(&mut self, name: &str, parameter_a: Parameter<'a>, parameter_b: Parameter<'a>, the_impl: AstNode<'a>) {
        let a_name = parameter_a.data_type.data_class_name();
        let b_name = parameter_b.data_type.data_class_name();
        self.pair_args.insert((name.to_string(), a_name.to_string(), b_name.to_string()), the_impl);
    }

    fn add_single_impl(&mut self, name: &str, parameter_a: Parameter<'a>, the_impl: AstNode<'a>) {
        let a_name = parameter_a.data_type.data_class_name();
        self.single_args.insert((name.to_string(), a_name.to_string()), the_impl);
    }

    fn add_class_impl(&mut self, name: &str, class_a: &'a MultiVectorClass, the_impl: AstNode<'a>) {
        let a_name = class_a.class_name.clone();
        self.class_level.insert((name.to_string(), a_name.to_string()), the_impl);
    }

    fn get_pair_impl(&self, name: &str, parameter_a: &Parameter<'a>, parameter_b: &Parameter<'a>) -> Option<&AstNode<'a>> {
        let a_name = parameter_a.data_type.data_class_name();
        let b_name = parameter_b.data_type.data_class_name();
        return self.pair_args.get(&(name.to_string(), a_name, b_name));
    }

    fn get_single_impl(&self, name: &str, parameter_a: &Parameter<'a>) -> Option<&AstNode<'a>> {
        let a_name = parameter_a.data_type.data_class_name();
        return self.single_args.get(&(name.to_string(), a_name));
    }

    fn get_class_impl(&self, name: &str, class_a: &'a MultiVectorClass) -> Option<&AstNode<'a>> {
        let a_name = class_a.class_name.clone();
        return self.class_level.get(&(name.to_string(), a_name.to_string()));
    }

    fn get_pair_impl_and_result(&self, name: &str, parameter_a: &Parameter<'a>, parameter_b: &Parameter<'a>) -> Option<(&AstNode<'a>, &Parameter<'a>)> {
        let a_name = parameter_a.data_type.data_class_name();
        let b_name = parameter_b.data_type.data_class_name();
        let the_impl = self.pair_args.get(&(name.to_string(), a_name, b_name))?;
        let result = result_of_trait!(the_impl);
        return Some((the_impl, result));
    }

    fn get_single_impl_and_result(&self, name: &str, parameter_a: &Parameter<'a>) -> Option<(&AstNode<'a>, &Parameter<'a>)> {
        let a_name = parameter_a.data_type.data_class_name();
        let the_impl = self.single_args.get(&(name.to_string(), a_name))?;
        let result = result_of_trait!(the_impl);
        return Some((the_impl, result));
    }

    fn get_class_impl_and_result(&self, name: &str, class_a: &'a MultiVectorClass) -> Option<(&AstNode<'a>, &Parameter<'a>)> {
        let a_name = class_a.class_name.clone();
        let the_impl = self.class_level.get(&(name.to_string(), a_name))?;
        let result = result_of_trait!(the_impl);
        return Some((the_impl, result));
    }

    fn get_pair_invocation(&self, name: &str, a: Expression<'a>, b: Expression<'a>) -> Option<Expression<'a>> {
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

    fn get_single_invocation(&self, name: &str, a: Expression<'a>) -> Option<Expression<'a>> {
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

    fn get_class_invocation(&self, name: &'static str, class_a: &'a MultiVectorClass) -> Option<Expression<'a>> {
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

pub struct CodeGenerator<'r, GA> {
    algebra: GA,
    trait_impls: TraitImpls<'r>,
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
                let grade_impl = MultiVectorClass::derive_grade("Grade", &param_a, grade);
                self.trait_impls.add_class_impl("Grade", param_a.multi_vector_class(), grade_impl);

                let anti_grade_impl = MultiVectorClass::derive_grade("AntiGrade", &param_a, anti_grade);
                self.trait_impls.add_class_impl("AntiGrade", param_a.multi_vector_class(), anti_grade_impl);
            }
        }

        // Involutions
        let involutions = Involution::involutions(&self.algebra);
        for param_a in registry.single_parameters() {
            for (name, involution, _) in involutions.iter() {
                let ast_node = MultiVectorClass::involution(name, involution, &param_a, registry, false);
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
                let ast_node = MultiVectorClass::involution("Into", &Involution::projection(param_b.multi_vector_class()), &param_a, registry, true);
                if ast_node != AstNode::None {
                    self.trait_impls.add_pair_impl("Into", param_a.clone(), param_b.clone(), ast_node);
                }
            }
        }

        // Add, Subtract
        for (param_a, param_b) in registry.pair_parameters() {
            for name in &["Add", "Sub"] {
                let ast_node = MultiVectorClass::element_wise(*name, &param_a, &param_b, registry);
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
                let ast_node = MultiVectorClass::element_wise(*name, &param_a, &param_b, registry);
                if ast_node != AstNode::None {
                    self.trait_impls.add_pair_impl(name, param_a.clone(), param_b.clone(), ast_node);
                }
            }
        }

        // Products from Geometric Algebra
        let products = Product::products(&self.algebra);
        for (param_a, param_b) in registry.pair_parameters() {
            for (name, product, _) in products.iter() {
                let ast_node = MultiVectorClass::product(name, product, &param_a, &param_b, registry);
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
    }

    /// Step 3: Create some fancy norms
    /// These items require some special insight per Geometric Algebra, for example if there are
    /// multiple/special projective dimensions with different meanings.
    pub fn fancy_norms(&mut self, _prefix: &'static str, _projective_basis: BasisElement, _registry: &'r MultiVectorClassRegistry) {
        // TODO SOONER

        // TODO this is all kinds of fucked currently. Not sure how to generate norms generically
        //  for both RGA and CGA. I could get all of RGA and the flat objects of CGA by doing
        //  bulk and weight first, then get bulkNorm and WeightNorm from those by doing the dot product
        //  square root thing. This doesn't work in the round objects though. Or maybe it does, and that is why
        //  "position/center/radius norm" are 3 different things instead of 2 different things. In any case.
        //  I need to figure out what's the deal with CenterNorm and RadiusNorm.

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
        //         let scale = MultiVectorClass::derive_scale(name, gp, &param_a, &param_b);
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
        //         let signum = MultiVectorClass::derive_signum("Signum", gp, magnitude, &param_a);
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
        //         let inverse = MultiVectorClass::derive_inverse("Inverse", gp, squared_magnitude, reversal, &param_a);
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
                let unitize = MultiVectorClass::derive_unitize(name, gp, weight_norm, &param_a, &param_b);
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
        //         let power_of_integer = MultiVectorClass::derive_power_of_integer(
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
        //         let division = MultiVectorClass::derive_division(name, gp, inverse, &param_a, &param_b);
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
        //         let transformation = MultiVectorClass::derive_sandwich_product(
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

                let sandwich = MultiVectorClass::derive_sandwich_product("Sandwich", gp, gp2, reversal, into, &param_a, &param_b);
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

            let bulk = MultiVectorClass::derive_bulk_or_weight("Bulk", &param_a, &projective_basis, false, flat_basis.clone(), true, registry);
            if bulk != AstNode::None {
                self.trait_impls.add_single_impl("Bulk", param_a.clone(), bulk);
            }

            let weight = MultiVectorClass::derive_bulk_or_weight("Weight", &param_a, &projective_basis, true, flat_basis.clone(), true, registry);
            if weight != AstNode::None {
                self.trait_impls.add_single_impl("Weight", param_a.clone(), weight);
            }

            if self.algebra.algebra_name().contains("cga") {

                let round_bulk = MultiVectorClass::derive_bulk_or_weight("RoundBulk", &param_a, &projective_basis, false, flat_basis.clone(), false, registry);
                if round_bulk != AstNode::None {
                    self.trait_impls.add_single_impl("RoundBulk", param_a.clone(), round_bulk);
                }

                let round_weight = MultiVectorClass::derive_bulk_or_weight("RoundWeight", &param_a, &projective_basis, true, flat_basis.clone(), false, registry);
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

        // TODO make CosineAngle and SineAngle return plain floats instead of Scalars.
        //  I mean... unless you figure out some special reason for them to be Scalars, like
        //  if it is somehow possible to feed back into GA operations in a useful manner.

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
                let cosine = single_expression_pair_trait_impl(name, &param_a, &param_b, bn);
                self.trait_impls.add_pair_impl(name, param_a, param_b, cosine);
            };
        }

        for (param_a, param_b) in registry.pair_parameters() {
            let name = "SineAngle";
            let _: Option<()> = try {
                let cos = self.trait_impls.get_pair_invocation("CosineAngle", variable(&param_a), variable(&param_b))?;
                let scalar = match cos.data_type_hint {
                    Some(DataType::MultiVector(scalar)) => scalar,
                    _ => continue,
                };
                let one = self.trait_impls.get_class_invocation("One", scalar)?;
                let const2 = Expression {
                    size: 1,
                    content: ExpressionContent::Constant(DataType::Integer, vec![2]),
                    data_type_hint: Some(DataType::Integer),
                };
                // TODO this is failing because no Powi implementation
                let pow2 = self.trait_impls.get_pair_invocation("Powi", cos, const2)?;
                let sub = self.trait_impls.get_pair_invocation("Sub", one, pow2)?;
                let sqrt = self.trait_impls.get_single_invocation("Sqrt", sub)?;
                let sine = single_expression_pair_trait_impl(name, &param_a, &param_b, sqrt);
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


            let name = "Carrier";
            let _: Option<()> = try {
                let construct = construct_infinity.clone()?;
                let wedge = self.trait_impls.get_pair_invocation("Wedge", variable(&param_a), construct)?;
                let carrier = single_expression_single_trait_impl(name, &param_a, wedge);
                self.trait_impls.add_single_impl(name, param_a.clone(), carrier)
            };

            let name = "CoCarrier";
            let _: Option<()> = try {
                let construct = construct_infinity.clone()?;
                let right_weight_dual = self.trait_impls.get_single_invocation("RightRoundWeightDual", variable(&param_a))?;
                let wedge = self.trait_impls.get_pair_invocation("Wedge", right_weight_dual, construct)?;
                let carrier = single_expression_single_trait_impl(name, &param_a, wedge);
                self.trait_impls.add_single_impl(name, param_a.clone(), carrier)
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
            if trait_names.contains(name) && name.as_str() != "GeometricNorm" {
                emitter.emit(ast)?;
            }
        }
        self.emit_exact_name_match_trait_impls(&["GeometricNorm"], emitter)?;
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

        // TODO make grade and anti_grade class-level traits
        emitter.emit(&AstNode::TraitDefinition {
            name: "Grade".to_string(),
            params: 1,
            docs: "
            Grade
            https://rigidgeometricalgebra.org/wiki/index.php?title=Grade_and_antigrade
            ".to_string(),
        })?;

        emitter.emit(&AstNode::TraitDefinition {
            name: "AntiGrade".to_string(),
            params: 1,
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
                The Center of a round object is the Radial (RoundPoint) having the same center and radius.
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

        let trait_names = ["Sqrt", "Grade", "AntiGrade", "Attitude", "Carrier", "CoCarrier", "Container", "Center", "Partner"];
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

pub fn validate_glsl(algebra_name: &str, file_path: PathBuf) {
    // Prepare some of naga's clutter
    let mut glsl_frontend = naga::front::glsl::Frontend::default();
    let mut validator = naga::valid::Validator::new(ValidationFlags::default(), Capabilities::default());
    let options = naga::front::glsl::Options {
        stage: ShaderStage::Compute,
        defines: Default::default(),
    };

    // Read the glsl
    let glsl_file_name = file_path.with_extension("glsl");
    let mut glsl_file = std::fs::File::open(glsl_file_name).unwrap();
    let mut glsl_contents = String::new();
    glsl_file.read_to_string(&mut glsl_contents).unwrap();
    // Append a dummy entry point
    glsl_contents.push_str("\nvoid main() {}");

    let module = match glsl_frontend.parse(&options, glsl_contents.as_str()) {
        Ok(m) => m,
        Err(err) => {
            let mut line = "??".to_string();
            let mut line_position = "??".to_string();
            if let Some(Error { meta, .. }) = err.first() {
                let location = meta.location(glsl_contents.as_str());
                line = location.line_number.to_string();
                line_position = location.line_position.to_string();
            }
            panic!("Error generating {algebra_name} glsl on line {line} at line position {line_position}: {err:?}")
        }
    };
    if let Err(err) = validator.validate(&module) {
        panic!("Error generating {algebra_name}: {err:?}")
    };
    // glsl success, woo hoo!
}

pub fn validate_wgsl(algebra_name: &str, file_path: PathBuf) {
    // Prepare some of naga's clutter
    let mut wgsl_frontend = naga::front::wgsl::Frontend::new();
    let mut validator = naga::valid::Validator::new(ValidationFlags::default(), Capabilities::default());

    // Read the wgsl
    let wgsl_file_name = file_path.with_extension("wgsl");
    let mut wgsl_file = std::fs::File::open(wgsl_file_name).unwrap();
    let mut wgsl_contents = String::new();
    wgsl_file.read_to_string(&mut wgsl_contents).unwrap();

    // Parse, prune, and validate the naga module
    let module = match wgsl_frontend.parse(wgsl_contents.as_str()) {
        Ok(m) => m,
        Err(err) => {
            let mut line = "??".to_string();
            let mut line_position = "??".to_string();
            if let Some(loc) = err.location(wgsl_contents.as_str()) {
                line = loc.line_number.to_string();
                line_position = loc.line_position.to_string();
            }
            panic!("Error generating {algebra_name} wgsl on line {line} at line position {line_position}: {err:?}")
        }
    };
    if let Err(err) = validator.validate(&module) {
        panic!("Error generating {algebra_name}: {err:?}")
    };
    // wgsl success, woo hoo!
}
