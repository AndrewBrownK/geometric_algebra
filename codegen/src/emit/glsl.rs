use crate::ast::GatherData;
use crate::{
    ast::{AstNode, DataType, Expression, ExpressionContent},
    emit::{camel_to_snake_case, emit_indentation},
};

const COMPONENT: &[&str] = &["x", "y", "z", "w"];

fn emit_data_type<W: std::io::Write>(collector: &mut W, data_type: &DataType) -> std::io::Result<()> {
    match data_type {
        DataType::Integer => collector.write_all(b"int"),
        DataType::SimdVector(size) if *size == 1 => collector.write_all(b"float"),
        DataType::SimdVector(size) => collector.write_fmt(format_args!("vec{}", *size)),
        DataType::MultiVector(class) => collector.write_fmt(format_args!("{}", class.class_name)),
    }
}

fn get_data_type(data_type: &DataType) -> String {
    match data_type {
        DataType::Integer => "int".to_string(),
        DataType::SimdVector(size) if *size == 1 => "float".to_string(),
        DataType::SimdVector(size) => format_args!("vec{}", *size).to_string(),
        DataType::MultiVector(class) => format_args!("{}", class.class_name).to_string(),
    }
}

fn emit_expression<W: std::io::Write>(collector: &mut W, expression: &Expression) -> std::io::Result<()> {
    match &expression.content {
        ExpressionContent::None => unreachable!(),
        ExpressionContent::Variable(name) => {
            collector.write_all(name.bytes().collect::<Vec<_>>().as_slice())?;
        }
        ExpressionContent::InvokeClassMethod(_, _, arguments) | ExpressionContent::InvokeInstanceMethod(_, _, _, arguments) => {
            match &expression.content {
                ExpressionContent::InvokeInstanceMethod(result_class, inner_expression, method_name, _) => {
                    if let DataType::MultiVector(result_class) = result_class {
                        camel_to_snake_case(collector, &result_class.class_name)?;
                        collector.write_all(b"__")?;
                    }
                    camel_to_snake_case(collector, method_name)?;
                    for (argument_class, _argument) in arguments.iter() {
                        if let DataType::MultiVector(argument_class) = argument_class {
                            collector.write_all(b"__")?;
                            camel_to_snake_case(collector, &argument_class.class_name)?;
                        }
                    }
                    collector.write_all(b"(")?;
                    emit_expression(collector, inner_expression)?;
                    if !arguments.is_empty() {
                        collector.write_all(b", ")?;
                    }
                }
                ExpressionContent::InvokeClassMethod(class, method_name, _) => {
                    if *method_name == "Constructor" {
                        collector.write_fmt(format_args!("{}", &class.class_name))?;
                    } else {
                        camel_to_snake_case(collector, &class.class_name)?;
                        collector.write_all(b"__")?;
                        camel_to_snake_case(collector, method_name)?;
                    }
                    collector.write_all(b"(")?;
                }
                _ => unreachable!(),
            }
            for (i, (_argument_class, argument)) in arguments.iter().enumerate() {
                if i > 0 {
                    collector.write_all(b", ")?;
                }
                emit_expression(collector, argument)?;
            }
            collector.write_all(b")")?;
        }
        ExpressionContent::Conversion(source_class, destination_class, inner_expression) => {
            camel_to_snake_case(collector, &source_class.class_name)?;
            collector.write_all(b"__into__")?;
            camel_to_snake_case(collector, &destination_class.class_name)?;
            collector.write_all(b"(")?;
            emit_expression(collector, inner_expression)?;
            collector.write_all(b")")?;
        }
        ExpressionContent::Select(condition_expression, then_expression, else_expression) => {
            collector.write_all(b"(")?;
            emit_expression(collector, condition_expression)?;
            collector.write_all(b") ? ")?;
            emit_expression(collector, then_expression)?;
            collector.write_all(b" : ")?;
            emit_expression(collector, else_expression)?;
        }
        ExpressionContent::Access(inner_expression, array_index) => {
            emit_expression(collector, inner_expression)?;
            collector.write_fmt(format_args!(".g{}", array_index))?;
        }
        ExpressionContent::Swizzle(inner_expression, indices) => {
            emit_expression(collector, inner_expression)?;
            collector.write_all(b".")?;
            for component_index in indices.iter() {
                collector.write_all(COMPONENT[*component_index].bytes().collect::<Vec<_>>().as_slice())?;
            }
        }
        ExpressionContent::Gather(inner_expression, indices) => {
            if expression.size > 1 {
                emit_data_type(collector, &DataType::SimdVector(expression.size))?;
                collector.write_all(b"(")?;
            }
            for (i, gather_data) in indices.iter().enumerate() {
                if i > 0 {
                    collector.write_all(b", ")?;
                }
                match gather_data {
                    GatherData::Usual(gd) => {
                        if gd.negate {
                            collector.write_all(b"-")?;
                        }
                        emit_expression(collector, inner_expression)?;
                        collector.write_fmt(format_args!(".g{}", gd.group))?;
                        if gd.group_size > 1 {
                            collector.write_fmt(format_args!(".{}", COMPONENT[gd.element]))?;
                        }
                    }
                    GatherData::RawZero => collector.write_all(b"0.0")?,
                }
            }
            if expression.size > 1 {
                collector.write_all(b")")?;
            }
        }
        ExpressionContent::Constant(data_type, values) => match data_type {
            DataType::Integer => collector.write_fmt(format_args!("{}", values[0] as f32))?,
            DataType::SimdVector(_size) => {
                if expression.size == 1 {
                    collector.write_fmt(format_args!("{:.1}", values[0] as f32))?
                } else {
                    emit_data_type(collector, &DataType::SimdVector(expression.size))?;
                    collector.write_fmt(format_args!("({})", values.iter().map(|value| format!("{:.1}", *value as f32)).collect::<Vec<_>>().join(", ")))?
                }
            }
            _ => unreachable!(),
        },
        ExpressionContent::ConstructVec(data_type, values) => match data_type {
            DataType::SimdVector(size) => {
                emit_data_type(collector, &DataType::SimdVector(*size))?;
                collector.write_all(b"(")?;
                let mut first = true;
                for value in values {
                    if !first {
                        collector.write_all(b", ")?;
                    }
                    first = false;
                    emit_expression(collector, value)?;
                }
                collector.write_all(b")")?;
            }
            _ => unreachable!(),
        }
        ExpressionContent::SquareRoot(inner_expression) => {
            collector.write_all(b"sqrt(")?;
            emit_expression(collector, inner_expression)?;
            collector.write_all(b")")?;
        }
        ExpressionContent::Add(lhs, rhs)
        | ExpressionContent::Subtract(lhs, rhs)
        | ExpressionContent::Multiply(lhs, rhs)
        | ExpressionContent::Divide(lhs, rhs)
        | ExpressionContent::LessThan(lhs, rhs)
        | ExpressionContent::Equal(lhs, rhs)
        | ExpressionContent::LogicAnd(lhs, rhs)
        | ExpressionContent::BitShiftRight(lhs, rhs) => {
            if let ExpressionContent::LogicAnd(_, _) = expression.content {
                collector.write_all(b"(")?;
            }
            emit_expression(collector, lhs)?;
            collector.write_all(match expression.content {
                ExpressionContent::Add(_, _) => b" + ",
                ExpressionContent::Subtract(_, _) => b" - ",
                ExpressionContent::Multiply(_, _) => b" * ",
                ExpressionContent::Divide(_, _) => b" / ",
                ExpressionContent::LessThan(_, _) => b" < ",
                ExpressionContent::Equal(_, _) => b" == ",
                ExpressionContent::LogicAnd(_, _) => b" & ",
                ExpressionContent::BitShiftRight(_, _) => b" >> ",
                _ => unreachable!(),
            })?;
            emit_expression(collector, rhs)?;
            if let ExpressionContent::LogicAnd(_, _) = expression.content {
                collector.write_all(b")")?;
            }
        }
    }
    Ok(())
}

