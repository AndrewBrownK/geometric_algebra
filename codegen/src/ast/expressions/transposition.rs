

fn transpose_vec2_product(
    float_product_0: &mut Vec<(FloatExpr, f32)>,
    float_product_1: &mut Vec<(FloatExpr, f32)>,
    coalesce_product_literal: [f32; 2]
) -> Option<Vec2Expr> {
    advanced_transpose_vec2_product(false, float_product_0, float_product_1, coalesce_product_literal)
}

fn advanced_transpose_vec2_product(
    aggressive: bool,
    float_product_0: &mut Vec<(FloatExpr, f32)>,
    float_product_1: &mut Vec<(FloatExpr, f32)>,
    mut coalesce_product_literal: [f32; 2]
) -> Option<Vec2Expr> {
    use crate::ast::expressions::FloatExpr::*;
    // See if we can pull out a Vec2Expr::Product
    let mut vec2_product = vec![];
    float_product_0.retain_mut(|(e0, f0)| {
        let mut pulling_out_factor = false;
        float_product_1.retain_mut(|(e1, f1)| {
            if pulling_out_factor {
                return true;
            }
            pulling_out_factor = vec2_product_extract(&mut vec2_product, &mut coalesce_product_literal, aggressive, e0, f0, e1, f1);
            !pulling_out_factor
        });
        !pulling_out_factor
    });

    if vec2_product.is_empty() && coalesce_product_literal == [1.0; 2] {
        return None;
    }
    let mut keep_remaining = false;
    let p0 = if float_product_0.is_empty() {
        Literal(1.0)
    } else {
        keep_remaining = true;
        Product(float_product_0.take_as_owned(), 1.0)
    };
    let p1 = if float_product_1.is_empty() {
        Literal(1.0)
    } else {
        keep_remaining = true;
        Product(float_product_1.take_as_owned(), 1.0)
    };
    if keep_remaining {
        // TODO reattempt but aggressive, if not already
        vec2_product.push((Vec2Expr::Gather2(p0, p1), 1.0));
    }
    let mut result = Vec2Expr::Product(vec2_product, coalesce_product_literal);

    // Since this was a non-trivial transposition of structures,
    // run simplification again on the result.
    result.simplify();
    Some(result)
}

fn vec2_product_extract(
    vec2_product: &mut Vec<(Vec2Expr, f32)>,
    coalesce_product_literals: &mut [f32; 2],
    aggressive: bool,
    e0: &mut FloatExpr,
    f0: &mut f32,
    e1: &mut FloatExpr,
    f1: &mut f32,
) -> bool {
    use crate::ast::expressions::FloatExpr::*;
    let mut pulled_out_literal = false;
    if let Literal(f) = e0 {
        if *f != 1.0 {
            coalesce_product_literals[0] *= f32::powf(*f, *f0);
            *f = 1.0;
            *f0 = 0.0;
            pulled_out_literal = true;
        }
    }
    if let Literal(f) = e1 {
        if *f != 1.0 {
            coalesce_product_literals[1] *= f32::powf(*f, *f1);
            *f = 1.0;
            *f1 = 0.0;
            pulled_out_literal = true;
        }
    }
    if pulled_out_literal {
        return false;
    }
    if e0 == e1 && f0 == f1 {
        vec2_product.push((Vec2Expr::Gather1(e0.clone()), *f0));
        return true;
    }
    return match (e0, e1) {
        (
            AccessVec2(box v0, i0),
            AccessVec2(box v1, i1)
        ) if v0 == v1 && f0 == f1 => {
            // The swizzle will later be simplified, if applicable
            vec2_product.push((Vec2Expr::SwizzleVec2(Box::new(v0.clone()), *i0, *i1), *f0));
            true
        }
        (
            AccessVec3(box v0, i0),
            AccessVec3(box v1, i1)
        ) if v0 == v1 && f0 == f1 => {
            vec2_product.push((Vec2Expr::SwizzleVec2(Box::new(Vec2Expr::Truncate3to2(Box::new(v0.clone()))), *i0, *i1), *f0));
            true
        }
        (
            AccessVec4(box v0, i0),
            AccessVec4(box v1, i1)
        ) if v0 == v1 && f0 == f1 => {
            vec2_product.push((Vec2Expr::SwizzleVec2(Box::new(Vec2Expr::Truncate4to2(Box::new(v0.clone()))), *i0, *i1), *f0));
            true
        }
        (
            Sum(v0, a0),
            Sum(v1, a1)
        ) if f0 == f1 => {
            let a = [*a0, *a1];
            let Some(transposed) = advanced_transpose_vec2_sum(aggressive, v0, v1, a) else { return false };
            vec2_product.push((transposed, *f0));
            true
        }
        _ => false,
    };
}

