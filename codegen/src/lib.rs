pub mod algebra;
mod ast;
mod compile;
mod emit;
mod glsl;
mod rust;

use std::io::{Read, Write};
use naga;
use naga::back::wgsl::WriterFlags;
use naga::{Module, ShaderStage};
use naga::front::glsl::Error;
use naga::valid::{Capabilities, ModuleInfo, ValidationFlags};
use crate::{
    algebra::{BasisElement, GeometricAlgebra, Involution, MultiVectorClass, MultiVectorClassRegistry, Product},
    ast::{AstNode, DataType, Parameter},
    emit::Emitter,
};

// TODO see also the already existing "GeometricAlgebra" that is just the generator squares
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
        // TODO note the splits on '|' for the grouped_basis property
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
    let mut trait_implementations = std::collections::BTreeMap::new();
    for class_a in registry.classes.iter() {
        let parameter_a = Parameter {
            name: "self",
            data_type: DataType::MultiVector(class_a),
        };
        let mut single_trait_implementations = std::collections::BTreeMap::new();
        for name in &["Zero", "One"] {
            let ast_node = class_a.constant(name);
            emitter.emit(&ast_node).unwrap();
            if ast_node != AstNode::None {
                single_trait_implementations.insert(name.to_string(), ast_node);
            }
        }
        for (name, involution) in involutions.iter() {
            let ast_node = MultiVectorClass::involution(name, involution, &parameter_a, &registry, false);
            emitter.emit(&ast_node).unwrap();
            if ast_node != AstNode::None {
                single_trait_implementations.insert(name.to_string(), ast_node);
            }
        }
        let mut pair_trait_implementations = std::collections::BTreeMap::new();
        for class_b in registry.classes.iter() {
            let mut trait_implementations = std::collections::BTreeMap::new();
            let parameter_b = Parameter {
                name: "other",
                data_type: DataType::MultiVector(class_b),
            };
            if class_a != class_b {
                let name = "Into";
                let ast_node = MultiVectorClass::involution(name, &Involution::projection(class_b), &parameter_a, &registry, true);
                emitter.emit(&ast_node).unwrap();
                if ast_node != AstNode::None {
                    trait_implementations.insert(name.to_string(), ast_node);
                }
            }
            for name in &["Add", "Sub"] {
                let ast_node = MultiVectorClass::element_wise(*name, &parameter_a, &parameter_b, &registry);
                emitter.emit(&ast_node).unwrap();
                if ast_node != AstNode::None {
                    trait_implementations.insert(name.to_string(), ast_node);
                }
            }
            if class_a == class_b {
                for name in &["Mul", "Div"] {
                    let ast_node = MultiVectorClass::element_wise(*name, &parameter_a, &parameter_b, &registry);
                    emitter.emit(&ast_node).unwrap();
                    if ast_node != AstNode::None {
                        trait_implementations.insert(name.to_string(), ast_node);
                    }
                }
            }
            for (name, product) in products.iter() {
                let ast_node = MultiVectorClass::product(name, product, &parameter_a, &parameter_b, &registry);
                emitter.emit(&ast_node).unwrap();
                if ast_node != AstNode::None {
                    trait_implementations.insert(name.to_string(), ast_node);
                }
            }
            pair_trait_implementations.insert(
                parameter_b.multi_vector_class().class_name.clone(),
                (parameter_b.clone(), trait_implementations),
            );
        }
        for (parameter_b, pair_trait_implementations) in pair_trait_implementations.values() {
            if let Some(scalar_product) = pair_trait_implementations.get("ScalarProduct") {
                if let Some(reversal) = single_trait_implementations.get("Reversal") {
                    if parameter_a.multi_vector_class() == parameter_b.multi_vector_class() {
                        let squared_magnitude =
                            MultiVectorClass::derive_squared_magnitude("SquaredMagnitude", scalar_product, reversal, &parameter_a);
                        emitter.emit(&squared_magnitude).unwrap();
                        let magnitude = MultiVectorClass::derive_magnitude("Magnitude", &squared_magnitude, &parameter_a);
                        emitter.emit(&magnitude).unwrap();
                        single_trait_implementations.insert(result_of_trait!(squared_magnitude).name.to_string(), squared_magnitude);
                        single_trait_implementations.insert(result_of_trait!(magnitude).name.to_string(), magnitude);
                    }
                }
            }
        }
        for (parameter_b, pair_trait_implementations) in pair_trait_implementations.values() {
            if let Some(geometric_product) = pair_trait_implementations.get("GeometricProduct") {
                if parameter_b.multi_vector_class().grouped_basis == vec![vec![BasisElement::from_index(0)]] {
                    let scale = MultiVectorClass::derive_scale("Scale", geometric_product, &parameter_a, parameter_b);
                    emitter.emit(&scale).unwrap();
                    if let Some(magnitude) = single_trait_implementations.get("Magnitude") {
                        let signum = MultiVectorClass::derive_signum("Signum", geometric_product, magnitude, &parameter_a);
                        emitter.emit(&signum).unwrap();
                        single_trait_implementations.insert(result_of_trait!(signum).name.to_string(), signum);
                    }
                    if let Some(squared_magnitude) = single_trait_implementations.get("SquaredMagnitude") {
                        if let Some(reversal) = single_trait_implementations.get("Reversal") {
                            let inverse = MultiVectorClass::derive_inverse("Inverse", geometric_product, squared_magnitude, reversal, &parameter_a);
                            emitter.emit(&inverse).unwrap();
                            single_trait_implementations.insert(result_of_trait!(inverse).name.to_string(), inverse);
                        }
                    }
                }
            }
        }
        trait_implementations.insert(
            parameter_a.multi_vector_class().class_name.clone(),
            (parameter_a.clone(), single_trait_implementations, pair_trait_implementations),
        );
    }
    for (parameter_a, single_trait_implementations, pair_trait_implementations) in trait_implementations.values() {
        for (parameter_b, pair_trait_implementations) in pair_trait_implementations.values() {
            if let Some(geometric_product) = pair_trait_implementations.get("GeometricProduct") {
                let geometric_product_result = result_of_trait!(geometric_product);
                if parameter_a.multi_vector_class() == parameter_b.multi_vector_class()
                    && geometric_product_result.multi_vector_class() == parameter_a.multi_vector_class()
                {
                    if let Some(constant_one) = single_trait_implementations.get("One") {
                        if let Some(inverse) = single_trait_implementations.get("Inverse") {
                            let power_of_integer = MultiVectorClass::derive_power_of_integer(
                                "Powi",
                                geometric_product,
                                constant_one,
                                inverse,
                                parameter_a,
                                &Parameter {
                                    name: "exponent",
                                    data_type: DataType::Integer,
                                },
                            );
                            emitter.emit(&power_of_integer).unwrap();
                        }
                    }
                }
                if let Some(b_trait_implementations) = trait_implementations.get(&parameter_b.multi_vector_class().class_name) {
                    if let Some(inverse) = b_trait_implementations.1.get("Inverse") {
                        let division = MultiVectorClass::derive_division("GeometricQuotient", geometric_product, inverse, parameter_a, parameter_b);
                        emitter.emit(&division).unwrap();
                    }
                }
                if let Some(reversal) = single_trait_implementations.get("Reversal") {
                    if let Some(b_trait_implementations) = trait_implementations.get(&geometric_product_result.multi_vector_class().class_name) {
                        if let Some(b_pair_trait_implementations) = b_trait_implementations.2.get(&parameter_a.multi_vector_class().class_name) {
                            if let Some(geometric_product_2) = b_pair_trait_implementations.1.get("GeometricProduct") {
                                let geometric_product_2_result = result_of_trait!(geometric_product_2);
                                if let Some(c_trait_implementations) =
                                    trait_implementations.get(&geometric_product_2_result.multi_vector_class().class_name)
                                {
                                    if let Some(c_pair_trait_implementations) =
                                        c_trait_implementations.2.get(&parameter_b.multi_vector_class().class_name)
                                    {
                                        let transformation = MultiVectorClass::derive_sandwich_product(
                                            "Transformation",
                                            geometric_product,
                                            geometric_product_2,
                                            reversal,
                                            c_pair_trait_implementations.1.get("Into"),
                                            parameter_a,
                                            parameter_b,
                                        );
                                        emitter.emit(&transformation).unwrap();
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // Let naga do wgsl:
    // - Good because low maintenance here.
    // - Bad because it erases useful comments.

    // Prepare some of naga's clutter
    let mut glsl_frontend = naga::front::glsl::Frontend::default();
    let mut wgsl_backend = naga::back::wgsl::Writer::new(String::new(), WriterFlags::EXPLICIT_TYPES);
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

    // Parse and validate the naga module
    let module = match glsl_frontend.parse(&options, glsl_contents.as_str()) {
        Ok(m) => m,
        Err(err) => panic!("Problem generating wgsl for {algebra_name}: {err:?}")
    };
    let mut validator = naga::valid::Validator::new(ValidationFlags::default(), Capabilities::default());
    let module_info = validator.validate(&module).unwrap();

    // Write the wgsl
    wgsl_backend.write(&module, &module_info).unwrap();
    let mut wgsl_contents = wgsl_backend.finish();
    let mut wgsl_file = std::fs::File::create(file_path.with_extension("wgsl")).unwrap();
    wgsl_file.write(wgsl_contents.as_bytes()).unwrap();
}
