#![feature(try_blocks)]

use std::collections::{BTreeMap, HashMap};
use std::io::{Read, Write};
use std::path::PathBuf;

use naga;
use naga::back::wgsl::WriterFlags;
use naga::front::glsl::Error;
use naga::ShaderStage;
use naga::valid::{Capabilities, ValidationFlags};
use naga_oil::prune::PartReq;
use algebra::basis_element::BasisElement;
use algebra::rigid::RigidGeometricAlgebra;

use crate::{
    algebra::{Involution, MultiVectorClass, MultiVectorClassRegistry, Product},
    ast::{AstNode, DataType, Parameter},
    emit::Emitter,
};
use crate::algebra::GeometricAlgebraTrait;
use crate::ast::{Expression, ExpressionContent};
use crate::compile::{single_expression_pair_trait_impl, single_expression_single_trait_impl, variable};

pub mod algebra;
mod ast;
mod compile;
pub mod emit;
mod glsl;
mod rust;

pub struct AlgebraDescriptor {
    pub algebra_name: String,
    pub generator_squares: Vec<isize>,
    pub multi_vectors: Vec<MultiVectorClass>,
}



pub fn read_config_from_env() -> AlgebraDescriptor {

    let mut args = std::env::args();
    let _executable = args.next().unwrap();

    // Example:
    // epga3d:1,1,1,1;Scalar:1;MultiVector:1,e23,-e13,e12|e0,-e023,e013,-e012|e123,e1,e2,e3|e0123,e01,e02,e03;Rotor:1,e23,-e13,e12;Point:e123,-e023,e013,-e012;IdealPoint:e01,e02,e03;Plane:e0,e1,e2,e3;Line:e01,e02,e03|e23,-e13,e12;Translator:1,e01,e02,e03;Motor:1,e23,-e13,e12|e0123,e01,e02,e03;PointAndPlane:e123,-e023,e013,-e012|e0,e1,e2,e3
    let config = args.next().unwrap();
    read_config_from_str(config.as_str())
}
pub fn read_config_from_str(config: &str) -> AlgebraDescriptor {


    // Example:
    // epga3d:1,1,1,1;Scalar:1;MultiVector:1,e23,-e13,e12|e0,-e023,e013,-e012|e123,e1,e2,e3|e0123,e01,e02,e03;Rotor:1,e23,-e13,e12;Point:e123,-e023,e013,-e012;IdealPoint:e01,e02,e03;Plane:e0,e1,e2,e3;Line:e01,e02,e03|e23,-e13,e12;Translator:1,e01,e02,e03;Motor:1,e23,-e13,e12|e0123,e01,e02,e03;PointAndPlane:e123,-e023,e013,-e012|e0,e1,e2,e3
    let config = config.to_string();

    // epga3d:1,1,1,1
    // Scalar:1
    // MultiVector:1,e23,-e13,e12|e0,-e023,e013,-e012|e123,e1,e2,e3|e0123,e01,e02,e03
    // Rotor:1,e23,-e13,e12
    // Point:e123,-e023,e013,-e012
    // IdealPoint:e01,e02,e03
    // Plane:e0,e1,e2,e3
    // Line:e01,e02,e03|e23,-e13,e12
    // Translator:1,e01,e02,e03
    // Motor:1,e23,-e13,e12|e0123,e01,e02,e03
    // PointAndPlane:e123,-e023,e013,-e012|e0,e1,e2,e3
    let mut config_iter = config.split(';');

    // epga3d:1,1,1,1
    let algebra_descriptor = config_iter.next().unwrap();
    let mut algebra_descriptor_iter = algebra_descriptor.split(':');
    // epga3d
    let algebra_name = algebra_descriptor_iter.next().unwrap();

    // vec![1,1,1,1]
    let generator_squares = algebra_descriptor_iter
        // 1,1,1,1
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse::<isize>().unwrap())
        .collect::<Vec<_>>();

    todo!()

    // let algebra = RigidGeometricAlgebra {
    //     generator_squares: generator_squares.as_slice(),
    // };
    //
    // let mut multi_vectors = vec![];
    // for multi_vector_descriptor in config_iter {
    //     multi_vectors.push(read_multi_vector_from_str(multi_vector_descriptor, &algebra));
    // }
    //
    // AlgebraDescriptor {
    //     algebra_name: algebra_name.to_string(),
    //     generator_squares,
    //     multi_vectors,
    // }
}