fn transpose_vec2_sum(
    float_sum_0: &mut Vec<(FloatExpr, f32)>,
    float_sum_1: &mut Vec<(FloatExpr, f32)>,
    coalesce_sum_literal: [f32; 2]
) -> Option<Vec2Expr> {
    advanced_transpose_vec2_sum(false, float_sum_0, float_sum_1, coalesce_sum_literal)
}

fn advanced_transpose_vec2_sum(
    aggressive: bool,
    float_sum_0: &mut Vec<(FloatExpr, f32)>,
    float_sum_1: &mut Vec<(FloatExpr, f32)>,
    mut coalesce_sum_literal: [f32; 2]
) -> Option<Vec2Expr> {
    use crate::ast::expressions::FloatExpr::*;
    // See if we can pull out a Vec2Expr::Sum
    let mut vec2_sum = vec![];
    float_sum_0.retain_mut(|(e0, f0)| {
        let mut pulling_out_addend = false;
        float_sum_1.retain_mut(|(e1, f1)| {
            if pulling_out_addend {
                return true;
            }
            pulling_out_addend = vec2_sum_extract(&mut vec2_sum, &mut coalesce_sum_literal, aggressive, e0, f0, e1, f1);
            !pulling_out_addend
        });
        !pulling_out_addend
    });

    if vec2_sum.is_empty() && coalesce_sum_literal == [0.0; 2] {
        return None;
    }
    let mut keep_remaining = false;
    let p0 = if float_sum_0.is_empty() {
        Literal(0.0)
    } else {
        keep_remaining = true;
        Sum(float_sum_0.take_as_owned(), 0.0)
    };
    let p1 = if float_sum_1.is_empty() {
        Literal(0.0)
    } else {
        keep_remaining = true;
        Sum(float_sum_1.take_as_owned(), 0.0)
    };
    if keep_remaining {
        // TODO reattempt but aggressive, if not already
        vec2_sum.push((Vec2Expr::Gather2(p0, p1), 1.0));
    }
    let mut result = Vec2Expr::Sum(vec2_sum, coalesce_sum_literal);

    // Since this was a non-trivial transposition of structures,
    // run simplification again on the result.
    result.simplify();
    Some(result)
}

fn vec2_sum_extract(
    vec2_sum: &mut Vec<(Vec2Expr, f32)>,
    coalesce_sum_literals: &mut [f32; 2],
    aggressive: bool,
    e0: &mut FloatExpr,
    f0: &mut f32,
    e1: &mut FloatExpr,
    f1: &mut f32
) -> bool {
    use crate::ast::expressions::FloatExpr::*;
    let mut pulled_out_literal = false;
    if let Literal(f) = e0 {
        if *f != 0.0 {
            coalesce_sum_literals[0] += *f * *f0;
            *f = 0.0;
            *f0 = 0.0;
            pulled_out_literal = true;
        }
    }
    if let Literal(f) = e1 {
        if *f != 0.0 {
            coalesce_sum_literals[1] += *f * *f1;
            *f = 0.0;
            *f1 = 0.0;
            pulled_out_literal = true;
        }
    }
    if pulled_out_literal {
        return false;
    }
    if e0 == e1 && f0 == f1 {
        vec2_sum.push((Vec2Expr::Gather1(e0.clone()), *f0));
        return true;
    }
    return match (e0, e1) {
        (
            AccessVec2(box v0, i0),
            AccessVec2(box v1, i1)
        ) if v0 == v1 && f0 == f1 => {
            // The swizzle will later be simplified, if applicable
            vec2_sum.push((Vec2Expr::SwizzleVec2(Box::new(v0.clone()), *i0, *i1), *f0));
            true
        }
        (
            AccessVec3(box v0, i0),
            AccessVec3(box v1, i1)
        ) if v0 == v1 && f0 == f1 => {
            vec2_sum.push((Vec2Expr::SwizzleVec2(Box::new(Vec2Expr::Truncate3to2(Box::new(v0.clone()))), *i0, *i1), *f0));
            true
        }
        (
            AccessVec4(box v0, i0),
            AccessVec4(box v1, i1)
        ) if v0 == v1 && f0 == f1 => {
            vec2_sum.push((Vec2Expr::SwizzleVec2(Box::new(Vec2Expr::Truncate4to2(Box::new(v0.clone()))), *i0, *i1), *f0));
            true
        }
        (
            Product(v0, a0),
            Product(v1, a1)
        ) if f0 == f1 => {
            let a = [*a0, *a1];
            let Some(transposed) = advanced_transpose_vec2_product(aggressive, v0, v1, a) else { return false };
            vec2_sum.push((transposed, *f0));
            true
        }
        _ => false,
    };
}



