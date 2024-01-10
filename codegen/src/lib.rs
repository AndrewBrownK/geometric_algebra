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
    raw: BTreeMap<String, (Parameter<'a>, BTreeMap<String, AstNode<'a>>, BTreeMap<String, (Parameter<'a>, BTreeMap<String, AstNode<'a>>)>)>,
    // TODO use these
    singles: BTreeMap<(String, String), AstNode<'a>>,
    pairs: BTreeMap<(String, String, String), AstNode<'a>>
}

impl<'a> TraitImpls<'a> {
    fn new() -> Self {
        TraitImpls {
            raw: BTreeMap::new(),
            singles: BTreeMap::new(),
            pairs: BTreeMap::new(),
        }
    }

    fn add_pair_impl(&mut self, name: &str, parameter_a: Parameter<'a>, parameter_b: Parameter<'a>, the_impl: AstNode<'a>) {
        let a_name = parameter_a.multi_vector_class().class_name.clone();
        let b_name = parameter_b.multi_vector_class().class_name.clone();
        let (_, _, pairs) = self.raw.entry(a_name).or_insert((parameter_a, BTreeMap::new(), BTreeMap::new()));
        let (_, pair_impls) = pairs.entry(b_name).or_insert((parameter_b, BTreeMap::new()));
        pair_impls.insert(name.to_string(), the_impl);
    }

    fn add_single_impl(&mut self, name: &str, parameter_a: Parameter<'a>, the_impl: AstNode<'a>) {
        let a_name = parameter_a.multi_vector_class().class_name.clone();
        let (_, singles, _) = self.raw.entry(a_name).or_insert((parameter_a, BTreeMap::new(), BTreeMap::new()));
        singles.insert(name.to_string(), the_impl);
    }

