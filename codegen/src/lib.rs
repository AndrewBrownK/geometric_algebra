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

use crate::{
    algebra::{BasisElement, GeometricAlgebra, Involution, MultiVectorClass, MultiVectorClassRegistry, Product},
    ast::{AstNode, DataType, Parameter},
    emit::Emitter,
};
use crate::ast::{Expression, ExpressionContent};
use crate::compile::{single_expression_pair_trait_impl, single_expression_single_trait_impl, variable};

pub mod algebra;
mod ast;
mod compile;
mod emit;
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
    let algebra = GeometricAlgebra {
        generator_squares: generator_squares.as_slice(),
    };
    let mut multi_vectors = vec![];
    for multi_vector_descriptor in config_iter {
        let mut multi_vector_descriptor_iter = multi_vector_descriptor.split(':');
        multi_vectors.push(MultiVectorClass {
            class_name: multi_vector_descriptor_iter.next().unwrap().to_owned(),
            grouped_basis: multi_vector_descriptor_iter
                .next()
                .unwrap()
                .split('|')
                .map(|group_descriptor| {
                    group_descriptor
                        .split(',')
                        .map(|element_name| BasisElement::parse(element_name, &algebra))
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>(),
        });
    }

    AlgebraDescriptor {
        algebra_name: algebra_name.to_string(),
        generator_squares,
        multi_vectors,
    }
}

struct TraitImpls<'a> {
    singles: BTreeMap<(String, String), AstNode<'a>>,
    pairs: BTreeMap<(String, String, String), AstNode<'a>>
}

impl<'a> TraitImpls<'a> {
    fn new() -> Self {
        TraitImpls {
            singles: BTreeMap::new(),
            pairs: BTreeMap::new(),
        }
    }

    fn add_pair_impl(&mut self, name: &str, parameter_a: Parameter<'a>, parameter_b: Parameter<'a>, the_impl: AstNode<'a>) {
        let a_name = parameter_a.multi_vector_class().class_name.clone();
        let b_name = parameter_b.multi_vector_class().class_name.clone();
        self.pairs.insert((name.to_string(), a_name.to_string(), b_name.to_string()), the_impl);
    }

    fn add_single_impl(&mut self, name: &str, parameter_a: Parameter<'a>, the_impl: AstNode<'a>) {
        let a_name = parameter_a.multi_vector_class().class_name.clone();
        self.singles.insert((name.to_string(), a_name.to_string()), the_impl);
    }

    fn get_pair_impl(&self, name: &str, parameter_a: &Parameter<'a>, parameter_b: &Parameter<'a>) -> Option<&AstNode<'a>> {
        let a_name = parameter_a.multi_vector_class().class_name.clone();
        let b_name = parameter_b.multi_vector_class().class_name.clone();
        return self.pairs.get(&(name.to_string(), a_name, b_name));
    }

    fn get_single_impl(&self, name: &str, parameter_a: &Parameter<'a>) -> Option<&AstNode<'a>> {
        let a_name = parameter_a.multi_vector_class().class_name.clone();
        return self.singles.get(&(name.to_string(), a_name));
    }

    fn get_pair_impl_and_result(&self, name: &str, parameter_a: &Parameter<'a>, parameter_b: &Parameter<'a>) -> Option<(&AstNode<'a>, &Parameter<'a>)> {
        let a_name = parameter_a.multi_vector_class().class_name.clone();
        let b_name = parameter_b.multi_vector_class().class_name.clone();
        let the_impl = self.pairs.get(&(name.to_string(), a_name, b_name))?;
        let result = result_of_trait!(the_impl);
        return Some((the_impl, result));
    }

    fn get_single_impl_and_result(&self, name: &str, parameter_a: &Parameter<'a>) -> Option<(&AstNode<'a>, &Parameter<'a>)> {
        let a_name = parameter_a.multi_vector_class().class_name.clone();
        let the_impl = self.singles.get(&(name.to_string(), a_name))?;
        let result = result_of_trait!(the_impl);
        return Some((the_impl, result));
    }

    fn get_pair_invocation(&self, name: &str, a: Expression<'a>, b: Expression<'a>) -> Option<Expression<'a>> {
        let class_a = match a.data_type_hint {
            Some(DataType::MultiVector(c)) => c,
            _ => panic!("TraitImpls.get_pair_invocation for {name} requires MultiVectorClass data_type_hints on \"a\" {a:?}"),
        };
        let class_b = match b.data_type_hint {
            Some(DataType::MultiVector(c)) => c,
            _ => panic!("TraitImpls.get_pair_invocation for {name} requires MultiVectorClass data_type_hints on \"b\" {b:?}"),
        };
        let a_name = class_a.class_name.clone();
        let b_name = class_b.class_name.clone();
        let the_impl = self.pairs.get(&(name.to_string(), a_name, b_name))?;
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
                DataType::MultiVector(class_a),
                Box::new(a),
                result.name,
                vec![
                    (DataType::MultiVector(class_b), b)
                ]
            ),
        })
    }

    fn get_single_invocation(&self, name: &str, a: Expression<'a>) -> Option<Expression<'a>> {
        let class_a = match a.data_type_hint {
            Some(DataType::MultiVector(c)) => c,
            _ => panic!("TraitImpls.get_single_invocation for {name} requires MultiVectorClass data_type_hints on \"a\" {a:?}"),
        };
        let a_name = class_a.class_name.clone();
        let the_impl = self.singles.get(&(name.to_string(), a_name))?;
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
                DataType::MultiVector(class_a),
                Box::new(a),
                result.name,
                vec![]
            ),
        })
    }
}