fn transpose_vec3_product(
    float_product_0: &mut Vec<(FloatExpr, f32)>,
    float_product_1: &mut Vec<(FloatExpr, f32)>,
    float_product_2: &mut Vec<(FloatExpr, f32)>,
    coalesce_product_literal: [f32; 3],
) -> Option<Vec3Expr> {
    advanced_transpose_vec3_product(false, float_product_0, float_product_1, float_product_2, coalesce_product_literal)
}

fn advanced_transpose_vec3_product(
    aggressive: bool,
    float_product_0: &mut Vec<(FloatExpr, f32)>,
    float_product_1: &mut Vec<(FloatExpr, f32)>,
    float_product_2: &mut Vec<(FloatExpr, f32)>,
    mut coalesce_product_literal: [f32; 3],
) -> Option<Vec3Expr> {
    use crate::ast::expressions::FloatExpr::*;
    // See if we can pull out a Vec3Expr::Product
    let mut vec3_product = vec![];
    float_product_0.retain_mut(|(e0, f0)| {
        let mut pulling_out_factor = false;
        float_product_1.retain_mut(|(e1, f1)| {
            if pulling_out_factor {
                return true;
            }
            float_product_2.retain_mut(|(e2, f2)| {
                if pulling_out_factor {
                    return true;
                }
                pulling_out_factor = vec3_product_extract(&mut vec3_product, &mut coalesce_product_literal, aggressive, e0, f0, e1, f1, e2, f2);
                !pulling_out_factor
            });
            !pulling_out_factor
        });
        !pulling_out_factor
    });

    if vec3_product.is_empty() && coalesce_product_literal == [1.0; 3] {
        return None;
    }
    let mut keep_remaining = false;
    let p0 = if float_product_0.is_empty() {
        Literal(1.0)
    } else {
        keep_remaining = true;
        Product(float_product_0.take_as_owned(), 1.0)
    };
    let p1 = if float_product_1.is_empty() {
        Literal(1.0)
    } else {
        keep_remaining = true;
        Product(float_product_1.take_as_owned(), 1.0)
    };
    let p2 = if float_product_2.is_empty() {
        Literal(1.0)
    } else {
        keep_remaining = true;
        Product(float_product_2.take_as_owned(), 1.0)
    };
    if keep_remaining {
        // TODO reattempt but aggressive, if not already
        vec3_product.push((Vec3Expr::Gather3(p0, p1, p2), 1.0));
    }
    let mut result = Vec3Expr::Product(vec3_product, coalesce_product_literal);

    // Since this was a non-trivial transposition of structures,
    // run simplification again on the result.
    result.simplify();
    Some(result)
}

fn vec3_product_extract(
    vec3_product: &mut Vec<(Vec3Expr, f32)>,
    coalesce_product_literals: &mut [f32; 3],
    aggressive: bool,
    e0: &mut FloatExpr,
    f0: &mut f32,
    e1: &mut FloatExpr,
    f1: &mut f32,
    e2: &mut FloatExpr,
    f2: &mut f32,
) -> bool {
    use crate::ast::expressions::FloatExpr::*;
    let mut pulled_out_literal = false;
    if let Literal(f) = e0 {
        if *f != 1.0 {
            coalesce_product_literals[0] *= f32::powf(*f, *f0);
            *f = 1.0;
            *f0 = 0.0;
            pulled_out_literal = true;
        }
    }
    if let Literal(f) = e1 {
        if *f != 1.0 {
            coalesce_product_literals[1] *= f32::powf(*f, *f1);
            *f = 1.0;
            *f1 = 0.0;
            pulled_out_literal = true;
        }
    }
    if let Literal(f) = e2 {
        if *f != 1.0 {
            coalesce_product_literals[2] *= f32::powf(*f, *f2);
            *f = 1.0;
            *f2 = 0.0;
            pulled_out_literal = true;
        }
    }
    if pulled_out_literal {
        return false;
    }
    if e0 == e1 && e1 == e2 && f0 == f1 && f1 == f2 {
        vec3_product.push((Vec3Expr::Gather1(e0.clone()), *f0));
        return true;
    }
    return match (e0, e1, e2) {
        (
            AccessVec3(box v0, i0),
            AccessVec3(box v1, i1),
            AccessVec3(box v2, i2)
        ) if v0 == v1 && v1 == v2 && f0 == f1 && f1 == f2 => {
            // The swizzle will later be simplified, if applicable
            vec3_product.push((Vec3Expr::SwizzleVec3(Box::new(v0.clone()), *i0, *i1, *i2), *f0));
            true
        }
        (
            AccessVec2(box v0, i0),
            AccessVec2(box v1, i1),
            z
        ) if v0 == v1 && f0 == f1 && f1 == f2 => {
            vec3_product.push((Vec3Expr::Extend2to3(Vec2Expr::SwizzleVec2(Box::new(v0.clone()), *i0, *i1), z.clone()), *f0));
            true
        }
        (
            AccessVec4(box v0, i0),
            AccessVec4(box v1, i1),
            AccessVec4(box v2, i2)
        ) if v0 == v1 && v1 == v2 && f0 == f1 && f1 == f2 => {
            vec3_product.push((Vec3Expr::SwizzleVec3(Box::new(Vec3Expr::Truncate4to3(Box::new(v0.clone()))), *i0, *i1, *i2), *f0));
            true
        }
        (
            Sum(v0, a0),
            Sum(v1, a1),
            Sum(v2, a2)
        ) if f0 == f1 && f1 == f2 => {
            let a = [*a0, *a1, *a2];
            let Some(transposed) = advanced_transpose_vec3_sum(aggressive, v0, v1, v2, a) else { return false };
            vec3_product.push((transposed, *f0));
            true
        }
        (
            Sum(v0, a0),
            Sum(v1, a1),
            z,
        ) if f0 == f1 && f1 == f2 => {
            let a = [*a0, *a1];
            let Some(transposed) = advanced_transpose_vec2_sum(aggressive, v0, v1, a) else { return false };
            vec3_product.push((Vec3Expr::Extend2to3(transposed, z.clone()), *f0));
            true
        }
        _ => false,
    };
}