pub fn read_multi_vector_from_str(multi_vector_descriptor: &str, algebra: &RigidGeometricAlgebra) -> MultiVectorClass {
    let mut multi_vector_descriptor_iter = multi_vector_descriptor.split(':');
    MultiVectorClass {
        class_name: multi_vector_descriptor_iter.next().unwrap().to_owned(),
        grouped_basis: multi_vector_descriptor_iter
            .next()
            .unwrap()
            .split('|')
            .map(|group_descriptor| {
                group_descriptor
                    .split(',')
                    .map(|element_name| algebra.parse(element_name))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>(),
    }
}

struct TraitImpls<'a> {
    class_level: BTreeMap<(String, String), AstNode<'a>>,
    single_args: BTreeMap<(String, String), AstNode<'a>>,
    pair_args: BTreeMap<(String, String, String), AstNode<'a>>
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
            content: ExpressionContent::InvokeInstanceMethod(
                datatype_a,
                Box::new(a),
                result.name,
                vec![
                    (datatype_b, b)
                ]
            ),
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
            content: ExpressionContent::InvokeInstanceMethod(
                datatype_a,
                Box::new(a),
                result.name,
                vec![]
            ),
        })
    }

    fn get_class_invocation(&self, name: &'static str, class_a: &'a MultiVectorClass) -> Option<Expression<'a>> {
        let a_name = class_a.class_name.clone();
        let the_impl = self.class_level.get(&(name.to_string(), a_name))?;
        let result = result_of_trait!(the_impl);
        Some(Expression {
            size: 1,
            data_type_hint: Some(result.data_type.clone()),
            content: ExpressionContent::InvokeClassMethod(
                class_a,
                name,
                vec![]
            )
        })
    }
}


pub struct CodeGenerator<'r, GA> {
    algebra: GA,
    trait_impls: TraitImpls<'r>
}


impl<'r, GA: GeometricAlgebraTrait> CodeGenerator<'r, GA> {
    pub fn new(algebra: GA) -> Self {
        CodeGenerator {
            algebra,
            trait_impls: TraitImpls::new()
        }
    }