// TODO hack in cga stuff
pub fn generate_code(desc: AlgebraDescriptor, path: &str) {
    let algebra = GeometricAlgebra {
        generator_squares: desc.generator_squares.as_slice(),
    };
    let involutions = Involution::involutions(&algebra);
    let products = Product::products(&algebra);
    let basis = algebra.sorted_basis();
    for b in basis.iter() {
        for a in basis.iter() {
            print!("{:1$} ", BasisElement::product(a, b, &algebra), algebra.generator_squares.len() + 2);
        }
        println!();
    }
    let mut registry = MultiVectorClassRegistry::default();
    for multi_vector_class in desc.multi_vectors {
        registry.register(multi_vector_class);
    }
    let algebra_name = desc.algebra_name.as_str();
    let file_path = std::path::Path::new(&path).join(std::path::Path::new(algebra_name));
    let mut emitter = Emitter::new(&file_path);
    emitter.emit(&AstNode::Preamble).unwrap();
    for class in registry.classes.iter() {
        emitter.emit(&AstNode::ClassDefinition { class }).unwrap();
    }

    let mut trait_impls = TraitImpls::new();

    for param_a in registry.single_parameters() {
        let class_a = param_a.multi_vector_class();
        for name in &["Zero", "One"] {
            let ast_node = class_a.constant(name);
            emitter.emit(&ast_node).unwrap();
            if ast_node != AstNode::None {
                trait_impls.add_single_impl(name, param_a.clone(), ast_node);
            }
        }

        if let Some((grade, unanimous)) = class_a.flat_basis().iter().map(|a| (a.grade(), true)).reduce(|a, b| (a.0, a.0 == b.0 && a.1 && b.1)) {
            if unanimous {
                let anti_grade = algebra.generator_squares.len() - grade;
                let grade_impl = MultiVectorClass::derive_grade("Grade", &param_a, grade);
                emitter.emit(&grade_impl).unwrap();
                trait_impls.add_single_impl("Grade", param_a.clone(), grade_impl);

                let anti_grade_impl = MultiVectorClass::derive_grade("AntiGrade", &param_a, anti_grade);
                emitter.emit(&anti_grade_impl).unwrap();
                trait_impls.add_single_impl("AntiGrade", param_a.clone(), anti_grade_impl);
            }
        }

        // TODO for some reason not all involutions are being output for CGA,
        //  for example search "impl Dual" in cga3d.rs vs ppga3d.rs.
        //  This is strange because some involutions are written, like Reversal.
        //  I am wondering if this is because of the extra projective dimension. Sheesh.
        for (name, involution) in involutions.iter() {
            let ast_node = MultiVectorClass::involution(name, involution, &param_a, &registry, false);
            emitter.emit(&ast_node).unwrap();
            if ast_node != AstNode::None {
                trait_impls.add_single_impl(name, param_a.clone(), ast_node);
            }
        }
    }


    for (param_a, param_b) in registry.pair_parameters() {
        let class_a = param_a.multi_vector_class();
        let class_b = param_b.multi_vector_class();

        if class_a != class_b {
            let ast_node = MultiVectorClass::involution("Into", &Involution::projection(param_b.multi_vector_class()), &param_a, &registry, true);
            emitter.emit(&ast_node).unwrap();
            if ast_node != AstNode::None {
                trait_impls.add_pair_impl("Into", param_a.clone(), param_b.clone(), ast_node);
            }
        }
        for name in &["Add", "Sub"] {
            let ast_node = MultiVectorClass::element_wise(*name, &param_a, &param_b, &registry);
            emitter.emit(&ast_node).unwrap();
            if ast_node != AstNode::None {
                trait_impls.add_pair_impl(name, param_a.clone(), param_b.clone(), ast_node);
            }
        }
        if class_a == class_b {
            for name in &["Mul", "Div"] {
                let ast_node = MultiVectorClass::element_wise(*name, &param_a, &param_b, &registry);
                emitter.emit(&ast_node).unwrap();
                if ast_node != AstNode::None {
                    trait_impls.add_pair_impl(name, param_a.clone(), param_b.clone(), ast_node);
                }
            }
        }

        // TODO I think here is the critical spot for CGA
        //  hmm.. I'm looking at "impl GeometricProduct<Motor>" as a sanity test
        //  So far it only exists for round objects, and does not output the object its transforming
        //  You can see from this source that multiplying by e5 is the correct idea
        //  https://conformalgeometricalgebra.com/wiki/index.php?title=Translation
        //  So I bet it is my extra projective is throwing things off
        for (name, product) in products.iter() {
            let ast_node = MultiVectorClass::product(name, product, &param_a, &param_b, &registry);
            emitter.emit(&ast_node).unwrap();
            if ast_node != AstNode::None {
                trait_impls.add_pair_impl(name, param_a.clone(), param_b.clone(), ast_node);
            }
        }
    }


    for (param_a, param_b) in registry.pair_parameters() {
        if param_a.multi_vector_class() != param_b.multi_vector_class() {
            continue
        }
        let _: Option<()> = try {
            let scalar_product = trait_impls.get_pair_impl("ScalarProduct", &param_a, &param_b)?;
            let reversal = trait_impls.get_single_impl("Reversal", &param_a)?;

            let squared_magnitude = MultiVectorClass::derive_squared_magnitude("SquaredMagnitude", scalar_product, reversal, &param_a);
            let magnitude = MultiVectorClass::derive_magnitude("Magnitude", &squared_magnitude, &param_a);
            let bulk_norm = MultiVectorClass::derive_magnitude("BulkNorm", &squared_magnitude, &param_a);

            emitter.emit(&squared_magnitude).unwrap();
            emitter.emit(&magnitude).unwrap();
            emitter.emit(&bulk_norm).unwrap();

            trait_impls.add_single_impl("SquaredMagnitude", param_a.clone(), squared_magnitude);
            trait_impls.add_single_impl("Magnitude", param_a.clone(), magnitude);
            trait_impls.add_single_impl("BulkNorm", param_a.clone(), bulk_norm);
        };
    }

    for (param_a, param_b) in registry.pair_parameters() {
        if param_a.multi_vector_class() != param_b.multi_vector_class() {
            continue
        }
        let _: Option<()> = try {
            let (bulk_norm, bulk_norm_result) = trait_impls.get_single_impl_and_result("BulkNorm", &param_a)?;
            let anti_scalar_product = trait_impls.get_pair_impl("AntiScalarProduct", &param_a, &param_b)?;
            let anti_reversal = trait_impls.get_single_impl("AntiReversal", &param_a)?;

            let squared_anti_magnitude = MultiVectorClass::derive_squared_magnitude("SquaredAntiMagnitude", anti_scalar_product, anti_reversal, &param_a);
            let weight_norm = MultiVectorClass::derive_magnitude("WeightNorm", &squared_anti_magnitude, &param_a);
            let weight_norm_result = result_of_trait!(weight_norm);
            let add = trait_impls.get_pair_impl("Add", bulk_norm_result, weight_norm_result)?;
            let geometric_norm = MultiVectorClass::derive_geometric_norm("GeometricNorm", &bulk_norm, &weight_norm, &registry, &param_a, &add);

            emitter.emit(&squared_anti_magnitude).unwrap();
            emitter.emit(&weight_norm).unwrap();
            emitter.emit(&geometric_norm).unwrap();

            trait_impls.add_single_impl("SquaredAntiMagnitude", param_a.clone(), squared_anti_magnitude);
            trait_impls.add_single_impl("WeightNorm", param_a.clone(), weight_norm);
            trait_impls.add_single_impl("GeometricNorm", param_a.clone(), geometric_norm);
        };
    }


    for (param_a, param_b) in registry.pair_parameters() {
        if param_b.multi_vector_class().grouped_basis != vec![vec![BasisElement::from_index(0)]] {
            continue;
        }
        let _: Option<()> = try {
            let gp = trait_impls.get_pair_impl("GeometricProduct", &param_a, &param_b)?;
            let scale = MultiVectorClass::derive_scale("Scale", gp, &param_a, &param_b);
            let magnitude = trait_impls.get_single_impl("Magnitude", &param_a)?;
            let signum = MultiVectorClass::derive_signum("Signum", gp, magnitude, &param_a);

            emitter.emit(&scale).unwrap();
            emitter.emit(&signum).unwrap();

            trait_impls.add_single_impl("Scale", param_a.clone(), scale);
            trait_impls.add_single_impl("Signum", param_a.clone(), signum);
        };
    }

    for (param_a, param_b) in registry.pair_parameters() {
        if param_b.multi_vector_class().grouped_basis != vec![vec![BasisElement::from_index(0)]] {
            continue;
        }
        let _: Option<()> = try {
            let gp = trait_impls.get_pair_impl("GeometricProduct", &param_a, &param_b)?;
            let squared_magnitude = trait_impls.get_single_impl("SquaredMagnitude", &param_a)?;
            let reversal = trait_impls.get_single_impl("Reversal", &param_a)?;
            let inverse = MultiVectorClass::derive_inverse("Inverse", gp, squared_magnitude, reversal, &param_a);
            emitter.emit(&inverse).unwrap();
            trait_impls.add_single_impl("Inverse", param_a.clone(), inverse);
        };
    }


    for (param_a, param_b) in registry.pair_parameters() {
        if param_b.multi_vector_class().grouped_basis != vec![vec![BasisElement::from_index(0)]] {
            continue;
        }
        let _: Option<()> = try {
            let gp = trait_impls.get_pair_impl("GeometricProduct", &param_a, &param_b)?;
            let weight_norm = trait_impls.get_single_impl("WeightNorm", &param_a)?;
            let unitize = MultiVectorClass::derive_unitize("Unitize", gp, weight_norm, &param_a, &param_b);
            emitter.emit(&unitize).unwrap();
            trait_impls.add_single_impl("Unitize", param_a.clone(), unitize);
        };
    }

    for (param_a, param_b) in registry.pair_parameters() {
        let _: Option<()> = try {
            let anti_wedge_product = trait_impls.get_pair_impl("RegressiveProduct", &param_a, &param_b)?;
            let bases = param_b.multi_vector_class().flat_basis();
            let nzd = algebra.generator_squares.iter().filter(|it| **it != 0isize).count();

            if bases.iter().any(|it| it.grade() != nzd) {
                continue
            }

            let special_base = bases.iter().find(|it| BasisElement::product(&it, &it, &algebra).scalar != 0)?;

            let attitude = MultiVectorClass::derive_attitude(
                "Attitude", anti_wedge_product, &param_a, &param_b, &special_base
            );
            emitter.emit(&attitude).unwrap();
            trait_impls.add_single_impl("Attitude", param_a.clone(), attitude);
        };
    }

    for (param_a, param_b) in registry.pair_parameters() {
        if param_a.multi_vector_class() != param_b.multi_vector_class() {
            continue
        }
        let _: Option<()> = try {
            let (gp, gp_r) = trait_impls.get_pair_impl_and_result("GeometricProduct", &param_a, &param_b)?;
            if gp_r.multi_vector_class() != param_a.multi_vector_class() {
                continue
            }
            let constant_one = trait_impls.get_single_impl("One", &param_a)?;
            let inverse = trait_impls.get_single_impl("Inverse", &param_a)?;
            let exponent = Parameter {
                name: "exponent",
                data_type: DataType::Integer,
            };
            let power_of_integer = MultiVectorClass::derive_power_of_integer(
                "Powi", gp, constant_one, inverse, &param_a, &exponent,
            );
            emitter.emit(&power_of_integer).unwrap();
        };
    }

    for (param_a, param_b) in registry.pair_parameters() {
        let _: Option<()> = try {
            let gp = trait_impls.get_pair_impl("GeometricProduct", &param_a, &param_b)?;
            let inverse = trait_impls.get_single_impl("Inverse", &param_b)?;
            let division = MultiVectorClass::derive_division("GeometricQuotient", gp, inverse, &param_a, &param_b);
            emitter.emit(&division).unwrap();
            trait_impls.add_pair_impl("GeometricQuotient", param_a, param_b, division);
        };
    }

    for (param_a, param_b) in registry.pair_parameters() {
        let _: Option<()> = try {
            let (gp, gp_r) = trait_impls.get_pair_impl_and_result("GeometricProduct", &param_a, &param_b)?;
            let reversal = trait_impls.get_single_impl("Reversal", &param_a)?;
            let (gp2, gp2_r) = trait_impls.get_pair_impl_and_result("GeometricProduct", &gp_r, &param_a)?;
            let into = trait_impls.get_pair_impl("Into", &gp2_r, &param_b);
            let transformation = MultiVectorClass::derive_sandwich_product(
                "Transformation", gp, gp2, reversal, into, &param_a, &param_b
            );
            emitter.emit(&transformation).unwrap();
            trait_impls.add_pair_impl("Transformation", param_a, param_b, transformation);
        };
    }

    for (param_a, param_b) in registry.pair_parameters() {
        let _: Option<()> = try {
            let (gp, gp_r) = trait_impls.get_pair_impl_and_result("GeometricAntiProduct", &param_a, &param_b)?;
            let (reversal, reversal_r) = trait_impls.get_single_impl_and_result("AntiReversal", &param_a)?;
            let (gp2, gp2_r) = trait_impls.get_pair_impl_and_result("GeometricAntiProduct", &gp_r, &reversal_r)?;
            let into = trait_impls.get_pair_impl("Into", &gp2_r, &param_b);
            let sandwich = MultiVectorClass::derive_sandwich_product(
                "Sandwich", gp, gp2, reversal, into, &param_a, &param_b
            );
            emitter.emit(&sandwich).unwrap();
            trait_impls.add_pair_impl("Sandwich", param_a, param_b, sandwich);
        };
    }

    for (param_a, param_b) in registry.pair_parameters() {
        // https://rigidgeometricalgebra.org/wiki/index.php?title=Euclidean_distance
        let name = "Distance";
        let _: Option<()> = try {
            let bulk_wedge = trait_impls.get_pair_invocation("Wedge", variable(&param_a), variable(&param_b))?;
            let bulk_attitude = trait_impls.get_single_invocation("Attitude", bulk_wedge)?;
            let weight_attitude = trait_impls.get_single_invocation("Attitude", variable(&param_b))?;
            let weight_wedge = trait_impls.get_pair_invocation("Wedge", variable(&param_a), weight_attitude)?;
            let bulk_norm = trait_impls.get_single_invocation("BulkNorm", bulk_attitude)?;
            let weight_norm = trait_impls.get_single_invocation("WeightNorm", weight_wedge)?;
            let add = trait_impls.get_pair_invocation("Add", bulk_norm, weight_norm)?;
            let ed = single_expression_pair_trait_impl(name, &param_a, &param_b, add);
            emitter.emit(&ed).unwrap();
            trait_impls.add_pair_impl(name, param_a, param_b, ed);
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
            let unitize = trait_impls.get_single_invocation("Unitize", variable(&param_a))?;
            let sandwich = trait_impls.get_pair_invocation("Sandwich", unitize, variable(&param_b))?;
            let i = single_expression_pair_trait_impl(name, &param_a, &param_b, sandwich);
            emitter.emit(&i).unwrap();
            trait_impls.add_pair_impl(name, param_a, param_b, i);
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
            let unitize = trait_impls.get_single_invocation("Unitize", variable(&param_a))?;
            let sandwich = trait_impls.get_pair_invocation("Sandwich", unitize, variable(&param_b))?;
            let i = single_expression_pair_trait_impl(name, &param_a, &param_b, sandwich);
            emitter.emit(&i).unwrap();
            trait_impls.add_pair_impl(name, param_a, param_b, i);
        };
    }

    for param_a in registry.single_parameters() {
        let projective_basis = if desc.algebra_name == "rga3d" {
            // TODO rename to e4 when the time is right
            BasisElement::parse("e3", &algebra)
        } else {
            break
        };

        let bulk = MultiVectorClass::derive_bulk_or_weight(
            "Bulk", &param_a, &projective_basis, false, &algebra, &registry
        );
        emitter.emit(&bulk).unwrap();
        trait_impls.add_single_impl("Bulk", param_a.clone(), bulk);

        let weight = MultiVectorClass::derive_bulk_or_weight(
            "Weight", &param_a, &projective_basis, true, &algebra, &registry
        );
        emitter.emit(&weight).unwrap();
        trait_impls.add_single_impl("Weight", param_a, weight);
    }





    for param_a in registry.single_parameters() {
        let name = "RightBulkDual";
        let _: Option<()> = try {
            // Right bulk dual is right complement of bulk
            let bulk = trait_impls.get_single_invocation("Bulk", variable(&param_a))?;
            let right_comp = trait_impls.get_single_invocation("RightComplement", bulk)?;
            let rbd = single_expression_single_trait_impl(name, &param_a, right_comp);
            emitter.emit(&rbd).unwrap();
            trait_impls.add_single_impl(name, param_a, rbd);
        };
    }
    for param_a in registry.single_parameters() {
        let name = "RightWeightDual";
        let _: Option<()> = try {
            // Right weight dual is right complement of weight
            let weight = trait_impls.get_single_invocation("Weight", variable(&param_a))?;
            let right_comp = trait_impls.get_single_invocation("RightComplement", weight)?;
            let rwd = single_expression_single_trait_impl(name, &param_a, right_comp);
            emitter.emit(&rwd).unwrap();
            trait_impls.add_single_impl(name, param_a, rwd);
        };
    }
    for param_a in registry.single_parameters() {
        let name = "LeftBulkDual";
        let _: Option<()> = try {
            // Left bulk dual is left complement of bulk
            let bulk = trait_impls.get_single_invocation("Bulk", variable(&param_a))?;
            let left_comp = trait_impls.get_single_invocation("LeftComplement", bulk)?;
            let lbd = single_expression_single_trait_impl(name, &param_a, left_comp);
            emitter.emit(&lbd).unwrap();
            trait_impls.add_single_impl(name, param_a, lbd);
        };
    }
    for param_a in registry.single_parameters() {
        let name = "LeftWeightDual";
        let _: Option<()> = try {
            // Left weight dual is left complement of weight
            let weight = trait_impls.get_single_invocation("Weight", variable(&param_a))?;
            let left_comp = trait_impls.get_single_invocation("LeftComplement", weight)?;
            let lwd = single_expression_single_trait_impl(name, &param_a, left_comp);
            emitter.emit(&lwd).unwrap();
            trait_impls.add_single_impl(name, param_a, lwd);
        };
    }






    for (param_a, param_b) in registry.pair_parameters() {
        let name = "BulkContraction";
        let _: Option<()> = try {
            // Bulk contraction is the antiwedge on a right bulk dual
            let rbd = trait_impls.get_single_invocation("RightBulkDual", variable(&param_b))?;
            let aw = trait_impls.get_pair_invocation("AntiWedge", variable(&param_a), rbd)?;
            let bc = single_expression_pair_trait_impl(name, &param_a, &param_b, aw);
            emitter.emit(&bc).unwrap();
            trait_impls.add_pair_impl(name, param_a, param_b, bc);
        };
    }
    for (param_a, param_b) in registry.pair_parameters() {
        let name = "WeightContraction";
        let _: Option<()> = try {
            // Weight contraction is the antiwedge on a right weight dual
            let rwd = trait_impls.get_single_invocation("RightWeightDual", variable(&param_b))?;
            let aw = trait_impls.get_pair_invocation("AntiWedge", variable(&param_a), rwd)?;
            let wc = single_expression_pair_trait_impl(name, &param_a, &param_b, aw);
            emitter.emit(&wc).unwrap();
            trait_impls.add_pair_impl(name, param_a, param_b, wc);
        };
    }
    for (param_a, param_b) in registry.pair_parameters() {
        let name = "BulkExpansion";
        let _: Option<()> = try {
            // Bulk expansion is the wedge on a right bulk dual
            let rbd = trait_impls.get_single_invocation("RightBulkDual", variable(&param_b))?;
            let w = trait_impls.get_pair_invocation("Wedge", variable(&param_a), rbd)?;
            let be = single_expression_pair_trait_impl(name, &param_a, &param_b, w);
            emitter.emit(&be).unwrap();
            trait_impls.add_pair_impl(name, param_a, param_b, be);
        };
    }
    for (param_a, param_b) in registry.pair_parameters() {
        let name = "WeightExpansion";
        let _: Option<()> = try {
            // Weight expansion is the wedge on a right weight dual
            let rwd = trait_impls.get_single_invocation("RightWeightDual", variable(&param_b))?;
            let w = trait_impls.get_pair_invocation("Wedge", variable(&param_a), rwd)?;
            let we = single_expression_pair_trait_impl(name, &param_a, &param_b, w);
            emitter.emit(&we).unwrap();
            trait_impls.add_pair_impl(name, param_a, param_b, we);
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
            let we = trait_impls.get_pair_invocation("WeightExpansion", variable(&param_a), variable(&param_b))?;
            let anti_wedge = trait_impls.get_pair_invocation("AntiWedge", variable(&param_b), we)?;
            let po = single_expression_pair_trait_impl(name, &param_a, &param_b, anti_wedge);
            emitter.emit(&po).unwrap();
            trait_impls.add_pair_impl(name, param_a, param_b, po);
        };
    }
    for (param_a, param_b) in registry.pair_parameters() {
        let name = "AntiProjectOrthogonallyOnto";
        let _: Option<()> = try {
            let wc = trait_impls.get_pair_invocation("WeightContraction", variable(&param_a), variable(&param_b))?;
            let wedge = trait_impls.get_pair_invocation("Wedge", variable(&param_b), wc)?;
            let apo = single_expression_pair_trait_impl(name, &param_a, &param_b, wedge);
            emitter.emit(&apo).unwrap();
            trait_impls.add_pair_impl(name, param_a, param_b, apo);
        };
    }
    for (param_a, param_b) in registry.pair_parameters() {
        let name = "ProjectThroughOriginOnto";
        let _: Option<()> = try {
            let be = trait_impls.get_pair_invocation("BulkExpansion", variable(&param_a), variable(&param_b))?;
            let anti_wedge = trait_impls.get_pair_invocation("AntiWedge", variable(&param_b), be)?;
            let po = single_expression_pair_trait_impl(name, &param_a, &param_b, anti_wedge);
            emitter.emit(&po).unwrap();
            trait_impls.add_pair_impl(name, param_a, param_b, po);
        };
    }
    for (param_a, param_b) in registry.pair_parameters() {
        let name = "AntiProjectThroughOriginOnto";
        let _: Option<()> = try {
            let bc = trait_impls.get_pair_invocation("BulkContraction", variable(&param_a), variable(&param_b))?;
            let wedge = trait_impls.get_pair_invocation("Wedge", variable(&param_b), bc)?;
            let apo = single_expression_pair_trait_impl(name, &param_a, &param_b, wedge);
            emitter.emit(&apo).unwrap();
            trait_impls.add_pair_impl(name, param_a, param_b, apo);
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
            let _ = trait_impls.get_single_impl("Grade", &param_a)?;
            let _ = trait_impls.get_single_impl("Grade", &param_b)?;

            // We can return a Scalar and ignore the HomogeneousMagnitude fluff if we Unitize up front
            let a_unitize = trait_impls.get_single_invocation("Unitize", variable(&param_a))?;
            let b_unitize = trait_impls.get_single_invocation("Unitize", variable(&param_b))?;

            // The actual cosine part of the definition
            let wc = trait_impls.get_pair_invocation("WeightContraction", a_unitize, b_unitize)?;
            let bn = trait_impls.get_single_invocation("BulkNorm", wc)?;
            let cosine = single_expression_pair_trait_impl(name, &param_a, &param_b, bn);
            emitter.emit(&cosine).unwrap();
            trait_impls.add_pair_impl(name, param_a, param_b, cosine);
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

    do_wgsl(algebra_name, file_path);
}

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