fn transpose_vec3_sum(
    float_sum_0: &mut Vec<(FloatExpr, f32)>,
    float_sum_1: &mut Vec<(FloatExpr, f32)>,
    float_sum_2: &mut Vec<(FloatExpr, f32)>,
    coalesce_sum_literal: [f32; 3],
) -> Option<Vec3Expr> {
    advanced_transpose_vec3_sum(false, float_sum_0, float_sum_1, float_sum_2, coalesce_sum_literal)
}

fn advanced_transpose_vec3_sum(
    aggressive: bool,
    float_sum_0: &mut Vec<(FloatExpr, f32)>,
    float_sum_1: &mut Vec<(FloatExpr, f32)>,
    float_sum_2: &mut Vec<(FloatExpr, f32)>,
    mut coalesce_sum_literal: [f32; 3],
) -> Option<Vec3Expr> {
    use crate::ast::expressions::FloatExpr::*;
    // See if we can pull out a Vec3Expr::Sum
    let mut vec3_sum = vec![];
    float_sum_0.retain_mut(|(e0, f0)| {
        let mut pulling_out_addend = false;
        float_sum_1.retain_mut(|(e1, f1)| {
            if pulling_out_addend {
                return true;
            }
            float_sum_2.retain_mut(|(e2, f2)| {
                if pulling_out_addend {
                    return true;
                }
                pulling_out_addend = vec3_sum_extract(&mut vec3_sum, &mut coalesce_sum_literal, aggressive, e0, f0, e1, f1, e2, f2);
                !pulling_out_addend
            });
            !pulling_out_addend
        });
        !pulling_out_addend
    });

    if vec3_sum.is_empty() && coalesce_sum_literal == [0.0; 3] {
        return None;
    }
    let mut keep_remaining = false;
    let p0 = if float_sum_0.is_empty() {
        Literal(0.0)
    } else {
        keep_remaining = true;
        Sum(float_sum_0.take_as_owned(), 0.0)
    };
    let p1 = if float_sum_1.is_empty() {
        Literal(0.0)
    } else {
        keep_remaining = true;
        Sum(float_sum_1.take_as_owned(), 0.0)
    };
    let p2 = if float_sum_2.is_empty() {
        Literal(0.0)
    } else {
        keep_remaining = true;
        Sum(float_sum_2.take_as_owned(), 0.0)
    };
    if keep_remaining {
        // TODO reattempt but aggressive, if not already
        vec3_sum.push((Vec3Expr::Gather3(p0, p1, p2), 1.0));
    }
    let mut result = Vec3Expr::Sum(vec3_sum, coalesce_sum_literal);

    // Since this was a non-trivial transposition of structures,
    // run simplification again on the result.
    result.simplify();
    Some(result)
}