    /// Step 1: These items are somewhat universal across geometric algebras
    pub fn preamble_and_universal_traits<'s, 'e>(&'s mut self, registry: &'r MultiVectorClassRegistry, emitter: &'e mut Emitter<std::fs::File>) -> std::io::Result<()> {
        // Preamble
        emitter.emit(&AstNode::Preamble)?;

        // Class Definitions
        for class in registry.classes.iter() {
            emitter.emit(&AstNode::ClassDefinition { class })?;
        }

        // Constants
        for param_a in registry.single_parameters() {
            let class_a = param_a.multi_vector_class();
            for name in &["Zero", "One"] {
                let ast_node = class_a.constant(name);
                emitter.emit(&ast_node).unwrap();
                if ast_node != AstNode::None {
                    self.trait_impls.add_class_impl(name, param_a.multi_vector_class(), ast_node);
                }
            }
        }

        // Uniform Grades
        for param_a in registry.single_parameters() {
            let class_a = param_a.multi_vector_class();

            let grade_unanimity = class_a.flat_basis().iter()
                .map(|a| (a.grade(), true))
                .reduce(|(a_grade, unanimous), (b_grade, _)| (a_grade, a_grade == b_grade && unanimous));

            if let Some((grade, true)) = grade_unanimity {
                let anti_grade = (self.algebra.anti_scalar_element().grade() as isize - grade as isize).unsigned_abs();
                let grade_impl = MultiVectorClass::derive_grade("Grade", &param_a, grade);
                emitter.emit(&grade_impl).unwrap();
                self.trait_impls.add_class_impl("Grade", param_a.multi_vector_class(), grade_impl);

                let anti_grade_impl = MultiVectorClass::derive_grade("AntiGrade", &param_a, anti_grade);
                emitter.emit(&anti_grade_impl).unwrap();
                self.trait_impls.add_class_impl("AntiGrade", param_a.multi_vector_class(), anti_grade_impl);
            }
        }

        // Involutions
        let involutions = Involution::involutions(&self.algebra);
        for param_a in registry.single_parameters() {
            // TODO for some reason not all involutions are being output for CGA,
            //  for example search "impl Dual" in cga3d.rs vs ppga3d.rs.
            //  This is strange because some involutions are written, like Reversal.
            for (name, involution) in involutions.iter() {
                let ast_node = MultiVectorClass::involution(name, involution, &param_a, registry, false);
                emitter.emit(&ast_node).unwrap();
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
                continue
            }
            let access = Expression {
                size: 1,
                data_type_hint: None,
                content: ExpressionContent::Access(
                    Box::new(variable(&param_a)),
                    0
                )
            };
            let sqrt = Expression {
                size: 1,
                data_type_hint: None,
                content: ExpressionContent::SquareRoot(Box::new(access))
            };
            let construct = Expression {
                size: 1,
                data_type_hint: Some(param_a.data_type.clone()),
                content: ExpressionContent::InvokeClassMethod(
                    param_a.multi_vector_class(),
                    "Constructor",
                    vec![(DataType::SimdVector(1), sqrt)]
                )
            };
            let name = "Sqrt";
            let sqrt = single_expression_single_trait_impl(name, &param_a, construct);
            emitter.emit(&sqrt).unwrap();
            self.trait_impls.add_single_impl(name, param_a, sqrt);
        }


        // Into
        for (param_a, param_b) in registry.pair_parameters() {
            let class_a = param_a.multi_vector_class();
            let class_b = param_b.multi_vector_class();
            if class_a != class_b {
                let ast_node = MultiVectorClass::involution("Into", &Involution::projection(param_b.multi_vector_class()), &param_a, registry, true);
                emitter.emit(&ast_node).unwrap();
                if ast_node != AstNode::None {
                    self.trait_impls.add_pair_impl("Into", param_a.clone(), param_b.clone(), ast_node);
                }
            }
        }

        // Add, Subtract
        for (param_a, param_b) in registry.pair_parameters() {
            for name in &["Add", "Sub"] {
                let ast_node = MultiVectorClass::element_wise(*name, &param_a, &param_b, registry);
                emitter.emit(&ast_node).unwrap();
                if ast_node != AstNode::None {
                    self.trait_impls.add_pair_impl(name, param_a.clone(), param_b.clone(), ast_node);
                }
            }
        }

        // Multiply, Divide
        for (param_a, param_b) in registry.pair_parameters() {
            if param_a.multi_vector_class() != param_b.multi_vector_class() {
                continue
            }
            for name in &["Mul", "Div"] {
                let ast_node = MultiVectorClass::element_wise(*name, &param_a, &param_b, registry);
                emitter.emit(&ast_node).unwrap();
                if ast_node != AstNode::None {
                    self.trait_impls.add_pair_impl(name, param_a.clone(), param_b.clone(), ast_node);
                }
            }
        }

        // Products from Geometric Algebra
        let products = Product::products(&self.algebra);
        for (param_a, param_b) in registry.pair_parameters() {
            for (name, product) in products.iter() {
                let ast_node = MultiVectorClass::product(name, product, &param_a, &param_b, registry);
                emitter.emit(&ast_node).unwrap();
                if ast_node != AstNode::None {
                    self.trait_impls.add_pair_impl(name, param_a.clone(), param_b.clone(), ast_node);
                }
            }
        }


        Ok(())
    }

    /// Step 2: Create some basic norms
    pub fn basic_norms<'s, 'e>(&'s mut self, registry: &'r MultiVectorClassRegistry, emitter: &'e mut Emitter<std::fs::File>) {
        // for param_a in registry.single_parameters() {
        //     let name = "SquaredMagnitude";
        //     let _: Option<()> = try {
        //         let reversal = self.trait_impls.get_single_invocation("Reversal", variable(&param_a))?;
        //         let dot = self.algebra.dialect().dot_product.first()?;
        //         let dot = self.trait_impls.get_pair_invocation(dot, variable(&param_a), reversal)?;
        //         let squared_magnitude = single_expression_single_trait_impl(name, &param_a, dot);
        //         emitter.emit(&squared_magnitude).unwrap();
        //         self.trait_impls.add_single_impl(name, param_a.clone(), squared_magnitude);
        //     };
        // }
        //
        // for param_a in registry.single_parameters() {
        //     let name = "Magnitude";
        //     let _: Option<()> = try {
        //         let sm = self.trait_impls.get_single_invocation("SquaredMagnitude", variable(&param_a))?;
        //         let m = self.trait_impls.get_single_invocation("Sqrt", sm)?;
        //         let magnitude = single_expression_single_trait_impl(name, &param_a, m);
        //         emitter.emit(&magnitude).unwrap();
        //         self.trait_impls.add_single_impl(name, param_a.clone(), magnitude);
        //     };
        // }

        // TODO BulkNormSquared, WeightNormSquared

        for param_a in registry.single_parameters() {
            let name = "BulkNorm";
            let _: Option<()> = try {
                let dot = self.algebra.dialect().dot_product.first()?;
                let dot = self.trait_impls.get_pair_invocation(dot, variable(&param_a), variable(&param_a))?;
                let m = self.trait_impls.get_single_invocation("Sqrt", dot)?;
                let bulk_norm = single_expression_single_trait_impl(name, &param_a, m);
                emitter.emit(&bulk_norm).unwrap();
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
                emitter.emit(&bulk_norm).unwrap();
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
                emitter.emit(&gn).unwrap();
                self.trait_impls.add_single_impl(name, param_a.clone(), gn);
            };
        }
    }

