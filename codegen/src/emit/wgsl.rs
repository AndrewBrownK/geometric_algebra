use crate::ast::GatherData;
use crate::emit::lower_camel_case;
use crate::{
    ast::{AstNode, DataType, Expression, ExpressionContent},
    emit::emit_indentation,
};

const COMPONENT: &[&str] = &["x", "y", "z", "w"];

fn emit_data_type<W: std::io::Write>(collector: &mut W, data_type: &DataType) -> std::io::Result<()> {
    match data_type {
        DataType::Integer => collector.write_all(b"i32"),
        DataType::SimdVector(size) if *size == 1 => collector.write_all(b"f32"),
        DataType::SimdVector(size) => collector.write_fmt(format_args!("vec{}<f32>", *size)),
        DataType::MultiVector(class) => collector.write_fmt(format_args!("{}", class.class_name)),
    }
}

fn emit_expression<W: std::io::Write>(collector: &mut W, expression: &Expression) -> std::io::Result<()> {
    match &expression.content {
        ExpressionContent::None => unreachable!(),
        ExpressionContent::Variable(name) => {
            let name = if name.to_string() == "self" { "self_" } else { name };
            collector.write_all(name.bytes().collect::<Vec<_>>().as_slice())?;
        }
        ExpressionContent::InvokeClassMethod(_, _, arguments) | ExpressionContent::InvokeInstanceMethod(_, _, _, arguments) => {
            match &expression.content {
                ExpressionContent::InvokeInstanceMethod(result_class, inner_expression, method_name, _) => {
                    if let DataType::MultiVector(result_class) = result_class {
                        lower_camel_case(collector, &result_class.class_name)?;
                        collector.write_all(b"_")?;
                    }
                    lower_camel_case(collector, method_name)?;
                    for (argument_class, _argument) in arguments.iter() {
                        if let DataType::MultiVector(argument_class) = argument_class {
                            collector.write_all(b"_")?;
                            lower_camel_case(collector, &argument_class.class_name)?;
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
                        lower_camel_case(collector, &class.class_name)?;
                        collector.write_all(b"_")?;
                        lower_camel_case(collector, method_name)?;
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
            lower_camel_case(collector, &source_class.class_name)?;
            collector.write_all(b"_into_")?;
            lower_camel_case(collector, &destination_class.class_name)?;
            collector.write_all(b"(")?;
            emit_expression(collector, inner_expression)?;
            collector.write_all(b")")?;
        }
        ExpressionContent::Select(condition_expression, then_expression, else_expression) => {
            collector.write_all(b"select(")?;
            emit_expression(collector, else_expression)?;
            collector.write_all(b", ")?;
            emit_expression(collector, then_expression)?;
            collector.write_all(b", ")?;
            emit_expression(collector, condition_expression)?;
            collector.write_all(b")")?;
        }
        ExpressionContent::Access(inner_expression, array_index) => {
            emit_expression(collector, inner_expression)?;
            collector.write_fmt(format_args!(".g{}_", array_index))?;
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
                        collector.write_fmt(format_args!(".g{}_", gd.group))?;
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
                if *size == 1 && values.len() == 1 {
                    emit_expression(collector, &values[0])?;
                } else {
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
            }
            _ => unreachable!(),
        },
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
            let mut group_lhs = false;
            let mut group_rhs = false;
            match &expression.content {
                ExpressionContent::Subtract(_, r) => {
                    group_rhs = match &r.content {
                        ExpressionContent::Add(_, _) => true,
                        ExpressionContent::Subtract(_, _) => true,
                        _ => false,
                    };
                }
                ExpressionContent::Multiply(l, r) => {
                    group_lhs = match &l.content {
                        ExpressionContent::Add(_, _) => true,
                        ExpressionContent::Subtract(_, _) => true,
                        ExpressionContent::Divide(_, _) => true,
                        _ => false,
                    };
                    group_rhs = match &r.content {
                        ExpressionContent::Add(_, _) => true,
                        ExpressionContent::Subtract(_, _) => true,
                        ExpressionContent::Divide(_, _) => true,
                        _ => false,
                    };
                }
                ExpressionContent::Divide(l, r) => {
                    group_lhs = match &l.content {
                        ExpressionContent::Add(_, _) => true,
                        ExpressionContent::Subtract(_, _) => true,
                        ExpressionContent::Divide(_, _) => true,
                        _ => false,
                    };
                    group_rhs = match &r.content {
                        ExpressionContent::Add(_, _) => true,
                        ExpressionContent::Subtract(_, _) => true,
                        ExpressionContent::Multiply(_, _) => true,
                        ExpressionContent::Divide(_, _) => true,
                        _ => false,
                    };
                }
                _ => {}
            }
            if let ExpressionContent::LogicAnd(_, _) = expression.content {
                collector.write_all(b"(")?;
            }
            if group_lhs {
                collector.write_all(b"(")?;
            }
            emit_expression(collector, lhs)?;
            if group_lhs {
                collector.write_all(b")")?;
            }
            collector.write_all(match &expression.content {
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
            if group_rhs {
                collector.write_all(b"(")?;
            }
            emit_expression(collector, rhs)?;
            if group_rhs {
                collector.write_all(b")")?;
            }
            if let ExpressionContent::LogicAnd(_, _) = expression.content {
                collector.write_all(b")")?;
            }
        }
        ExpressionContent::Pow(inner_expression, power) => {
            collector.write_all(b"pow(")?;
            emit_expression(collector, inner_expression)?;
            collector.write_all(b", ")?;
            emit_expression(collector, power)?;
            collector.write_all(b")")?;
        }
        ExpressionContent::Exp(inner_expression) => {
            collector.write_all(b"exp(")?;
            emit_expression(collector, inner_expression)?;
            collector.write_all(b")")?;
        }
        ExpressionContent::Sin(inner_expression) => {
            collector.write_all(b"sin(")?;
            emit_expression(collector, inner_expression)?;
            collector.write_all(b")")?;
        }
        ExpressionContent::Cos(inner_expression) => {
            collector.write_all(b"cos(")?;
            emit_expression(collector, inner_expression)?;
            collector.write_all(b")")?;
        }
        ExpressionContent::Tan(inner_expression) => {
            collector.write_all(b"tan(")?;
            emit_expression(collector, inner_expression)?;
            collector.write_all(b")")?;
        }
        ExpressionContent::Sinh(inner_expression) => {
            collector.write_all(b"sinh(")?;
            emit_expression(collector, inner_expression)?;
            collector.write_all(b")")?;
        }
        ExpressionContent::Cosh(inner_expression) => {
            collector.write_all(b"cosh(")?;
            emit_expression(collector, inner_expression)?;
            collector.write_all(b")")?;
        }
        ExpressionContent::Tanh(inner_expression) => {
            collector.write_all(b"tanh(")?;
            emit_expression(collector, inner_expression)?;
            collector.write_all(b")")?;
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
                collector.write_fmt(format_args!(" g{}_: ", i))?;
                emit_data_type(collector, &DataType::SimdVector(group.len()))?;
                collector.write(b",\n")?;
            }
            emit_indentation(collector, indentation)?;
            collector.write_all(b"}\n\n")?;
        }
        AstNode::ReturnStatement { expression } => {
            collector.write_all(b"return ")?;
            emit_expression(collector, expression)?;
            collector.write_all(b";\n")?;
        }
        AstNode::VariableAssignment { name, data_type, expression } => {
            collector.write_all(b"let ")?;
            let name = if name.to_string() == "self" { "self_" } else { name };
            collector.write_all(name.bytes().collect::<Vec<_>>().as_slice())?;
            if let Some(data_type) = data_type {
                collector.write_all(b": ")?;
                emit_data_type(collector, data_type)?;
            }
            collector.write_all(b" = ")?;
            emit_expression(collector, expression)?;
            collector.write_all(b";\n")?;
        }
        AstNode::IfThenBlock { condition, body } | AstNode::WhileLoopBlock { condition, body } => {
            collector.write_all(match &ast_node {
                AstNode::IfThenBlock { .. } => b"if ",
                AstNode::WhileLoopBlock { .. } => b"while ",
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
            collector.write_all(b"fn ")?;
            match parameters.len() {
                0 => {
                    lower_camel_case(collector, class.class_name.as_str())?;
                    collector.write_all(b"_")?;
                    lower_camel_case(collector, result.name)?;
                }
                1 if result.name == "Into" => {
                    lower_camel_case(collector, &parameters[0].data_type.data_class_name())?;
                    collector.write_all(b"_")?;
                    lower_camel_case(collector, result.name)?;
                    collector.write_all(b"_")?;
                    lower_camel_case(collector, &result.data_type.data_class_name())?;
                }
                1 => {
                    lower_camel_case(collector, &parameters[0].data_type.data_class_name())?;
                    collector.write_all(b"_")?;
                    lower_camel_case(collector, result.name)?;
                }
                2 if !matches!(parameters[1].data_type, DataType::MultiVector(_)) => {
                    lower_camel_case(collector, &parameters[0].data_type.data_class_name())?;
                    collector.write_all(b"_")?;
                    lower_camel_case(collector, result.name)?;
                }
                2 => {
                    lower_camel_case(collector, &parameters[0].data_type.data_class_name())?;
                    collector.write_all(b"_")?;
                    lower_camel_case(collector, result.name)?;
                    collector.write_all(b"_")?;
                    lower_camel_case(collector, &parameters[1].data_type.data_class_name())?;
                }
                _ => unreachable!(),
            }
            collector.write_all(b"(")?;
            for (i, parameter) in parameters.iter().enumerate() {
                if i > 0 {
                    collector.write_all(b", ")?;
                }
                let parameter_name = if parameter.name == "self" { "self_" } else { parameter.name };
                collector.write_fmt(format_args!("{}", parameter_name))?;
                collector.write_all(b": ")?;
                emit_data_type(collector, &parameter.data_type)?;
            }
            collector.write_all(b") -> ")?;
            emit_data_type(collector, &result.data_type)?;
            collector.write_all(b" {\n")?;
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