fn vec3_sum_extract(
    vec3_sum: &mut Vec<(Vec3Expr, f32)>,
    coalesce_sum_literals: &mut [f32; 3],
    aggressive: bool,
    e0: &mut FloatExpr,
    f0: &mut f32,
    e1: &mut FloatExpr,
    f1: &mut f32,
    e2: &mut FloatExpr,
    f2: &mut f32,
) -> bool {
    use crate::ast::expressions::FloatExpr::*;
    let mut pulled_out_literal = false;
    if let Literal(f) = e0 {
        if *f != 0.0 {
            coalesce_sum_literals[0] += *f * *f0;
            *f = 0.0;
            *f0 = 0.0;
            pulled_out_literal = true;
        }
    }
    if let Literal(f) = e1 {
        if *f != 0.0 {
            coalesce_sum_literals[1] += *f * *f1;
            *f = 0.0;
            *f1 = 0.0;
            pulled_out_literal = true;
        }
    }
    if let Literal(f) = e2 {
        if *f != 0.0 {
            coalesce_sum_literals[2] += *f * *f2;
            *f = 0.0;
            *f2 = 0.0;
            pulled_out_literal = true;
        }
    }
    if pulled_out_literal {
        return false;
    }
    if e0 == e1 && e1 == e2 && f0 == f1 && f1 == f2 {
        vec3_sum.push((Vec3Expr::Gather1(e0.clone()), *f0));
        return true;
    }
    return match (e0, e1, e2) {
        (
            AccessVec3(box v0, i0),
            AccessVec3(box v1, i1),
            AccessVec3(box v2, i2)
        ) if v0 == v1 && v1 == v2 && f0 == f1 && f1 == f2 => {
            // The swizzle will later be simplified, if applicable
            vec3_sum.push((Vec3Expr::SwizzleVec3(Box::new(v0.clone()), *i0, *i1, *i2), *f0));
            true
        }
        (
            AccessVec2(box v0, i0),
            AccessVec2(box v1, i1),
            z
        ) if v0 == v1 && f0 == f1 && f1 == f2 => {
            vec3_sum.push((Vec3Expr::Extend2to3(Vec2Expr::SwizzleVec2(Box::new(v0.clone()), *i0, *i1), z.clone()), *f0));
            true
        }
        (
            AccessVec4(box v0, i0),
            AccessVec4(box v1, i1),
            AccessVec4(box v2, i2)
        ) if v0 == v1 && v1 == v2 && f0 == f1 && f1 == f2 => {
            vec3_sum.push((Vec3Expr::SwizzleVec3(Box::new(Vec3Expr::Truncate4to3(Box::new(v0.clone()))), *i0, *i1, *i2), *f0));
            true
        }
        (
            Product(v0, a0),
            Product(v1, a1),
            Product(v2, a2)
        ) if f0 == f1 && f1 == f2 => {
            let a = [*a0, *a1, *a2];
            let Some(transposed) = advanced_transpose_vec3_product(aggressive, v0, v1, v2, a) else { return false };
            vec3_sum.push((transposed, *f0));
            true
        }
        (
            Product(v0, a0),
            Product(v1, a1),
            z,
        ) if f0 == f1 && f1 == f2 => {
            let a = [*a0, *a1];
            let Some(transposed) = advanced_transpose_vec2_product(aggressive, v0, v1, a) else { return false };
            vec3_sum.push((Vec3Expr::Extend2to3(transposed, z.clone()), *f0));
            true
        }
        _ => false,
    };
}


fn transpose_vec4_product(
    float_product_0: &mut Vec<(FloatExpr, f32)>,
    float_product_1: &mut Vec<(FloatExpr, f32)>,
    float_product_2: &mut Vec<(FloatExpr, f32)>,
    float_product_3: &mut Vec<(FloatExpr, f32)>,
    coalesce_product_literal: [f32; 4],
) -> Option<Vec4Expr> {
    advanced_transpose_vec4_product(false, float_product_0, float_product_1, float_product_2, float_product_3, coalesce_product_literal)
}

fn advanced_transpose_vec4_product(
    aggressive: bool,
    float_product_0: &mut Vec<(FloatExpr, f32)>,
    float_product_1: &mut Vec<(FloatExpr, f32)>,
    float_product_2: &mut Vec<(FloatExpr, f32)>,
    float_product_3: &mut Vec<(FloatExpr, f32)>,
    mut coalesce_product_literal: [f32; 4],
) -> Option<Vec4Expr> {
    use crate::ast::expressions::FloatExpr::*;
    // See if we can pull out a Vec4Expr::Product
    let mut vec4_product = vec![];
    float_product_0.retain_mut(|(e0, f0)| {
        let mut pulling_out_factor = false;
        float_product_1.retain_mut(|(e1, f1)| {
            if pulling_out_factor {
                return true;
            }
            float_product_2.retain_mut(|(e2, f2)| {
                if pulling_out_factor {
                    return true;
                }
                float_product_3.retain_mut(|(e3, f3)| {
                    if pulling_out_factor {
                        return true;
                    }
                    pulling_out_factor = vec4_product_extract(&mut vec4_product, &mut coalesce_product_literal, aggressive, e0, f0, e1, f1, e2, f2, e3, f3);
                    !pulling_out_factor
                });
                !pulling_out_factor
            });
            !pulling_out_factor
        });
        !pulling_out_factor
    });

    if vec4_product.is_empty() && coalesce_product_literal == [1.0; 4] {
        return None;
    }
    let mut keep_remaining = false;
    let p0 = if float_product_0.is_empty() {
        Literal(1.0)
    } else {
        keep_remaining = true;
        Product(float_product_0.take_as_owned(), 1.0)
    };
    let p1 = if float_product_1.is_empty() {
        Literal(1.0)
    } else {
        keep_remaining = true;
        Product(float_product_1.take_as_owned(), 1.0)
    };
    let p2 = if float_product_2.is_empty() {
        Literal(1.0)
    } else {
        keep_remaining = true;
        Product(float_product_2.take_as_owned(), 1.0)
    };
    let p3 = if float_product_3.is_empty() {
        Literal(1.0)
    } else {
        keep_remaining = true;
        Product(float_product_3.take_as_owned(), 1.0)
    };
    if keep_remaining {
        // TODO reattempt but aggressive, if not already
        vec4_product.push((Vec4Expr::Gather4(p0, p1, p2, p3), 1.0));
    }
    let mut result = Vec4Expr::Product(vec4_product, coalesce_product_literal);

    // Since this was a non-trivial transposition of structures,
    // run simplification again on the result.
    result.simplify();
    Some(result)
}