    fn get_pair_impl(&self, name: &str, parameter_a: &Parameter<'a>, parameter_b: &Parameter<'a>) -> Option<&AstNode<'a>> {
        let (_, _, pairs) = self.raw.get(&parameter_a.multi_vector_class().class_name)?;
        let (_, pair_impls) = pairs.get(&parameter_b.multi_vector_class().class_name)?;
        let the_impl = pair_impls.get(name)?;
        return Some(the_impl);
    }

    fn get_single_impl(&self, name: &str, parameter_a: &Parameter<'a>) -> Option<&AstNode<'a>> {
        let (_, singles, _) = self.raw.get(&parameter_a.multi_vector_class().class_name)?;
        let the_impl = singles.get(name)?;
        return Some(the_impl);
    }

    fn get_pair_impl_and_result(&self, name: &str, parameter_a: &Parameter<'a>, parameter_b: &Parameter<'a>) -> Option<(&AstNode<'a>, &Parameter<'a>)> {
        let (_, _, pairs) = self.raw.get(&parameter_a.multi_vector_class().class_name)?;
        let (_, pair_impls) = pairs.get(&parameter_b.multi_vector_class().class_name)?;
        let the_impl = pair_impls.get(name)?;
        let result = result_of_trait!(the_impl);
        return Some((the_impl, result));
    }

    fn get_single_impl_and_result(&self, name: &str, parameter_a: &Parameter<'a>) -> Option<(&AstNode<'a>, &Parameter<'a>)> {
        let (_, singles, _) = self.raw.get(&parameter_a.multi_vector_class().class_name)?;
        let the_impl = singles.get(name)?;
        let result = result_of_trait!(the_impl);
        return Some((the_impl, result));
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
    for class_a in registry.classes.iter() {
        let parameter_a = Parameter {
            name: "self",
            data_type: DataType::MultiVector(class_a),
        };
        for name in &["Zero", "One"] {
            let ast_node = class_a.constant(name);
            emitter.emit(&ast_node).unwrap();
            if ast_node != AstNode::None {
                trait_impls.add_single_impl(name, parameter_a.clone(), ast_node);
            }
        }

        if let Some((grade, unanimous)) = class_a.flat_basis().iter().map(|a| (a.grade(), true)).reduce(|a, b| (a.0, a.0 == b.0 && a.1 && b.1)) {
            if unanimous {
                let anti_grade = algebra.generator_squares.len() - grade;
                let grade_impl = MultiVectorClass::derive_grade("Grade", &parameter_a, grade);
                emitter.emit(&grade_impl).unwrap();
                trait_impls.add_single_impl("Grade", parameter_a.clone(), grade_impl);

                let anti_grade_impl = MultiVectorClass::derive_grade("AntiGrade", &parameter_a, anti_grade);
                emitter.emit(&anti_grade_impl).unwrap();
                trait_impls.add_single_impl("AntiGrade", parameter_a.clone(), anti_grade_impl);
            }
        }

        // TODO for some reason not all involutions are being output for CGA,
        //  for example search "impl Dual" in cga3d.rs vs ppga3d.rs.
        //  This is strange because some involutions are written, like Reversal.
        //  I am wondering if this is because of the extra projective dimension. Sheesh.
        for (name, involution) in involutions.iter() {
            let ast_node = MultiVectorClass::involution(name, involution, &parameter_a, &registry, false);
            emitter.emit(&ast_node).unwrap();
            if ast_node != AstNode::None {
                trait_impls.add_single_impl(name, parameter_a.clone(), ast_node);
            }
        }
        for class_b in registry.classes.iter() {
            let parameter_b = Parameter {
                name: "other",
                data_type: DataType::MultiVector(class_b),
            };
            if class_a != class_b {
                let name = "Into";
                let ast_node = MultiVectorClass::involution(name, &Involution::projection(class_b), &parameter_a, &registry, true);
                emitter.emit(&ast_node).unwrap();
                if ast_node != AstNode::None {
                    trait_impls.add_pair_impl(name, parameter_a.clone(), parameter_b.clone(), ast_node);
                }
            }
            for name in &["Add", "Sub"] {
                let ast_node = MultiVectorClass::element_wise(*name, &parameter_a, &parameter_b, &registry);
                emitter.emit(&ast_node).unwrap();
                if ast_node != AstNode::None {
                    trait_impls.add_pair_impl(name, parameter_a.clone(), parameter_b.clone(), ast_node);
                }
            }
            if class_a == class_b {
                for name in &["Mul", "Div"] {
                    let ast_node = MultiVectorClass::element_wise(*name, &parameter_a, &parameter_b, &registry);
                    emitter.emit(&ast_node).unwrap();
                    if ast_node != AstNode::None {
                        trait_impls.add_pair_impl(name, parameter_a.clone(), parameter_b.clone(), ast_node);
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
                let ast_node = MultiVectorClass::product(name, product, &parameter_a, &parameter_b, &registry);
                emitter.emit(&ast_node).unwrap();
                if ast_node != AstNode::None {
                    trait_impls.add_pair_impl(name, parameter_a.clone(), parameter_b.clone(), ast_node);
                }
            }
        }


        for class_b in registry.classes.iter() {
            let parameter_b = Parameter {
                name: "other",
                data_type: DataType::MultiVector(class_b),
            };
            if parameter_a.multi_vector_class() != parameter_b.multi_vector_class() {
                continue
            }

            let scalar_product = trait_impls.get_pair_impl("ScalarProduct", &parameter_a, &parameter_b);
            let reversal = trait_impls.get_single_impl("Reversal", &parameter_a);

            let (scalar_product, reversal) = match (scalar_product, reversal) {
                (Some(sp), Some(r)) => (sp, r),
                (_, _) => continue,
            };

            let squared_magnitude = MultiVectorClass::derive_squared_magnitude("SquaredMagnitude", scalar_product, reversal, &parameter_a);
            emitter.emit(&squared_magnitude).unwrap();
            let magnitude = MultiVectorClass::derive_magnitude("Magnitude", &squared_magnitude, &parameter_a);
            emitter.emit(&magnitude).unwrap();

            let bulk_norm = MultiVectorClass::derive_magnitude("BulkNorm", &squared_magnitude, &parameter_a);

            trait_impls.add_single_impl("SquaredMagnitude", parameter_a.clone(), squared_magnitude);
            trait_impls.add_single_impl("Magnitude", parameter_a.clone(), magnitude);



            let anti_scalar_product = trait_impls.get_pair_impl("AntiScalarProduct", &parameter_a, &parameter_b);
            let anti_reversal = trait_impls.get_single_impl("AntiReversal", &parameter_a);

            let (anti_scalar_product, anti_reversal) = match (anti_scalar_product, anti_reversal) {
                (Some(sp), Some(r)) => (sp, r),
                (_, _) => continue,
            };

            emitter.emit(&bulk_norm).unwrap();
            let squared_anti_magnitude = MultiVectorClass::derive_squared_magnitude("SquaredAntiMagnitude", anti_scalar_product, anti_reversal, &parameter_a);
            emitter.emit(&squared_anti_magnitude).unwrap();
            let weight_norm = MultiVectorClass::derive_magnitude("WeightNorm", &squared_anti_magnitude, &parameter_a);
            emitter.emit(&weight_norm).unwrap();

            let bulk_norm_result = result_of_trait!(bulk_norm);
            let weight_norm_result = result_of_trait!(weight_norm);

            if let Some(add) = trait_impls.get_pair_impl("Add", bulk_norm_result, weight_norm_result) {
                let geometric_norm = MultiVectorClass::derive_geometric_norm("GeometricNorm", &bulk_norm, &weight_norm, &registry, &parameter_a, &add);
                emitter.emit(&geometric_norm).unwrap();
                trait_impls.add_single_impl("GeometricNorm", parameter_a.clone(), geometric_norm);
            }
            trait_impls.add_single_impl("BulkNorm", parameter_a.clone(), bulk_norm);
            trait_impls.add_single_impl("SquaredAntiMagnitude", parameter_a.clone(), squared_anti_magnitude);
            trait_impls.add_single_impl("WeightNorm", parameter_a.clone(), weight_norm);
        }

        for class_b in registry.classes.iter() {
            let parameter_b = Parameter {
                name: "other",
                data_type: DataType::MultiVector(class_b),
            };


            if parameter_b.multi_vector_class().grouped_basis != vec![vec![BasisElement::from_index(0)]] {
                continue;
            }

            // If this type has a GeometricProduct with scalar, then we can implement some extra stuff
            let geometric_product = match trait_impls.get_pair_impl("GeometricProduct", &parameter_a, &parameter_b) {
                Some(gp) => gp,
                None => continue,
            };

            // If this type has a GeometricProduct, then we can implement Scale
            let scale = MultiVectorClass::derive_scale("Scale", geometric_product, &parameter_a, &parameter_b);
            emitter.emit(&scale).unwrap();

            // If this type also has a Magnitude, then we can implement Signum
            if let Some(magnitude) = trait_impls.get_single_impl("Magnitude", &parameter_a) {
                let signum = MultiVectorClass::derive_signum("Signum", geometric_product, magnitude, &parameter_a);
                emitter.emit(&signum).unwrap();
                trait_impls.add_single_impl("Signum", parameter_a.clone(), signum);
            }

            // If this type has a GeometricProduct with scalar, then we can implement some extra stuff
            let geometric_product = match trait_impls.get_pair_impl("GeometricProduct", &parameter_a, &parameter_b) {
                Some(gp) => gp,
                None => continue,
            };

            // If this type also has a SquaredMagnitude and Reversal, then we can implement Inverse
            if let Some(squared_magnitude) = trait_impls.get_single_impl("SquaredMagnitude", &parameter_a) {
                if let Some(reversal) = trait_impls.get_single_impl("Reversal", &parameter_a) {
                    let inverse = MultiVectorClass::derive_inverse("Inverse", geometric_product, squared_magnitude, reversal, &parameter_a);
                    emitter.emit(&inverse).unwrap();
                    trait_impls.add_single_impl("Inverse", parameter_a.clone(), inverse);
                }
            }
        }

        for class_b in registry.classes.iter() {
            let parameter_b = Parameter {
                name: "other",
                data_type: DataType::MultiVector(class_b),
            };


            if parameter_b.multi_vector_class().grouped_basis != vec![vec![BasisElement::from_index(0)]] {
                continue;
            }

            // If this type has a GeometricProduct with scalar, then we can implement some extra stuff
            let geometric_product = match trait_impls.get_pair_impl("GeometricProduct", &parameter_a, &parameter_b) {
                Some(gp) => gp,
                None => continue,
            };

            if let Some(weight_norm) = trait_impls.get_single_impl("WeightNorm", &parameter_a) {
                let unitize = MultiVectorClass::derive_unitize("Unitize", geometric_product, weight_norm, &parameter_a, &parameter_b);
                emitter.emit(&unitize).unwrap();
                trait_impls.add_single_impl("Unitize", parameter_a.clone(), unitize);
            }
        }

        for class_b in registry.classes.iter() {
            let parameter_b = Parameter {
                name: "other",
                data_type: DataType::MultiVector(class_b),
            };

            let anti_wedge_product = match trait_impls.get_pair_impl("RegressiveProduct", &parameter_a, &parameter_b) {
                Some(aw) => aw,
                None => continue
            };


            let bases = parameter_b.multi_vector_class().flat_basis();
            let nzd = algebra.generator_squares.iter().filter(|it| **it != 0isize).count();

            if bases.iter().any(|it| it.grade() != nzd) {
                continue
            }
            let special_base = match bases.iter().find(|it| BasisElement::product(&it, &it, &algebra).scalar != 0) {
                Some(b) => b,
                None => continue,
            };

            let attitude = MultiVectorClass::derive_attitude(
                "Attitude",
                anti_wedge_product,
                &parameter_a,
                &parameter_b,
                &special_base
            );
            emitter.emit(&attitude).unwrap();
            trait_impls.add_single_impl("Attitude", parameter_a.clone(), attitude);
        }
    }




    for class_a in registry.classes.iter() {
        let parameter_a = Parameter {
            name: "self",
            data_type: DataType::MultiVector(class_a),
        };
        for class_b in registry.classes.iter() {
            let parameter_b = Parameter {
                name: "other",
                data_type: DataType::MultiVector(class_b),
            };

            let (geometric_product, geometric_product_result) = match trait_impls.get_pair_impl_and_result("GeometricProduct", &parameter_a, &parameter_b) {
                Some(gp) => gp,
                None => continue
            };

            if parameter_a.multi_vector_class() == parameter_b.multi_vector_class()
                && geometric_product_result.multi_vector_class() == parameter_a.multi_vector_class()
            {
                //
                let constant_one = trait_impls.get_single_impl("One", &parameter_a);
                let inverse = trait_impls.get_single_impl("Inverse", &parameter_a);
                if let (Some(constant_one), Some(inverse)) = (constant_one, inverse) {
                    let power_of_integer = MultiVectorClass::derive_power_of_integer(
                        "Powi",
                        geometric_product,
                        constant_one,
                        inverse,
                        &parameter_a,
                        &Parameter {
                            name: "exponent",
                            data_type: DataType::Integer,
                        },
                    );
                    emitter.emit(&power_of_integer).unwrap();
                }
            }

            if let Some(inverse) = trait_impls.get_single_impl("Inverse", &parameter_b) {
                let division = MultiVectorClass::derive_division("GeometricQuotient", geometric_product, inverse, &parameter_a, &parameter_b);
                emitter.emit(&division).unwrap();
            }
        }
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
        let _: Option<()> = try {
            let (bulk_wedge, bw_r) = trait_impls.get_pair_impl_and_result("OuterProduct", &param_a, &param_b)?;
            let (bulk_attitude, ba_r) = trait_impls.get_single_impl_and_result("Attitude", &bw_r)?;
            let (weight_attitude, wa_r) = trait_impls.get_single_impl_and_result("Attitude", &param_b)?;
            let (weight_wedge, ww_r) = trait_impls.get_pair_impl_and_result("OuterProduct", &param_a, &wa_r)?;
            let (bulk_norm, bn_r) = trait_impls.get_single_impl_and_result("BulkNorm", &ba_r)?;
            let (weight_norm, wn_r) = trait_impls.get_single_impl_and_result("WeightNorm", &ww_r)?;
            let (add, add_r) = trait_impls.get_pair_impl_and_result("Add", &bn_r, &wn_r)?;
            let ed = MultiVectorClass::derive_euclidean_distance(
                "Distance", &param_a, &param_b, &add_r, &bulk_wedge, &bulk_attitude,
                &bulk_norm, &weight_attitude, &weight_wedge, &weight_norm, &add
            );
            emitter.emit(&ed).unwrap();
            trait_impls.add_pair_impl("Distance", param_a, param_b, ed);
        };
    }

    // TODO:
    //  - Inversion?
    //    https://rigidgeometricalgebra.org/wiki/index.php?title=Inversion
    //  - Transflection?
    //    https://rigidgeometricalgebra.org/wiki/index.php?title=Transflection
    //  - Reflection?
    //    https://rigidgeometricalgebra.org/wiki/index.php?title=Reflection
    //  - Projection? requires "weight expansion"?
    //    https://rigidgeometricalgebra.org/wiki/index.php?title=Projections
    //  - Commutators?
    //    https://rigidgeometricalgebra.org/wiki/index.php?title=Commutators
    //  - Geometric Property?
    //    https://rigidgeometricalgebra.org/wiki/index.php?title=Geometric_property

    // TODO aha: https://projectivegeometricalgebra.org/projgeomalg.pdf

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