    /// Step 3: Create some fancy norms
    /// These items require some special insight per Geometric Algebra, for example if there are
    /// multiple/special projective dimensions with different meanings.
    pub fn fancy_norms(
        &mut self, prefix: &'static str, projective_basis: BasisElement, registry: &'r MultiVectorClassRegistry, emitter: &mut Emitter<std::fs::File>
    ) {
        // TODO this is all kinds of fucked currently. Not sure how to generate norms generically
        //  for both RGA and CGA. I could get all of RGA and the flat objects of CGA by doing
        //  bulk and weight first, then get bulkNorm and WeightNorm from those by doing the dot product
        //  square root thing. This doesn't work in the round objects though. Or maybe it does, and that is why
        //  "position/center/radius norm" are 3 different things instead of 2 different things. In any case.
        //  I need to figure out what's the deal with CenterNorm and RadiusNorm.

        todo!()
    }

    /// Step 4: Create some more stuff that depends on norms
    pub fn post_norm_universal_stuff<'s, 'e>(&'s mut self, registry: &'r MultiVectorClassRegistry, emitter: &'e mut Emitter<std::fs::File>) {

        // Scale
        for (param_a, param_b) in registry.pair_parameters() {
            let name = "Scale";
            if param_b.multi_vector_class().grouped_basis != vec![vec![self.algebra.scalar_element()]] {
                continue
            }
            let _: Option<()> = try {
                let gp = self.algebra.dialect().geometric_product.first()?;

                // These commented lines will implement "Scale" using Scalar
                // let gp = self.trait_impls.get_pair_invocation(gp, variable(&param_a), variable(&param_b))?;
                // let scale = single_expression_pair_trait_impl(name, &param_a, &param_b, gp);

                // The following implementation will implement "Scale" using SimdVector(1) instead of a Scalar
                let gp = self.trait_impls.get_pair_impl(gp, &param_a, &param_b)?;
                let scale = MultiVectorClass::derive_scale(name, gp, &param_a, &param_b);

                emitter.emit(&scale).unwrap();
                self.trait_impls.add_pair_impl(name, param_a, param_b, scale);
            };
        }

        // Signum
        for (param_a, param_b) in registry.pair_parameters() {
            let name = "Signum";
            if param_b.multi_vector_class().grouped_basis != vec![vec![self.algebra.scalar_element()]] {
                continue
            }
            let _: Option<()> = try {
                let gp = self.algebra.dialect().geometric_product.first()?;
                let gp = self.trait_impls.get_pair_impl(gp, &param_a, &param_b)?;
                let magnitude = self.trait_impls.get_single_impl("Magnitude", &param_a)?;
                let signum = MultiVectorClass::derive_signum("Signum", gp, magnitude, &param_a);

                emitter.emit(&signum).unwrap();
                self.trait_impls.add_pair_impl(name, param_a, param_b, signum);
            };
        }

        // Inverse
        for (param_a, param_b) in registry.pair_parameters() {
            if param_b.multi_vector_class().grouped_basis != vec![vec![self.algebra.scalar_element()]] {
                continue;
            }
            let _: Option<()> = try {
                let gp = self.algebra.dialect().geometric_product.first()?;
                let gp = self.trait_impls.get_pair_impl(gp, &param_a, &param_b)?;
                let squared_magnitude = self.trait_impls.get_single_impl("SquaredMagnitude", &param_a)?;
                let reversal = self.trait_impls.get_single_impl("Reversal", &param_a)?;
                let inverse = MultiVectorClass::derive_inverse("Inverse", gp, squared_magnitude, reversal, &param_a);
                emitter.emit(&inverse).unwrap();
                self.trait_impls.add_single_impl("Inverse", param_a.clone(), inverse);
            };
        }

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
                emitter.emit(&unitize).unwrap();
                self.trait_impls.add_single_impl(name, param_a.clone(), unitize);
            };
        }

        // Powi
        for param_a in registry.single_parameters() {
            let name = "Powi";
            let _: Option<()> = try {
                let gp = self.algebra.dialect().geometric_product.first()?;
                let (gp, gp_r) = self.trait_impls.get_pair_impl_and_result(gp, &param_a, &param_a)?;
                if gp_r.multi_vector_class() != param_a.multi_vector_class() {
                    continue
                }
                let constant_one = self.trait_impls.get_class_impl("One", param_a.multi_vector_class())?;
                let inverse = self.trait_impls.get_single_impl("Inverse", &param_a)?;
                let exponent = Parameter {
                    name: "exponent",
                    data_type: DataType::Integer,
                };
                let power_of_integer = MultiVectorClass::derive_power_of_integer(
                    name, gp, constant_one, inverse, &param_a, &exponent,
                );
                emitter.emit(&power_of_integer).unwrap();
                self.trait_impls.add_pair_impl(name, param_a, exponent, power_of_integer);
            };
        }

        for (param_a, param_b) in registry.pair_parameters() {
            let name = "GeometricQuotient";
            let _: Option<()> = try {
                let gp = self.algebra.dialect().geometric_product.first()?;
                let gp = self.trait_impls.get_pair_impl(gp, &param_a, &param_b)?;
                let inverse = self.trait_impls.get_single_impl("Inverse", &param_b)?;
                let division = MultiVectorClass::derive_division(name, gp, inverse, &param_a, &param_b);
                emitter.emit(&division).unwrap();
                self.trait_impls.add_pair_impl(name, param_a, param_b, division);
            };
        }

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

        for (param_a, param_b) in registry.pair_parameters() {
            let _: Option<()> = try {
                let gap = self.algebra.dialect().geometric_anti_product.first()?;
                let (gp, gp_r) = self.trait_impls.get_pair_impl_and_result(gap, &param_a, &param_b)?;
                let (reversal, reversal_r) = self.trait_impls.get_single_impl_and_result("AntiReversal", &param_a)?;
                let (gp2, gp2_r) = self.trait_impls.get_pair_impl_and_result(gap, &gp_r, &reversal_r)?;
                let into = self.trait_impls.get_pair_impl("Into", &gp2_r, &param_b);
                let sandwich = MultiVectorClass::derive_sandwich_product(
                    "Sandwich", gp, gp2, reversal, into, &param_a, &param_b
                );
                emitter.emit(&sandwich).unwrap();
                self.trait_impls.add_pair_impl("Sandwich", param_a, param_b, sandwich);
            };
        }

        for (param_a, param_b) in registry.pair_parameters() {
            // Invert (Inversion)
            // https://rigidgeometricalgebra.org/wiki/index.php?title=Inversion
            // The choice of what class should constitute a "Point" is somewhat contrived.
            // It will need extra consideration for CGA.
            if param_a.multi_vector_class().class_name != "Point" {
                continue
            }
            let name = "Invert";
            let _: Option<()> = try {
                let unitize = self.trait_impls.get_single_invocation("Unitize", variable(&param_a))?;
                let sandwich = self.trait_impls.get_pair_invocation("Sandwich", unitize, variable(&param_b))?;
                let i = single_expression_pair_trait_impl(name, &param_a, &param_b, sandwich);
                emitter.emit(&i).unwrap();
                self.trait_impls.add_pair_impl(name, param_a, param_b, i);
            };
        }

        for (param_a, param_b) in registry.pair_parameters() {
            // Reflect (Reflection)
            // https://rigidgeometricalgebra.org/wiki/index.php?title=Reflection
            // The choice of what class should constitute a "Plane" is somewhat contrived.
            // It will need extra consideration for CGA.
            if param_a.multi_vector_class().class_name != "Plane" {
                continue
            }
            let name = "Reflect";
            let _: Option<()> = try {
                let unitize = self.trait_impls.get_single_invocation("Unitize", variable(&param_a))?;
                let sandwich = self.trait_impls.get_pair_invocation("Sandwich", unitize, variable(&param_b))?;
                let i = single_expression_pair_trait_impl(name, &param_a, &param_b, sandwich);
                emitter.emit(&i).unwrap();
                self.trait_impls.add_pair_impl(name, param_a, param_b, i);
            };
        }

        for param_a in registry.single_parameters() {
            let projective_basis = if self.algebra.algebra_name() == "rga3d" {
                // TODO rename to e4 when the time is right
                self.algebra.parse("e3")
            } else {
                break
            };

            let bulk = MultiVectorClass::derive_bulk_or_weight(
                "Bulk", &param_a, &projective_basis, false, &self.algebra, registry
            );
            if bulk != AstNode::None {
                emitter.emit(&bulk).unwrap();
                self.trait_impls.add_single_impl("Bulk", param_a.clone(), bulk);
            }

            let weight = MultiVectorClass::derive_bulk_or_weight(
                "Weight", &param_a, &projective_basis, true, &self.algebra, registry
            );
            if weight != AstNode::None {
                emitter.emit(&weight).unwrap();
                self.trait_impls.add_single_impl("Weight", param_a, weight);
            }
        }


        for param_a in registry.single_parameters() {
            let name = "RightBulkDual";
            let _: Option<()> = try {
                // Right bulk dual is right complement of bulk
                let bulk = self.trait_impls.get_single_invocation("Bulk", variable(&param_a))?;
                let right_comp = self.trait_impls.get_single_invocation("RightComplement", bulk)?;
                let rbd = single_expression_single_trait_impl(name, &param_a, right_comp);
                emitter.emit(&rbd).unwrap();
                self.trait_impls.add_single_impl(name, param_a, rbd);
            };
        }
        for param_a in registry.single_parameters() {
            let name = "RightWeightDual";
            let _: Option<()> = try {
                // Right weight dual is right complement of weight
                let weight = self.trait_impls.get_single_invocation("Weight", variable(&param_a))?;
                let right_comp = self.trait_impls.get_single_invocation("RightComplement", weight)?;
                let rwd = single_expression_single_trait_impl(name, &param_a, right_comp);
                emitter.emit(&rwd).unwrap();
                self.trait_impls.add_single_impl(name, param_a, rwd);
            };
        }
        for param_a in registry.single_parameters() {
            let name = "LeftBulkDual";
            let _: Option<()> = try {
                // Left bulk dual is left complement of bulk
                let bulk = self.trait_impls.get_single_invocation("Bulk", variable(&param_a))?;
                let left_comp = self.trait_impls.get_single_invocation("LeftComplement", bulk)?;
                let lbd = single_expression_single_trait_impl(name, &param_a, left_comp);
                emitter.emit(&lbd).unwrap();
                self.trait_impls.add_single_impl(name, param_a, lbd);
            };
        }
        for param_a in registry.single_parameters() {
            let name = "LeftWeightDual";
            let _: Option<()> = try {
                // Left weight dual is left complement of weight
                let weight = self.trait_impls.get_single_invocation("Weight", variable(&param_a))?;
                let left_comp = self.trait_impls.get_single_invocation("LeftComplement", weight)?;
                let lwd = single_expression_single_trait_impl(name, &param_a, left_comp);
                emitter.emit(&lwd).unwrap();
                self.trait_impls.add_single_impl(name, param_a, lwd);
            };
        }


        for (param_a, param_b) in registry.pair_parameters() {
            let name = "BulkContraction";
            let _: Option<()> = try {
                // Bulk contraction is the antiwedge on a right bulk dual
                let rbd = self.trait_impls.get_single_invocation("RightBulkDual", variable(&param_b))?;
                let aw = self.trait_impls.get_pair_invocation("AntiWedge", variable(&param_a), rbd)?;
                let bc = single_expression_pair_trait_impl(name, &param_a, &param_b, aw);
                emitter.emit(&bc).unwrap();
                self.trait_impls.add_pair_impl(name, param_a, param_b, bc);
            };
        }
        for (param_a, param_b) in registry.pair_parameters() {
            let name = "WeightContraction";
            let _: Option<()> = try {
                // Weight contraction is the antiwedge on a right weight dual
                let rwd = self.trait_impls.get_single_invocation("RightWeightDual", variable(&param_b))?;
                let aw = self.trait_impls.get_pair_invocation("AntiWedge", variable(&param_a), rwd)?;
                let wc = single_expression_pair_trait_impl(name, &param_a, &param_b, aw);
                emitter.emit(&wc).unwrap();
                self.trait_impls.add_pair_impl(name, param_a, param_b, wc);
            };
        }
        for (param_a, param_b) in registry.pair_parameters() {
            let name = "BulkExpansion";
            let _: Option<()> = try {
                // Bulk expansion is the wedge on a right bulk dual
                let rbd = self.trait_impls.get_single_invocation("RightBulkDual", variable(&param_b))?;
                let w = self.trait_impls.get_pair_invocation("Wedge", variable(&param_a), rbd)?;
                let be = single_expression_pair_trait_impl(name, &param_a, &param_b, w);
                emitter.emit(&be).unwrap();
                self.trait_impls.add_pair_impl(name, param_a, param_b, be);
            };
        }
        for (param_a, param_b) in registry.pair_parameters() {
            let name = "WeightExpansion";
            let _: Option<()> = try {
                // Weight expansion is the wedge on a right weight dual
                let rwd = self.trait_impls.get_single_invocation("RightWeightDual", variable(&param_b))?;
                let w = self.trait_impls.get_pair_invocation("Wedge", variable(&param_a), rwd)?;
                let we = single_expression_pair_trait_impl(name, &param_a, &param_b, w);
                emitter.emit(&we).unwrap();
                self.trait_impls.add_pair_impl(name, param_a, param_b, we);
            };
        }


        // TODO I feel like (but am not sure) there might be excess implementations of ProjectOnto and AntiProjectOnto.
        //  https://rigidgeometricalgebra.org/wiki/index.php?title=Projections
        //  The article shows a clear pattern of projecting lower dimensions onto higher dimensions, and vice versa
        //  for anti-projection. Currently I am getting impls for all kinds of combinations that violate that pattern.
        //  Such as for example projecting a scalar onto a line. I don't even know what that means.
        //  It might be the case that these implementations really do stuff, and we just don't know what it means yet.
        //  Or it is possible that these implementations are degenerate cases and we'd rather omit them.
        //  I'll have to play around and test to find out.

        for (param_a, param_b) in registry.pair_parameters() {
            let name = "ProjectOrthogonallyOnto";
            let _: Option<()> = try {
                let we = self.trait_impls.get_pair_invocation("WeightExpansion", variable(&param_a), variable(&param_b))?;
                let anti_wedge = self.algebra.dialect().exterior_anti_product.first()?;
                let anti_wedge = self.trait_impls.get_pair_invocation(anti_wedge, variable(&param_b), we)?;
                let po = single_expression_pair_trait_impl(name, &param_a, &param_b, anti_wedge);
                emitter.emit(&po).unwrap();
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
                emitter.emit(&apo).unwrap();
                self.trait_impls.add_pair_impl(name, param_a, param_b, apo);
            };
        }
        for (param_a, param_b) in registry.pair_parameters() {
            let name = "ProjectThroughOriginOnto";
            let _: Option<()> = try {
                let be = self.trait_impls.get_pair_invocation("BulkExpansion", variable(&param_a), variable(&param_b))?;
                let anti_wedge = self.algebra.dialect().exterior_anti_product.first()?;
                let anti_wedge = self.trait_impls.get_pair_invocation(anti_wedge, variable(&param_b), be)?;
                let po = single_expression_pair_trait_impl(name, &param_a, &param_b, anti_wedge);
                emitter.emit(&po).unwrap();
                self.trait_impls.add_pair_impl(name, param_a, param_b, po);
            };
        }
        for (param_a, param_b) in registry.pair_parameters() {
            let name = "AntiProjectThroughOriginOnto";
            let _: Option<()> = try {
                let bc = self.trait_impls.get_pair_invocation("BulkContraction", variable(&param_a), variable(&param_b))?;
                let wedge = self.algebra.dialect().exterior_product.first()?;
                let wedge = self.trait_impls.get_pair_invocation(wedge, variable(&param_b), bc)?;
                let apo = single_expression_pair_trait_impl(name, &param_a, &param_b, wedge);
                emitter.emit(&apo).unwrap();
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
                let cosine = single_expression_pair_trait_impl(name, &param_a, &param_b, bn);
                emitter.emit(&cosine).unwrap();
                self.trait_impls.add_pair_impl(name, param_a, param_b, cosine);
            };
        }

        for (param_a, param_b) in registry.pair_parameters() {
            let name = "SineAngle";
            let _: Option<()> = try {
                let cos = self.trait_impls.get_pair_invocation("CosineAngle", variable(&param_a), variable(&param_b))?;
                let scalar = match cos.data_type_hint {
                    Some(DataType::MultiVector(scalar)) => scalar,
                    _ => continue
                };
                let one = self.trait_impls.get_class_invocation("One", scalar)?;
                let const2 = Expression {
                    size: 1,
                    content: ExpressionContent::Constant(DataType::Integer, vec![2]),
                    data_type_hint: Some(DataType::Integer)
                };
                let pow2 = self.trait_impls.get_pair_invocation("Powi", cos, const2)?;
                let sub = self.trait_impls.get_pair_invocation("Sub", one, pow2)?;
                let sqrt = self.trait_impls.get_single_invocation("Sqrt", sub)?;
                let sine = single_expression_pair_trait_impl(name, &param_a, &param_b, sqrt);
                emitter.emit(&sine).unwrap();
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
    pub fn attitude_and_dependencies<'s, 'e>(&'s mut self, origin_class_name: &str, registry: &'r MultiVectorClassRegistry, emitter: &'e mut Emitter<std::fs::File>) {

        // Attitude
        for param_a in registry.single_parameters() {
            let name = "Attitude";
            let _: Option<()> = try {
                let origin = registry.classes.iter().find(|it| it.class_name == origin_class_name)?;
                let one = self.trait_impls.get_class_invocation("One", origin)?;
                let rc = self.trait_impls.get_single_invocation("RightComplement", one)?;
                let anti_wedge = self.algebra.dialect().exterior_anti_product.first()?;
                let anti_wedge = self.trait_impls.get_pair_invocation(anti_wedge, variable(&param_a), rc)?;
                let attitude = single_expression_single_trait_impl(name, &param_a, anti_wedge);

                // TODO do I need to do any special output type derivation? or does it figure itself out
                //  on its own?
                emitter.emit(&attitude).unwrap();
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
                emitter.emit(&ed).unwrap();
                self.trait_impls.add_pair_impl(name, param_a, param_b, ed);
            };
        }
    }
}

// TODO replace with direct codegen instead of naga output, but still validate with naga.
fn do_wgsl(algebra_name: &str, file_path: PathBuf) {
    // Let naga do wgsl:
    // - Good because low maintenance here.
    // - Bad because it erases useful comments.

    // Prepare some of naga's clutter
    let mut glsl_frontend = naga::front::glsl::Frontend::default();
    let mut wgsl_backend = naga::back::wgsl::Writer::new(String::new(), WriterFlags::EXPLICIT_TYPES);
    let mut validator = naga::valid::Validator::new(ValidationFlags::default(), Capabilities::default());
    let options = naga::front::glsl::Options {
        stage: ShaderStage::Compute,
        defines: Default::default(),
    };

    // Read the glsl
    let mut glsl_file = std::fs::File::open(file_path.with_extension("glsl")).unwrap();
    let mut glsl_contents = String::new();
    glsl_file.read_to_string(&mut glsl_contents).unwrap();
    // Append a dummy entry point
    glsl_contents.push_str("\nvoid main() {}");

    // Parse, prune, and validate the naga module
    let module = match glsl_frontend.parse(&options, glsl_contents.as_str()) {
        Ok(m) => m,
        Err(err) => {
            let mut line = "??".to_string();
            if let Some(Error { meta, .. }) = err.first() {
                line = meta.location(glsl_contents.as_str()).line_number.to_string();
            }
            panic!("Error generating {algebra_name} on line {line}: {err:?}")
        }
    };
    let mut pruner = naga_oil::prune::Pruner::new(&module);
    for (hf, _) in module.functions.iter() {
        pruner.add_function(hf, HashMap::new(), Some(PartReq::All));
    }
    let module = pruner.rewrite();
    let module_info = validator.validate(&module).unwrap();

    // Write the wgsl
    wgsl_backend.write(&module, &module_info).unwrap();
    let wgsl_contents = wgsl_backend.finish();
    let mut wgsl_file = std::fs::File::create(file_path.with_extension("wgsl")).unwrap();
    wgsl_file.write(wgsl_contents.as_bytes()).unwrap();
}