fn vec4_product_extract(
    vec4_product: &mut Vec<(Vec4Expr, f32)>,
    coalesce_product_literals: &mut [f32; 4],
    aggressive: bool,
    e0: &mut FloatExpr,
    f0: &mut f32,
    e1: &mut FloatExpr,
    f1: &mut f32,
    e2: &mut FloatExpr,
    f2: &mut f32,
    e3: &mut FloatExpr,
    f3: &mut f32,
) -> bool {
    use crate::ast::expressions::FloatExpr::*;
    let mut pulled_out_literal = false;
    if let Literal(f) = e0 {
        if *f != 1.0 {
            coalesce_product_literals[0] *= f32::powf(*f, *f0);
            *f = 1.0;
            *f0 = 0.0;
            pulled_out_literal = true;
        }
    }
    if let Literal(f) = e1 {
        if *f != 1.0 {
            coalesce_product_literals[1] *= f32::powf(*f, *f1);
            *f = 1.0;
            *f1 = 0.0;
            pulled_out_literal = true;
        }
    }
    if let Literal(f) = e2 {
        if *f != 1.0 {
            coalesce_product_literals[2] *= f32::powf(*f, *f2);
            *f = 1.0;
            *f2 = 0.0;
            pulled_out_literal = true;
        }
    }
    if let Literal(f) = e3 {
        if *f != 1.0 {
            coalesce_product_literals[3] *= f32::powf(*f, *f3);
            *f = 1.0;
            *f3 = 0.0;
            pulled_out_literal = true;
        }
    }
    if pulled_out_literal {
        return false;
    }
    if e0 == e1 && e1 == e2 && e2 == e3 && f0 == f1 && f1 == f2 && f2 == f3 {
        vec4_product.push((Vec4Expr::Gather1(e0.clone()), *f0));
        return true;
    }
    return match (e0, e1, e2, e3) {
        (
            AccessVec4(box v0, i0),
            AccessVec4(box v1, i1),
            AccessVec4(box v2, i2),
            AccessVec4(box v3, i3)
        ) if v0 == v1 && v1 == v2 && v2 == v3 && f0 == f1 && f1 == f2 && f2 == f3 =>
            {
                // The swizzle will later be simplified, if applicable
                vec4_product.push((Vec4Expr::SwizzleVec4(Box::new(v0.clone()), *i0, *i1, *i2, *i3), *f0));
                true
            }
        (
            AccessVec3(box v0, i0),
            AccessVec3(box v1, i1),
            AccessVec3(box v2, i2),
            w
        ) if v0 == v1 && v1 == v2 && f0 == f1 && f1 == f2 && f2 == f3 => {
            vec4_product.push((Vec4Expr::Extend3to4(Vec3Expr::SwizzleVec3(Box::new(v0.clone()), *i0, *i1, *i2), w.clone()), *f0));
            true
        }
        (
            AccessVec2(box v0, i0),
            AccessVec2(box v1, i1),
            z,
            w
        ) if v0 == v1 && f0 == f1 && f1 == f2 && f2 == f3 => {
            vec4_product.push((Vec4Expr::Extend2to4(Vec2Expr::SwizzleVec2(Box::new(v0.clone()), *i0, *i1), z.clone(), w.clone()), *f0));
            true
        }
        (
            Sum(v0, a0),
            Sum(v1, a1),
            Sum(v2, a2),
            Sum(v3, a3)
        ) if f0 == f1 && f1 == f2 && f2 == f3 => {
            let a = [*a0, *a1, *a2, *a3];
            let Some(transposed) = advanced_transpose_vec4_sum(aggressive, v0, v1, v2, v3, a) else { return false };
            vec4_product.push((transposed, *f0));
            true
        }
        (
            Sum(v0, a0),
            Sum(v1, a1),
            Sum(v2, a2),
            w
        ) if f0 == f1 && f1 == f2 && f2 == f3 => {
            let a = [*a0, *a1, *a2];
            let Some(transposed) = advanced_transpose_vec3_sum(aggressive, v0, v1, v2, a) else { return false };
            vec4_product.push((Vec4Expr::Extend3to4(transposed, w.clone()), *f0));
            true
        }
        (
            Sum(v0, a0),
            Sum(v1, a1),
            z,
            w
        ) if f0 == f1 && f1 == f2 && f2 == f3 => {
            let a = [*a0, *a1];
            let Some(transposed) = advanced_transpose_vec2_sum(aggressive, v0, v1, a) else { return false };
            vec4_product.push((Vec4Expr::Extend2to4(transposed, z.clone(), w.clone()), *f0));
            true
        }
        _ => false,
    };
}