pub fn emit_code<W: std::io::Write>(collector: &mut W, ast_node: &AstNode, indentation: usize) -> std::io::Result<()> {
    match ast_node {
        AstNode::None => {}
        AstNode::Preamble => {}
        AstNode::TraitDefinition { .. } => {}
        AstNode::TypeAlias(_, _) => {}
        AstNode::ClassDefinition { class } => {
            collector.write_fmt(format_args!("struct {} {{\n", class.class_name))?;
            for (i, group) in class.grouped_basis.iter().enumerate() {
                emit_indentation(collector, indentation + 1)?;
                collector.write_all(b"// ")?;
                for (i, element) in group.iter().enumerate() {
                    if i > 0 {
                        collector.write_all(b", ")?;
                    }
                    collector.write_fmt(format_args!("{}", element))?;
                }
                collector.write_all(b"\n")?;
                emit_indentation(collector, indentation + 1)?;
                emit_data_type(collector, &DataType::SimdVector(group.len()))?;
                collector.write_fmt(format_args!(" g{};\n", i))?;
            }
            emit_indentation(collector, indentation)?;
            collector.write_all(b"};\n\n")?;
        }
        AstNode::ReturnStatement { expression } => {
            collector.write_all(b"return ")?;
            emit_expression(collector, expression)?;
            collector.write_all(b";\n")?;
        }
        AstNode::VariableAssignment { name, data_type, expression } => {
            if let Some(data_type) = data_type {
                emit_data_type(collector, data_type)?;
                collector.write_all(b" ")?;
            }
            collector.write_fmt(format_args!("{} = ", name))?;
            emit_expression(collector, expression)?;
            collector.write_all(b";\n")?;
        }
        AstNode::IfThenBlock { condition, body } | AstNode::WhileLoopBlock { condition, body } => {
            collector.write_all(match &ast_node {
                AstNode::IfThenBlock { .. } => b"if",
                AstNode::WhileLoopBlock { .. } => b"while",
                _ => unreachable!(),
            })?;
            collector.write_all(b"(")?;
            emit_expression(collector, condition)?;
            collector.write_all(b") {\n")?;
            for statement in body.iter() {
                emit_indentation(collector, indentation + 1)?;
                emit_code(collector, statement, indentation + 1)?;
            }
            emit_indentation(collector, indentation)?;
            collector.write_all(b"}\n")?;
        }
        AstNode::TraitImplementation { result, class, parameters, body } => {
            let result_type_name = get_data_type(&result.data_type);
            collector.write_fmt(format_args!("{} ", result_type_name))?;
            match parameters.len() {
                0 => {
                    camel_to_snake_case(collector, class.class_name.as_str())?;
                    collector.write_all(b"__")?;
                    camel_to_snake_case(collector, result.name)?;
                },
                1 if result.name == "Into" => {
                    camel_to_snake_case(collector, &parameters[0].multi_vector_class().class_name)?;
                    collector.write_all(b"__")?;
                    camel_to_snake_case(collector, result.name)?;
                    collector.write_all(b"__")?;
                    camel_to_snake_case(collector, &result_type_name)?;
                }
                1 => {
                    camel_to_snake_case(collector, &parameters[0].multi_vector_class().class_name)?;
                    collector.write_all(b"__")?;
                    camel_to_snake_case(collector, result.name)?;
                },
                2 if !matches!(parameters[1].data_type, DataType::MultiVector(_)) => {
                    camel_to_snake_case(collector, &parameters[0].multi_vector_class().class_name)?;
                    collector.write_all(b"__")?;
                    camel_to_snake_case(collector, result.name)?;
                },
                2 => {
                    camel_to_snake_case(collector, &parameters[0].multi_vector_class().class_name)?;
                    collector.write_all(b"__")?;
                    camel_to_snake_case(collector, result.name)?;
                    collector.write_all(b"__")?;
                    camel_to_snake_case(collector, &parameters[1].multi_vector_class().class_name)?;
                }
                _ => unreachable!(),
            }
            collector.write_all(b"(")?;
            for (i, parameter) in parameters.iter().enumerate() {
                if i > 0 {
                    collector.write_all(b", ")?;
                }
                emit_data_type(collector, &parameter.data_type)?;
                collector.write_fmt(format_args!(" {}", parameter.name))?;
            }
            collector.write_all(b") {\n")?;
            for statement in body.iter() {
                emit_indentation(collector, indentation + 1)?;
                emit_code(collector, statement, indentation + 1)?;
            }
            emit_indentation(collector, indentation)?;
            collector.write_all(b"}\n\n")?;
        }
    }
    Ok(())
}