fn transpose_vec4_sum(
    float_sum_0: &mut Vec<(FloatExpr, f32)>,
    float_sum_1: &mut Vec<(FloatExpr, f32)>,
    float_sum_2: &mut Vec<(FloatExpr, f32)>,
    float_sum_3: &mut Vec<(FloatExpr, f32)>,
    coalesce_sum_literal: [f32; 4],
) -> Option<Vec4Expr> {
    advanced_transpose_vec4_sum(false, float_sum_0, float_sum_1, float_sum_2, float_sum_3, coalesce_sum_literal)
}

fn advanced_transpose_vec4_sum(
    aggressive: bool,
    float_sum_0: &mut Vec<(FloatExpr, f32)>,
    float_sum_1: &mut Vec<(FloatExpr, f32)>,
    float_sum_2: &mut Vec<(FloatExpr, f32)>,
    float_sum_3: &mut Vec<(FloatExpr, f32)>,
    mut coalesce_sum_literal: [f32; 4],
) -> Option<Vec4Expr> {
    use crate::ast::expressions::FloatExpr::*;
    // See if we can pull out a Vec4Expr::Sum
    let mut vec4_sum = vec![];
    float_sum_0.retain_mut(|(e0, f0)| {
        let mut pulling_out_addend = false;
        float_sum_1.retain_mut(|(e1, f1)| {
            if pulling_out_addend {
                return true;
            }
            float_sum_2.retain_mut(|(e2, f2)| {
                if pulling_out_addend {
                    return true;
                }
                float_sum_3.retain_mut(|(e3, f3)| {
                    if pulling_out_addend {
                        return true;
                    }
                    pulling_out_addend = vec4_sum_extract(&mut vec4_sum, &mut coalesce_sum_literal, aggressive, e0, f0, e1, f1, e2, f2, e3, f3);
                    !pulling_out_addend
                });
                !pulling_out_addend
            });
            !pulling_out_addend
        });
        !pulling_out_addend
    });

    if vec4_sum.is_empty() && coalesce_sum_literal == [0.0; 4] {
        return None;
    }
    let mut keep_remaining = false;
    let mut p0 = if float_sum_0.is_empty() {
        Literal(0.0)
    } else {
        keep_remaining = true;
        Sum(float_sum_0.take_as_owned(), 0.0)
    };
    let mut p1 = if float_sum_1.is_empty() {
        Literal(0.0)
    } else {
        keep_remaining = true;
        Sum(float_sum_1.take_as_owned(), 0.0)
    };
    let mut p2 = if float_sum_2.is_empty() {
        Literal(0.0)
    } else {
        keep_remaining = true;
        Sum(float_sum_2.take_as_owned(), 0.0)
    };
    let mut p3 = if float_sum_3.is_empty() {
        Literal(0.0)
    } else {
        keep_remaining = true;
        Sum(float_sum_3.take_as_owned(), 0.0)
    };
    if keep_remaining {
        // if !aggressive {
        //     let mut f0 = 1.0;
        //     let mut f1 = 1.0;
        //     let mut f2 = 1.0;
        //     let mut f3 = 1.0;
        //     // TODO I don't feel like this is right, I feel like I should be calling advanced_transpose, but that has all vec arguments
        //     vec4_sum_extract(&mut vec4_sum, &mut coalesce_sum_literal, true, &mut p0, &mut f0, &mut p1, &mut f1, &mut p2, &mut f2, &mut p3, &mut f3);
        //
        // }
        // TODO reattempt but aggressive, if not already
        vec4_sum.push((Vec4Expr::Gather4(p0, p1, p2, p3), 1.0));
    }
    let mut result = Vec4Expr::Sum(vec4_sum, coalesce_sum_literal);

    // Since this was a non-trivial transposition of structures,
    // run simplification again on the result.
    result.simplify();
    Some(result)
}

fn vec4_sum_extract(
    vec4_sum: &mut Vec<(Vec4Expr, f32)>,
    coalesce_sum_literals: &mut [f32; 4],
    aggressive: bool,
    e0: &mut FloatExpr,
    f0: &mut f32,
    e1: &mut FloatExpr,
    f1: &mut f32,
    e2: &mut FloatExpr,
    f2: &mut f32,
    e3: &mut FloatExpr,
    f3: &mut f32,
) -> bool {
    use crate::ast::expressions::FloatExpr::*;
    let mut pulled_out_literal = false;
    if let Literal(f) = e0 {
        if *f != 0.0 {
            coalesce_sum_literals[0] += *f * *f0;
            *f = 0.0;
            *f0 = 0.0;
            pulled_out_literal = true;
        }
    }
    if let Literal(f) = e1 {
        if *f != 0.0 {
            coalesce_sum_literals[1] += *f * *f1;
            *f = 0.0;
            *f1 = 0.0;
            pulled_out_literal = true;
        }
    }
    if let Literal(f) = e2 {
        if *f != 0.0 {
            coalesce_sum_literals[2] += *f * *f2;
            *f = 0.0;
            *f2 = 0.0;
            pulled_out_literal = true;
        }
    }
    if let Literal(f) = e3 {
        if *f != 0.0 {
            coalesce_sum_literals[3] += *f * *f3;
            *f = 0.0;
            *f3 = 0.0;
            pulled_out_literal = true;
        }
    }
    if pulled_out_literal {
        return false;
    }
    if e0 == e1 && e1 == e2 && e2 == e3 && f0 == f1 && f1 == f2 && f2 == f3 {
        vec4_sum.push((Vec4Expr::Gather1(e0.clone()), *f0));
        return true;
    }
    return match (e0, e1, e2, e3) {
        (
            AccessVec4(box v0, i0),
            AccessVec4(box v1, i1),
            AccessVec4(box v2, i2),
            AccessVec4(box v3, i3)
        ) if v0 == v1 && v1 == v2 && v2 == v3 && f0 == f1 && f1 == f2 && f2 == f3 => {
            // The swizzle will later be simplified, if applicable
            vec4_sum.push((Vec4Expr::SwizzleVec4(Box::new(v0.clone()), *i0, *i1, *i2, *i3), *f0));
            true
        }
        (
            AccessVec3(box v0, i0),
            AccessVec3(box v1, i1),
            AccessVec3(box v2, i2),
            w
        ) if v0 == v1 && v1 == v2 && f0 == f1 && f1 == f2 && f2 == f3 => {
            vec4_sum.push((Vec4Expr::Extend3to4(Vec3Expr::SwizzleVec3(Box::new(v0.clone()), *i0, *i1, *i2), w.clone()), *f0));
            true
        }
        (
            AccessVec2(box v0, i0),
            AccessVec2(box v1, i1),
            z,
            w
        ) if v0 == v1 && f0 == f1 && f1 == f2 && f2 == f3 => {
            vec4_sum.push((Vec4Expr::Extend2to4(Vec2Expr::SwizzleVec2(Box::new(v0.clone()), *i0, *i1), z.clone(), w.clone()), *f0));
            true
        }
        (
            Product(v0, a0),
            Product(v1, a1),
            Product(v2, a2),
            Product(v3, a3)
        ) if f0 == f1 && f1 == f2 && f2 == f3 => {
            let a = [*a0, *a1, *a2, *a3];
            let Some(transposed) = advanced_transpose_vec4_product(aggressive, v0, v1, v2, v3, a) else { return false };
            vec4_sum.push((transposed, *f0));
            true
        }
        (
            Product(v0, a0),
            Product(v1, a1),
            Product(v2, a2),
            w
        ) if f0 == f1 && f1 == f2 && f2 == f3 => {
            let a = [*a0, *a1, *a2];
            let Some(transposed) = advanced_transpose_vec3_product(aggressive, v0, v1, v2, a) else { return false };
            vec4_sum.push((Vec4Expr::Extend3to4(transposed, w.clone()), *f0));
            true
        }
        (
            Product(v0, a0),
            Product(v1, a1),
            z,
            w
        ) if f0 == f1 && f1 == f2 && f2 == f3 => {
            let a = [*a0, *a1];
            let Some(transposed) = advanced_transpose_vec2_product(aggressive, v0, v1, a) else { return false };
            vec4_sum.push((Vec4Expr::Extend2to4(transposed, z.clone(), w.clone()), *f0));
            true
        }
        _ => false,
    };
}

